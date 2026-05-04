use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fmt, fs, io,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::paths::{PathText, RepoPaths};

pub struct CourseCatalogValidationService<'a> {
    repo: &'a RepoPaths,
}

impl<'a> CourseCatalogValidationService<'a> {
    pub fn new(repo: &'a RepoPaths) -> Self {
        Self { repo }
    }

    pub fn validate(
        &self,
        write: bool,
    ) -> Result<CourseCatalogValidationSummary, CatalogValidationError> {
        let path = self.repo.course_index_path();
        let content = fs::read_to_string(&path).map_err(|source| CatalogValidationError::Read {
            path: path.clone(),
            source,
        })?;
        let mut index: CourseIndex =
            serde_json::from_str(&content).map_err(|source| CatalogValidationError::Parse {
                path: path.clone(),
                source,
            })?;

        let duplicate_course_ids =
            Self::duplicates(index.courses.iter().map(|course| course.course_id.as_str()));
        let duplicate_lesson_ids = Self::duplicates(index.courses.iter().flat_map(|course| {
            course
                .chapters
                .iter()
                .map(|chapter| chapter.lesson_id.as_str())
        }));

        for course in &mut index.courses {
            let errors = self.validate_course(course, &duplicate_course_ids, &duplicate_lesson_ids);
            course.status = if errors.is_empty() {
                CourseStatus::Ready
            } else {
                CourseStatus::Blocked
            };
            course.validation_errors = errors;
        }

        if write {
            let serialized =
                serde_json::to_string_pretty(&index).map_err(CatalogValidationError::Serialize)?;
            fs::write(&path, format!("{serialized}\n")).map_err(|source| {
                CatalogValidationError::Write {
                    path: path.clone(),
                    source,
                }
            })?;
        }

        let ready = index
            .courses
            .iter()
            .filter(|course| course.status == CourseStatus::Ready)
            .count();
        let blocked = index.courses.len().saturating_sub(ready);

        if blocked > 0 {
            return Err(CatalogValidationError::BlockedCourses {
                path,
                ready,
                blocked,
                written: write,
            });
        }

        Ok(CourseCatalogValidationSummary {
            path,
            ready,
            blocked,
            written: write,
        })
    }

    fn validate_course(
        &self,
        course: &CourseIndexEntry,
        duplicate_course_ids: &BTreeSet<String>,
        duplicate_lesson_ids: &BTreeSet<String>,
    ) -> Vec<CourseValidationError> {
        let mut errors = Vec::new();

        if course.course_id.trim().is_empty() {
            errors.push(CourseValidationError::course(
                "MISSING_COURSE_ID",
                "Course is missing courseId.",
            ));
        }
        if course.title.trim().is_empty() {
            errors.push(CourseValidationError::course(
                "MISSING_COURSE_TITLE",
                "Course is missing title.",
            ));
        }
        if duplicate_course_ids.contains(course.course_id.as_str()) {
            errors.push(CourseValidationError::course(
                "DUPLICATE_COURSE_ID",
                format!("Duplicate courseId '{}'.", course.course_id),
            ));
        }
        if course.chapters.is_empty() {
            errors.push(CourseValidationError::course(
                "EMPTY_COURSE_CHAPTERS",
                "Course has no chapters.",
            ));
        }

        let duplicate_chapter_ids = Self::duplicates(
            course
                .chapters
                .iter()
                .map(|chapter| chapter.chapter_id.as_str()),
        );
        for chapter in &course.chapters {
            if duplicate_chapter_ids.contains(chapter.chapter_id.as_str()) {
                errors.push(CourseValidationError::chapter(
                    "DUPLICATE_CHAPTER_ID",
                    &chapter.chapter_id,
                    format!(
                        "Duplicate chapterId '{}' inside course '{}'.",
                        chapter.chapter_id, course.course_id
                    ),
                ));
            }
            if duplicate_lesson_ids.contains(chapter.lesson_id.as_str()) {
                errors.push(CourseValidationError::chapter(
                    "DUPLICATE_LESSON_ID",
                    &chapter.chapter_id,
                    format!("Duplicate lessonId '{}'.", chapter.lesson_id),
                ));
            }
            self.validate_chapter(course, chapter, &mut errors);
        }

        errors
    }

    fn validate_chapter(
        &self,
        course: &CourseIndexEntry,
        chapter: &ChapterIndexEntry,
        errors: &mut Vec<CourseValidationError>,
    ) {
        if chapter.chapter_id.trim().is_empty() {
            errors.push(CourseValidationError::chapter(
                "MISSING_CHAPTER_ID",
                &chapter.chapter_id,
                "Chapter is missing chapterId.",
            ));
        }
        if chapter.title.trim().is_empty() {
            errors.push(CourseValidationError::chapter(
                "MISSING_CHAPTER_TITLE",
                &chapter.chapter_id,
                "Chapter is missing title.",
            ));
        }
        if chapter.lesson_id.trim().is_empty() {
            errors.push(CourseValidationError::chapter(
                "MISSING_LESSON_ID",
                &chapter.chapter_id,
                "Chapter is missing lessonId.",
            ));
        }
        if chapter.guided_story_dir.trim().is_empty() {
            errors.push(CourseValidationError::chapter(
                "MISSING_GUIDED_STORY_DIR",
                &chapter.chapter_id,
                "Chapter is missing guidedStoryDir.",
            ));
            return;
        }

        let guided_story_dir = self.repo_path(&chapter.guided_story_dir);
        let manifest_path = guided_story_dir.join("manifest.json");
        let Some(manifest) = self.read_json(
            &manifest_path,
            "MISSING_MANIFEST",
            "INVALID_MANIFEST_JSON",
            &chapter.chapter_id,
            errors,
        ) else {
            self.validate_textbook(chapter, errors);
            return;
        };

        self.validate_identity(
            &manifest,
            "lesson_id",
            &chapter.lesson_id,
            "manifest",
            &manifest_path,
            &chapter.chapter_id,
            errors,
        );
        self.validate_identity(
            &manifest,
            "course_id",
            &course.course_id,
            "manifest",
            &manifest_path,
            &chapter.chapter_id,
            errors,
        );
        self.validate_identity(
            &manifest,
            "chapter_id",
            &chapter.chapter_id,
            "manifest",
            &manifest_path,
            &chapter.chapter_id,
            errors,
        );

        let Some(steps) = manifest.get("steps").and_then(Value::as_array) else {
            errors.push(CourseValidationError::chapter_path(
                "EMPTY_MANIFEST_STEPS",
                &chapter.chapter_id,
                self.repo.relative_display(&manifest_path),
                "Manifest must contain a non-empty steps array.",
            ));
            self.validate_textbook(chapter, errors);
            return;
        };

        if steps.is_empty() {
            errors.push(CourseValidationError::chapter_path(
                "EMPTY_MANIFEST_STEPS",
                &chapter.chapter_id,
                self.repo.relative_display(&manifest_path),
                "Manifest must contain a non-empty steps array.",
            ));
        }

        let mut sequence_ids = BTreeSet::new();
        for (index, step) in steps.iter().enumerate() {
            let Some(sequence_id) = step.get("sequence_id").and_then(Value::as_str) else {
                errors.push(CourseValidationError::chapter_path(
                    "MISSING_STEP_SEQUENCE_ID",
                    &chapter.chapter_id,
                    self.repo.relative_display(&manifest_path),
                    format!("Manifest step {} is missing sequence_id.", index + 1),
                ));
                continue;
            };
            if sequence_id.trim().is_empty() {
                errors.push(CourseValidationError::chapter_path(
                    "MISSING_STEP_SEQUENCE_ID",
                    &chapter.chapter_id,
                    self.repo.relative_display(&manifest_path),
                    format!("Manifest step {} has an empty sequence_id.", index + 1),
                ));
                continue;
            }
            if !sequence_ids.insert(sequence_id.to_owned()) {
                errors.push(CourseValidationError::chapter_path(
                    "DUPLICATE_STEP_SEQUENCE_ID",
                    &chapter.chapter_id,
                    self.repo.relative_display(&manifest_path),
                    format!("Duplicate step sequence_id '{sequence_id}'."),
                ));
            }
            self.validate_step(course, chapter, &guided_story_dir, sequence_id, errors);
        }

        self.validate_textbook(chapter, errors);
    }

    fn validate_step(
        &self,
        course: &CourseIndexEntry,
        chapter: &ChapterIndexEntry,
        guided_story_dir: &Path,
        sequence_id: &str,
        errors: &mut Vec<CourseValidationError>,
    ) {
        let step_dir = guided_story_dir.join(sequence_id);
        let step_path = step_dir.join("step.json");
        let Some(step) = self.read_json(
            &step_path,
            "MISSING_STEP_FILE",
            "INVALID_STEP_JSON",
            &chapter.chapter_id,
            errors,
        ) else {
            return;
        };

        self.validate_identity(
            &step,
            "lesson_id",
            &chapter.lesson_id,
            "step",
            &step_path,
            &chapter.chapter_id,
            errors,
        );
        self.validate_identity(
            &step,
            "course_id",
            &course.course_id,
            "step",
            &step_path,
            &chapter.chapter_id,
            errors,
        );
        self.validate_identity(
            &step,
            "chapter_id",
            &chapter.chapter_id,
            "step",
            &step_path,
            &chapter.chapter_id,
            errors,
        );
        self.validate_identity(
            &step,
            "sequence_id",
            sequence_id,
            "step",
            &step_path,
            &chapter.chapter_id,
            errors,
        );

        let question_bank_path = step_dir.join("question_bank.json");
        let Some(question_bank) = self.read_json(
            &question_bank_path,
            "MISSING_QUESTION_BANK",
            "INVALID_QUESTION_BANK_JSON",
            &chapter.chapter_id,
            errors,
        ) else {
            return;
        };

        self.validate_identity(
            &question_bank,
            "lesson_id",
            &chapter.lesson_id,
            "question bank",
            &question_bank_path,
            &chapter.chapter_id,
            errors,
        );
        self.validate_identity(
            &question_bank,
            "course_id",
            &course.course_id,
            "question bank",
            &question_bank_path,
            &chapter.chapter_id,
            errors,
        );
        self.validate_identity(
            &question_bank,
            "chapter_id",
            &chapter.chapter_id,
            "question bank",
            &question_bank_path,
            &chapter.chapter_id,
            errors,
        );
        self.validate_identity(
            &question_bank,
            "sequence_id",
            sequence_id,
            "question bank",
            &question_bank_path,
            &chapter.chapter_id,
            errors,
        );
    }

    fn validate_textbook(
        &self,
        chapter: &ChapterIndexEntry,
        errors: &mut Vec<CourseValidationError>,
    ) {
        let Some(textbook_path) = &chapter.textbook_path else {
            return;
        };
        let path = self.repo_path(textbook_path);
        if !path.is_file() {
            errors.push(CourseValidationError::chapter_path(
                "MISSING_TEXTBOOK",
                &chapter.chapter_id,
                self.repo.relative_display(&path),
                "Declared textbookPath is missing.",
            ));
        }
    }

    fn validate_identity(
        &self,
        value: &Value,
        field: &'static str,
        expected: &str,
        label: &'static str,
        path: &Path,
        chapter_id: &str,
        errors: &mut Vec<CourseValidationError>,
    ) {
        let actual = value.get(field).and_then(Value::as_str);
        if actual != Some(expected) {
            errors.push(CourseValidationError::chapter_path(
                "IDENTITY_MISMATCH",
                chapter_id,
                self.repo.relative_display(path),
                format!(
                    "{label} field {field} expected '{expected}', got '{}'.",
                    actual.unwrap_or("<missing>")
                ),
            ));
        }
    }

    fn read_json(
        &self,
        path: &Path,
        missing_code: &'static str,
        invalid_code: &'static str,
        chapter_id: &str,
        errors: &mut Vec<CourseValidationError>,
    ) -> Option<Value> {
        let content = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(source) if source.kind() == io::ErrorKind::NotFound => {
                errors.push(CourseValidationError::chapter_path(
                    missing_code,
                    chapter_id,
                    self.repo.relative_display(path),
                    format!("{} is missing.", Self::asset_label(missing_code)),
                ));
                return None;
            }
            Err(source) => {
                errors.push(CourseValidationError::chapter_path(
                    missing_code,
                    chapter_id,
                    self.repo.relative_display(path),
                    format!(
                        "{} could not be read: {source}.",
                        Self::asset_label(missing_code)
                    ),
                ));
                return None;
            }
        };

        match serde_json::from_str(&content) {
            Ok(value) => Some(value),
            Err(source) => {
                errors.push(CourseValidationError::chapter_path(
                    invalid_code,
                    chapter_id,
                    self.repo.relative_display(path),
                    format!(
                        "{} contains invalid JSON: {source}.",
                        Self::asset_label(missing_code)
                    ),
                ));
                None
            }
        }
    }

    fn asset_label(code: &str) -> &'static str {
        match code {
            "MISSING_MANIFEST" => "Manifest",
            "MISSING_STEP_FILE" => "Step file",
            "MISSING_QUESTION_BANK" => "Question bank",
            _ => "Required asset",
        }
    }

    fn repo_path(&self, relative: &str) -> PathBuf {
        let path = PathBuf::from(relative);
        if path.is_absolute() {
            path
        } else {
            self.repo.root().join(path)
        }
    }

    fn duplicates<'b>(values: impl Iterator<Item = &'b str>) -> BTreeSet<String> {
        let mut seen = BTreeSet::new();
        let mut duplicates = BTreeSet::new();
        for value in values {
            if !seen.insert(value.to_owned()) {
                duplicates.insert(value.to_owned());
            }
        }
        duplicates
    }
}

#[derive(Debug)]
pub struct CourseCatalogValidationSummary {
    pub path: PathBuf,
    pub ready: usize,
    pub blocked: usize,
    pub written: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct CourseIndex {
    version: u64,
    courses: Vec<CourseIndexEntry>,
    #[serde(flatten)]
    extra: BTreeMap<String, Value>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct CourseIndexEntry {
    course_id: String,
    title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    brand_color: Option<String>,
    #[serde(default = "CourseStatus::default_ready")]
    status: CourseStatus,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    validation_errors: Vec<CourseValidationError>,
    #[serde(skip_serializing_if = "Option::is_none")]
    related_important_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exam_raw_dir: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exam_plain_root: Option<String>,
    chapters: Vec<ChapterIndexEntry>,
    #[serde(flatten)]
    extra: BTreeMap<String, Value>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct ChapterIndexEntry {
    chapter_id: String,
    title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    order: Option<u64>,
    lesson_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    raw_pdf_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    plain_output_dir: Option<String>,
    guided_story_dir: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    textbook_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    meta_dir: Option<String>,
    #[serde(flatten)]
    extra: BTreeMap<String, Value>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
enum CourseStatus {
    Ready,
    Blocked,
}

impl CourseStatus {
    fn default_ready() -> Self {
        Self::Ready
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CourseValidationError {
    code: String,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    chapter_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<String>,
}

impl CourseValidationError {
    fn course(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            chapter_id: None,
            path: None,
        }
    }

    fn chapter(code: impl Into<String>, chapter_id: &str, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            chapter_id: Some(chapter_id.to_owned()),
            path: None,
        }
    }

    fn chapter_path(
        code: impl Into<String>,
        chapter_id: &str,
        path: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            chapter_id: Some(chapter_id.to_owned()),
            path: Some(path.into()),
        }
    }
}

#[derive(Debug)]
pub enum CatalogValidationError {
    Read {
        path: PathBuf,
        source: io::Error,
    },
    Parse {
        path: PathBuf,
        source: serde_json::Error,
    },
    Serialize(serde_json::Error),
    Write {
        path: PathBuf,
        source: io::Error,
    },
    BlockedCourses {
        path: PathBuf,
        ready: usize,
        blocked: usize,
        written: bool,
    },
}

impl fmt::Display for CatalogValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Read { path, source } => write!(
                f,
                "failed to read course catalog {}: {source}",
                PathText::to_forward_slashes(path)
            ),
            Self::Parse { path, source } => write!(
                f,
                "failed to parse course catalog {}: {source}",
                PathText::to_forward_slashes(path)
            ),
            Self::Serialize(source) => {
                write!(f, "failed to serialize validated course catalog: {source}")
            }
            Self::Write { path, source } => write!(
                f,
                "failed to write validated course catalog {}: {source}",
                PathText::to_forward_slashes(path)
            ),
            Self::BlockedCourses {
                path,
                ready,
                blocked,
                written,
            } => {
                let action = if *written { "wrote" } else { "checked" };
                write!(
                    f,
                    "catalog validation failed after {action}: ready={ready} blocked={blocked} path={}",
                    PathText::to_forward_slashes(path)
                )
            }
        }
    }
}

impl Error for CatalogValidationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Read { source, .. } | Self::Write { source, .. } => Some(source),
            Self::Parse { source, .. } | Self::Serialize(source) => Some(source),
            Self::BlockedCourses { .. } => None,
        }
    }
}
