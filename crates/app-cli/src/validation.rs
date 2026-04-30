use std::{
    collections::BTreeSet,
    error::Error,
    fmt, fs, io,
    path::{Path, PathBuf},
};

use serde_json::Value;

use crate::paths::LessonPaths;

pub fn validate_outputs(lesson: &LessonPaths) -> Result<(), ValidationError> {
    let manifest = read_json(
        &lesson.guided_story_manifest_path(),
        "guided story manifest",
    )?;
    let step_files = manifest_step_files(lesson, &manifest)?;
    for step_file in &step_files {
        read_json(step_file, "guided story step")?;
    }

    let question_bank = read_json(&lesson.question_bank_path(), "question bank")?;
    let question_ids = collect_string_fields(&question_bank, &["question_id", "id"]);
    let family_ids = collect_string_fields(&question_bank, &["family_id", "familyId"]);
    let textbook =
        fs::read_to_string(lesson.textbook_path()).map_err(|source| ValidationError::Read {
            path: lesson.textbook_path(),
            source,
        })?;

    for question_id in component_refs(&textbook, "QuestionRef", "id") {
        if !question_ids.contains(&question_id) {
            return Err(ValidationError::BrokenQuestionRef { question_id });
        }
    }

    for family_id in component_refs(&textbook, "QuestionFamily", "familyId") {
        if !family_ids.contains(&family_id) {
            return Err(ValidationError::BrokenQuestionFamilyRef { family_id });
        }
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
        let path = resolve_manifest_file(lesson, file);
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

fn collect_string_fields(value: &Value, field_names: &[&str]) -> BTreeSet<String> {
    let mut fields = BTreeSet::new();
    collect_string_fields_into(value, field_names, &mut fields);
    fields
}

fn collect_string_fields_into(value: &Value, field_names: &[&str], fields: &mut BTreeSet<String>) {
    match value {
        Value::Object(object) => {
            for (key, value) in object {
                if field_names.contains(&key.as_str()) {
                    if let Some(value) = value.as_str() {
                        fields.insert(value.to_owned());
                    }
                }
                collect_string_fields_into(value, field_names, fields);
            }
        }
        Value::Array(values) => {
            for value in values {
                collect_string_fields_into(value, field_names, fields);
            }
        }
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {}
    }
}

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
        if let Some(value) = attr_value(tag, attr) {
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
    BrokenQuestionRef {
        question_id: String,
    },
    BrokenQuestionFamilyRef {
        family_id: String,
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
            Self::BrokenQuestionRef { question_id } => {
                write!(f, "textbook references unknown question id '{question_id}'")
            }
            Self::BrokenQuestionFamilyRef { family_id } => write!(
                f,
                "textbook references unknown question family '{family_id}'"
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
            | Self::BrokenQuestionRef { .. }
            | Self::BrokenQuestionFamilyRef { .. } => None,
        }
    }
}
