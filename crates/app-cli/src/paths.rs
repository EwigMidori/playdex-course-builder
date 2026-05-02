use std::{
    collections::BTreeSet,
    env,
    error::Error,
    fmt, fs, io,
    path::{Component, Path, PathBuf},
};

use serde::Deserialize;

const COURSE_INDEX_PATH: &str = "research/pipeline/course-index.json";
const RAW_PIPELINE_ROOT: &str = "research/pipeline/0-raw";
const PLAIN_PIPELINE_ROOT: &str = "research/pipeline/1-plain";
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

    pub fn course_index_path(&self) -> PathBuf {
        self.root.join(COURSE_INDEX_PATH)
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

    pub fn exam_plain_root(&self) -> PathBuf {
        self.root.join(PLAIN_PIPELINE_ROOT).join("exam")
    }

    pub fn exam_plain_output_dir(&self, pdf_stem: &str) -> PathBuf {
        self.exam_plain_root().join(pdf_stem)
    }

    pub fn exam_plain_text_path(&self, pdf_stem: &str) -> PathBuf {
        self.exam_plain_output_dir(pdf_stem).join("plain.txt")
    }

    pub fn resolve_target(
        &self,
        lesson_id: Option<&str>,
        course_id: Option<&str>,
        chapter_id: Option<&str>,
    ) -> Result<ChapterPaths, LessonPathError> {
        match (course_id, chapter_id) {
            (Some(course_id), Some(chapter_id)) => {
                let chapter = self.resolve_course_chapter(course_id, chapter_id)?;
                if let Some(lesson_id) = lesson_id {
                    validate_lesson_id(lesson_id)?;
                    if chapter.lesson_id() != lesson_id {
                        return Err(LessonPathError::TargetMismatch {
                            lesson_id: lesson_id.to_owned(),
                            course_id: course_id.to_owned(),
                            chapter_id: chapter_id.to_owned(),
                            indexed_lesson_id: chapter.lesson_id().to_owned(),
                        });
                    }
                }
                Ok(chapter)
            }
            (Some(_), None) => Err(LessonPathError::IncompleteTarget {
                missing: "chapter",
            }),
            (None, Some(_)) => Err(LessonPathError::IncompleteTarget {
                missing: "course",
            }),
            (None, None) => {
                let Some(lesson_id) = lesson_id else {
                    return Err(LessonPathError::MissingTarget);
                };
                self.resolve_lesson(lesson_id)
            }
        }
    }

    pub fn resolve_lesson(&self, lesson_id: &str) -> Result<ChapterPaths, LessonPathError> {
        validate_lesson_id(lesson_id)?;

        let index = self.load_course_index()?;

        let mut matches = Vec::new();
        for course in &index.courses {
            for chapter in &course.chapters {
                if chapter.lesson_id == lesson_id {
                    matches.push((course, chapter));
                }
            }
        }

        match matches.len() {
            0 => Err(LessonPathError::UnknownLessonId {
                lesson_id: lesson_id.to_owned(),
            }),
            1 => self.chapter_from_index(matches[0].0, matches[0].1),
            _ => Err(LessonPathError::AmbiguousLessonId {
                lesson_id: lesson_id.to_owned(),
                matches: matches
                    .into_iter()
                    .map(|(course, chapter)| format!("{}:{}", course.course_id, chapter.chapter_id))
                    .collect(),
            }),
        }
    }

    pub fn resolve_course_chapter(
        &self,
        course_id: &str,
        chapter_id: &str,
    ) -> Result<ChapterPaths, LessonPathError> {
        validate_scoped_id("course ID", course_id)?;
        validate_scoped_id("chapter ID", chapter_id)?;

        let index = self.load_course_index()?;

        let course = index
            .courses
            .iter()
            .find(|entry| entry.course_id == course_id)
            .ok_or_else(|| LessonPathError::UnknownCourseId {
                course_id: course_id.to_owned(),
            })?;

        let chapter = course
            .chapters
            .iter()
            .find(|entry| entry.chapter_id == chapter_id)
            .ok_or_else(|| LessonPathError::UnknownChapterId {
                course_id: course_id.to_owned(),
                chapter_id: chapter_id.to_owned(),
            })?;

        self.chapter_from_index(course, chapter)
    }

    pub fn relative_display(&self, path: &Path) -> String {
        path.strip_prefix(&self.root)
            .ok()
            .map(path_to_forward_slashes)
            .unwrap_or_else(|| path_to_forward_slashes(path))
    }

    fn load_course_index(&self) -> Result<CourseIndex, LessonPathError> {
        let path = self.course_index_path();
        let content = match fs::read_to_string(&path) {
            Ok(content) => content,
            Err(source) if source.kind() == io::ErrorKind::NotFound => {
                return Err(LessonPathError::MissingCourseIndex { path })
            }
            Err(source) => return Err(LessonPathError::ReadCourseIndex { path, source }),
        };

        let index: CourseIndex =
            serde_json::from_str(&content).map_err(|source| LessonPathError::ParseCourseIndex {
                path: path.clone(),
                source,
            })?;
        validate_course_index(&path, &index)?;
        Ok(index)
    }

    fn chapter_from_index(
        &self,
        course: &CourseIndexEntry,
        chapter: &ChapterIndexEntry,
    ) -> Result<ChapterPaths, LessonPathError> {
        let lesson_id = chapter.lesson_id.as_str();
        validate_lesson_id(lesson_id)?;

        let raw_pdf_relative = chapter
            .raw_pdf_path
            .as_deref()
            .map(|path| parse_repo_relative_path("rawPdfPath", path))
            .transpose()?
            .unwrap_or_else(|| PathBuf::from(RAW_PIPELINE_ROOT).join(format!("{lesson_id}.pdf")));
        let plain_output_relative = chapter
            .plain_output_dir
            .as_deref()
            .map(|path| parse_repo_relative_path("plainOutputDir", path))
            .transpose()?
            .unwrap_or_else(|| PathBuf::from(PLAIN_PIPELINE_ROOT).join(lesson_id));
        let guided_story_relative =
            parse_repo_relative_path("guidedStoryDir", &chapter.guided_story_dir)?;
        let textbook_relative = chapter
            .textbook_path
            .as_deref()
            .map(|path| parse_repo_relative_path("textbookPath", path))
            .transpose()?
            .unwrap_or_else(|| PathBuf::from(TEXTBOOK_ROOT).join(format!("{lesson_id}.mdx")));
        let meta_relative = chapter
            .meta_dir
            .as_deref()
            .map(|path| parse_repo_relative_path("metaDir", path))
            .transpose()?
            .unwrap_or_else(|| PathBuf::from(META_PIPELINE_ROOT).join(lesson_id));
        let related_important_relative = course
            .related_important_path
            .as_deref()
            .map(|path| parse_repo_relative_path("relatedImportantPath", path))
            .transpose()?;
        let exam_raw_relative = course
            .exam_raw_dir
            .as_deref()
            .map(|path| parse_repo_relative_path("examRawDir", path))
            .transpose()?;
        let exam_plain_relative = course
            .exam_plain_root
            .as_deref()
            .map(|path| parse_repo_relative_path("examPlainRoot", path))
            .transpose()?;

        Ok(ChapterPaths {
            repo: self.clone(),
            course_id: Some(course.course_id.clone()),
            chapter_id: Some(chapter.chapter_id.clone()),
            course_title: Some(course.title.clone()),
            chapter_title: Some(chapter.title.clone()),
            lesson_id: lesson_id.to_owned(),
            raw_pdf_relative,
            plain_output_relative,
            guided_story_relative,
            textbook_relative,
            meta_relative,
            related_important_relative,
            exam_raw_relative,
            exam_plain_relative,
        })
    }
}

#[derive(Clone, Debug)]
pub struct ChapterPaths {
    repo: RepoPaths,
    course_id: Option<String>,
    chapter_id: Option<String>,
    course_title: Option<String>,
    chapter_title: Option<String>,
    lesson_id: String,
    raw_pdf_relative: PathBuf,
    plain_output_relative: PathBuf,
    guided_story_relative: PathBuf,
    textbook_relative: PathBuf,
    meta_relative: PathBuf,
    related_important_relative: Option<PathBuf>,
    exam_raw_relative: Option<PathBuf>,
    exam_plain_relative: Option<PathBuf>,
}

pub type LessonPaths = ChapterPaths;

impl ChapterPaths {
    pub fn repo(&self) -> &RepoPaths {
        &self.repo
    }

    pub fn repo_root(&self) -> &Path {
        self.repo.root()
    }

    pub fn course_id(&self) -> Option<&str> {
        self.course_id.as_deref()
    }

    pub fn chapter_id(&self) -> Option<&str> {
        self.chapter_id.as_deref()
    }

    pub fn chapter_title(&self) -> Option<&str> {
        self.chapter_title.as_deref()
    }

    pub fn course_title(&self) -> Option<&str> {
        self.course_title.as_deref()
    }

    pub fn lesson_id(&self) -> &str {
        &self.lesson_id
    }

    pub fn target_display(&self) -> String {
        match (self.course_id(), self.chapter_id()) {
            (Some(course_id), Some(chapter_id)) => {
                format!("{course_id}/{chapter_id} (lesson {})", self.lesson_id())
            }
            _ => self.lesson_id().to_owned(),
        }
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
        self.repo.root().join(&self.plain_output_relative)
    }

    pub fn plain_text_path(&self) -> PathBuf {
        self.plain_output_dir().join("plain.txt")
    }

    pub fn guided_story_dir(&self) -> PathBuf {
        self.repo.root().join(&self.guided_story_relative)
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
        self.repo.root().join(&self.textbook_relative)
    }

    pub fn related_important_path(&self) -> PathBuf {
        self.related_important_relative
            .as_ref()
            .map(|relative| self.repo.root().join(relative))
            .unwrap_or_else(|| self.repo.related_important_path())
    }

    pub fn meta_dir(&self) -> PathBuf {
        self.repo.root().join(&self.meta_relative)
    }

    pub fn exam_raw_dir(&self) -> PathBuf {
        self.exam_raw_relative
            .as_ref()
            .map(|relative| self.repo.root().join(relative))
            .unwrap_or_else(|| self.repo.exam_raw_dir())
    }

    pub fn exam_plain_root(&self) -> PathBuf {
        self.exam_plain_relative
            .as_ref()
            .map(|relative| self.repo.root().join(relative))
            .unwrap_or_else(|| self.repo.exam_plain_root())
    }

    pub fn exam_plain_text_path(&self, pdf_stem: &str) -> PathBuf {
        self.exam_plain_root().join(pdf_stem).join("plain.txt")
    }

    pub fn audit_stage_dir(&self, stage: &str) -> PathBuf {
        self.meta_dir().join("mvp").join(stage)
    }

    pub fn relevance_dir(&self) -> PathBuf {
        self.meta_dir().join("relevance")
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
    InvalidScopedId {
        kind: &'static str,
        value: String,
        reason: &'static str,
    },
    MissingTarget,
    IncompleteTarget {
        missing: &'static str,
    },
    TargetMismatch {
        lesson_id: String,
        course_id: String,
        chapter_id: String,
        indexed_lesson_id: String,
    },
    MissingCourseIndex {
        path: PathBuf,
    },
    ReadCourseIndex {
        path: PathBuf,
        source: io::Error,
    },
    ParseCourseIndex {
        path: PathBuf,
        source: serde_json::Error,
    },
    InvalidCourseIndex {
        path: PathBuf,
        reason: String,
    },
    UnknownCourseId {
        course_id: String,
    },
    UnknownChapterId {
        course_id: String,
        chapter_id: String,
    },
    UnknownLessonId {
        lesson_id: String,
    },
    AmbiguousLessonId {
        lesson_id: String,
        matches: Vec<String>,
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
            Self::InvalidScopedId {
                kind,
                value,
                reason,
            } => write!(f, "invalid {kind} '{value}': {reason}"),
            Self::MissingTarget => write!(
                f,
                "missing target: provide a lesson ID or both --course and --chapter"
            ),
            Self::IncompleteTarget { missing } => write!(
                f,
                "incomplete target selector: missing --{missing} alongside the other chapter selector"
            ),
            Self::TargetMismatch {
                lesson_id,
                course_id,
                chapter_id,
                indexed_lesson_id,
            } => write!(
                f,
                "target selector mismatch: lesson '{lesson_id}' does not match course/chapter '{course_id}/{chapter_id}' (indexed lesson: '{indexed_lesson_id}')"
            ),
            Self::MissingCourseIndex { path } => write!(
                f,
                "missing multi-course source of truth: {}",
                path_to_forward_slashes(path)
            ),
            Self::ReadCourseIndex { path, source } => write!(
                f,
                "failed to read course index {}: {source}",
                path_to_forward_slashes(path)
            ),
            Self::ParseCourseIndex { path, source } => write!(
                f,
                "failed to parse course index {}: {source}",
                path_to_forward_slashes(path)
            ),
            Self::InvalidCourseIndex { path, reason } => write!(
                f,
                "invalid course index {}: {reason}",
                path_to_forward_slashes(path)
            ),
            Self::UnknownCourseId { course_id } => {
                write!(f, "unknown course ID '{course_id}' in course index")
            }
            Self::UnknownChapterId {
                course_id,
                chapter_id,
            } => write!(
                f,
                "unknown chapter ID '{chapter_id}' for course '{course_id}' in course index"
            ),
            Self::UnknownLessonId { lesson_id } => {
                write!(f, "unknown lesson ID '{lesson_id}' in course index")
            }
            Self::AmbiguousLessonId { lesson_id, matches } => write!(
                f,
                "lesson '{lesson_id}' maps to multiple course-index chapters ({}); use --course and --chapter",
                matches.join(", ")
            ),
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
            Self::ReadCourseIndex { source, .. } => Some(source),
            Self::ParseCourseIndex { source, .. } => Some(source),
            Self::NotRepositoryRoot { .. }
            | Self::InvalidLessonId { .. }
            | Self::InvalidScopedId { .. }
            | Self::MissingTarget
            | Self::IncompleteTarget { .. }
            | Self::TargetMismatch { .. }
            | Self::MissingCourseIndex { .. }
            | Self::InvalidCourseIndex { .. }
            | Self::UnknownCourseId { .. }
            | Self::UnknownChapterId { .. }
            | Self::UnknownLessonId { .. }
            | Self::AmbiguousLessonId { .. }
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

fn validate_scoped_id(kind: &'static str, value: &str) -> Result<(), LessonPathError> {
    if value.is_empty() {
        return Err(LessonPathError::InvalidScopedId {
            kind,
            value: value.to_owned(),
            reason: "value must not be empty",
        });
    }

    if value == "." || value == ".." {
        return Err(LessonPathError::InvalidScopedId {
            kind,
            value: value.to_owned(),
            reason: "value must not be '.' or '..'",
        });
    }

    if value.contains('/') || value.contains('\\') {
        return Err(LessonPathError::InvalidScopedId {
            kind,
            value: value.to_owned(),
            reason: "value must not contain path separators",
        });
    }

    Ok(())
}

fn parse_repo_relative_path(
    field: &'static str,
    value: &str,
) -> Result<PathBuf, LessonPathError> {
    if value.is_empty() {
        return Err(LessonPathError::InvalidCourseIndex {
            path: PathBuf::from(COURSE_INDEX_PATH),
            reason: format!("{field} must not be empty"),
        });
    }

    let path = PathBuf::from(value);
    if path.is_absolute() {
        return Err(LessonPathError::InvalidCourseIndex {
            path: PathBuf::from(COURSE_INDEX_PATH),
            reason: format!("{field} must be repo-relative, got absolute path '{value}'"),
        });
    }

    for component in path.components() {
        if matches!(component, Component::ParentDir) {
            return Err(LessonPathError::InvalidCourseIndex {
                path: PathBuf::from(COURSE_INDEX_PATH),
                reason: format!("{field} must not traverse parent directories: '{value}'"),
            });
        }
    }

    Ok(path)
}

fn validate_course_index(path: &Path, index: &CourseIndex) -> Result<(), LessonPathError> {
    if index.version != 1 {
        return Err(LessonPathError::InvalidCourseIndex {
            path: path.to_path_buf(),
            reason: format!("unsupported version {}; expected 1", index.version),
        });
    }

    let mut course_ids = BTreeSet::new();
    for course in &index.courses {
        validate_scoped_id("course ID", &course.course_id)?;
        if !course_ids.insert(course.course_id.clone()) {
            return Err(LessonPathError::InvalidCourseIndex {
                path: path.to_path_buf(),
                reason: format!("duplicate courseId '{}'", course.course_id),
            });
        }

        let mut chapter_ids = BTreeSet::new();
        for chapter in &course.chapters {
            validate_scoped_id("chapter ID", &chapter.chapter_id)?;
            validate_lesson_id(&chapter.lesson_id)?;
            parse_repo_relative_path("guidedStoryDir", &chapter.guided_story_dir).map_err(
                |error| match error {
                    LessonPathError::InvalidCourseIndex { reason, .. } => {
                        LessonPathError::InvalidCourseIndex {
                            path: path.to_path_buf(),
                            reason,
                        }
                    }
                    other => other,
                },
            )?;
            if let Some(textbook_path) = &chapter.textbook_path {
                parse_repo_relative_path("textbookPath", textbook_path).map_err(|error| {
                    match error {
                        LessonPathError::InvalidCourseIndex { reason, .. } => {
                            LessonPathError::InvalidCourseIndex {
                                path: path.to_path_buf(),
                                reason,
                            }
                        }
                        other => other,
                    }
                })?;
            }
            if let Some(raw_pdf_path) = &chapter.raw_pdf_path {
                parse_repo_relative_path("rawPdfPath", raw_pdf_path).map_err(|error| match error {
                    LessonPathError::InvalidCourseIndex { reason, .. } => {
                        LessonPathError::InvalidCourseIndex {
                            path: path.to_path_buf(),
                            reason,
                        }
                    }
                    other => other,
                })?;
            }
            if let Some(plain_output_dir) = &chapter.plain_output_dir {
                parse_repo_relative_path("plainOutputDir", plain_output_dir).map_err(
                    |error| match error {
                        LessonPathError::InvalidCourseIndex { reason, .. } => {
                            LessonPathError::InvalidCourseIndex {
                                path: path.to_path_buf(),
                                reason,
                            }
                        }
                        other => other,
                    },
                )?;
            }
            if let Some(meta_dir) = &chapter.meta_dir {
                parse_repo_relative_path("metaDir", meta_dir).map_err(|error| match error {
                    LessonPathError::InvalidCourseIndex { reason, .. } => {
                        LessonPathError::InvalidCourseIndex {
                            path: path.to_path_buf(),
                            reason,
                        }
                    }
                    other => other,
                })?;
            }

            if !chapter_ids.insert(chapter.chapter_id.clone()) {
                return Err(LessonPathError::InvalidCourseIndex {
                    path: path.to_path_buf(),
                    reason: format!(
                        "duplicate chapterId '{}' inside course '{}'",
                        chapter.chapter_id, course.course_id
                    ),
                });
            }
        }

        if let Some(related_important_path) = &course.related_important_path {
            parse_repo_relative_path("relatedImportantPath", related_important_path).map_err(
                |error| match error {
                    LessonPathError::InvalidCourseIndex { reason, .. } => {
                        LessonPathError::InvalidCourseIndex {
                            path: path.to_path_buf(),
                            reason,
                        }
                    }
                    other => other,
                },
            )?;
        }
        if let Some(exam_raw_dir) = &course.exam_raw_dir {
            parse_repo_relative_path("examRawDir", exam_raw_dir).map_err(|error| match error {
                LessonPathError::InvalidCourseIndex { reason, .. } => {
                    LessonPathError::InvalidCourseIndex {
                        path: path.to_path_buf(),
                        reason,
                    }
                }
                other => other,
            })?;
        }
        if let Some(exam_plain_root) = &course.exam_plain_root {
            parse_repo_relative_path("examPlainRoot", exam_plain_root).map_err(|error| {
                match error {
                    LessonPathError::InvalidCourseIndex { reason, .. } => {
                        LessonPathError::InvalidCourseIndex {
                            path: path.to_path_buf(),
                            reason,
                        }
                    }
                    other => other,
                }
            })?;
        }
    }

    Ok(())
}

fn looks_like_repo_root(root: &Path) -> bool {
    root.join(".git").exists()
        && root.join(".ci/agent/AGENTS.md").is_file()
        && root.join("docs/progress.json").is_file()
        && root.join("research/README.md").is_file()
}

#[derive(Clone, Debug, Deserialize)]
struct CourseIndex {
    version: u64,
    courses: Vec<CourseIndexEntry>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CourseIndexEntry {
    course_id: String,
    title: String,
    chapters: Vec<ChapterIndexEntry>,
    related_important_path: Option<String>,
    exam_raw_dir: Option<String>,
    exam_plain_root: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ChapterIndexEntry {
    chapter_id: String,
    title: String,
    lesson_id: String,
    guided_story_dir: String,
    textbook_path: Option<String>,
    raw_pdf_path: Option<String>,
    plain_output_dir: Option<String>,
    meta_dir: Option<String>,
}
