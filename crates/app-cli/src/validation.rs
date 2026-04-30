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
    let mut question_banks = Vec::new();
    for step_file in &step_files {
        read_json(step_file, "guided story step")?;
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
        question_banks.push(read_json(&question_bank_path, "step question bank")?);
    }

    let question_ids = collect_many_string_fields(&question_banks, &["question_id", "id"]);
    let family_ids = collect_many_string_fields(&question_banks, &["family_id", "familyId"]);
    validate_step_local_question_banks(&step_files, &question_banks)?;
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

        for linked_steps in collect_linked_steps(question_bank) {
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

fn collect_many_string_fields(values: &[Value], field_names: &[&str]) -> BTreeSet<String> {
    let mut fields = BTreeSet::new();
    for value in values {
        collect_string_fields_into(value, field_names, &mut fields);
    }
    fields
}

fn collect_linked_steps(value: &Value) -> Vec<Vec<String>> {
    let mut linked_steps = Vec::new();
    collect_linked_steps_into(value, &mut linked_steps);
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
                collect_linked_steps_into(value, linked_steps);
            }
        }
        Value::Array(values) => {
            for value in values {
                collect_linked_steps_into(value, linked_steps);
            }
        }
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {}
    }
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
    MissingStepQuestionBank {
        path: PathBuf,
    },
    CrossStepQuestion {
        sequence_id: String,
        linked_steps: Vec<String>,
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
            | Self::MissingStepQuestionBank { .. }
            | Self::CrossStepQuestion { .. }
            | Self::BrokenQuestionRef { .. }
            | Self::BrokenQuestionFamilyRef { .. } => None,
        }
    }
}
