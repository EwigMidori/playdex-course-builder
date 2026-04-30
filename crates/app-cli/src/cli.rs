use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "coursegen")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Run {
        lesson_id: String,
        #[arg(long, default_value = "zh-CN")]
        target_language: String,
    },
    Convert {
        lesson_id: String,
        #[arg(long)]
        resume: bool,
    },
    Validate {
        lesson_id: String,
    },
    ScoreRelevance {
        lesson_id: String,
        #[arg(long, default_value = "zh-CN")]
        target_language: String,
    },
}
