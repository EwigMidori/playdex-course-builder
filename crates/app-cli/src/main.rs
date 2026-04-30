mod cli;
mod config;
mod mineru;
mod mvp;
mod paths;
mod relevance;
mod validation;

use clap::Parser;
use std::{error::Error, fmt, process::ExitCode};

use crate::{
    cli::{Cli, Command},
    paths::RepoPaths,
};

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
    let repo = RepoPaths::from_current_dir()?;

    match cli.command {
        Command::Run {
            lesson_id,
            target_language,
        } => {
            let lesson = repo.resolve_lesson(&lesson_id)?;
            mvp::run_mvp(&repo, &lesson, &target_language)?;
            println!("mvp complete: lesson={lesson_id} target_language={target_language}");
        }
        Command::Convert { lesson_id, resume } => {
            let lesson = repo.resolve_lesson(&lesson_id)?;
            mineru::convert_lesson(&repo, &lesson, resume)?;
            println!(
                "convert complete: lesson={} plain_text={}",
                lesson.lesson_id(),
                lesson.relative_display(&lesson.plain_text_path())
            );
        }
        Command::Validate { lesson_id } => {
            let lesson = repo.resolve_lesson(&lesson_id)?;
            validation::validate_outputs(&lesson)?;
            println!("validation passed: lesson={lesson_id}");
        }
        Command::ScoreRelevance {
            lesson_id,
            target_language,
        } => {
            let lesson = repo.resolve_lesson(&lesson_id)?;
            relevance::score_relevance(&repo, &lesson, &target_language)?;
            println!(
                "relevance scoring complete: lesson={} report={}",
                lesson_id,
                lesson.relative_display(&lesson.relevance_report_path())
            );
        }
    }

    Ok(())
}

#[derive(Debug)]
enum AppError {
    Paths(paths::LessonPathError),
    Config(config::ConfigError),
    Convert(mineru::ConvertError),
    Mvp(mvp::MvpError),
    Relevance(relevance::RelevanceError),
    Validation(validation::ValidationError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Paths(error) => error.fmt(f),
            Self::Config(error) => error.fmt(f),
            Self::Convert(error) => error.fmt(f),
            Self::Mvp(error) => error.fmt(f),
            Self::Relevance(error) => error.fmt(f),
            Self::Validation(error) => error.fmt(f),
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Paths(error) => Some(error),
            Self::Config(error) => Some(error),
            Self::Convert(error) => Some(error),
            Self::Mvp(error) => Some(error),
            Self::Relevance(error) => Some(error),
            Self::Validation(error) => Some(error),
        }
    }
}

impl From<paths::LessonPathError> for AppError {
    fn from(error: paths::LessonPathError) -> Self {
        Self::Paths(error)
    }
}

impl From<config::ConfigError> for AppError {
    fn from(error: config::ConfigError) -> Self {
        Self::Config(error)
    }
}

impl From<mineru::ConvertError> for AppError {
    fn from(error: mineru::ConvertError) -> Self {
        Self::Convert(error)
    }
}

impl From<mvp::MvpError> for AppError {
    fn from(error: mvp::MvpError) -> Self {
        Self::Mvp(error)
    }
}

impl From<relevance::RelevanceError> for AppError {
    fn from(error: relevance::RelevanceError) -> Self {
        Self::Relevance(error)
    }
}

impl From<validation::ValidationError> for AppError {
    fn from(error: validation::ValidationError) -> Self {
        Self::Validation(error)
    }
}
