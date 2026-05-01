use clap::{Parser, Subcommand, ValueEnum};

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
        #[arg(long, value_enum, default_value = "none")]
        force_stage: ForceStage,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum ForceStage {
    None,
    Convert,
    GuidedStory,
    QuestionBank,
    Textbook,
    All,
}
