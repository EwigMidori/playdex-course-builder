use std::{
    error::Error,
    fmt, io,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub enum ConvertError {
    MissingRawPdf {
        source: pipeline_core::LessonPathError,
    },
    MissingToken {
        source: pipeline_io::ConfigError,
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
    StagingCleanup {
        path: PathBuf,
        source: io::Error,
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

impl ConvertError {
    pub(crate) fn with_cleanup(self, cleanup_path: &Path, cleanup_source: io::Error) -> Self {
        Self::CleanupDuringFailure {
            error: Box::new(self),
            cleanup_path: cleanup_path.to_path_buf(),
            cleanup_source,
        }
    }
}

impl fmt::Display for ConvertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingRawPdf { source } => source.fmt(f),
            Self::MissingToken { source } => source.fmt(f),
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
            Self::StagingCleanup { path, source } => {
                write!(
                    f,
                    "failed to remove convert staging directory {}: {source}",
                    path.display()
                )
            }
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
            Self::MissingRawPdf { source } => Some(source),
            Self::MissingToken { source } => Some(source),
            Self::OutputDirectoryCheck { source, .. }
            | Self::StagingCreate { source, .. }
            | Self::StagingCleanup { source, .. }
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
