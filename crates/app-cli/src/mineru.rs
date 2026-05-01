use std::{
    error::Error,
    fmt, fs,
    fs::File,
    io,
    path::{Component, Path, PathBuf},
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use reqwest::blocking::Client;
use serde::Serialize;
use serde_json::Value;
use zip::ZipArchive;

use crate::{
    config::{Config, ConfigError},
    paths::{LessonPathError, LessonPaths, RepoPaths},
};

const STAGING_ATTEMPTS: usize = 16;

pub fn convert_lesson(
    repo: &RepoPaths,
    lesson: &LessonPaths,
    resume: bool,
) -> Result<(), ConvertError> {
    if resume && lesson.plain_text_path().is_file() {
        return Ok(());
    }

    let raw_pdf_path = lesson.checked_raw_pdf_path()?;
    let config = Config::load(repo)?;
    let token = config.mineru.read_token()?;
    let stage = PreparedOutput::prepare(lesson.plain_output_dir(), lesson.lesson_id())?;

    let result = run_convert(&config, &token, &raw_pdf_path, &stage, lesson.lesson_id());
    match result {
        Ok(()) => Ok(()),
        Err(error) => Err(stage.cleanup(error)),
    }
}

pub fn convert_exam_pdfs(repo: &RepoPaths, resume: bool) -> Result<usize, ConvertError> {
    let raw_pdf_paths = list_exam_pdfs(repo)?;
    if raw_pdf_paths.is_empty() {
        return Ok(0);
    }

    let config = Config::load(repo)?;
    let token = config.mineru.read_token()?;
    let mut converted = 0;

    for raw_pdf_path in raw_pdf_paths {
        let stem = pdf_stem(&raw_pdf_path)?;
        let plain_text_path = repo.exam_plain_text_path(&stem);
        if resume && plain_text_path.is_file() {
            continue;
        }

        let output_dir = repo.exam_plain_output_dir(&stem);
        let data_id = format!("exam-{stem}");
        let stage = PreparedOutput::prepare(output_dir, &data_id)?;
        let result = run_convert(&config, &token, &raw_pdf_path, &stage, &data_id);
        match result {
            Ok(()) => converted += 1,
            Err(error) => return Err(stage.cleanup(error)),
        }
    }

    Ok(converted)
}

fn run_convert(
    config: &Config,
    token: &str,
    raw_pdf_path: &Path,
    stage: &PreparedOutput,
    data_id: &str,
) -> Result<(), ConvertError> {
    let upload = request_upload(config, token, raw_pdf_path, data_id)?;
    upload_file(config, &upload.upload_url, raw_pdf_path)?;
    let completed = poll_until_done(config, token, &upload.batch_id)?;
    download_archive(
        config,
        &completed.batch_id,
        &completed.full_zip_url,
        &stage.archive_path,
    )?;
    extract_archive(&stage.archive_path, &stage.artifacts_dir)?;
    let plain_text_bytes = materialize_plain_text(&stage.artifacts_dir, &stage.plain_text_path)?;
    write_job_metadata(
        stage,
        raw_pdf_path,
        &completed.batch_id,
        &completed.result,
        config,
    )?;

    if config.mineru.fail_on_empty_text && plain_text_bytes == 0 {
        return Err(ConvertError::EmptyPlainText {
            path: stage.plain_text_path.clone(),
        });
    }

    fs::rename(&stage.staging_dir, &stage.final_output_dir).map_err(|source| {
        ConvertError::FinalizeOutput {
            staging_dir: stage.staging_dir.clone(),
            output_dir: stage.final_output_dir.clone(),
            source,
        }
    })
}

struct PreparedOutput {
    final_output_dir: PathBuf,
    staging_dir: PathBuf,
    archive_path: PathBuf,
    artifacts_dir: PathBuf,
    plain_text_path: PathBuf,
    job_path: PathBuf,
}

impl PreparedOutput {
    fn prepare(final_output_dir: PathBuf, staging_prefix: &str) -> Result<Self, ConvertError> {
        match final_output_dir.try_exists() {
            Ok(true) => {
                return Err(ConvertError::OutputAlreadyExists {
                    path: final_output_dir,
                })
            }
            Ok(false) => {}
            Err(source) => {
                return Err(ConvertError::OutputDirectoryCheck {
                    path: final_output_dir,
                    source,
                })
            }
        }

        let parent = final_output_dir
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_else(|| final_output_dir.clone());
        fs::create_dir_all(&parent).map_err(|source| ConvertError::StagingCreate {
            path: parent.clone(),
            source,
        })?;

        let staging_dir = create_staging_dir(&parent, staging_prefix)?;
        Ok(Self {
            final_output_dir,
            archive_path: staging_dir.join("result.zip"),
            artifacts_dir: staging_dir.join("artifacts"),
            plain_text_path: staging_dir.join("plain.txt"),
            job_path: staging_dir.join("job.json"),
            staging_dir,
        })
    }

    fn cleanup(&self, error: ConvertError) -> ConvertError {
        match fs::remove_dir_all(&self.staging_dir) {
            Ok(()) => error,
            Err(source) if source.kind() == io::ErrorKind::NotFound => error,
            Err(source) => ConvertError::CleanupDuringFailure {
                error: Box::new(error),
                cleanup_path: self.staging_dir.clone(),
                cleanup_source: source,
            },
        }
    }
}

struct CreateUploadResponse {
    batch_id: String,
    upload_url: String,
}

struct CompletedBatch {
    batch_id: String,
    result: Value,
    full_zip_url: String,
}

fn request_upload(
    config: &Config,
    token: &str,
    raw_pdf_path: &Path,
    data_id: &str,
) -> Result<CreateUploadResponse, ConvertError> {
    let client = Client::builder()
        .timeout(Duration::from_secs(config.mineru.request_timeout_seconds))
        .build()
        .map_err(|source| ConvertError::CreateUploadRequest { source })?;
    let url = format!(
        "{}/api/v4/file-urls/batch",
        config.mineru.api_base_url.trim_end_matches('/')
    );
    let payload = CreateUploadPayload {
        enable_formula: config.mineru.enable_formula,
        language: &config.mineru.language,
        enable_table: config.mineru.enable_table,
        model_version: &config.mineru.model_version,
        files: vec![CreateUploadFile {
            name: raw_pdf_path
                .file_name()
                .and_then(|value| value.to_str())
                .unwrap_or("input.pdf"),
            is_ocr: config.mineru.is_ocr,
            data_id,
        }],
    };
    let response = client
        .post(url)
        .bearer_auth(token)
        .json(&payload)
        .send()
        .and_then(|response| response.error_for_status())
        .map_err(|source| ConvertError::CreateUploadRequest { source })?;
    let body: Value = response
        .json()
        .map_err(|source| ConvertError::CreateUploadRequest { source })?;
    let data = api_data(body, ApiOperation::CreateUpload)?;
    let batch_id = required_string(&data, "batch_id")
        .map_err(|field| ConvertError::CreateUploadField { field })?;
    let upload_url = first_upload_url(&data).ok_or(ConvertError::CreateUploadField {
        field: "data.file_urls[0]",
    })?;

    Ok(CreateUploadResponse {
        batch_id,
        upload_url,
    })
}

fn upload_file(config: &Config, upload_url: &str, raw_pdf_path: &Path) -> Result<(), ConvertError> {
    let client = Client::builder()
        .timeout(Duration::from_secs(config.mineru.upload_timeout_seconds))
        .build()
        .map_err(|source| ConvertError::Upload { source })?;
    let file = File::open(raw_pdf_path).map_err(|source| ConvertError::UploadWrite {
        path: raw_pdf_path.to_path_buf(),
        source,
    })?;

    client
        .put(upload_url)
        .body(file)
        .send()
        .and_then(|response| response.error_for_status())
        .map_err(|source| ConvertError::Upload { source })?;

    Ok(())
}

fn poll_until_done(
    config: &Config,
    token: &str,
    batch_id: &str,
) -> Result<CompletedBatch, ConvertError> {
    let batch_id_string = batch_id.to_owned();
    let client = Client::builder()
        .timeout(Duration::from_secs(config.mineru.request_timeout_seconds))
        .build()
        .map_err(|source| ConvertError::PollRequest {
            batch_id: batch_id_string.clone(),
            source,
        })?;
    let url = format!(
        "{}/api/v4/extract-results/batch/{}",
        config.mineru.api_base_url.trim_end_matches('/'),
        batch_id
    );
    let deadline = Instant::now() + Duration::from_secs(config.mineru.result_timeout_seconds);

    loop {
        let response_result = client
            .get(&url)
            .bearer_auth(token)
            .send()
            .and_then(|response| response.error_for_status());
        let response = match response_result {
            Ok(response) => response,
            Err(source) if Instant::now() < deadline => {
                eprintln!(
                    "[mineru] transient poll error for batch {}: {}; retrying",
                    batch_id_string, source
                );
                thread::sleep(Duration::from_secs(config.mineru.poll_interval_seconds));
                continue;
            }
            Err(source) => {
                return Err(ConvertError::PollRequest {
                    batch_id: batch_id_string.clone(),
                    source,
                })
            }
        };
        let body: Value = response
            .json()
            .map_err(|source| ConvertError::PollRequest {
                batch_id: batch_id_string.clone(),
                source,
            })?;
        let data = api_data(
            body,
            ApiOperation::Poll {
                batch_id: &batch_id_string,
            },
        )?;
        let extract_results = data
            .get("extract_result")
            .and_then(Value::as_array)
            .ok_or_else(|| ConvertError::PollField {
                batch_id: batch_id_string.clone(),
                field: "data.extract_result",
            })?;

        if let Some(result) = extract_results.first() {
            let state =
                required_string(result, "state").map_err(|field| ConvertError::PollField {
                    batch_id: batch_id_string.clone(),
                    field,
                })?;

            if state == "done" {
                let full_zip_url = required_string(result, "full_zip_url").map_err(|field| {
                    ConvertError::PollField {
                        batch_id: batch_id_string.clone(),
                        field,
                    }
                })?;
                return Ok(CompletedBatch {
                    batch_id: batch_id_string,
                    result: result.clone(),
                    full_zip_url,
                });
            }

            if state == "failed" {
                let message = result
                    .get("err_msg")
                    .and_then(Value::as_str)
                    .filter(|value| !value.is_empty())
                    .unwrap_or("unknown MinerU failure")
                    .to_owned();
                return Err(ConvertError::PollFailed {
                    batch_id: batch_id_string,
                    message,
                });
            }
        }

        if Instant::now() >= deadline {
            return Err(ConvertError::PollTimedOut {
                batch_id: batch_id_string,
                timeout_seconds: config.mineru.result_timeout_seconds,
            });
        }

        thread::sleep(Duration::from_secs(config.mineru.poll_interval_seconds));
    }
}

fn download_archive(
    config: &Config,
    batch_id: &str,
    archive_url: &str,
    archive_path: &Path,
) -> Result<(), ConvertError> {
    let client = Client::builder()
        .timeout(Duration::from_secs(config.mineru.download_timeout_seconds))
        .build()
        .map_err(|source| ConvertError::Download {
            batch_id: batch_id.to_owned(),
            source,
        })?;
    let mut response = client
        .get(archive_url)
        .send()
        .and_then(|response| response.error_for_status())
        .map_err(|source| ConvertError::Download {
            batch_id: batch_id.to_owned(),
            source,
        })?;
    let mut output = File::create(archive_path).map_err(|source| ConvertError::DownloadWrite {
        path: archive_path.to_path_buf(),
        source,
    })?;

    io::copy(&mut response, &mut output).map_err(|source| ConvertError::DownloadWrite {
        path: archive_path.to_path_buf(),
        source,
    })?;

    Ok(())
}

fn extract_archive(archive_path: &Path, artifacts_dir: &Path) -> Result<(), ConvertError> {
    fs::create_dir_all(artifacts_dir).map_err(|source| ConvertError::ZipEntryWrite {
        path: artifacts_dir.to_path_buf(),
        source,
    })?;
    let archive_file = File::open(archive_path).map_err(|source| ConvertError::ZipEntryWrite {
        path: archive_path.to_path_buf(),
        source,
    })?;
    let mut archive = ZipArchive::new(archive_file).map_err(|source| ConvertError::ZipOpen {
        path: archive_path.to_path_buf(),
        source,
    })?;

    for index in 0..archive.len() {
        let mut entry = archive
            .by_index(index)
            .map_err(|source| ConvertError::ZipOpen {
                path: archive_path.to_path_buf(),
                source,
            })?;
        let entry_name = entry.name().to_owned();
        reject_special_entry(
            archive_path,
            &entry_name,
            entry.unix_mode(),
            entry.is_dir(),
            entry.is_file(),
        )?;
        let relative_path =
            validate_entry_path(&entry_name).map_err(|reason| ConvertError::ZipEntryInvalid {
                archive_path: archive_path.to_path_buf(),
                entry: entry_name.clone(),
                reason,
            })?;
        let output_path =
            contained_artifact_path(archive_path, artifacts_dir, &entry_name, &relative_path)?;

        if entry.is_dir() {
            fs::create_dir_all(&output_path).map_err(|source| ConvertError::ZipEntryWrite {
                path: output_path,
                source,
            })?;
            continue;
        }

        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent).map_err(|source| ConvertError::ZipEntryWrite {
                path: parent.to_path_buf(),
                source,
            })?;
        }

        let mut output =
            File::create(&output_path).map_err(|source| ConvertError::ZipEntryWrite {
                path: output_path.clone(),
                source,
            })?;
        io::copy(&mut entry, &mut output).map_err(|source| ConvertError::ZipEntryRead {
            archive_path: archive_path.to_path_buf(),
            entry: entry_name,
            source,
        })?;
    }

    Ok(())
}

fn materialize_plain_text(
    artifacts_dir: &Path,
    plain_text_path: &Path,
) -> Result<u64, ConvertError> {
    let markdowns = list_markdown_files(artifacts_dir)?;
    let preferred = markdowns
        .iter()
        .find(|path| path.file_name().and_then(|value| value.to_str()) == Some("full.md"))
        .cloned()
        .or_else(|| markdowns.first().cloned());

    match preferred {
        Some(markdown_path) => {
            fs::copy(&markdown_path, plain_text_path).map_err(|source| {
                ConvertError::PlainTextMaterialize {
                    path: plain_text_path.to_path_buf(),
                    source,
                }
            })?;
        }
        None => {
            fs::write(plain_text_path, []).map_err(|source| {
                ConvertError::PlainTextMaterialize {
                    path: plain_text_path.to_path_buf(),
                    source,
                }
            })?;
        }
    }

    fs::metadata(plain_text_path)
        .map(|metadata| metadata.len())
        .map_err(|source| ConvertError::PlainTextMaterialize {
            path: plain_text_path.to_path_buf(),
            source,
        })
}

fn write_job_metadata(
    stage: &PreparedOutput,
    raw_pdf_path: &Path,
    batch_id: &str,
    extract_result: &Value,
    config: &Config,
) -> Result<(), ConvertError> {
    let metadata = JobMetadata {
        input_file: raw_pdf_path.display().to_string(),
        batch_id,
        output_dir: stage.final_output_dir.display().to_string(),
        model_version: &config.mineru.model_version,
        language: &config.mineru.language,
        enable_formula: config.mineru.enable_formula,
        enable_table: config.mineru.enable_table,
        is_ocr: config.mineru.is_ocr,
        result: extract_result,
    };
    let payload =
        serde_json::to_vec_pretty(&metadata).map_err(|source| ConvertError::JobSerialize {
            path: stage.job_path.clone(),
            source,
        })?;

    fs::write(&stage.job_path, payload).map_err(|source| ConvertError::JobWrite {
        path: stage.job_path.clone(),
        source,
    })
}

#[derive(Serialize)]
struct CreateUploadPayload<'a> {
    enable_formula: bool,
    language: &'a str,
    enable_table: bool,
    model_version: &'a str,
    files: Vec<CreateUploadFile<'a>>,
}

#[derive(Serialize)]
struct CreateUploadFile<'a> {
    name: &'a str,
    is_ocr: bool,
    data_id: &'a str,
}

#[derive(Serialize)]
struct JobMetadata<'a> {
    input_file: String,
    batch_id: &'a str,
    output_dir: String,
    model_version: &'a str,
    language: &'a str,
    enable_formula: bool,
    enable_table: bool,
    is_ocr: bool,
    result: &'a Value,
}

enum ApiOperation<'a> {
    CreateUpload,
    Poll { batch_id: &'a str },
}

fn api_data(body: Value, operation: ApiOperation<'_>) -> Result<Value, ConvertError> {
    let code = body
        .get("code")
        .and_then(Value::as_i64)
        .ok_or_else(|| match operation {
            ApiOperation::CreateUpload => ConvertError::CreateUploadField { field: "code" },
            ApiOperation::Poll { batch_id } => ConvertError::PollField {
                batch_id: batch_id.to_owned(),
                field: "code",
            },
        })?;
    if code != 0 {
        let message = body
            .get("msg")
            .and_then(Value::as_str)
            .filter(|value| !value.is_empty())
            .unwrap_or("unknown error")
            .to_owned();
        return Err(match operation {
            ApiOperation::CreateUpload => ConvertError::CreateUploadApi { message },
            ApiOperation::Poll { batch_id } => ConvertError::PollApi {
                batch_id: batch_id.to_owned(),
                message,
            },
        });
    }

    body.get("data").cloned().ok_or_else(|| match operation {
        ApiOperation::CreateUpload => ConvertError::CreateUploadField { field: "data" },
        ApiOperation::Poll { batch_id } => ConvertError::PollField {
            batch_id: batch_id.to_owned(),
            field: "data",
        },
    })
}

fn required_string(value: &Value, field: &'static str) -> Result<String, &'static str> {
    value
        .get(field)
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
        .ok_or(field)
}

fn first_upload_url(data: &Value) -> Option<String> {
    first_string_at(data.get("file_urls")).or_else(|| first_string_at(data.get("files")))
}

fn first_string_at(value: Option<&Value>) -> Option<String> {
    let values = value?.as_array()?;
    let entry = values.first()?.as_str()?.trim();
    if entry.is_empty() {
        None
    } else {
        Some(entry.to_owned())
    }
}

fn list_exam_pdfs(repo: &RepoPaths) -> Result<Vec<PathBuf>, ConvertError> {
    let exam_dir = repo.exam_raw_dir();
    let entries = match fs::read_dir(&exam_dir) {
        Ok(entries) => entries,
        Err(source) if source.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(source) => {
            return Err(ConvertError::ExamDirectoryRead {
                path: exam_dir,
                source,
            })
        }
    };

    let mut pdfs = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|source| ConvertError::ExamDirectoryRead {
            path: exam_dir.clone(),
            source,
        })?;
        let path = entry.path();
        if path.is_file()
            && path
                .extension()
                .and_then(|value| value.to_str())
                .is_some_and(|extension| extension.eq_ignore_ascii_case("pdf"))
        {
            pdfs.push(path);
        }
    }

    pdfs.sort();
    Ok(pdfs)
}

fn pdf_stem(path: &Path) -> Result<String, ConvertError> {
    let stem = path
        .file_stem()
        .and_then(|value| value.to_str())
        .filter(|value| !value.is_empty())
        .ok_or_else(|| ConvertError::InvalidExamPdfName {
            path: path.to_path_buf(),
        })?;

    if stem
        .chars()
        .all(|value| value.is_ascii_alphanumeric() || matches!(value, '-' | '_'))
    {
        Ok(stem.to_owned())
    } else {
        Err(ConvertError::InvalidExamPdfName {
            path: path.to_path_buf(),
        })
    }
}

fn create_staging_dir(parent: &Path, lesson_id: &str) -> Result<PathBuf, ConvertError> {
    let pid = std::process::id();
    let seed = unique_seed();

    for attempt in 0..STAGING_ATTEMPTS {
        let candidate = parent.join(format!(
            ".{lesson_id}.convert-staging.{pid}.{seed}.{attempt}"
        ));
        match fs::create_dir(&candidate) {
            Ok(()) => return Ok(candidate),
            Err(source) if source.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(source) => {
                return Err(ConvertError::StagingCreate {
                    path: candidate,
                    source,
                })
            }
        }
    }

    Err(ConvertError::StagingDirectoryExhausted {
        parent: parent.to_path_buf(),
    })
}

fn unique_seed() -> u128 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_nanos(),
        Err(_) => 0,
    }
}

fn reject_special_entry(
    archive_path: &Path,
    entry_name: &str,
    unix_mode: Option<u32>,
    is_dir: bool,
    is_file: bool,
) -> Result<(), ConvertError> {
    if let Some(mode) = unix_mode {
        match mode & 0o170000 {
            0 | 0o040000 | 0o100000 => {}
            0o120000 => {
                return Err(ConvertError::ZipEntryInvalid {
                    archive_path: archive_path.to_path_buf(),
                    entry: entry_name.to_owned(),
                    reason: "symlink entries are not allowed",
                })
            }
            _ => {
                return Err(ConvertError::ZipEntryInvalid {
                    archive_path: archive_path.to_path_buf(),
                    entry: entry_name.to_owned(),
                    reason: "special file entries are not allowed",
                })
            }
        }
    }

    if !is_dir && !is_file {
        return Err(ConvertError::ZipEntryInvalid {
            archive_path: archive_path.to_path_buf(),
            entry: entry_name.to_owned(),
            reason: "special file entries are not allowed",
        });
    }

    Ok(())
}

fn validate_entry_path(entry_name: &str) -> Result<PathBuf, &'static str> {
    if entry_name.is_empty() {
        return Err("entry path is empty");
    }
    if entry_name.starts_with('/') || entry_name.starts_with('\\') {
        return Err("absolute paths are not allowed");
    }
    if has_windows_drive_prefix(entry_name) {
        return Err("Windows drive-prefixed paths are not allowed");
    }

    let mut relative_path = PathBuf::new();
    for segment in entry_name.split(['/', '\\']) {
        if segment.is_empty() || segment == "." {
            continue;
        }
        if segment == ".." {
            return Err("path traversal is not allowed");
        }
        relative_path.push(segment);
    }

    if relative_path.as_os_str().is_empty() {
        return Err("entry path resolves to an empty path");
    }

    Ok(relative_path)
}

fn contained_artifact_path(
    archive_path: &Path,
    artifacts_dir: &Path,
    entry_name: &str,
    relative_path: &Path,
) -> Result<PathBuf, ConvertError> {
    let output_path = normalize_lexical_path(&artifacts_dir.join(relative_path));
    if output_path.strip_prefix(artifacts_dir).is_err() {
        return Err(ConvertError::ZipEntryInvalid {
            archive_path: archive_path.to_path_buf(),
            entry: entry_name.to_owned(),
            reason: "entry path escapes artifacts root after normalization",
        });
    }

    Ok(output_path)
}

fn normalize_lexical_path(path: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();

    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                normalized.pop();
            }
            Component::RootDir | Component::Prefix(_) | Component::Normal(_) => {
                normalized.push(component.as_os_str());
            }
        }
    }

    normalized
}

fn has_windows_drive_prefix(entry_name: &str) -> bool {
    let bytes = entry_name.as_bytes();
    bytes.len() >= 3
        && bytes[0].is_ascii_alphabetic()
        && bytes[1] == b':'
        && (bytes[2] == b'/' || bytes[2] == b'\\')
}

fn list_markdown_files(artifacts_dir: &Path) -> Result<Vec<PathBuf>, ConvertError> {
    let mut markdowns = Vec::new();
    collect_markdown_files(artifacts_dir, &mut markdowns)?;
    markdowns.sort();
    Ok(markdowns)
}

fn collect_markdown_files(
    current: &Path,
    markdowns: &mut Vec<PathBuf>,
) -> Result<(), ConvertError> {
    let mut entries = fs::read_dir(current)
        .map_err(|source| ConvertError::PlainTextMaterialize {
            path: current.to_path_buf(),
            source,
        })?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|source| ConvertError::PlainTextMaterialize {
            path: current.to_path_buf(),
            source,
        })?;

    entries.sort_by_key(|entry| entry.path());
    for entry in entries {
        let path = entry.path();
        let file_type = entry
            .file_type()
            .map_err(|source| ConvertError::PlainTextMaterialize {
                path: path.clone(),
                source,
            })?;

        if file_type.is_dir() {
            collect_markdown_files(&path, markdowns)?;
        } else if file_type.is_file()
            && path.extension().and_then(|value| value.to_str()) == Some("md")
        {
            markdowns.push(path);
        }
    }

    Ok(())
}

#[derive(Debug)]
pub enum ConvertError {
    LessonPath(LessonPathError),
    Config(ConfigError),
    ExamDirectoryRead {
        path: PathBuf,
        source: io::Error,
    },
    InvalidExamPdfName {
        path: PathBuf,
    },
    OutputDirectoryCheck {
        path: PathBuf,
        source: io::Error,
    },
    OutputAlreadyExists {
        path: PathBuf,
    },
    StagingCreate {
        path: PathBuf,
        source: io::Error,
    },
    StagingDirectoryExhausted {
        parent: PathBuf,
    },
    CreateUploadRequest {
        source: reqwest::Error,
    },
    CreateUploadApi {
        message: String,
    },
    CreateUploadField {
        field: &'static str,
    },
    Upload {
        source: reqwest::Error,
    },
    UploadWrite {
        path: PathBuf,
        source: io::Error,
    },
    PollRequest {
        batch_id: String,
        source: reqwest::Error,
    },
    PollApi {
        batch_id: String,
        message: String,
    },
    PollField {
        batch_id: String,
        field: &'static str,
    },
    PollFailed {
        batch_id: String,
        message: String,
    },
    PollTimedOut {
        batch_id: String,
        timeout_seconds: u64,
    },
    Download {
        batch_id: String,
        source: reqwest::Error,
    },
    DownloadWrite {
        path: PathBuf,
        source: io::Error,
    },
    ZipOpen {
        path: PathBuf,
        source: zip::result::ZipError,
    },
    ZipEntryInvalid {
        archive_path: PathBuf,
        entry: String,
        reason: &'static str,
    },
    ZipEntryRead {
        archive_path: PathBuf,
        entry: String,
        source: io::Error,
    },
    ZipEntryWrite {
        path: PathBuf,
        source: io::Error,
    },
    PlainTextMaterialize {
        path: PathBuf,
        source: io::Error,
    },
    EmptyPlainText {
        path: PathBuf,
    },
    JobSerialize {
        path: PathBuf,
        source: serde_json::Error,
    },
    JobWrite {
        path: PathBuf,
        source: io::Error,
    },
    FinalizeOutput {
        staging_dir: PathBuf,
        output_dir: PathBuf,
        source: io::Error,
    },
    CleanupDuringFailure {
        error: Box<ConvertError>,
        cleanup_path: PathBuf,
        cleanup_source: io::Error,
    },
}

impl fmt::Display for ConvertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LessonPath(source) => source.fmt(f),
            Self::Config(source) => source.fmt(f),
            Self::ExamDirectoryRead { path, source } => {
                write!(
                    f,
                    "failed to read exam PDF directory {}: {source}",
                    path.display()
                )
            }
            Self::InvalidExamPdfName { path } => write!(
                f,
                "invalid exam PDF file name {}; use ASCII letters, digits, '-' or '_' before .pdf",
                path.display()
            ),
            Self::OutputDirectoryCheck { path, source } => {
                write!(
                    f,
                    "failed to inspect convert output directory {}: {source}",
                    path.display()
                )
            }
            Self::OutputAlreadyExists { path } => {
                write!(f, "convert output already exists: {}", path.display())
            }
            Self::StagingCreate { path, source } => {
                write!(
                    f,
                    "failed to create convert staging directory {}: {source}",
                    path.display()
                )
            }
            Self::StagingDirectoryExhausted { parent } => write!(
                f,
                "failed to allocate a unique convert staging directory under {}",
                parent.display()
            ),
            Self::CreateUploadRequest { source } => {
                write!(f, "failed to request MinerU upload URL: {source}")
            }
            Self::CreateUploadApi { message } => {
                write!(f, "MinerU rejected the upload request: {message}")
            }
            Self::CreateUploadField { field } => {
                write!(f, "MinerU upload response missing required field '{field}'")
            }
            Self::Upload { source } => {
                write!(f, "failed to upload the lesson PDF to MinerU: {source}")
            }
            Self::UploadWrite { path, source } => {
                write!(
                    f,
                    "failed to read lesson PDF {} for upload: {source}",
                    path.display()
                )
            }
            Self::PollRequest { batch_id, source } => {
                write!(f, "failed to poll MinerU batch {batch_id}: {source}")
            }
            Self::PollApi { batch_id, message } => {
                write!(f, "MinerU poll failed for batch {batch_id}: {message}")
            }
            Self::PollField { batch_id, field } => {
                write!(
                    f,
                    "MinerU poll response for batch {batch_id} missing field '{field}'"
                )
            }
            Self::PollFailed { batch_id, message } => {
                write!(f, "MinerU marked batch {batch_id} as failed: {message}")
            }
            Self::PollTimedOut {
                batch_id,
                timeout_seconds,
            } => write!(
                f,
                "timed out waiting for MinerU batch {batch_id} after {timeout_seconds} seconds"
            ),
            Self::Download { batch_id, source } => {
                write!(
                    f,
                    "failed to download the MinerU result archive for batch {batch_id}: {source}"
                )
            }
            Self::DownloadWrite { path, source } => {
                write!(
                    f,
                    "failed to write downloaded archive {}: {source}",
                    path.display()
                )
            }
            Self::ZipOpen { path, source } => {
                write!(
                    f,
                    "failed to open MinerU archive {}: {source}",
                    path.display()
                )
            }
            Self::ZipEntryInvalid {
                archive_path,
                entry,
                reason,
            } => write!(
                f,
                "invalid zip entry '{entry}' in {}: {reason}",
                archive_path.display()
            ),
            Self::ZipEntryRead {
                archive_path,
                entry,
                source,
            } => write!(
                f,
                "failed to read zip entry '{entry}' from {}: {source}",
                archive_path.display()
            ),
            Self::ZipEntryWrite { path, source } => {
                write!(
                    f,
                    "failed to write extracted artifact {}: {source}",
                    path.display()
                )
            }
            Self::PlainTextMaterialize { path, source } => {
                write!(
                    f,
                    "failed to materialize plain text at {}: {source}",
                    path.display()
                )
            }
            Self::EmptyPlainText { path } => {
                write!(
                    f,
                    "plain text extraction produced an empty file at {}",
                    path.display()
                )
            }
            Self::JobSerialize { path, source } => {
                write!(
                    f,
                    "failed to serialize job metadata for {}: {source}",
                    path.display()
                )
            }
            Self::JobWrite { path, source } => {
                write!(
                    f,
                    "failed to write job metadata {}: {source}",
                    path.display()
                )
            }
            Self::FinalizeOutput {
                staging_dir,
                output_dir,
                source,
            } => write!(
                f,
                "failed to finalize convert output from {} to {}: {source}",
                staging_dir.display(),
                output_dir.display()
            ),
            Self::CleanupDuringFailure {
                error,
                cleanup_path,
                cleanup_source,
            } => write!(
                f,
                "{error}; additionally failed to remove staging directory {}: {cleanup_source}",
                cleanup_path.display()
            ),
        }
    }
}

impl Error for ConvertError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::LessonPath(source) => Some(source),
            Self::Config(source) => Some(source),
            Self::ExamDirectoryRead { source, .. }
            | Self::OutputDirectoryCheck { source, .. }
            | Self::StagingCreate { source, .. }
            | Self::UploadWrite { source, .. }
            | Self::DownloadWrite { source, .. }
            | Self::ZipEntryWrite { source, .. }
            | Self::PlainTextMaterialize { source, .. }
            | Self::JobWrite { source, .. }
            | Self::FinalizeOutput { source, .. } => Some(source),
            Self::CreateUploadRequest { source }
            | Self::Upload { source }
            | Self::PollRequest { source, .. }
            | Self::Download { source, .. } => Some(source),
            Self::ZipOpen { source, .. } => Some(source),
            Self::ZipEntryRead { source, .. } => Some(source),
            Self::JobSerialize { source, .. } => Some(source),
            Self::CleanupDuringFailure { error, .. } => Some(error),
            Self::OutputAlreadyExists { .. }
            | Self::InvalidExamPdfName { .. }
            | Self::StagingDirectoryExhausted { .. }
            | Self::CreateUploadApi { .. }
            | Self::CreateUploadField { .. }
            | Self::PollApi { .. }
            | Self::PollField { .. }
            | Self::PollFailed { .. }
            | Self::PollTimedOut { .. }
            | Self::ZipEntryInvalid { .. }
            | Self::EmptyPlainText { .. } => None,
        }
    }
}

impl From<LessonPathError> for ConvertError {
    fn from(error: LessonPathError) -> Self {
        Self::LessonPath(error)
    }
}

impl From<ConfigError> for ConvertError {
    fn from(error: ConfigError) -> Self {
        Self::Config(error)
    }
}
