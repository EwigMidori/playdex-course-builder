mod cli;

use clap::Parser;
use std::{error::Error, fmt, process::ExitCode};

use crate::cli::{Cli, Command};

const RESUME_UNAVAILABLE_MESSAGE: &str =
    "resume requested but no valid prior passed convert run is available for existing output";

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), AppError> {
    let cli = Cli::parse();
    let repo = pipeline_core::RepoPaths::from_current_dir().map_err(AppError::LessonPaths)?;
    let lesson = repo
        .resolve_lesson(cli.command.lesson_id())
        .map_err(AppError::LessonPaths)?;

    match &cli.command {
        Command::Convert { resume, .. } => run_convert(&repo, &lesson, *resume)?,
        _ => {
            println!(
                "placeholder: command={} stage={} lesson={} raw_pdf={}",
                cli.command.command_name(),
                cli.command.stage_name(),
                lesson.lesson_id(),
                lesson.raw_pdf_relative_string()
            );
        }
    }

    Ok(())
}

fn run_convert(
    repo: &pipeline_core::RepoPaths,
    lesson: &pipeline_core::LessonPaths,
    resume_requested: bool,
) -> Result<(), AppError> {
    let run = pipeline_io::create_run_record(lesson, "convert", resume_requested)
        .map_err(AppError::RunState)?;

    if resume_requested && try_resume_convert(lesson, &run)? {
        return Ok(());
    }

    pipeline_io::write_stage_pending(
        &run,
        pipeline_core::StageName::Convert,
        pipeline_io::StageMode::Executed,
    )
    .map_err(AppError::RunState)?;

    if let Err(error) = lesson.checked_raw_pdf_path() {
        return Err(record_operation_failure(
            &run,
            pipeline_io::StageMode::Executed,
            AppError::LessonPaths(error),
        ));
    }

    let config = match pipeline_io::load_process_config(repo) {
        Ok(config) => config,
        Err(error) => {
            return Err(record_operation_failure(
                &run,
                pipeline_io::StageMode::Executed,
                AppError::Config(error),
            ));
        }
    };

    pipeline_io::write_stage_running(
        &run,
        pipeline_core::StageName::Convert,
        pipeline_io::StageMode::Executed,
    )
    .map_err(AppError::RunState)?;

    match pipeline_mineru::convert_lesson(lesson, &config, &pipeline_mineru::ConvertOptions::new())
    {
        Ok(_) => pipeline_io::write_stage_passed(
            &run,
            pipeline_core::StageName::Convert,
            pipeline_io::StageMode::Executed,
            None,
        )
        .map(|_| ())
        .map_err(AppError::RunState),
        Err(error) => Err(record_operation_failure(
            &run,
            pipeline_io::StageMode::Executed,
            AppError::Convert(error),
        )),
    }
}

fn try_resume_convert(
    lesson: &pipeline_core::LessonPaths,
    run: &pipeline_io::RunRecord,
) -> Result<bool, AppError> {
    let candidate =
        pipeline_io::find_latest_resume_candidate(lesson, pipeline_core::StageName::Convert)
            .map_err(AppError::RunState)?;

    if let Some(candidate) = candidate {
        pipeline_io::write_stage_passed(
            run,
            pipeline_core::StageName::Convert,
            pipeline_io::StageMode::Reused,
            Some(candidate.run_id()),
        )
        .map_err(AppError::RunState)?;
        return Ok(true);
    }

    if lesson.plain_output_dir().exists() {
        return Err(record_operation_failure(
            run,
            pipeline_io::StageMode::Reused,
            AppError::ResumeUnavailable {
                lesson_id: lesson.lesson_id().to_owned(),
            },
        ));
    }

    Ok(false)
}

fn record_operation_failure(
    run: &pipeline_io::RunRecord,
    mode: pipeline_io::StageMode,
    operation: AppError,
) -> AppError {
    let message = match &operation {
        AppError::ResumeUnavailable { .. } => RESUME_UNAVAILABLE_MESSAGE.to_owned(),
        _ => operation.to_string(),
    };

    match pipeline_io::write_stage_failed(run, pipeline_core::StageName::Convert, mode, &message) {
        Ok(_) => operation,
        Err(metadata) => AppError::Composite {
            source: CompositeSource {
                operation_message: operation.to_string(),
                metadata,
            },
            operation: Box::new(operation),
        },
    }
}

#[derive(Debug)]
enum AppError {
    LessonPaths(pipeline_core::LessonPathError),
    Config(pipeline_io::ConfigError),
    Convert(pipeline_mineru::ConvertError),
    RunState(pipeline_io::RunStateError),
    ResumeUnavailable {
        lesson_id: String,
    },
    Composite {
        operation: Box<AppError>,
        source: CompositeSource,
    },
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LessonPaths(source) => source.fmt(f),
            Self::Config(source) => source.fmt(f),
            Self::Convert(source) => source.fmt(f),
            Self::RunState(source) => source.fmt(f),
            Self::ResumeUnavailable { lesson_id } => write!(
                f,
                "{RESUME_UNAVAILABLE_MESSAGE} for lesson '{lesson_id}'"
            ),
            Self::Composite {
                operation,
                source,
            } => write!(
                f,
                "operation failed and run metadata persistence also failed: operation: {operation}; metadata: {}",
                source.metadata
            ),
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::LessonPaths(source) => Some(source),
            Self::Config(source) => Some(source),
            Self::Convert(source) => Some(source),
            Self::RunState(source) => Some(source),
            Self::ResumeUnavailable { .. } => None,
            Self::Composite { source, .. } => Some(source),
        }
    }
}

#[derive(Debug)]
struct CompositeSource {
    operation_message: String,
    metadata: pipeline_io::RunStateError,
}

impl fmt::Display for CompositeSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.operation_message)
    }
}

impl Error for CompositeSource {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.metadata)
    }
}
