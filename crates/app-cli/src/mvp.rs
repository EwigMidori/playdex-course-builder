use std::{
    error::Error,
    fmt, fs, io,
    path::{Path, PathBuf},
    thread,
    time::Duration,
};

use reqwest::blocking::Client;
use serde::Serialize;
use serde_json::{json, Value};

use crate::{
    config::{Config, ConfigError, LlmReadyConfig},
    mineru::{self, ConvertError},
    paths::LessonPaths,
    validation::{self, ValidationError},
};

pub fn run_mvp(
    repo: &crate::paths::RepoPaths,
    lesson: &LessonPaths,
    target_language: &str,
) -> Result<(), MvpError> {
    if !lesson.plain_text_path().is_file() {
        mineru::convert_lesson(repo, lesson, false)?;
    }

    let config = Config::load(repo)?;
    let llm = config.llm.require_ready()?;
    let plain_text = read_required_text(&lesson.plain_text_path())?;
    let related_important = read_optional_text(&repo.related_important_path())?;

    generate_guided_story(
        lesson,
        &llm,
        target_language,
        &plain_text,
        &related_important,
    )?;
    generate_question_bank(
        lesson,
        &llm,
        target_language,
        &plain_text,
        &related_important,
    )?;
    generate_textbook(
        lesson,
        &llm,
        target_language,
        &plain_text,
        &related_important,
    )?;
    validation::validate_outputs(lesson)?;

    Ok(())
}

fn generate_guided_story(
    lesson: &LessonPaths,
    llm: &LlmReadyConfig,
    target_language: &str,
    plain_text: &str,
    related_important: &str,
) -> Result<(), MvpError> {
    let repo = lesson.repo();
    let system = read_prompt(repo, "guided_story_system.md")?;
    let user = render_prompt(
        &read_prompt(repo, "guided_story_user.md")?,
        &[
            ("target_language", target_language),
            (
                "step_scope",
                "full lecture; split into 4-8 natural learning steps unless the source is very short",
            ),
            ("related_important", related_important),
            ("plain_text", plain_text),
            ("image_hints", ""),
        ],
    );
    let response = call_chat_completion(lesson, llm, Stage::GuidedStory, &system, &user)?;
    let content = strip_code_fence(&response.content, Some("json"));
    let story_json: Value =
        serde_json::from_str(&content).map_err(|source| MvpError::JsonParse {
            stage: Stage::GuidedStory.as_str(),
            source,
        })?;

    if lesson.guided_story_dir().is_dir() {
        fs::remove_dir_all(lesson.guided_story_dir()).map_err(|source| MvpError::Write {
            path: lesson.guided_story_dir(),
            source,
        })?;
    }
    fs::create_dir_all(lesson.guided_story_dir()).map_err(|source| MvpError::Write {
        path: lesson.guided_story_dir(),
        source,
    })?;

    let manifest = write_guided_story_steps(lesson, &story_json)?;
    write_pretty_json(&lesson.guided_story_manifest_path(), &manifest)?;
    Ok(())
}

fn generate_question_bank(
    lesson: &LessonPaths,
    llm: &LlmReadyConfig,
    target_language: &str,
    plain_text: &str,
    related_important: &str,
) -> Result<(), MvpError> {
    let repo = lesson.repo();
    let system = read_prompt(repo, "question_bank_system.md")?;
    let manifest = read_required_text(&lesson.guided_story_manifest_path())?;
    let step_refs = read_manifest_step_refs(lesson)?;
    let step_count = step_refs.len();
    let steps = read_guided_story_steps(lesson, &step_refs)?;
    let source_outline = source_outline_from_plain_text(plain_text);
    let prompt = read_prompt(repo, "question_bank_user.md")?;

    for step_ref in step_refs {
        let current_step = read_required_text(&step_ref.path)?;
        let user = render_prompt(
            &prompt,
            &[
                ("target_language", target_language),
                ("lesson_id", lesson.lesson_id()),
                ("coverage_checklist", &source_outline),
                ("source_outline", &source_outline),
                ("lesson_map", &manifest),
                ("guided_story_manifest", &manifest),
                ("guided_story_steps", &steps),
                ("current_step_id", &step_ref.sequence_id),
                ("current_step", &current_step),
                ("plain_text", plain_text),
                ("related_important", related_important),
            ],
        );
        let audit_stage = if step_count == 1 {
            "question_bank".to_owned()
        } else {
            format!("question_bank/{}", step_ref.sequence_id)
        };
        let response = call_chat_completion_with_audit(
            lesson,
            llm,
            Stage::QuestionBank,
            &audit_stage,
            &system,
            &user,
        )?;
        let content = strip_code_fence(&response.content, Some("json"));
        let question_bank: Value =
            serde_json::from_str(&content).map_err(|source| MvpError::JsonParse {
                stage: Stage::QuestionBank.as_str(),
                source,
            })?;

        if let Some(parent) = step_ref.question_bank_path.parent() {
            fs::create_dir_all(parent).map_err(|source| MvpError::Write {
                path: parent.to_path_buf(),
                source,
            })?;
        }
        write_pretty_json(&step_ref.question_bank_path, &question_bank)?;
    }

    Ok(())
}

fn generate_textbook(
    lesson: &LessonPaths,
    llm: &LlmReadyConfig,
    target_language: &str,
    plain_text: &str,
    related_important: &str,
) -> Result<(), MvpError> {
    let repo = lesson.repo();
    let system = read_prompt(repo, "textbook_system.md")?;
    let manifest = read_required_text(&lesson.guided_story_manifest_path())?;
    let step_refs = read_manifest_step_refs(lesson)?;
    let steps = read_guided_story_steps(lesson, &step_refs)?;
    let question_bank = read_step_question_banks(lesson)?;
    let source_outline = source_outline_from_plain_text(plain_text);
    let user = render_prompt(
        &read_prompt(repo, "textbook_user.md")?,
        &[
            ("target_language", target_language),
            ("lesson_id", lesson.lesson_id()),
            ("coverage_checklist", &source_outline),
            ("source_outline", &source_outline),
            ("lesson_map", &manifest),
            ("guided_story_manifest", &manifest),
            ("guided_story_steps", &steps),
            ("question_bank", &question_bank),
            ("plain_text", plain_text),
            ("related_important", related_important),
        ],
    );
    let response = call_chat_completion(lesson, llm, Stage::Textbook, &system, &user)?;
    let content = strip_code_fence(&response.content, Some("mdx"));

    if let Some(parent) = lesson.textbook_path().parent() {
        fs::create_dir_all(parent).map_err(|source| MvpError::Write {
            path: parent.to_path_buf(),
            source,
        })?;
    }
    fs::write(lesson.textbook_path(), content).map_err(|source| MvpError::Write {
        path: lesson.textbook_path(),
        source,
    })?;
    Ok(())
}

fn call_chat_completion(
    lesson: &LessonPaths,
    llm: &LlmReadyConfig,
    stage: Stage,
    system: &str,
    user: &str,
) -> Result<LlmResponse, MvpError> {
    call_chat_completion_with_audit(lesson, llm, stage, stage.as_str(), system, user)
}

fn call_chat_completion_with_audit(
    lesson: &LessonPaths,
    llm: &LlmReadyConfig,
    stage: Stage,
    audit_stage: &str,
    system: &str,
    user: &str,
) -> Result<LlmResponse, MvpError> {
    for attempt in 1..=3 {
        match call_chat_completion_once(lesson, llm, stage, audit_stage, system, user) {
            Ok(response) => return Ok(response),
            Err(error @ MvpError::LlmRequest { .. }) if attempt < 3 => {
                eprintln!(
                    "{} LLM request failed on attempt {attempt}/3; retrying: {error}",
                    stage.as_str()
                );
                thread::sleep(Duration::from_secs(2 * attempt));
            }
            Err(error) => return Err(error),
        }
    }

    unreachable!("retry loop returns on success or final failure")
}

fn call_chat_completion_once(
    lesson: &LessonPaths,
    llm: &LlmReadyConfig,
    stage: Stage,
    audit_stage: &str,
    system: &str,
    user: &str,
) -> Result<LlmResponse, MvpError> {
    let audit_dir = lesson.audit_stage_dir(audit_stage);
    fs::create_dir_all(&audit_dir).map_err(|source| MvpError::Write {
        path: audit_dir.clone(),
        source,
    })?;
    fs::write(audit_dir.join("system.md"), system).map_err(|source| MvpError::Write {
        path: audit_dir.join("system.md"),
        source,
    })?;
    fs::write(audit_dir.join("user.md"), user).map_err(|source| MvpError::Write {
        path: audit_dir.join("user.md"),
        source,
    })?;

    let request = ChatRequest {
        model: &llm.model,
        messages: vec![
            ChatMessage {
                role: "system",
                content: system,
            },
            ChatMessage {
                role: "user",
                content: user,
            },
        ],
        temperature: 0.2,
        max_tokens: llm.max_tokens,
    };
    write_pretty_json(&audit_dir.join("request.json"), &request)?;

    let client = Client::builder()
        .http1_only()
        .timeout(Duration::from_secs(llm.timeout_seconds))
        .build()
        .map_err(|source| MvpError::LlmRequest {
            stage: stage.as_str(),
            source,
        })?;
    let url = format!("{}/chat/completions", llm.base_url.trim_end_matches('/'));
    let response = client
        .post(url)
        .header("Accept-Encoding", "identity")
        .bearer_auth(&llm.api_key)
        .json(&request)
        .send()
        .map_err(|source| MvpError::LlmRequest {
            stage: stage.as_str(),
            source,
        })?;
    let status = response.status();
    let raw_bytes = response.bytes().map_err(|source| MvpError::LlmRequest {
        stage: stage.as_str(),
        source,
    })?;
    let raw_text = String::from_utf8_lossy(&raw_bytes).into_owned();
    fs::write(audit_dir.join("raw_response.txt"), &raw_text).map_err(|source| MvpError::Write {
        path: audit_dir.join("raw_response.txt"),
        source,
    })?;
    if !status.is_success() {
        return Err(MvpError::LlmStatus {
            stage: stage.as_str(),
            status: status.as_u16(),
            body: raw_text,
        });
    }
    let raw: Value = serde_json::from_str(&raw_text).map_err(|source| MvpError::JsonParse {
        stage: stage.as_str(),
        source,
    })?;
    write_pretty_json(&audit_dir.join("raw_response.json"), &raw)?;

    let content = raw
        .pointer("/choices/0/message/content")
        .and_then(Value::as_str)
        .or_else(|| raw.pointer("/choices/0/text").and_then(Value::as_str))
        .ok_or(MvpError::LlmMissingContent {
            stage: stage.as_str(),
        })?
        .to_owned();
    fs::write(audit_dir.join("content.txt"), &content).map_err(|source| MvpError::Write {
        path: audit_dir.join("content.txt"),
        source,
    })?;

    Ok(LlmResponse { content })
}

#[derive(Clone, Copy)]
enum Stage {
    GuidedStory,
    QuestionBank,
    Textbook,
}

impl Stage {
    fn as_str(self) -> &'static str {
        match self {
            Self::GuidedStory => "guided_story",
            Self::QuestionBank => "question_bank",
            Self::Textbook => "textbook",
        }
    }
}

struct LlmResponse {
    content: String,
}

#[derive(Serialize)]
struct ChatRequest<'a> {
    model: &'a str,
    messages: Vec<ChatMessage<'a>>,
    temperature: f32,
    max_tokens: u64,
}

#[derive(Serialize)]
struct ChatMessage<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Clone, Debug)]
struct StepRef {
    sequence_id: String,
    path: PathBuf,
    question_bank_path: PathBuf,
}

fn read_prompt(repo: &crate::paths::RepoPaths, file_name: &str) -> Result<String, MvpError> {
    read_required_text(&repo.prompts_dir().join(file_name))
}

fn write_guided_story_steps(lesson: &LessonPaths, story_json: &Value) -> Result<Value, MvpError> {
    let mut manifest_steps = Vec::new();

    if let Some(steps) = story_json.get("steps").and_then(Value::as_array) {
        for (index, item) in steps.iter().enumerate() {
            let sequence_id = item
                .get("sequence_id")
                .and_then(Value::as_str)
                .map(ToOwned::to_owned)
                .unwrap_or_else(|| format!("step{}", index + 1));
            let concept = item
                .get("concept")
                .and_then(Value::as_str)
                .unwrap_or("Guided story step")
                .to_owned();
            let mut step_json = item.get("step").cloned().unwrap_or_else(|| item.clone());
            normalize_step_json(lesson, &sequence_id, &mut step_json);

            let step_path = lesson
                .guided_story_dir()
                .join(&sequence_id)
                .join("step.json");
            write_pretty_json(&step_path, &step_json)?;
            manifest_steps.push(json!({
                "sequence_id": sequence_id,
                "file": lesson.relative_display(&step_path),
                "concept": concept
            }));
        }
    }

    if manifest_steps.is_empty() {
        let mut step_json = story_json.clone();
        normalize_step_json(lesson, "step1", &mut step_json);
        write_pretty_json(&lesson.guided_story_step_path(1), &step_json)?;
        manifest_steps.push(json!({
            "sequence_id": "step1",
            "file": lesson.relative_display(&lesson.guided_story_step_path(1)),
            "concept": "Guided story step"
        }));
    }

    Ok(json!({
        "lesson_id": lesson.lesson_id(),
        "mode": "guided_story",
        "steps": manifest_steps
    }))
}

fn normalize_step_json(lesson: &LessonPaths, sequence_id: &str, step_json: &mut Value) {
    let Some(object) = step_json.as_object_mut() else {
        return;
    };
    object
        .entry("lesson_id")
        .or_insert_with(|| json!(lesson.lesson_id()));
    object
        .entry("sequence_id")
        .or_insert_with(|| json!(sequence_id));
    object
        .entry("mode")
        .or_insert_with(|| json!("guided_story"));
    object.entry("term_catalog").or_insert_with(|| json!({}));
}

fn read_manifest_step_refs(lesson: &LessonPaths) -> Result<Vec<StepRef>, MvpError> {
    let manifest = read_json(&lesson.guided_story_manifest_path(), Stage::GuidedStory)?;
    let steps = manifest
        .get("steps")
        .and_then(Value::as_array)
        .ok_or(MvpError::ManifestShape)?;
    let mut refs = Vec::new();

    for step in steps {
        let sequence_id = step
            .get("sequence_id")
            .and_then(Value::as_str)
            .ok_or(MvpError::ManifestShape)?
            .to_owned();
        let file = step
            .get("file")
            .and_then(Value::as_str)
            .ok_or(MvpError::ManifestShape)?;
        let path = resolve_manifest_file(lesson, file);
        let question_bank_path = path
            .parent()
            .map(|parent| parent.join("question_bank.json"))
            .ok_or_else(|| MvpError::Read {
                path: path.with_file_name("question_bank.json"),
                source: io::Error::new(io::ErrorKind::InvalidInput, "step path has no parent"),
            })?;
        refs.push(StepRef {
            sequence_id,
            path,
            question_bank_path,
        });
    }

    Ok(refs)
}

fn read_guided_story_steps(
    lesson: &LessonPaths,
    step_refs: &[StepRef],
) -> Result<String, MvpError> {
    let mut steps = Vec::new();
    for step_ref in step_refs {
        let content = read_required_text(&step_ref.path)?;
        let value: Value =
            serde_json::from_str(&content).map_err(|source| MvpError::JsonParse {
                stage: Stage::GuidedStory.as_str(),
                source,
            })?;
        steps.push(value);
    }

    let mut payload =
        serde_json::to_string_pretty(&steps).map_err(|source| MvpError::JsonWrite {
            path: lesson.guided_story_dir(),
            source,
        })?;
    payload.push('\n');
    Ok(payload)
}

fn read_json(path: &Path, stage: Stage) -> Result<Value, MvpError> {
    let content = read_required_text(path)?;
    serde_json::from_str(&content).map_err(|source| MvpError::JsonParse {
        stage: stage.as_str(),
        source,
    })
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

fn read_step_question_banks(lesson: &LessonPaths) -> Result<String, MvpError> {
    let mut banks = Vec::new();
    let step_dirs = match fs::read_dir(lesson.guided_story_dir()) {
        Ok(step_dirs) => step_dirs,
        Err(source) => {
            return Err(MvpError::Read {
                path: lesson.guided_story_dir(),
                source,
            })
        }
    };

    for entry in step_dirs {
        let entry = entry.map_err(|source| MvpError::Read {
            path: lesson.guided_story_dir(),
            source,
        })?;
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        if !name.starts_with("step") {
            continue;
        }

        let question_bank_path = path.join("question_bank.json");
        if question_bank_path.is_file() {
            banks.push(read_required_text(&question_bank_path)?);
        }
    }

    banks.sort();
    Ok(format!("[\n{}\n]", banks.join(",\n")))
}

fn read_required_text(path: &Path) -> Result<String, MvpError> {
    fs::read_to_string(path).map_err(|source| MvpError::Read {
        path: path.to_path_buf(),
        source,
    })
}

fn read_optional_text(path: &Path) -> Result<String, MvpError> {
    match fs::read_to_string(path) {
        Ok(contents) => Ok(contents),
        Err(source) if source.kind() == io::ErrorKind::NotFound => Ok(String::new()),
        Err(source) => Err(MvpError::Read {
            path: path.to_path_buf(),
            source,
        }),
    }
}

fn render_prompt(template: &str, vars: &[(&str, &str)]) -> String {
    let mut rendered = template.to_owned();
    for (key, value) in vars {
        rendered = rendered.replace(&format!("{{{{{key}}}}}"), value);
    }
    rendered
}

fn source_outline_from_plain_text(plain_text: &str) -> String {
    plain_text
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .take(60)
        .collect::<Vec<_>>()
        .join("\n")
}

fn strip_code_fence(content: &str, language: Option<&str>) -> String {
    let trimmed = content.trim();
    if !trimmed.starts_with("```") {
        return trimmed.to_owned();
    }

    let mut lines = trimmed.lines();
    let first = lines.next().unwrap_or_default().trim();
    let expected = language.unwrap_or_default();
    if !expected.is_empty()
        && first.trim_start_matches("```").trim() != expected
        && !first.trim_start_matches("```").trim().is_empty()
    {
        return trimmed.to_owned();
    }

    let body = lines.collect::<Vec<_>>().join("\n");
    body.strip_suffix("```").unwrap_or(&body).trim().to_owned()
}

fn write_pretty_json<T>(path: &Path, value: &T) -> Result<(), MvpError>
where
    T: Serialize,
{
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|source| MvpError::Write {
            path: parent.to_path_buf(),
            source,
        })?;
    }

    let mut payload = serde_json::to_vec_pretty(value).map_err(|source| MvpError::JsonWrite {
        path: path.to_path_buf(),
        source,
    })?;
    payload.push(b'\n');
    fs::write(path, payload).map_err(|source| MvpError::Write {
        path: path.to_path_buf(),
        source,
    })
}

#[derive(Debug)]
pub enum MvpError {
    Config(ConfigError),
    Convert(ConvertError),
    Validation(ValidationError),
    Read {
        path: PathBuf,
        source: io::Error,
    },
    Write {
        path: PathBuf,
        source: io::Error,
    },
    JsonWrite {
        path: PathBuf,
        source: serde_json::Error,
    },
    JsonParse {
        stage: &'static str,
        source: serde_json::Error,
    },
    ManifestShape,
    LlmRequest {
        stage: &'static str,
        source: reqwest::Error,
    },
    LlmStatus {
        stage: &'static str,
        status: u16,
        body: String,
    },
    LlmMissingContent {
        stage: &'static str,
    },
}

impl fmt::Display for MvpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Config(error) => error.fmt(f),
            Self::Convert(error) => error.fmt(f),
            Self::Validation(error) => error.fmt(f),
            Self::Read { path, source } => {
                write!(f, "failed to read {}: {source}", path.display())
            }
            Self::Write { path, source } => {
                write!(f, "failed to write {}: {source}", path.display())
            }
            Self::JsonWrite { path, source } => {
                write!(
                    f,
                    "failed to serialize JSON for {}: {source}",
                    path.display()
                )
            }
            Self::JsonParse { stage, source } => {
                write!(f, "{stage} model output was not valid JSON: {source}")
            }
            Self::ManifestShape => write!(f, "guided story manifest has invalid step shape"),
            Self::LlmRequest { stage, source } => {
                write!(f, "{stage} LLM request failed: {source:?}")
            }
            Self::LlmStatus {
                stage,
                status,
                body,
            } => write!(f, "{stage} LLM request returned HTTP {status}: {body}"),
            Self::LlmMissingContent { stage } => {
                write!(
                    f,
                    "{stage} LLM response did not include choices[0].message.content"
                )
            }
        }
    }
}

impl Error for MvpError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Config(error) => Some(error),
            Self::Convert(error) => Some(error),
            Self::Validation(error) => Some(error),
            Self::Read { source, .. } | Self::Write { source, .. } => Some(source),
            Self::JsonWrite { source, .. } | Self::JsonParse { source, .. } => Some(source),
            Self::LlmRequest { source, .. } => Some(source),
            Self::ManifestShape | Self::LlmStatus { .. } | Self::LlmMissingContent { .. } => None,
        }
    }
}

impl From<ConfigError> for MvpError {
    fn from(error: ConfigError) -> Self {
        Self::Config(error)
    }
}

impl From<ConvertError> for MvpError {
    fn from(error: ConvertError) -> Self {
        Self::Convert(error)
    }
}

impl From<ValidationError> for MvpError {
    fn from(error: ValidationError) -> Self {
        Self::Validation(error)
    }
}
