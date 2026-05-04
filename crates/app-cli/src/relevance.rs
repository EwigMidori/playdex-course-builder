use std::{
    collections::BTreeSet,
    error::Error,
    fmt, fs, io,
    path::{Path, PathBuf},
};

use serde::Serialize;
use serde_json::{json, Value};

use crate::{
    config::{Config, ConfigError, LlmReadyConfig},
    llm::{LlmClient, LlmClientError},
    paths::{LessonPaths, PathText, RepoPaths},
};

const STAGE: &str = "relevance";

pub struct RelevanceScorer;

impl RelevanceScorer {
    pub fn score(
        repo: &RepoPaths,
        lesson: &LessonPaths,
        target_language: &str,
    ) -> Result<(), RelevanceError> {
        let config = Config::load(repo)?;
        let llm = config.llm.require_ready()?;

        let input = Self::build_scorer_input(repo, lesson, target_language)?;
        let known_refs = KnownRefs::from_input(&input);
        let system = Self::system_prompt();
        let user = Self::render_user_prompt(&input)?;
        let response = Self::call_chat_completion(lesson, &llm, &system, &user)?;
        let content = Self::strip_code_fence(&response.content, Some("json"));
        let report: Value =
            serde_json::from_str(&content).map_err(|source| RelevanceError::JsonParse {
                stage: STAGE,
                source,
            })?;
        Self::validate_report(&report, &known_refs)?;
        Self::write_pretty_json(&lesson.relevance_report_path(), &report)?;

        Ok(())
    }

    fn build_scorer_input(
        repo: &RepoPaths,
        lesson: &LessonPaths,
        target_language: &str,
    ) -> Result<Value, RelevanceError> {
        let course_description = Self::read_optional_text(&lesson.related_important_path())?;
        let plain_text = Self::read_required_text(&lesson.plain_text_path())?;
        let manifest = Self::read_json(
            &lesson.guided_story_manifest_path(),
            "guided story manifest",
        )?;
        let step_assets = Self::read_step_assets(lesson)?;
        let textbook = Self::read_required_text(&lesson.textbook_path())?;
        let textbook_sections = Self::extract_textbook_sections(&textbook);
        let exam_signal = Self::read_exam_signal(repo, lesson)?;

        Ok(json!({
            "course_id": lesson.course_id(),
            "course_title": lesson.course_title(),
            "chapter_id": lesson.chapter_id(),
            "chapter_title": lesson.chapter_title(),
            "lesson_id": lesson.lesson_id(),
            "target_language": target_language,
            "course_description": course_description,
            "plain_text": plain_text,
            "exam_signal": exam_signal,
            "guided_story_manifest": manifest,
            "step_assets": step_assets,
            "textbook": {
                "path": lesson.relative_display(&lesson.textbook_path()),
                "sections": textbook_sections,
                "content": textbook
            }
        }))
    }

    fn read_step_assets(lesson: &LessonPaths) -> Result<Vec<Value>, RelevanceError> {
        let mut step_dirs = Vec::new();
        for entry in
            fs::read_dir(lesson.guided_story_dir()).map_err(|source| RelevanceError::Read {
                path: lesson.guided_story_dir(),
                source,
            })?
        {
            let entry = entry.map_err(|source| RelevanceError::Read {
                path: lesson.guided_story_dir(),
                source,
            })?;
            let path = entry.path();
            if path.is_dir()
                && path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .is_some_and(|name| name.starts_with("step"))
            {
                step_dirs.push(path);
            }
        }
        step_dirs.sort_by_key(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .map(Self::step_sort_key)
                .unwrap_or((usize::MAX, String::new()))
        });

        let mut assets = Vec::new();
        for step_dir in step_dirs {
            let step_path = step_dir.join("step.json");
            let question_bank_path = step_dir.join("question_bank.json");
            let step = Self::read_json(&step_path, "guided story step")?;
            let question_bank = Self::read_json(&question_bank_path, "step question bank")?;
            let sequence_id = step
                .get("sequence_id")
                .and_then(Value::as_str)
                .or_else(|| question_bank.get("sequence_id").and_then(Value::as_str))
                .or_else(|| step_dir.file_name().and_then(|name| name.to_str()))
                .unwrap_or_default()
                .to_owned();
            let question_families = Self::collect_question_families(&question_bank);
            assets.push(json!({
                "sequence_id": sequence_id,
                "step_path": lesson.relative_display(&step_path),
                "question_bank_path": lesson.relative_display(&question_bank_path),
                "step": step,
                "question_bank": question_bank,
                "question_families": question_families
            }));
        }

        Ok(assets)
    }

    fn read_exam_signal(repo: &RepoPaths, lesson: &LessonPaths) -> Result<Value, RelevanceError> {
        let exam_dir = lesson.exam_raw_dir();
        let mut text_files = Vec::new();
        let mut pdf_files = Vec::new();
        let mut other_files = Vec::new();

        let entries = match fs::read_dir(&exam_dir) {
            Ok(entries) => entries,
            Err(source) if source.kind() == io::ErrorKind::NotFound => {
                return Ok(json!({
                    "path": repo.relative_display(&exam_dir),
                    "text_files": [],
                    "pdf_files": [],
                    "other_files": [],
                    "notes": ["exam_dir_missing"]
                }));
            }
            Err(source) => {
                return Err(RelevanceError::Read {
                    path: exam_dir,
                    source,
                });
            }
        };

        for entry in entries {
            let entry = entry.map_err(|source| RelevanceError::Read {
                path: exam_dir.clone(),
                source,
            })?;
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            let file_name = path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or_default()
                .to_owned();
            match path.extension().and_then(|ext| ext.to_str()) {
                Some(ext) if ext.eq_ignore_ascii_case("txt") || ext.eq_ignore_ascii_case("md") => {
                    text_files.push(json!({
                        "file": file_name,
                        "path": repo.relative_display(&path),
                        "content": Self::read_required_text(&path)?
                    }));
                }
                Some(ext) if ext.eq_ignore_ascii_case("pdf") => {
                    let cache_path = Self::exam_pdf_plain_text_path(lesson, &path);
                    match cache_path {
                        Some(cache_path) if cache_path.is_file() => {
                            pdf_files.push(json!({
                                "file": file_name,
                                "path": repo.relative_display(&path),
                                "plain_text_path": repo.relative_display(&cache_path),
                                "content": Self::read_required_text(&cache_path)?,
                                "note": "exam_pdf_text_cached"
                            }));
                        }
                        Some(cache_path) => {
                            pdf_files.push(json!({
                                "file": file_name,
                                "path": repo.relative_display(&path),
                                "plain_text_path": repo.relative_display(&cache_path),
                                "note": "exam_pdf_text_missing"
                            }));
                        }
                        None => {
                            pdf_files.push(json!({
                                "file": file_name,
                                "path": repo.relative_display(&path),
                                "note": "exam_pdf_text_unavailable"
                            }));
                        }
                    }
                }
                _ => {
                    other_files.push(json!({
                        "file": file_name,
                        "path": repo.relative_display(&path)
                    }));
                }
            }
        }

        text_files.sort_by_key(|value| Self::json_string_field(value, "file"));
        pdf_files.sort_by_key(|value| Self::json_string_field(value, "file"));
        other_files.sort_by_key(|value| Self::json_string_field(value, "file"));

        let mut notes = BTreeSet::new();
        for pdf_file in &pdf_files {
            if let Some(note) = pdf_file.get("note").and_then(Value::as_str) {
                notes.insert(note);
            }
        }
        let notes = notes.into_iter().collect::<Vec<_>>();

        Ok(json!({
            "path": repo.relative_display(&exam_dir),
            "text_files": text_files,
            "pdf_files": pdf_files,
            "other_files": other_files,
            "notes": notes
        }))
    }

    fn exam_pdf_plain_text_path(lesson: &LessonPaths, path: &Path) -> Option<PathBuf> {
        let stem = path.file_stem()?.to_str()?;
        Some(lesson.exam_plain_text_path(stem))
    }

    fn extract_textbook_sections(textbook: &str) -> Vec<Value> {
        let mut sections = Vec::new();
        let mut current_title = String::new();
        let mut current_level = 0_usize;
        let mut current_content = Vec::new();

        for line in textbook.lines() {
            if let Some((level, title)) = Self::markdown_heading(line) {
                if !current_title.is_empty() {
                    sections.push(Self::textbook_section_value(
                        sections.len() + 1,
                        current_level,
                        &current_title,
                        &current_content.join("\n"),
                    ));
                }
                current_level = level;
                current_title = title.to_owned();
                current_content.clear();
            } else if !current_title.is_empty() {
                current_content.push(line.to_owned());
            }
        }

        if !current_title.is_empty() {
            sections.push(Self::textbook_section_value(
                sections.len() + 1,
                current_level,
                &current_title,
                &current_content.join("\n"),
            ));
        }

        sections
    }

    fn textbook_section_value(index: usize, level: usize, title: &str, content: &str) -> Value {
        json!({
            "section_id": format!("section_{index}"),
            "level": level,
            "title": title,
            "content": content
        })
    }

    fn markdown_heading(line: &str) -> Option<(usize, &str)> {
        let trimmed = line.trim_start();
        let level = trimmed.chars().take_while(|ch| *ch == '#').count();
        if level == 0 || level > 6 {
            return None;
        }
        let title = trimmed.get(level..)?.trim();
        if title.is_empty() {
            return None;
        }
        Some((level, title))
    }

    fn collect_question_families(question_bank: &Value) -> Vec<Value> {
        let mut families = Vec::new();
        for field in ["flashcard_families", "quiz_families", "longform_families"] {
            if let Some(values) = question_bank.get(field).and_then(Value::as_array) {
                for value in values {
                    if let Some(family_id) = value.get("family_id").and_then(Value::as_str) {
                        families.push(json!({
                            "family_id": family_id,
                            "kind": field,
                            "summary": value
                        }));
                    }
                }
            }
        }
        families
    }

    fn system_prompt() -> String {
        r#"You are an asset-level relevance scoring agent for a course generation pipeline.

Score the assets only. Do not rewrite, delete, filter, summarize for replacement, or edit any step, question, or textbook text.

Use the provided course description, lesson plain text, guided story manifest, step content, step-local question families, textbook sections, and exam signal. The exam signal is evidence for relevance, but it is weak when it comes from a single sample. Do not mark absent topics low solely because they do not appear in the exam. Judge each asset by its role in the course, the lesson source material, likely assessability, and alignment with the course outcomes.

Return JSON only. Use scores from 0 to 1 for importance, course_relevance, and exam_relevance. recommended_treatment should be one of: keep, emphasize, condense, de_emphasize, review.

The JSON must include:
{
  "lesson_id": string,
  "target_language": string,
  "exam_signal": object,
  "step_scores": [{"sequence_id": string, "importance": number, "course_relevance": number, "exam_relevance": number, "recommended_treatment": string, "reason": string}],
  "question_family_scores": [{"sequence_id": string, "family_id": string, "importance": number, "course_relevance": number, "exam_relevance": number, "recommended_treatment": string, "reason": string}],
  "textbook_section_scores": [{"section_id": string, "title": string, "importance": number, "course_relevance": number, "exam_relevance": number, "recommended_treatment": string, "reason": string}],
  "coverage_scores": [{"coverage_id": string, "title": string, "importance": number, "course_relevance": number, "exam_relevance": number, "recommended_treatment": string, "reason": string}]
}"#
        .to_owned()
    }

    fn render_user_prompt(input: &Value) -> Result<String, RelevanceError> {
        let input = serde_json::to_string_pretty(input)
            .map_err(|source| RelevanceError::JsonRender { source })?;
        Ok(format!(
        "Score asset-level relevance for the following lesson inputs. Output JSON only.\n\n<SCORER_INPUT_JSON>\n{input}\n</SCORER_INPUT_JSON>\n"
    ))
    }

    fn call_chat_completion(
        lesson: &LessonPaths,
        llm: &LlmReadyConfig,
        system: &str,
        user: &str,
    ) -> Result<LlmResponse, RelevanceError> {
        let audit_dir = lesson.relevance_dir();
        fs::create_dir_all(&audit_dir).map_err(|source| RelevanceError::Write {
            path: audit_dir.clone(),
            source,
        })?;
        fs::write(audit_dir.join("system.md"), system).map_err(|source| RelevanceError::Write {
            path: audit_dir.join("system.md"),
            source,
        })?;
        fs::write(audit_dir.join("user.md"), user).map_err(|source| RelevanceError::Write {
            path: audit_dir.join("user.md"),
            source,
        })?;

        let client = LlmClient::new(llm);
        let request = client.request(system, user, 0.1);
        Self::write_pretty_json(&audit_dir.join("request.json"), &request)?;

        let response = client.send(&request).map_err(Self::map_llm_error)?;
        fs::write(audit_dir.join("raw_response.txt"), &response.raw_text).map_err(|source| {
            RelevanceError::Write {
                path: audit_dir.join("raw_response.txt"),
                source,
            }
        })?;
        Self::write_pretty_json(&audit_dir.join("raw_response.json"), &response.raw_json)?;
        fs::write(audit_dir.join("content.txt"), &response.content).map_err(|source| {
            RelevanceError::Write {
                path: audit_dir.join("content.txt"),
                source,
            }
        })?;

        Ok(LlmResponse {
            content: response.content,
        })
    }

    fn map_llm_error(error: LlmClientError) -> RelevanceError {
        match error {
            LlmClientError::Request(source) => RelevanceError::LlmRequest { source },
            LlmClientError::Status { status, body } => RelevanceError::LlmStatus { status, body },
            LlmClientError::Json(source) => RelevanceError::JsonParse {
                stage: STAGE,
                source,
            },
            LlmClientError::MissingContent => RelevanceError::LlmMissingContent,
        }
    }

    fn validate_report(report: &Value, known_refs: &KnownRefs) -> Result<(), RelevanceError> {
        let step_scores = report.get("step_scores").and_then(Value::as_array).ok_or(
            RelevanceError::ReportShape {
                message: "report.step_scores must be an array".to_owned(),
            },
        )?;
        for score in step_scores {
            let sequence_id = Self::required_string(score, "sequence_id").ok_or_else(|| {
                RelevanceError::ReportShape {
                    message: "each step_scores item must include sequence_id".to_owned(),
                }
            })?;
            if !known_refs.sequence_ids.contains(sequence_id) {
                return Err(RelevanceError::UnknownStepScore {
                    sequence_id: sequence_id.to_owned(),
                });
            }
        }

        let family_scores = report
            .get("question_family_scores")
            .and_then(Value::as_array)
            .ok_or(RelevanceError::ReportShape {
                message: "report.question_family_scores must be an array".to_owned(),
            })?;
        for score in family_scores {
            let sequence_id = Self::required_string(score, "sequence_id").ok_or_else(|| {
                RelevanceError::ReportShape {
                    message: "each question_family_scores item must include sequence_id".to_owned(),
                }
            })?;
            let family_id = Self::required_string(score, "family_id").ok_or_else(|| {
                RelevanceError::ReportShape {
                    message: "each question_family_scores item must include family_id".to_owned(),
                }
            })?;
            if !known_refs
                .family_ids
                .contains(&(sequence_id.to_owned(), family_id.to_owned()))
            {
                return Err(RelevanceError::UnknownQuestionFamilyScore {
                    sequence_id: sequence_id.to_owned(),
                    family_id: family_id.to_owned(),
                });
            }
        }

        if report
            .get("textbook_section_scores")
            .is_some_and(|value| !value.is_array())
        {
            return Err(RelevanceError::ReportShape {
                message: "report.textbook_section_scores must be an array when present".to_owned(),
            });
        }

        if report
            .get("coverage_scores")
            .is_some_and(|value| !value.is_array())
        {
            return Err(RelevanceError::ReportShape {
                message: "report.coverage_scores must be an array when present".to_owned(),
            });
        }

        Ok(())
    }
}

#[derive(Debug)]
struct KnownRefs {
    sequence_ids: BTreeSet<String>,
    family_ids: BTreeSet<(String, String)>,
}

impl KnownRefs {
    fn from_input(input: &Value) -> Self {
        let mut sequence_ids = BTreeSet::new();
        let mut family_ids = BTreeSet::new();
        if let Some(assets) = input.get("step_assets").and_then(Value::as_array) {
            for asset in assets {
                let Some(sequence_id) = asset.get("sequence_id").and_then(Value::as_str) else {
                    continue;
                };
                sequence_ids.insert(sequence_id.to_owned());
                if let Some(families) = asset.get("question_families").and_then(Value::as_array) {
                    for family in families {
                        if let Some(family_id) = family.get("family_id").and_then(Value::as_str) {
                            family_ids.insert((sequence_id.to_owned(), family_id.to_owned()));
                        }
                    }
                }
            }
        }

        Self {
            sequence_ids,
            family_ids,
        }
    }
}

struct LlmResponse {
    content: String,
}

impl RelevanceScorer {
    fn read_json(path: &Path, label: &'static str) -> Result<Value, RelevanceError> {
        let content = Self::read_required_text(path)?;
        serde_json::from_str(&content).map_err(|source| RelevanceError::InputJsonParse {
            label,
            path: path.to_path_buf(),
            source,
        })
    }

    fn read_required_text(path: &Path) -> Result<String, RelevanceError> {
        fs::read_to_string(path).map_err(|source| RelevanceError::Read {
            path: path.to_path_buf(),
            source,
        })
    }

    fn read_optional_text(path: &Path) -> Result<String, RelevanceError> {
        match fs::read_to_string(path) {
            Ok(contents) => Ok(contents),
            Err(source) if source.kind() == io::ErrorKind::NotFound => Ok(String::new()),
            Err(source) => Err(RelevanceError::Read {
                path: path.to_path_buf(),
                source,
            }),
        }
    }

    fn write_pretty_json<T>(path: &Path, value: &T) -> Result<(), RelevanceError>
    where
        T: Serialize,
    {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|source| RelevanceError::Write {
                path: parent.to_path_buf(),
                source,
            })?;
        }

        let mut payload =
            serde_json::to_vec_pretty(value).map_err(|source| RelevanceError::JsonWrite {
                path: path.to_path_buf(),
                source,
            })?;
        payload.push(b'\n');
        fs::write(path, payload).map_err(|source| RelevanceError::Write {
            path: path.to_path_buf(),
            source,
        })
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

    fn step_sort_key(name: &str) -> (usize, String) {
        let number = name
            .strip_prefix("step")
            .and_then(|value| value.parse::<usize>().ok())
            .unwrap_or(usize::MAX);
        (number, name.to_owned())
    }

    fn json_string_field(value: &Value, field: &str) -> String {
        value
            .get(field)
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_owned()
    }

    fn required_string<'a>(value: &'a Value, field: &str) -> Option<&'a str> {
        value.get(field).and_then(Value::as_str)
    }
}

#[derive(Debug)]
pub enum RelevanceError {
    Config(ConfigError),
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
    JsonRender {
        source: serde_json::Error,
    },
    InputJsonParse {
        label: &'static str,
        path: PathBuf,
        source: serde_json::Error,
    },
    JsonParse {
        stage: &'static str,
        source: serde_json::Error,
    },
    LlmRequest {
        source: reqwest::Error,
    },
    LlmStatus {
        status: u16,
        body: String,
    },
    LlmMissingContent,
    ReportShape {
        message: String,
    },
    UnknownStepScore {
        sequence_id: String,
    },
    UnknownQuestionFamilyScore {
        sequence_id: String,
        family_id: String,
    },
}

impl fmt::Display for RelevanceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Config(error) => error.fmt(f),
            Self::Read { path, source } => {
                write!(f, "failed to read relevance input {}: {source}", path.display())
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
            Self::JsonRender { source } => write!(f, "failed to render scorer input: {source}"),
            Self::InputJsonParse {
                label,
                path,
                source,
            } => write!(
                f,
                "{label} JSON is invalid at {}: {source}",
                PathText::to_forward_slashes(path)
            ),
            Self::JsonParse { stage, source } => {
                write!(f, "{stage} model output was not valid JSON: {source}")
            }
            Self::LlmRequest { source } => {
                write!(f, "relevance LLM request failed: {source:?}")
            }
            Self::LlmStatus { status, body } => {
                write!(f, "relevance LLM request returned HTTP {status}: {body}")
            }
            Self::LlmMissingContent => {
                write!(
                    f,
                    "relevance LLM response did not include choices[0].message.content"
                )
            }
            Self::ReportShape { message } => write!(f, "invalid relevance report: {message}"),
            Self::UnknownStepScore { sequence_id } => write!(
                f,
                "invalid relevance report: step_scores references unknown sequence_id '{sequence_id}'"
            ),
            Self::UnknownQuestionFamilyScore {
                sequence_id,
                family_id,
            } => write!(
                f,
                "invalid relevance report: question_family_scores references unknown family_id '{family_id}' for sequence_id '{sequence_id}'"
            ),
        }
    }
}

impl Error for RelevanceError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Config(error) => Some(error),
            Self::Read { source, .. } | Self::Write { source, .. } => Some(source),
            Self::JsonWrite { source, .. }
            | Self::JsonRender { source }
            | Self::InputJsonParse { source, .. }
            | Self::JsonParse { source, .. } => Some(source),
            Self::LlmRequest { source } => Some(source),
            Self::LlmStatus { .. }
            | Self::LlmMissingContent
            | Self::ReportShape { .. }
            | Self::UnknownStepScore { .. }
            | Self::UnknownQuestionFamilyScore { .. } => None,
        }
    }
}

impl From<ConfigError> for RelevanceError {
    fn from(error: ConfigError) -> Self {
        Self::Config(error)
    }
}
