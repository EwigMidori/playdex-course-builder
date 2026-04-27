use std::{
    error::Error,
    fmt, fs, io,
    path::{Path, PathBuf},
    process, thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use pipeline_core::{LessonPaths, StageName};
use serde::{Deserialize, Serialize};

const SCHEMA_VERSION: u8 = 1;
const STAGES_DIR_NAME: &str = "stages";

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum StageStatus {
    Pending,
    Running,
    Passed,
    Failed,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum StageMode {
    Executed,
    Reused,
}

#[derive(Clone, Debug)]
pub struct RunRecord {
    lesson: LessonPaths,
    run_id: String,
    command_name: String,
    resume_requested: bool,
    created_at_epoch_ms: u64,
}

#[derive(Clone, Debug)]
pub struct StageRecord {
    run_id: String,
    lesson_id: String,
    stage: String,
    status: StageStatus,
    mode: StageMode,
    output_dir: String,
    reused_from_run_id: Option<String>,
    error: Option<String>,
    updated_at_epoch_ms: u64,
}

#[derive(Clone, Debug)]
pub struct ResumeCandidate {
    run_id: String,
    output_dir: PathBuf,
}

impl ResumeCandidate {
    pub fn run_id(&self) -> &str {
        &self.run_id
    }

    pub fn output_dir(&self) -> &Path {
        &self.output_dir
    }
}

pub fn create_run_record(
    lesson: &LessonPaths,
    command_name: &str,
    resume_requested: bool,
) -> Result<RunRecord, RunStateError> {
    let run_root_dir = lesson.run_root_dir();
    fs::create_dir_all(&run_root_dir).map_err(|source| RunStateError::CreateRunDir {
        path: run_root_dir.clone(),
        source,
    })?;

    let (run_id, created_at_epoch_ms) = create_unique_run_id(lesson, &run_root_dir)?;
    let run_dir = lesson.run_dir(&run_id);
    let stage_dir = run_dir.join(STAGES_DIR_NAME);

    fs::create_dir_all(&stage_dir).map_err(|source| RunStateError::CreateRunDir {
        path: stage_dir.clone(),
        source,
    })?;

    let run = RunRecord {
        lesson: lesson.clone(),
        run_id: run_id.clone(),
        command_name: command_name.to_owned(),
        resume_requested,
        created_at_epoch_ms,
    };

    let run_json = RunJson {
        schema_version: SCHEMA_VERSION,
        run_id,
        lesson_id: lesson.lesson_id().to_owned(),
        command: command_name.to_owned(),
        resume_requested,
        resumed_from_run_id: None,
        status: RunStatus::Running,
        created_at_epoch_ms,
        updated_at_epoch_ms: created_at_epoch_ms,
    };

    write_run_json(&lesson.run_metadata_path(&run.run_id), &run_json)?;
    Ok(run)
}

pub fn write_stage_pending(
    run: &RunRecord,
    stage: StageName,
    mode: StageMode,
) -> Result<StageRecord, RunStateError> {
    write_stage_state(run, stage, StageStatus::Pending, mode, None, None)
}

pub fn write_stage_running(
    run: &RunRecord,
    stage: StageName,
    mode: StageMode,
) -> Result<StageRecord, RunStateError> {
    write_stage_state(run, stage, StageStatus::Running, mode, None, None)
}

pub fn write_stage_passed(
    run: &RunRecord,
    stage: StageName,
    mode: StageMode,
    reused_from_run_id: Option<&str>,
) -> Result<StageRecord, RunStateError> {
    write_stage_state(
        run,
        stage,
        StageStatus::Passed,
        mode,
        reused_from_run_id,
        None,
    )
}

pub fn write_stage_failed(
    run: &RunRecord,
    stage: StageName,
    mode: StageMode,
    message: &str,
) -> Result<StageRecord, RunStateError> {
    write_stage_state(run, stage, StageStatus::Failed, mode, None, Some(message))
}

pub fn find_latest_resume_candidate(
    lesson: &LessonPaths,
    stage: StageName,
) -> Result<Option<ResumeCandidate>, RunStateError> {
    let run_root_dir = lesson.run_root_dir();
    if !run_root_dir.is_dir() {
        return Ok(None);
    }

    let read_dir = fs::read_dir(&run_root_dir).map_err(|source| RunStateError::ScanRunDir {
        path: run_root_dir.clone(),
        source,
    })?;

    let mut run_ids = Vec::new();
    for entry in read_dir {
        let entry = entry.map_err(|source| RunStateError::ScanRunDir {
            path: run_root_dir.clone(),
            source,
        })?;

        let file_type = match entry.file_type() {
            Ok(file_type) => file_type,
            Err(_) => continue,
        };
        if !file_type.is_dir() {
            continue;
        }

        let run_id = match entry.file_name().into_string() {
            Ok(run_id) => run_id,
            Err(_) => continue,
        };
        run_ids.push(run_id);
    }

    run_ids.sort();
    for run_id in run_ids.into_iter().rev() {
        if let Some(candidate) = load_resume_candidate(lesson, &run_id, stage) {
            return Ok(Some(candidate));
        }
    }

    Ok(None)
}

#[derive(Debug)]
pub enum RunStateError {
    CreateRunDir { path: PathBuf, source: io::Error },
    WriteRunRecord { path: PathBuf, source: io::Error },
    WriteStageRecord { path: PathBuf, source: io::Error },
    ScanRunDir { path: PathBuf, source: io::Error },
}

impl fmt::Display for RunStateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreateRunDir { path, source } => {
                write!(
                    f,
                    "failed to create run metadata directory {}: {source}",
                    path.display()
                )
            }
            Self::WriteRunRecord { path, source } => {
                write!(
                    f,
                    "failed to write run metadata {}: {source}",
                    path.display()
                )
            }
            Self::WriteStageRecord { path, source } => write!(
                f,
                "failed to write stage metadata {}: {source}",
                path.display()
            ),
            Self::ScanRunDir { path, source } => {
                write!(
                    f,
                    "failed to scan run metadata directory {}: {source}",
                    path.display()
                )
            }
        }
    }
}

impl Error for RunStateError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::CreateRunDir { source, .. }
            | Self::WriteRunRecord { source, .. }
            | Self::WriteStageRecord { source, .. }
            | Self::ScanRunDir { source, .. } => Some(source),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
enum RunStatus {
    Running,
    Passed,
    Failed,
}

#[derive(Debug, Deserialize, Serialize)]
struct RunJson {
    schema_version: u8,
    run_id: String,
    lesson_id: String,
    command: String,
    resume_requested: bool,
    resumed_from_run_id: Option<String>,
    status: RunStatus,
    created_at_epoch_ms: u64,
    updated_at_epoch_ms: u64,
}

#[derive(Debug, Deserialize, Serialize)]
struct StageJson {
    schema_version: u8,
    run_id: String,
    lesson_id: String,
    stage: String,
    status: StageStatus,
    mode: StageMode,
    output_dir: String,
    reused_from_run_id: Option<String>,
    error: Option<String>,
    updated_at_epoch_ms: u64,
}

fn write_stage_state(
    run: &RunRecord,
    stage: StageName,
    status: StageStatus,
    mode: StageMode,
    reused_from_run_id: Option<&str>,
    message: Option<&str>,
) -> Result<StageRecord, RunStateError> {
    let updated_at_epoch_ms = current_epoch_ms();
    let output_dir = plain_output_dir_string(run.lesson.lesson_id());
    let stage_path = run.lesson.stage_state_path(&run.run_id, stage);
    let error = match message {
        Some(message) => Some(non_empty_message(message)),
        None => None,
    };
    let reused_from_run_id = reused_from_run_id.map(str::to_owned);

    let stage_record = StageRecord {
        run_id: run.run_id.clone(),
        lesson_id: run.lesson.lesson_id().to_owned(),
        stage: stage.as_str().to_owned(),
        status,
        mode,
        output_dir: output_dir.clone(),
        reused_from_run_id: reused_from_run_id.clone(),
        error: error.clone(),
        updated_at_epoch_ms,
    };

    let stage_json = StageJson {
        schema_version: SCHEMA_VERSION,
        run_id: stage_record.run_id.clone(),
        lesson_id: stage_record.lesson_id.clone(),
        stage: stage_record.stage.clone(),
        status: stage_record.status,
        mode: stage_record.mode,
        output_dir: stage_record.output_dir.clone(),
        reused_from_run_id: stage_record.reused_from_run_id.clone(),
        error: stage_record.error.clone(),
        updated_at_epoch_ms: stage_record.updated_at_epoch_ms,
    };
    write_stage_json(&stage_path, &stage_json)?;

    let run_json = RunJson {
        schema_version: SCHEMA_VERSION,
        run_id: run.run_id.clone(),
        lesson_id: run.lesson.lesson_id().to_owned(),
        command: run.command_name.clone(),
        resume_requested: run.resume_requested,
        resumed_from_run_id: if status == StageStatus::Passed && mode == StageMode::Reused {
            reused_from_run_id.clone()
        } else {
            None
        },
        status: match status {
            StageStatus::Passed => RunStatus::Passed,
            StageStatus::Failed => RunStatus::Failed,
            StageStatus::Pending | StageStatus::Running => RunStatus::Running,
        },
        created_at_epoch_ms: run.created_at_epoch_ms,
        updated_at_epoch_ms,
    };
    write_run_json(&run.lesson.run_metadata_path(&run.run_id), &run_json)?;

    Ok(stage_record)
}

fn load_resume_candidate(
    lesson: &LessonPaths,
    run_id: &str,
    stage: StageName,
) -> Option<ResumeCandidate> {
    let run_json = read_json::<RunJson>(&lesson.run_metadata_path(run_id))?;
    if run_json.schema_version != SCHEMA_VERSION
        || run_json.run_id != run_id
        || run_json.lesson_id != lesson.lesson_id()
        || run_json.command != "convert"
        || run_json.status != RunStatus::Passed
    {
        return None;
    }

    let stage_json = read_json::<StageJson>(&lesson.stage_state_path(run_id, stage))?;
    if stage_json.schema_version != SCHEMA_VERSION
        || stage_json.run_id != run_id
        || stage_json.lesson_id != lesson.lesson_id()
        || stage_json.stage != stage.as_str()
        || stage_json.status != StageStatus::Passed
        || stage_json.output_dir != plain_output_dir_string(lesson.lesson_id())
    {
        return None;
    }

    let output_dir = lesson.plain_output_dir();
    if !output_dir.is_dir() {
        return None;
    }

    Some(ResumeCandidate {
        run_id: run_id.to_owned(),
        output_dir,
    })
}

fn create_unique_run_id(
    lesson: &LessonPaths,
    run_root_dir: &Path,
) -> Result<(String, u64), RunStateError> {
    loop {
        let epoch_ms = current_epoch_ms();
        let run_id = format!("{epoch_ms}-{}", process::id());
        let run_dir = lesson.run_dir(&run_id);

        match fs::create_dir(&run_dir) {
            Ok(()) => return Ok((run_id, epoch_ms)),
            Err(source) if source.kind() == io::ErrorKind::AlreadyExists => {
                thread::sleep(Duration::from_millis(1));
            }
            Err(source) => {
                return Err(RunStateError::CreateRunDir {
                    path: run_root_dir.to_path_buf(),
                    source,
                });
            }
        }
    }
}

fn write_run_json(path: &Path, run_json: &RunJson) -> Result<(), RunStateError> {
    write_json_atomically(path, run_json).map_err(|source| RunStateError::WriteRunRecord {
        path: path.to_path_buf(),
        source,
    })
}

fn write_stage_json(path: &Path, stage_json: &StageJson) -> Result<(), RunStateError> {
    write_json_atomically(path, stage_json).map_err(|source| RunStateError::WriteStageRecord {
        path: path.to_path_buf(),
        source,
    })
}

fn write_json_atomically<T>(path: &Path, value: &T) -> Result<(), io::Error>
where
    T: Serialize,
{
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut content = serde_json::to_vec_pretty(value).map_err(io::Error::other)?;
    content.push(b'\n');

    let temp_path = temp_path_for(path);
    fs::write(&temp_path, content)?;

    match fs::rename(&temp_path, path) {
        Ok(()) => Ok(()),
        Err(source) => {
            let _ = fs::remove_file(&temp_path);
            Err(source)
        }
    }
}

fn read_json<T>(path: &Path) -> Option<T>
where
    T: for<'de> Deserialize<'de>,
{
    let content = fs::read_to_string(path).ok()?;
    serde_json::from_str(&content).ok()
}

fn temp_path_for(path: &Path) -> PathBuf {
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("metadata.json");

    path.with_file_name(format!(
        ".{file_name}.tmp.{}-{}",
        process::id(),
        current_epoch_ms()
    ))
}

fn current_epoch_ms() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_millis().min(u64::MAX as u128) as u64,
        Err(_) => Duration::from_secs(0).as_millis() as u64,
    }
}

fn plain_output_dir_string(lesson_id: &str) -> String {
    format!("research/pipeline/1-plain/{lesson_id}")
}

fn non_empty_message(message: &str) -> String {
    let trimmed = message.trim();
    if trimmed.is_empty() {
        "stage failed".to_owned()
    } else {
        trimmed.to_owned()
    }
}
