use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(name = "coursegen")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Run { lesson_id: String },
    Convert { lesson_id: String },
    Generate { stage: Stage, lesson_id: String },
    Review { stage: Stage, lesson_id: String },
    Validate { stage: Stage, lesson_id: String },
}

impl Command {
    pub fn command_name(&self) -> &'static str {
        match self {
            Self::Run { .. } => "run",
            Self::Convert { .. } => "convert",
            Self::Generate { .. } => "generate",
            Self::Review { .. } => "review",
            Self::Validate { .. } => "validate",
        }
    }

    pub fn lesson_id(&self) -> &str {
        match self {
            Self::Run { lesson_id }
            | Self::Convert { lesson_id }
            | Self::Generate { lesson_id, .. }
            | Self::Review { lesson_id, .. }
            | Self::Validate { lesson_id, .. } => lesson_id,
        }
    }

    pub fn stage_name(&self) -> &'static str {
        match self {
            Self::Run { .. } | Self::Convert { .. } => "-",
            Self::Generate { stage, .. }
            | Self::Review { stage, .. }
            | Self::Validate { stage, .. } => stage.as_str(),
        }
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum Stage {
    #[value(name = "guided-story")]
    GuidedStory,
    #[value(name = "question-bank")]
    QuestionBank,
    #[value(name = "textbook")]
    Textbook,
}

impl Stage {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::GuidedStory => "guided-story",
            Self::QuestionBank => "question-bank",
            Self::Textbook => "textbook",
        }
    }
}
