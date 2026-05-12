use std::{
    collections::BTreeSet,
    error::Error,
    fmt, fs, io,
    path::{Path, PathBuf},
};

use serde_json::Value;

use crate::paths::LessonPaths;

const SUPPORTED_GUIDED_STORY_EXERCISE_KINDS: &[&str] = &[
    "single_choice",
    "fill_in_blank",
    "short_reflection",
    "ordering",
    "multi_select",
];

pub struct OutputValidator<'a> {
    lesson: &'a LessonPaths,
}

impl<'a> OutputValidator<'a> {
    pub fn new(lesson: &'a LessonPaths) -> Self {
        Self { lesson }
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        let lesson = self.lesson;
        let manifest = Self::read_json(
            &lesson.guided_story_manifest_path(),
            "guided story manifest",
        )?;
        let step_files = Self::manifest_step_files(lesson, &manifest)?;
        let mut question_banks = Vec::new();
        for step_file in &step_files {
            let step = Self::read_json(step_file, "guided story step")?;
            let question_bank_path = step_file
                .parent()
                .map(|parent| parent.join("question_bank.json"))
                .ok_or_else(|| ValidationError::MissingStepQuestionBank {
                    path: step_file.with_file_name("question_bank.json"),
                })?;
            if !question_bank_path.is_file() {
                return Err(ValidationError::MissingStepQuestionBank {
                    path: question_bank_path,
                });
            }
            Self::validate_step_identity(lesson, &manifest, step_file, &step)?;
            Self::validate_guided_story_step_exercise_kinds(step_file, &step)?;
            let question_bank = Self::read_json(&question_bank_path, "step question bank")?;
            Self::validate_question_bank_identity(
                lesson,
                &step,
                &question_bank_path,
                &question_bank,
            )?;
            question_banks.push(question_bank);
        }

        let question_ids =
            Self::collect_many_string_fields(&question_banks, &["question_id", "id"]);
        let family_ids =
            Self::collect_many_string_fields(&question_banks, &["family_id", "familyId"]);
        Self::validate_step_local_question_banks(&step_files, &question_banks)?;
        let textbook =
            fs::read_to_string(lesson.textbook_path()).map_err(|source| ValidationError::Read {
                path: lesson.textbook_path(),
                source,
            })?;
        if let Some(line) = MdxComponentRefs::first_heading_anchor_line(&textbook) {
            return Err(ValidationError::UnsupportedTextbookHeadingAnchor { line });
        }

        for question_id in MdxComponentRefs::component_refs(&textbook, "QuestionRef", "id") {
            if !question_ids.contains(&question_id) {
                return Err(ValidationError::BrokenQuestionRef { question_id });
            }
        }

        for family_id in MdxComponentRefs::component_refs(&textbook, "QuestionFamily", "familyId") {
            if !family_ids.contains(&family_id) {
                return Err(ValidationError::BrokenQuestionFamilyRef { family_id });
            }
        }

        Ok(())
    }

    pub(crate) fn validate_guided_story_step_exercise_kinds(
        step_path: &Path,
        step: &Value,
    ) -> Result<(), ValidationError> {
        let Some(screens) = step.get("screens").and_then(Value::as_array) else {
            return Ok(());
        };

        for (index, screen) in screens.iter().enumerate() {
            let Some(exercise) = screen.get("exercise") else {
                continue;
            };
            let screen_id = Self::screen_id(screen, index);
            let Some(kind) = exercise.get("kind").and_then(Value::as_str) else {
                return Err(ValidationError::UnsupportedGuidedStoryExerciseKind {
                    path: step_path.to_path_buf(),
                    screen_id,
                    kind: "<missing>".to_owned(),
                });
            };
            if !SUPPORTED_GUIDED_STORY_EXERCISE_KINDS.contains(&kind) {
                return Err(ValidationError::UnsupportedGuidedStoryExerciseKind {
                    path: step_path.to_path_buf(),
                    screen_id,
                    kind: kind.to_owned(),
                });
            }
            Self::validate_guided_story_exercise_shape(step_path, &screen_id, kind, exercise)?;
        }

        Ok(())
    }

    fn validate_guided_story_exercise_shape(
        step_path: &Path,
        screen_id: &str,
        kind: &str,
        exercise: &Value,
    ) -> Result<(), ValidationError> {
        match kind {
            "single_choice" => {
                let Some(options) = exercise.get("options").and_then(Value::as_array) else {
                    return Err(Self::invalid_guided_story_exercise_shape(
                        step_path,
                        screen_id,
                        kind,
                        "single_choice must define a non-empty options array",
                    ));
                };
                if options.is_empty() {
                    return Err(Self::invalid_guided_story_exercise_shape(
                        step_path,
                        screen_id,
                        kind,
                        "single_choice must define a non-empty options array",
                    ));
                }
                let answer = exercise.get("answer").and_then(Value::as_u64);
                if !answer
                    .map(|value| (value as usize) < options.len())
                    .unwrap_or(false)
                {
                    return Err(Self::invalid_guided_story_exercise_shape(
                        step_path,
                        screen_id,
                        kind,
                        "single_choice.answer must be an integer option index",
                    ));
                }
            }
            "multi_select" => {
                let Some(options) = exercise.get("options").and_then(Value::as_array) else {
                    return Err(Self::invalid_guided_story_exercise_shape(
                        step_path,
                        screen_id,
                        kind,
                        "multi_select must define a non-empty options array",
                    ));
                };
                if options.is_empty() {
                    return Err(Self::invalid_guided_story_exercise_shape(
                        step_path,
                        screen_id,
                        kind,
                        "multi_select must define a non-empty options array",
                    ));
                }
                let Some(answer) = exercise
                    .get("answer")
                    .and_then(Value::as_array)
                    .filter(|values| !values.is_empty())
                else {
                    return Err(Self::invalid_guided_story_exercise_shape(
                        step_path,
                        screen_id,
                        kind,
                        "multi_select.answer must be a non-empty array of unique option indices",
                    ));
                };

                let mut seen = BTreeSet::new();
                for value in answer {
                    let Some(index) = value.as_u64() else {
                        return Err(Self::invalid_guided_story_exercise_shape(
                            step_path,
                            screen_id,
                            kind,
                            "multi_select.answer must be a non-empty array of unique option indices",
                        ));
                    };
                    let index = index as usize;
                    if index >= options.len() || !seen.insert(index) {
                        return Err(Self::invalid_guided_story_exercise_shape(
                            step_path,
                            screen_id,
                            kind,
                            "multi_select.answer must be a non-empty array of unique option indices",
                        ));
                    }
                }
            }
            "fill_in_blank" => {
                let Some(answers) = exercise
                    .get("answers")
                    .and_then(Value::as_array)
                    .filter(|values| !values.is_empty())
                else {
                    return Err(Self::invalid_guided_story_exercise_shape(
                        step_path,
                        screen_id,
                        kind,
                        "fill_in_blank must define a non-empty answers array",
                    ));
                };

                if answers.iter().any(|value| {
                    value
                        .as_str()
                        .map(|answer| answer.trim().is_empty())
                        .unwrap_or(true)
                }) {
                    return Err(Self::invalid_guided_story_exercise_shape(
                        step_path,
                        screen_id,
                        kind,
                        "fill_in_blank answers must all be non-empty strings",
                    ));
                }
            }
            "ordering" => {
                let Some(items) = exercise.get("items").and_then(Value::as_array) else {
                    return Err(Self::invalid_guided_story_exercise_shape(
                        step_path,
                        screen_id,
                        kind,
                        "ordering must define a non-empty items array",
                    ));
                };
                if items.is_empty() {
                    return Err(Self::invalid_guided_story_exercise_shape(
                        step_path,
                        screen_id,
                        kind,
                        "ordering must define a non-empty items array",
                    ));
                }
                let Some(answer_order) = exercise
                    .get("answer_order")
                    .and_then(Value::as_array)
                    .filter(|values| values.len() == items.len())
                else {
                    return Err(Self::invalid_guided_story_exercise_shape(
                        step_path,
                        screen_id,
                        kind,
                        "ordering.answer_order must be a complete permutation of item indices",
                    ));
                };

                let mut seen = BTreeSet::new();
                for value in answer_order {
                    let Some(index) = value.as_u64() else {
                        return Err(Self::invalid_guided_story_exercise_shape(
                            step_path,
                            screen_id,
                            kind,
                            "ordering.answer_order must be a complete permutation of item indices",
                        ));
                    };
                    let index = index as usize;
                    if index >= items.len() || !seen.insert(index) {
                        return Err(Self::invalid_guided_story_exercise_shape(
                            step_path,
                            screen_id,
                            kind,
                            "ordering.answer_order must be a complete permutation of item indices",
                        ));
                    }
                }
            }
            "short_reflection" => {}
            _ => {}
        }

        Ok(())
    }

    fn invalid_guided_story_exercise_shape(
        step_path: &Path,
        screen_id: &str,
        kind: &str,
        reason: &'static str,
    ) -> ValidationError {
        ValidationError::InvalidGuidedStoryExerciseShape {
            path: step_path.to_path_buf(),
            screen_id: screen_id.to_owned(),
            kind: kind.to_owned(),
            reason,
        }
    }

    fn validate_step_local_question_banks(
        step_files: &[PathBuf],
        question_banks: &[Value],
    ) -> Result<(), ValidationError> {
        for (step_file, question_bank) in step_files.iter().zip(question_banks.iter()) {
            let Some(step_dir) = step_file.parent() else {
                continue;
            };
            let Some(sequence_id) = step_dir.file_name().and_then(|name| name.to_str()) else {
                continue;
            };

            for linked_steps in Self::collect_linked_steps(question_bank) {
                if linked_steps.as_slice() != [sequence_id] {
                    return Err(ValidationError::CrossStepQuestion {
                        sequence_id: sequence_id.to_owned(),
                        linked_steps,
                    });
                }
            }
        }

        Ok(())
    }

    fn validate_step_identity(
        lesson: &LessonPaths,
        manifest: &Value,
        step_path: &Path,
        step: &Value,
    ) -> Result<(), ValidationError> {
        Self::validate_identity_field(
            lesson.lesson_id(),
            manifest,
            "lesson_id",
            "guided story manifest",
            step_path,
            step,
            "guided story step",
        )?;
        Self::validate_optional_identity_field(
            lesson.course_id(),
            manifest,
            "course_id",
            "guided story manifest",
            step_path,
            step,
            "guided story step",
        )?;
        Self::validate_optional_identity_field(
            lesson.chapter_id(),
            manifest,
            "chapter_id",
            "guided story manifest",
            step_path,
            step,
            "guided story step",
        )?;
        Ok(())
    }

    fn validate_question_bank_identity(
        lesson: &LessonPaths,
        step: &Value,
        question_bank_path: &Path,
        question_bank: &Value,
    ) -> Result<(), ValidationError> {
        Self::validate_identity_field(
            lesson.lesson_id(),
            step,
            "lesson_id",
            "guided story step",
            question_bank_path,
            question_bank,
            "step question bank",
        )?;
        Self::validate_optional_identity_field(
            lesson.course_id(),
            step,
            "course_id",
            "guided story step",
            question_bank_path,
            question_bank,
            "step question bank",
        )?;
        Self::validate_optional_identity_field(
            lesson.chapter_id(),
            step,
            "chapter_id",
            "guided story step",
            question_bank_path,
            question_bank,
            "step question bank",
        )?;
        if let Some(sequence_id) = Self::string_field(step, "sequence_id") {
            Self::validate_identity_field(
                sequence_id,
                step,
                "sequence_id",
                "guided story step",
                question_bank_path,
                question_bank,
                "step question bank",
            )?;
        }
        Ok(())
    }

    fn validate_identity_field(
        expected: &str,
        source: &Value,
        field: &'static str,
        source_label: &'static str,
        path: &Path,
        target: &Value,
        target_label: &'static str,
    ) -> Result<(), ValidationError> {
        let source_value =
            Self::string_field(source, field).ok_or(ValidationError::ManifestShape)?;
        let target_value =
            Self::string_field(target, field).ok_or_else(|| ValidationError::IdentityMismatch {
                path: path.to_path_buf(),
                label: target_label,
                field,
                expected: expected.to_owned(),
                actual: "<missing>".to_owned(),
                source_label,
            })?;
        if source_value != expected || target_value != expected {
            return Err(ValidationError::IdentityMismatch {
                path: path.to_path_buf(),
                label: target_label,
                field,
                expected: expected.to_owned(),
                actual: target_value.to_owned(),
                source_label,
            });
        }
        Ok(())
    }

    fn validate_optional_identity_field(
        expected: Option<&str>,
        source: &Value,
        field: &'static str,
        source_label: &'static str,
        path: &Path,
        target: &Value,
        target_label: &'static str,
    ) -> Result<(), ValidationError> {
        let Some(expected) = expected else {
            return Ok(());
        };
        let source_value = Self::string_field(source, field);
        let target_value = Self::string_field(target, field);
        if source_value.is_none() && target_value.is_none() {
            return Ok(());
        }
        if source_value != Some(expected) || target_value != Some(expected) {
            return Err(ValidationError::IdentityMismatch {
                path: path.to_path_buf(),
                label: target_label,
                field,
                expected: expected.to_owned(),
                actual: target_value.unwrap_or("<missing>").to_owned(),
                source_label,
            });
        }
        Ok(())
    }

    fn read_json(path: &Path, label: &'static str) -> Result<Value, ValidationError> {
        let content = fs::read_to_string(path).map_err(|source| ValidationError::Read {
            path: path.to_path_buf(),
            source,
        })?;
        serde_json::from_str(&content).map_err(|source| ValidationError::JsonParse {
            label,
            path: path.to_path_buf(),
            source,
        })
    }

    fn manifest_step_files(
        lesson: &LessonPaths,
        manifest: &Value,
    ) -> Result<Vec<PathBuf>, ValidationError> {
        let steps = manifest
            .get("steps")
            .and_then(Value::as_array)
            .ok_or(ValidationError::ManifestShape)?;
        if steps.is_empty() {
            return Err(ValidationError::ManifestShape);
        }

        let mut files = Vec::new();
        for step in steps {
            let file = step
                .get("file")
                .and_then(Value::as_str)
                .ok_or(ValidationError::ManifestShape)?;
            let path = Self::resolve_manifest_file(lesson, file);
            if !path.is_file() {
                return Err(ValidationError::MissingManifestStep { path });
            }
            files.push(path);
        }

        Ok(files)
    }

    fn resolve_manifest_file(lesson: &LessonPaths, file: &str) -> PathBuf {
        let path = PathBuf::from(file);
        if path.is_absolute() {
            return path;
        }

        if file.starts_with("research/") {
            return lesson.repo_root().join(path);
        }

        if file.starts_with("pipeline/") {
            return lesson.repo_root().join("research").join(path);
        }

        lesson.guided_story_dir().join(path)
    }

    fn collect_many_string_fields(values: &[Value], field_names: &[&str]) -> BTreeSet<String> {
        let mut fields = BTreeSet::new();
        for value in values {
            Self::collect_string_fields_into(value, field_names, &mut fields);
        }
        fields
    }

    fn collect_linked_steps(value: &Value) -> Vec<Vec<String>> {
        let mut linked_steps = Vec::new();
        Self::collect_linked_steps_into(value, &mut linked_steps);
        linked_steps
    }

    fn collect_linked_steps_into(value: &Value, linked_steps: &mut Vec<Vec<String>>) {
        match value {
            Value::Object(object) => {
                for (key, value) in object {
                    if key == "linked_steps" {
                        if let Some(values) = value.as_array() {
                            linked_steps.push(
                                values
                                    .iter()
                                    .filter_map(Value::as_str)
                                    .map(ToOwned::to_owned)
                                    .collect(),
                            );
                        }
                    }
                    Self::collect_linked_steps_into(value, linked_steps);
                }
            }
            Value::Array(values) => {
                for value in values {
                    Self::collect_linked_steps_into(value, linked_steps);
                }
            }
            Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {}
        }
    }

    fn collect_string_fields_into(
        value: &Value,
        field_names: &[&str],
        fields: &mut BTreeSet<String>,
    ) {
        match value {
            Value::Object(object) => {
                for (key, value) in object {
                    if field_names.contains(&key.as_str()) {
                        if let Some(value) = value.as_str() {
                            fields.insert(value.to_owned());
                        }
                    }
                    Self::collect_string_fields_into(value, field_names, fields);
                }
            }
            Value::Array(values) => {
                for value in values {
                    Self::collect_string_fields_into(value, field_names, fields);
                }
            }
            Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {}
        }
    }

    fn string_field<'b>(value: &'b Value, field: &str) -> Option<&'b str> {
        value.get(field).and_then(Value::as_str)
    }

    fn screen_id(screen: &Value, index: usize) -> String {
        Self::string_field(screen, "id")
            .map(ToOwned::to_owned)
            .unwrap_or_else(|| format!("screen{}", index + 1))
    }
}

struct MdxComponentRefs;

impl MdxComponentRefs {
    fn component_refs(text: &str, component: &str, attr: &str) -> Vec<String> {
        let needle = format!("<{component}");
        let mut refs = Vec::new();
        let mut remainder = text;

        while let Some(start) = remainder.find(&needle) {
            remainder = &remainder[start + needle.len()..];
            let Some(end) = remainder.find('>') else {
                break;
            };
            let tag = &remainder[..end];
            if let Some(value) = Self::attr_value(tag, attr) {
                refs.push(value);
            }
            remainder = &remainder[end + 1..];
        }

        refs
    }

    fn attr_value(tag: &str, attr: &str) -> Option<String> {
        let needle = format!("{attr}=");
        let start = tag.find(&needle)? + needle.len();
        let rest = tag[start..].trim_start();
        let quote = rest.chars().next()?;
        if quote != '"' && quote != '\'' {
            return None;
        }
        let rest = &rest[quote.len_utf8()..];
        let end = rest.find(quote)?;
        Some(rest[..end].to_owned())
    }

    fn first_heading_anchor_line(text: &str) -> Option<String> {
        text.lines().find_map(|line| {
            let trimmed = line.trim();
            if trimmed.starts_with('#') && trimmed.contains(" {#") && trimmed.ends_with('}') {
                Some(trimmed.to_owned())
            } else {
                None
            }
        })
    }
}

#[derive(Debug)]
pub enum ValidationError {
    Read {
        path: PathBuf,
        source: io::Error,
    },
    JsonParse {
        label: &'static str,
        path: PathBuf,
        source: serde_json::Error,
    },
    ManifestShape,
    MissingManifestStep {
        path: PathBuf,
    },
    MissingStepQuestionBank {
        path: PathBuf,
    },
    CrossStepQuestion {
        sequence_id: String,
        linked_steps: Vec<String>,
    },
    IdentityMismatch {
        path: PathBuf,
        label: &'static str,
        field: &'static str,
        expected: String,
        actual: String,
        source_label: &'static str,
    },
    BrokenQuestionRef {
        question_id: String,
    },
    BrokenQuestionFamilyRef {
        family_id: String,
    },
    UnsupportedTextbookHeadingAnchor {
        line: String,
    },
    UnsupportedGuidedStoryExerciseKind {
        path: PathBuf,
        screen_id: String,
        kind: String,
    },
    InvalidGuidedStoryExerciseShape {
        path: PathBuf,
        screen_id: String,
        kind: String,
        reason: &'static str,
    },
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Read { path, source } => {
                write!(
                    f,
                    "failed to read validation input {}: {source}",
                    path.display()
                )
            }
            Self::JsonParse {
                label,
                path,
                source,
            } => write!(f, "{label} JSON is invalid at {}: {source}", path.display()),
            Self::ManifestShape => write!(
                f,
                "guided story manifest must contain a non-empty steps array with file entries"
            ),
            Self::MissingManifestStep { path } => {
                write!(
                    f,
                    "guided story manifest references missing step file {}",
                    path.display()
                )
            }
            Self::MissingStepQuestionBank { path } => {
                write!(f, "missing step-local question bank {}", path.display())
            }
            Self::CrossStepQuestion {
                sequence_id,
                linked_steps,
            } => write!(
                f,
                "question bank for {sequence_id} must only link to itself, got linked_steps={linked_steps:?}"
            ),
            Self::IdentityMismatch {
                path,
                label,
                field,
                expected,
                actual,
                source_label,
            } => write!(
                f,
                "{label} {} has {field}='{actual}', expected '{expected}' to match {source_label}",
                path.display()
            ),
            Self::BrokenQuestionRef { question_id } => {
                write!(f, "textbook references unknown question id '{question_id}'")
            }
            Self::BrokenQuestionFamilyRef { family_id } => write!(
                f,
                "textbook references unknown question family '{family_id}'"
            ),
            Self::UnsupportedTextbookHeadingAnchor { line } => write!(
                f,
                "textbook uses unsupported MDX heading anchor syntax: '{line}'"
            ),
            Self::UnsupportedGuidedStoryExerciseKind {
                path,
                screen_id,
                kind,
            } => write!(
                f,
                "guided story step {} screen '{}' uses unsupported exercise kind '{}'; expected single_choice, fill_in_blank, short_reflection, ordering, or multi_select",
                path.display(),
                screen_id,
                kind
            ),
            Self::InvalidGuidedStoryExerciseShape {
                path,
                screen_id,
                kind,
                reason,
            } => write!(
                f,
                "guided story step {} screen '{}' has invalid {} payload: {}",
                path.display(),
                screen_id,
                kind,
                reason
            ),
        }
    }
}

impl Error for ValidationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Read { source, .. } => Some(source),
            Self::JsonParse { source, .. } => Some(source),
            Self::ManifestShape
            | Self::MissingManifestStep { .. }
            | Self::MissingStepQuestionBank { .. }
            | Self::CrossStepQuestion { .. }
            | Self::IdentityMismatch { .. }
            | Self::BrokenQuestionRef { .. }
            | Self::BrokenQuestionFamilyRef { .. }
            | Self::UnsupportedTextbookHeadingAnchor { .. }
            | Self::UnsupportedGuidedStoryExerciseKind { .. }
            | Self::InvalidGuidedStoryExerciseShape { .. } => None,
        }
    }
}
