use std::path::PathBuf;

use crate::lesson::LessonPaths;

const META_PIPELINE_ROOT: &str = "research/pipeline/meta";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StageName {
    Convert,
}

impl StageName {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Convert => "convert",
        }
    }
}

impl LessonPaths {
    pub fn meta_root_dir(&self) -> PathBuf {
        self.repo_root()
            .join(META_PIPELINE_ROOT)
            .join(self.lesson_id())
    }

    pub fn run_root_dir(&self) -> PathBuf {
        self.meta_root_dir().join("runs")
    }

    pub fn run_dir(&self, run_id: &str) -> PathBuf {
        self.run_root_dir().join(run_id)
    }

    pub fn run_metadata_path(&self, run_id: &str) -> PathBuf {
        self.run_dir(run_id).join("run.json")
    }

    pub fn stage_state_path(&self, run_id: &str, stage: StageName) -> PathBuf {
        self.run_dir(run_id)
            .join("stages")
            .join(format!("{}.json", stage.as_str()))
    }
}
