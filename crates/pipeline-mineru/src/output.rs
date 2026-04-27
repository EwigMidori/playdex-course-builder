use std::{
    fs,
    fs::File,
    io,
    path::{Component, Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use serde::Serialize;
use serde_json::Value;
use zip::ZipArchive;

use crate::error::ConvertError;

const STAGING_ATTEMPTS: usize = 16;

pub(crate) struct PreparedOutput {
    final_output_dir: PathBuf,
    staging_dir: PathBuf,
    archive_path: PathBuf,
    artifacts_dir: PathBuf,
    plain_text_path: PathBuf,
    job_path: PathBuf,
}

impl PreparedOutput {
    pub(crate) fn final_output_dir(&self) -> &Path {
        &self.final_output_dir
    }

    pub(crate) fn staging_dir(&self) -> &Path {
        &self.staging_dir
    }

    pub(crate) fn archive_path(&self) -> &Path {
        &self.archive_path
    }

    pub(crate) fn artifacts_dir(&self) -> &Path {
        &self.artifacts_dir
    }

    pub(crate) fn plain_text_path(&self) -> &Path {
        &self.plain_text_path
    }

    pub(crate) fn job_path(&self) -> &Path {
        &self.job_path
    }
}

pub(crate) fn prepare_output(
    lesson: &pipeline_core::LessonPaths,
) -> Result<PreparedOutput, ConvertError> {
    let final_output_dir = lesson.plain_output_dir();
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

    let staging_dir = create_staging_dir(&parent, lesson.lesson_id())?;
    let archive_path = staging_dir.join("result.zip");
    let artifacts_dir = staging_dir.join("artifacts");
    let plain_text_path = staging_dir.join("plain.txt");
    let job_path = staging_dir.join("job.json");

    Ok(PreparedOutput {
        final_output_dir,
        staging_dir,
        archive_path,
        artifacts_dir,
        plain_text_path,
        job_path,
    })
}

pub(crate) fn cleanup_staging(stage: &PreparedOutput, error: ConvertError) -> ConvertError {
    match fs::remove_dir_all(stage.staging_dir()) {
        Ok(()) => error,
        Err(source) if source.kind() == io::ErrorKind::NotFound => error,
        Err(source) => error.with_cleanup(stage.staging_dir(), source),
    }
}

pub(crate) fn extract_archive(
    archive_path: &Path,
    artifacts_dir: &Path,
) -> Result<(), ConvertError> {
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

pub(crate) fn materialize_plain_text(
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

pub(crate) fn write_job_metadata(
    stage: &PreparedOutput,
    raw_pdf_path: &Path,
    batch_id: &str,
    extract_result: &Value,
    config: &pipeline_io::ResolvedConfig,
) -> Result<(), ConvertError> {
    let metadata = JobMetadata {
        input_file: raw_pdf_path.display().to_string(),
        batch_id,
        output_dir: stage.final_output_dir().display().to_string(),
        model_version: config.mineru().model_version(),
        language: config.mineru().language(),
        enable_formula: config.mineru().enable_formula(),
        enable_table: config.mineru().enable_table(),
        is_ocr: config.mineru().is_ocr(),
        result: extract_result,
    };
    let payload =
        serde_json::to_vec_pretty(&metadata).map_err(|source| ConvertError::JobSerialize {
            path: stage.job_path().to_path_buf(),
            source,
        })?;

    fs::write(stage.job_path(), payload).map_err(|source| ConvertError::JobWrite {
        path: stage.job_path().to_path_buf(),
        source,
    })
}

pub(crate) fn finalize_output(stage: &PreparedOutput) -> Result<(), ConvertError> {
    fs::rename(stage.staging_dir(), stage.final_output_dir()).map_err(|source| {
        ConvertError::FinalizeOutput {
            staging_dir: stage.staging_dir().to_path_buf(),
            output_dir: stage.final_output_dir().to_path_buf(),
            source,
        }
    })
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
            0 => {}
            0o040000 | 0o100000 => {}
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
    collect_markdown_files(artifacts_dir, artifacts_dir, &mut markdowns)?;
    markdowns.sort_by_key(|path| normalized_relative_string(artifacts_dir, path));
    Ok(markdowns)
}

fn collect_markdown_files(
    root: &Path,
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
            collect_markdown_files(root, &path, markdowns)?;
        } else if file_type.is_file()
            && path.extension().and_then(|value| value.to_str()) == Some("md")
        {
            markdowns.push(path);
        }
    }

    Ok(())
}

fn normalized_relative_string(root: &Path, path: &Path) -> String {
    match path.strip_prefix(root) {
        Ok(relative) => relative.to_string_lossy().replace('\\', "/"),
        Err(_) => path.to_string_lossy().replace('\\', "/"),
    }
}
