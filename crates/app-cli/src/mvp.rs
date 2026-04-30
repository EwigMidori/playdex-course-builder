use std::{
    error::Error,
    fmt, fs, io,
    path::{Path, PathBuf},
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
            ("step_scope", "full lesson MVP slice"),
            ("related_important", related_important),
            ("plain_text", plain_text),
            ("image_hints", ""),
        ],
    );
    let response = call_chat_completion(lesson, llm, Stage::GuidedStory, &system, &user)?;
    let content = strip_code_fence(&response.content, Some("json"));
    let step_json: Value =
        serde_json::from_str(&content).map_err(|source| MvpError::JsonParse {
            stage: Stage::GuidedStory.as_str(),
            source,
        })?;

    fs::create_dir_all(lesson.guided_story_dir()).map_err(|source| MvpError::Write {
        path: lesson.guided_story_dir(),
        source,
    })?;
    write_pretty_json(&lesson.guided_story_step_path(1), &step_json)?;

    let manifest = json!({
        "lesson_id": lesson.lesson_id(),
        "mode": "guided_story",
        "steps": [{
            "sequence_id": "step1",
            "file": format!("research/pipeline/3-guided_story/{}.step1.json", lesson.lesson_id()),
            "concept": "MVP lesson slice"
        }]
    });
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
    let steps = read_required_text(&lesson.guided_story_step_path(1))?;
    let source_outline = source_outline_from_plain_text(plain_text);
    let user = render_prompt(
        &read_prompt(repo, "question_bank_user.md")?,
        &[
            ("target_language", target_language),
            ("lesson_id", lesson.lesson_id()),
            ("coverage_checklist", &source_outline),
            ("source_outline", &source_outline),
            ("lesson_map", &manifest),
            ("guided_story_manifest", &manifest),
            ("guided_story_steps", &steps),
            ("plain_text", plain_text),
            ("related_important", related_important),
        ],
    );
    let response = call_chat_completion(lesson, llm, Stage::QuestionBank, &system, &user)?;
    let content = strip_code_fence(&response.content, Some("json"));
    let question_bank: Value =
        serde_json::from_str(&content).map_err(|source| MvpError::JsonParse {
            stage: Stage::QuestionBank.as_str(),
            source,
        })?;

    if let Some(parent) = lesson.question_bank_path().parent() {
        fs::create_dir_all(parent).map_err(|source| MvpError::Write {
            path: parent.to_path_buf(),
            source,
        })?;
    }
    write_pretty_json(&lesson.question_bank_path(), &question_bank)?;
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
    let steps = read_required_text(&lesson.guided_story_step_path(1))?;
    let question_bank = read_required_text(&lesson.question_bank_path())?;
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
    let audit_dir = lesson.audit_stage_dir(stage.as_str());
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
    };
    write_pretty_json(&audit_dir.join("request.json"), &request)?;

    let client = Client::builder()
        .build()
        .map_err(|source| MvpError::LlmRequest {
            stage: stage.as_str(),
            source,
        })?;
    let url = format!("{}/chat/completions", llm.base_url.trim_end_matches('/'));
    let response = client
        .post(url)
        .bearer_auth(&llm.api_key)
        .json(&request)
        .send()
        .and_then(|response| response.error_for_status())
        .map_err(|source| MvpError::LlmRequest {
            stage: stage.as_str(),
            source,
        })?;
    let raw: Value = response.json().map_err(|source| MvpError::LlmRequest {
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
}

#[derive(Serialize)]
struct ChatMessage<'a> {
    role: &'a str,
    content: &'a str,
}

fn read_prompt(repo: &crate::paths::RepoPaths, file_name: &str) -> Result<String, MvpError> {
    read_required_text(&repo.prompts_dir().join(file_name))
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
    LlmRequest {
        stage: &'static str,
        source: reqwest::Error,
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
            Self::LlmRequest { stage, source } => {
                write!(f, "{stage} LLM request failed: {source}")
            }
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
            Self::LlmMissingContent { .. } => None,
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
