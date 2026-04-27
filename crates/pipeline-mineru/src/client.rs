use std::{
    fs::File,
    io,
    path::Path,
    thread,
    time::{Duration, Instant},
};

use reqwest::blocking::Client;
use serde::Serialize;
use serde_json::Value;

use crate::error::ConvertError;

pub(crate) struct CreateUploadResponse {
    pub(crate) batch_id: String,
    pub(crate) upload_url: String,
}

pub(crate) struct CompletedBatch {
    pub(crate) batch_id: String,
    pub(crate) result: Value,
    pub(crate) full_zip_url: String,
}

pub(crate) fn request_upload(
    config: &pipeline_io::ResolvedConfig,
    token: &str,
    lesson: &pipeline_core::LessonPaths,
    raw_pdf_path: &Path,
) -> Result<CreateUploadResponse, ConvertError> {
    let client = Client::builder()
        .timeout(Duration::from_secs(
            config.mineru().request_timeout_seconds(),
        ))
        .build()
        .map_err(|source| ConvertError::CreateUploadRequest { source })?;
    let url = format!(
        "{}/api/v4/file-urls/batch",
        config.mineru().api_base_url().trim_end_matches('/')
    );
    let payload = CreateUploadPayload {
        enable_formula: config.mineru().enable_formula(),
        language: config.mineru().language(),
        enable_table: config.mineru().enable_table(),
        model_version: config.mineru().model_version(),
        files: vec![CreateUploadFile {
            name: raw_pdf_path
                .file_name()
                .and_then(|value| value.to_str())
                .unwrap_or("input.pdf"),
            is_ocr: config.mineru().is_ocr(),
            data_id: lesson.lesson_id(),
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

pub(crate) fn upload_file(
    config: &pipeline_io::ResolvedConfig,
    upload_url: &str,
    raw_pdf_path: &Path,
) -> Result<(), ConvertError> {
    let client = Client::builder()
        .timeout(Duration::from_secs(
            config.mineru().upload_timeout_seconds(),
        ))
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

pub(crate) fn poll_until_done(
    config: &pipeline_io::ResolvedConfig,
    token: &str,
    batch_id: &str,
) -> Result<CompletedBatch, ConvertError> {
    let batch_id_string = batch_id.to_owned();
    let client = Client::builder()
        .timeout(Duration::from_secs(
            config.mineru().request_timeout_seconds(),
        ))
        .build()
        .map_err(|source| ConvertError::PollRequest {
            batch_id: batch_id_string.clone(),
            source,
        })?;
    let url = format!(
        "{}/api/v4/extract-results/batch/{}",
        config.mineru().api_base_url().trim_end_matches('/'),
        batch_id
    );
    let deadline = Instant::now() + Duration::from_secs(config.mineru().result_timeout_seconds());

    loop {
        let response = client
            .get(&url)
            .bearer_auth(token)
            .send()
            .and_then(|response| response.error_for_status())
            .map_err(|source| ConvertError::PollRequest {
                batch_id: batch_id_string.clone(),
                source,
            })?;
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
                let message = optional_string(result, "err_msg")
                    .filter(|value| !value.is_empty())
                    .unwrap_or_else(|| "unknown MinerU failure".to_owned());
                return Err(ConvertError::PollFailed {
                    batch_id: batch_id_string,
                    message,
                });
            }
        }

        if Instant::now() >= deadline {
            return Err(ConvertError::PollTimedOut {
                batch_id: batch_id_string,
                timeout_seconds: config.mineru().result_timeout_seconds(),
            });
        }

        thread::sleep(Duration::from_secs(config.mineru().poll_interval_seconds()));
    }
}

pub(crate) fn download_archive(
    config: &pipeline_io::ResolvedConfig,
    batch_id: &str,
    archive_url: &str,
    archive_path: &Path,
) -> Result<(), ConvertError> {
    let client = Client::builder()
        .timeout(Duration::from_secs(
            config.mineru().download_timeout_seconds(),
        ))
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

fn optional_string(value: &Value, field: &'static str) -> Option<String> {
    value
        .get(field)
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
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
