mod cli;

use clap::Parser;
use std::process::ExitCode;

use crate::cli::Cli;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), pipeline_core::LessonPathError> {
    let cli = Cli::parse();
    let lesson = pipeline_core::resolve_lesson(cli.command.lesson_id())?;

    println!(
        "placeholder: command={} stage={} lesson={} raw_pdf={}",
        cli.command.command_name(),
        cli.command.stage_name(),
        lesson.lesson_id(),
        lesson.raw_pdf_relative_string()
    );

    Ok(())
}
