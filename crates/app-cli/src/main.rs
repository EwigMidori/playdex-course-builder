mod cli;

use clap::Parser;
use std::{error::Error, fmt, process::ExitCode};

use crate::cli::{Cli, Command};

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
        Command::Convert { .. } => {
            lesson
                .checked_raw_pdf_path()
                .map_err(AppError::LessonPaths)?;
            let config = pipeline_io::load_process_config(&repo).map_err(AppError::Config)?;
            pipeline_mineru::convert_lesson(
                &lesson,
                &config,
                &pipeline_mineru::ConvertOptions::new(),
            )
            .map_err(AppError::Convert)?;
        }
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

#[derive(Debug)]
enum AppError {
    LessonPaths(pipeline_core::LessonPathError),
    Config(pipeline_io::ConfigError),
    Convert(pipeline_mineru::ConvertError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LessonPaths(source) => source.fmt(f),
            Self::Config(source) => source.fmt(f),
            Self::Convert(source) => source.fmt(f),
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::LessonPaths(source) => Some(source),
            Self::Config(source) => Some(source),
            Self::Convert(source) => Some(source),
        }
    }
}
