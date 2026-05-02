use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(name = "coursegen")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Clone, Debug, Args)]
pub struct TargetArgs {
    pub lesson_id: Option<String>,
    #[arg(long)]
    pub course: Option<String>,
    #[arg(long)]
    pub chapter: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Run {
        #[command(flatten)]
        target: TargetArgs,
        #[arg(long, default_value = "zh-CN")]
        target_language: String,
        #[arg(long, value_enum, default_value = "none")]
        force_stage: ForceStage,
    },
    Convert {
        #[command(flatten)]
        target: TargetArgs,
        #[arg(long)]
        resume: bool,
    },
    ConvertExams {
        #[arg(long)]
        force: bool,
    },
    Validate {
        #[command(flatten)]
        target: TargetArgs,
    },
    ScoreRelevance {
        #[command(flatten)]
        target: TargetArgs,
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
