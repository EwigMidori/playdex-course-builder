use std::path::{Path, PathBuf};

use crate::{
    client,
    error::ConvertError,
    output::{self, PreparedOutput},
};

#[derive(Clone, Debug, Default)]
pub struct ConvertOptions;

impl ConvertOptions {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Clone, Debug)]
pub struct ConvertSuccess {
    batch_id: String,
    output_dir: PathBuf,
    archive_path: PathBuf,
    artifacts_dir: PathBuf,
    plain_text_path: PathBuf,
    plain_text_bytes: u64,
}

impl ConvertSuccess {
    pub fn batch_id(&self) -> &str {
        &self.batch_id
    }

    pub fn output_dir(&self) -> &Path {
        &self.output_dir
    }

    pub fn archive_path(&self) -> &Path {
        &self.archive_path
    }

    pub fn artifacts_dir(&self) -> &Path {
        &self.artifacts_dir
    }

    pub fn plain_text_path(&self) -> &Path {
        &self.plain_text_path
    }

    pub fn plain_text_bytes(&self) -> u64 {
        self.plain_text_bytes
    }
}

pub fn convert_lesson(
    lesson: &pipeline_core::LessonPaths,
    config: &pipeline_io::ResolvedConfig,
    _options: &ConvertOptions,
) -> Result<ConvertSuccess, ConvertError> {
    let raw_pdf_path = lesson
        .checked_raw_pdf_path()
        .map_err(|source| ConvertError::MissingRawPdf { source })?;
    let stage = output::prepare_output(lesson)?;

    let result = run_convert(lesson, config, &raw_pdf_path, &stage);
    match result {
        Ok(success) => Ok(success),
        Err(error) => Err(output::cleanup_staging(&stage, error)),
    }
}

fn run_convert(
    lesson: &pipeline_core::LessonPaths,
    config: &pipeline_io::ResolvedConfig,
    raw_pdf_path: &Path,
    stage: &PreparedOutput,
) -> Result<ConvertSuccess, ConvertError> {
    let token = config
        .read_mineru_token()
        .map_err(|source| ConvertError::MissingToken { source })?;
    let upload = client::request_upload(config, &token, lesson, raw_pdf_path)?;
    client::upload_file(config, &upload.upload_url, raw_pdf_path)?;
    let completed = client::poll_until_done(config, &token, &upload.batch_id)?;
    client::download_archive(
        config,
        &completed.batch_id,
        &completed.full_zip_url,
        stage.archive_path(),
    )?;
    output::extract_archive(stage.archive_path(), stage.artifacts_dir())?;
    let plain_text_bytes =
        output::materialize_plain_text(stage.artifacts_dir(), stage.plain_text_path())?;
    output::write_job_metadata(
        stage,
        raw_pdf_path,
        &completed.batch_id,
        &completed.result,
        config,
    )?;

    if config.mineru().fail_on_empty_text() && plain_text_bytes == 0 {
        return Err(ConvertError::EmptyPlainText {
            path: stage.plain_text_path().to_path_buf(),
        });
    }

    output::finalize_output(stage)?;

    let output_dir = stage.final_output_dir().to_path_buf();
    Ok(ConvertSuccess {
        batch_id: completed.batch_id,
        archive_path: output_dir.join("result.zip"),
        artifacts_dir: output_dir.join("artifacts"),
        plain_text_path: output_dir.join("plain.txt"),
        output_dir,
        plain_text_bytes,
    })
}
