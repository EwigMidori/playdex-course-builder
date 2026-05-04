mod catalog_validation;
mod cli;
mod config;
mod llm;
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
            target,
            target_language,
            force_stage,
        } => {
            let lesson = repo.resolve_target(
                target.lesson_id.as_deref(),
                target.course.as_deref(),
                target.chapter.as_deref(),
            )?;
            mvp::MvpRunner::run(&repo, &lesson, &target_language, force_stage)?;
            println!(
                "mvp complete: target={} target_language={target_language}",
                lesson.target_display()
            );
        }
        Command::Convert { target, resume } => {
            let lesson = repo.resolve_target(
                target.lesson_id.as_deref(),
                target.course.as_deref(),
                target.chapter.as_deref(),
            )?;
            mineru::MineruConverter::convert_lesson(&repo, &lesson, resume)?;
            println!(
                "convert complete: target={} plain_text={}",
                lesson.target_display(),
                lesson.relative_display(&lesson.plain_text_path())
            );
        }
        Command::ConvertExams { force } => {
            let converted = mineru::MineruConverter::convert_exam_pdfs(&repo, !force)?;
            println!(
                "convert exams complete: converted={} raw={} plain={}",
                converted,
                repo.relative_display(&repo.exam_raw_dir()),
                repo.relative_display(&repo.exam_plain_root())
            );
        }
        Command::Validate { target } => {
            let lesson = repo.resolve_target(
                target.lesson_id.as_deref(),
                target.course.as_deref(),
                target.chapter.as_deref(),
            )?;
            validation::OutputValidator::new(&lesson).validate()?;
            println!("validation passed: target={}", lesson.target_display());
        }
        Command::ValidateCatalog { write } => {
            let summary =
                catalog_validation::CourseCatalogValidationService::new(&repo).validate(write)?;
            let action = if summary.written { "wrote" } else { "checked" };
            println!(
                "catalog validation {action}: ready={} blocked={} path={}",
                summary.ready,
                summary.blocked,
                repo.relative_display(&summary.path)
            );
        }
        Command::ScoreRelevance {
            target,
            target_language,
        } => {
            let lesson = repo.resolve_target(
                target.lesson_id.as_deref(),
                target.course.as_deref(),
                target.chapter.as_deref(),
            )?;
            relevance::RelevanceScorer::score(&repo, &lesson, &target_language)?;
            println!(
                "relevance scoring complete: target={} report={}",
                lesson.target_display(),
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
    CatalogValidation(catalog_validation::CatalogValidationError),
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
            Self::CatalogValidation(error) => error.fmt(f),
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
            Self::CatalogValidation(error) => Some(error),
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

impl From<catalog_validation::CatalogValidationError> for AppError {
    fn from(error: catalog_validation::CatalogValidationError) -> Self {
        Self::CatalogValidation(error)
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
