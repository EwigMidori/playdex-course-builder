use std::{
    env,
    error::Error,
    fmt, io,
    path::{Path, PathBuf},
};

const RAW_PIPELINE_ROOT: &str = "research/pipeline/0-raw";
const PLAIN_PIPELINE_ROOT: &str = "research/pipeline/1-plain";
const GUIDED_STORY_ROOT: &str = "research/pipeline/3-guided_story";
const TEXTBOOK_ROOT: &str = "research/pipeline/5-textbook";
const META_PIPELINE_ROOT: &str = "research/pipeline/meta";

#[derive(Clone, Debug)]
pub struct RepoPaths {
    root: PathBuf,
}

impl RepoPaths {
    pub fn from_current_dir() -> Result<Self, LessonPathError> {
        let root = env::current_dir().map_err(LessonPathError::CurrentDirectory)?;
        Self::from_root(root)
    }

    pub fn from_root(root: PathBuf) -> Result<Self, LessonPathError> {
        if !looks_like_repo_root(&root) {
            return Err(LessonPathError::NotRepositoryRoot { cwd: root });
        }

        Ok(Self { root })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn dotenv_path(&self) -> PathBuf {
        self.root.join(".env")
    }

    pub fn default_mineru_token_file_path(&self) -> PathBuf {
        self.root.join("mineru_token.txt")
    }

    pub fn prompts_dir(&self) -> PathBuf {
        self.root.join("research/prompts")
    }

    pub fn related_important_path(&self) -> PathBuf {
        self.root
            .join("research/pipeline/2-related_important/course_desc.md")
    }

    pub fn exam_raw_dir(&self) -> PathBuf {
        self.root.join(RAW_PIPELINE_ROOT).join("exam")
    }

    pub fn resolve_lesson(&self, lesson_id: &str) -> Result<LessonPaths, LessonPathError> {
        validate_lesson_id(lesson_id)?;

        Ok(LessonPaths {
            repo: self.clone(),
            lesson_id: lesson_id.to_owned(),
            raw_pdf_relative: PathBuf::from(RAW_PIPELINE_ROOT).join(format!("{lesson_id}.pdf")),
        })
    }

    pub fn relative_display(&self, path: &Path) -> String {
        path.strip_prefix(&self.root)
            .ok()
            .map(path_to_forward_slashes)
            .unwrap_or_else(|| path_to_forward_slashes(path))
    }
}

#[derive(Clone, Debug)]
pub struct LessonPaths {
    repo: RepoPaths,
    lesson_id: String,
    raw_pdf_relative: PathBuf,
}

impl LessonPaths {
    pub fn repo(&self) -> &RepoPaths {
        &self.repo
    }

    pub fn repo_root(&self) -> &Path {
        self.repo.root()
    }

    pub fn lesson_id(&self) -> &str {
        &self.lesson_id
    }

    pub fn raw_pdf_relative(&self) -> &Path {
        &self.raw_pdf_relative
    }

    pub fn raw_pdf_path(&self) -> PathBuf {
        self.repo.root().join(self.raw_pdf_relative())
    }

    pub fn checked_raw_pdf_path(&self) -> Result<PathBuf, LessonPathError> {
        let raw_pdf_path = self.raw_pdf_path();

        if raw_pdf_path.is_file() {
            Ok(raw_pdf_path)
        } else {
            Err(LessonPathError::MissingRawPdf {
                lesson_id: self.lesson_id.clone(),
                raw_pdf_relative: self.raw_pdf_relative.clone(),
            })
        }
    }

    pub fn plain_output_dir(&self) -> PathBuf {
        self.repo
            .root()
            .join(PLAIN_PIPELINE_ROOT)
            .join(&self.lesson_id)
    }

    pub fn plain_text_path(&self) -> PathBuf {
        self.plain_output_dir().join("plain.txt")
    }

    pub fn guided_story_dir(&self) -> PathBuf {
        self.repo
            .root()
            .join(GUIDED_STORY_ROOT)
            .join(&self.lesson_id)
    }

    pub fn guided_story_manifest_path(&self) -> PathBuf {
        self.guided_story_dir().join("manifest.json")
    }

    pub fn guided_story_step_dir(&self, step: usize) -> PathBuf {
        self.guided_story_dir().join(format!("step{step}"))
    }

    pub fn guided_story_step_path(&self, step: usize) -> PathBuf {
        self.guided_story_step_dir(step).join("step.json")
    }

    pub fn textbook_path(&self) -> PathBuf {
        self.repo
            .root()
            .join(TEXTBOOK_ROOT)
            .join(format!("{}.mdx", self.lesson_id))
    }

    pub fn audit_stage_dir(&self, stage: &str) -> PathBuf {
        self.repo
            .root()
            .join(META_PIPELINE_ROOT)
            .join(&self.lesson_id)
            .join("mvp")
            .join(stage)
    }

    pub fn relevance_dir(&self) -> PathBuf {
        self.repo
            .root()
            .join(META_PIPELINE_ROOT)
            .join(&self.lesson_id)
            .join("relevance")
    }

    pub fn relevance_report_path(&self) -> PathBuf {
        self.relevance_dir().join("report.json")
    }

    pub fn relative_display(&self, path: &Path) -> String {
        self.repo.relative_display(path)
    }
}

#[derive(Debug)]
pub enum LessonPathError {
    CurrentDirectory(io::Error),
    NotRepositoryRoot {
        cwd: PathBuf,
    },
    InvalidLessonId {
        lesson_id: String,
        reason: &'static str,
    },
    MissingRawPdf {
        lesson_id: String,
        raw_pdf_relative: PathBuf,
    },
}

impl fmt::Display for LessonPathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CurrentDirectory(source) => {
                write!(f, "failed to read the current working directory: {source}")
            }
            Self::NotRepositoryRoot { cwd } => write!(
                f,
                "repository root required: run this command from the repository root (current directory: {})",
                cwd.display()
            ),
            Self::InvalidLessonId { lesson_id, reason } => {
                write!(f, "invalid lesson ID '{lesson_id}': {reason}")
            }
            Self::MissingRawPdf {
                lesson_id,
                raw_pdf_relative,
            } => write!(
                f,
                "missing raw PDF for lesson '{lesson_id}': {}; provide this file or run with a configured MinerU setup",
                path_to_forward_slashes(raw_pdf_relative)
            ),
        }
    }
}

impl Error for LessonPathError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::CurrentDirectory(source) => Some(source),
            Self::NotRepositoryRoot { .. }
            | Self::InvalidLessonId { .. }
            | Self::MissingRawPdf { .. } => None,
        }
    }
}

pub fn path_to_forward_slashes(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn validate_lesson_id(lesson_id: &str) -> Result<(), LessonPathError> {
    if lesson_id.is_empty() {
        return Err(LessonPathError::InvalidLessonId {
            lesson_id: lesson_id.to_owned(),
            reason: "lesson ID must not be empty",
        });
    }

    if lesson_id == "." || lesson_id == ".." {
        return Err(LessonPathError::InvalidLessonId {
            lesson_id: lesson_id.to_owned(),
            reason: "lesson ID must not be '.' or '..'",
        });
    }

    if lesson_id.contains('/') || lesson_id.contains('\\') {
        return Err(LessonPathError::InvalidLessonId {
            lesson_id: lesson_id.to_owned(),
            reason: "lesson ID must not contain path separators",
        });
    }

    Ok(())
}

fn looks_like_repo_root(root: &Path) -> bool {
    root.join(".git").exists()
        && root.join(".ci/agent/AGENTS.md").is_file()
        && root.join("docs/progress.json").is_file()
        && root.join("research/README.md").is_file()
}
