use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fmt, fs, io,
    path::{Path, PathBuf},
    thread,
    time::Duration,
};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    cli::ForceStage,
    config::{Config, ConfigError, LlmReadyConfig},
    llm::{LlmClient, LlmClientError},
    mineru::{ConvertError, MineruConverter},
    paths::LessonPaths,
    validation::{OutputValidator, ValidationError},
};

pub struct MvpRunner;
const GUIDED_STORY_VALIDATION_MAX_ROUNDS: usize = 3;

impl MvpRunner {
    pub fn run(
        repo: &crate::paths::RepoPaths,
        lesson: &LessonPaths,
        target_language: &str,
        force_stage: ForceStage,
    ) -> Result<(), MvpError> {
        let force_convert = matches!(force_stage, ForceStage::Convert | ForceStage::All);
        let force_guided_story = matches!(force_stage, ForceStage::GuidedStory | ForceStage::All);
        let force_question_bank = matches!(
            force_stage,
            ForceStage::GuidedStory | ForceStage::QuestionBank | ForceStage::All
        );
        let force_textbook = matches!(
            force_stage,
            ForceStage::GuidedStory
                | ForceStage::QuestionBank
                | ForceStage::Textbook
                | ForceStage::All
        );

        if force_convert || !lesson.plain_text_path().is_file() {
            MineruConverter::convert_lesson(repo, lesson, false)?;
        } else {
            eprintln!(
                "reusing plain text: {}",
                lesson.relative_display(&lesson.plain_text_path())
            );
        }

        let plain_text = Self::read_required_text(&lesson.plain_text_path())?;
        let related_important = lesson
            .related_important_path()
            .map(|path| Self::read_optional_text(&path))
            .transpose()?
            .unwrap_or_default();
        let mut llm = None;

        if force_guided_story || !Self::guided_story_ready(lesson) {
            let llm = Self::ready_llm(repo, &mut llm)?;
            Self::generate_guided_story(
                lesson,
                llm,
                target_language,
                &plain_text,
                &related_important,
            )?;
        } else {
            eprintln!(
                "reusing guided story: {}",
                lesson.relative_display(&lesson.guided_story_manifest_path())
            );
        }

        let generated_question_banks = Self::generate_question_bank(
            lesson,
            &mut llm,
            force_question_bank,
            target_language,
            &plain_text,
            &related_important,
        )?;

        if force_textbook || generated_question_banks || !Self::textbook_ready(lesson) {
            let llm = Self::ready_llm(repo, &mut llm)?;
            Self::generate_textbook(
                lesson,
                llm,
                target_language,
                &plain_text,
                &related_important,
            )?;
        } else {
            eprintln!(
                "reusing textbook: {}",
                lesson.relative_display(&lesson.textbook_path())
            );
        }

        OutputValidator::new(lesson).validate()?;

        Ok(())
    }

    fn ready_llm<'a>(
        repo: &crate::paths::RepoPaths,
        llm: &'a mut Option<LlmReadyConfig>,
    ) -> Result<&'a LlmReadyConfig, MvpError> {
        if llm.is_none() {
            let config = Config::load(repo)?;
            *llm = Some(config.llm.require_ready()?);
        }
        Ok(llm.as_ref().expect("llm config should be initialized"))
    }

    fn guided_story_ready(lesson: &LessonPaths) -> bool {
        let Ok(step_refs) = Self::read_manifest_step_refs(lesson) else {
            return false;
        };
        if step_refs.is_empty() {
            return false;
        }
        step_refs.iter().all(|step_ref| {
            matches!(
                Self::read_json(&step_ref.path, Stage::GuidedStory),
                Ok(step) if OutputValidator::validate_guided_story_step_exercise_kinds(
                    &step_ref.path,
                    &step,
                )
                .is_ok()
            )
        })
    }

    fn question_bank_ready(path: &Path) -> bool {
        Self::read_json(path, Stage::QuestionBank).is_ok()
    }

    fn question_bank_gate_ready(path: &Path) -> bool {
        Self::read_json(
            &Self::rejected_question_bank_path(path),
            Stage::QuestionBankGate,
        )
        .is_ok()
    }

    fn textbook_ready(lesson: &LessonPaths) -> bool {
        lesson.textbook_path().is_file() && OutputValidator::new(lesson).validate().is_ok()
    }

    fn generate_guided_story(
        lesson: &LessonPaths,
        llm: &LlmReadyConfig,
        target_language: &str,
        plain_text: &str,
        related_important: &str,
    ) -> Result<(), MvpError> {
        let repo = lesson.repo();
        let course_id = lesson.course_id().unwrap_or("");
        let course_title = lesson.course_title().unwrap_or("");
        let chapter_id = lesson.chapter_id().unwrap_or("");
        let chapter_title = lesson.chapter_title().unwrap_or("");
        let draft_system = Self::read_prompt(repo, "guided_story_system.md")?;
        let draft_user = Self::render_prompt(
            &Self::read_prompt(repo, "guided_story_user.md")?,
            &[
                ("target_language", target_language),
                ("course_id", course_id),
                ("course_title", course_title),
                ("chapter_id", chapter_id),
                ("chapter_title", chapter_title),
                ("lesson_id", lesson.lesson_id()),
                (
                    "step_scope",
                    "full lecture; split into 4-8 natural learning steps unless the source is very short",
                ),
                ("related_important", related_important),
                ("plain_text", plain_text),
                ("image_hints", ""),
            ],
        );
        let draft_response = Self::call_chat_completion_with_audit(
            lesson,
            llm,
            Stage::GuidedStoryDraft,
            "guided_story/draft",
            &draft_system,
            &draft_user,
        )?;
        let draft_content = Self::strip_code_fence(&draft_response.content, Some("json"));
        let draft_story_json = Self::parse_llm_json(
            lesson,
            Stage::GuidedStoryDraft,
            "guided_story/draft",
            &draft_content,
        )?;
        let input_draft = Self::to_pretty_json_string(
            &lesson.audit_stage_dir("guided_story/draft"),
            &draft_story_json,
        )?;

        let planner_user = Self::render_prompt(
            &Self::read_prompt(repo, "guided_story_planner_user.md")?,
            &[("INPUT_DRAFT", &input_draft)],
        );
        let planner_response = Self::call_chat_completion_with_audit(
            lesson,
            llm,
            Stage::GuidedStoryPlanner,
            "guided_story/planner",
            "",
            &planner_user,
        )?;
        let planner_content = Self::strip_code_fence(&planner_response.content, Some("json"));
        let planner_json = Self::parse_llm_json(
            lesson,
            Stage::GuidedStoryPlanner,
            "guided_story/planner",
            &planner_content,
        )?;
        let planner_output = Self::to_pretty_json_string(
            &lesson.audit_stage_dir("guided_story/planner"),
            &planner_json,
        )?;

        let reviewer_user = Self::render_prompt(
            &Self::read_prompt(repo, "guided_story_contract_reviewer_user.md")?,
            &[
                ("INPUT_DRAFT", &input_draft),
                ("PLANNER_OUTPUT", &planner_output),
            ],
        );
        let reviewer_response = Self::call_chat_completion_with_audit(
            lesson,
            llm,
            Stage::GuidedStoryReviewer,
            "guided_story/reviewer",
            "",
            &reviewer_user,
        )?;
        let reviewer_content = Self::strip_code_fence(&reviewer_response.content, Some("json"));
        let reviewer_json = Self::parse_llm_json(
            lesson,
            Stage::GuidedStoryReviewer,
            "guided_story/reviewer",
            &reviewer_content,
        )?;
        let revised_outline = Self::to_pretty_json_string(
            &lesson.audit_stage_dir("guided_story/reviewer"),
            reviewer_json.get("revised_outline").unwrap_or(&Value::Null),
        )?;
        let patch_plan = Self::to_pretty_json_string(
            &lesson.audit_stage_dir("guided_story/reviewer"),
            reviewer_json.get("patch_plan").unwrap_or(&Value::Null),
        )?;
        let execution_contract = Self::to_pretty_json_string(
            &lesson.audit_stage_dir("guided_story/reviewer"),
            reviewer_json
                .get("execution_contract")
                .unwrap_or(&Value::Null),
        )?;

        let executor_prompt = Self::read_prompt(repo, "guided_story_contract_executor_user.md")?;
        let validator_prompt = Self::read_prompt(repo, "guided_story_validator_user.md")?;
        let mut previous_output = "无。第一轮没有上一版正文。".to_owned();
        let mut retry_feedback = "无。第一轮直接按合同执行。".to_owned();
        let mut final_story_json = draft_story_json;
        let mut last_validator = None;
        let mut validator_rounds = 0usize;

        for round in 1..=GUIDED_STORY_VALIDATION_MAX_ROUNDS {
            let executor_user = Self::render_prompt(
                &executor_prompt,
                &[
                    ("INPUT_DRAFT", &input_draft),
                    ("REVISED_OUTLINE", &revised_outline),
                    ("PATCH_PLAN", &patch_plan),
                    ("EXECUTION_CONTRACT", &execution_contract),
                    ("RETRY_FEEDBACK", &retry_feedback),
                    ("PREVIOUS_OUTPUT", &previous_output),
                ],
            );
            let executor_audit_stage = format!("guided_story/executor/round{round}");
            let executor_response = Self::call_chat_completion_with_audit(
                lesson,
                llm,
                Stage::GuidedStoryExecutor,
                &executor_audit_stage,
                "",
                &executor_user,
            )?;
            let executor_content = Self::strip_code_fence(&executor_response.content, Some("json"));
            let executor_json = Self::parse_llm_json(
                lesson,
                Stage::GuidedStoryExecutor,
                &executor_audit_stage,
                &executor_content,
            )?;
            previous_output = Self::to_pretty_json_string(
                &lesson.audit_stage_dir(&executor_audit_stage),
                &executor_json,
            )?;
            final_story_json = executor_json;

            let validator_user = Self::render_prompt(
                &validator_prompt,
                &[
                    ("REVISED_OUTLINE", &revised_outline),
                    ("PATCH_PLAN", &patch_plan),
                    ("EXECUTION_CONTRACT", &execution_contract),
                    ("EXECUTOR_OUTPUT", &previous_output),
                ],
            );
            let validator_audit_stage = format!("guided_story/validator/round{round}");
            let validator_response = Self::call_chat_completion_with_audit(
                lesson,
                llm,
                Stage::GuidedStoryValidator,
                &validator_audit_stage,
                "",
                &validator_user,
            )?;
            let validator_content =
                Self::strip_code_fence(&validator_response.content, Some("json"));
            let validator_json = Self::parse_llm_json(
                lesson,
                Stage::GuidedStoryValidator,
                &validator_audit_stage,
                &validator_content,
            )?;
            validator_rounds = round;
            let passed = Self::guided_story_validator_pass(&validator_json);
            retry_feedback = Self::to_pretty_json_string(
                &lesson.audit_stage_dir(&validator_audit_stage),
                validator_json.get("retry_feedback").unwrap_or(&Value::Null),
            )?;
            last_validator = Some(validator_json);
            if passed {
                break;
            }
        }

        if let Some(validator_json) = last_validator.as_ref() {
            let scores = validator_json.get("scores").cloned().unwrap_or(Value::Null);
            if Self::guided_story_validator_pass(validator_json) {
                eprintln!(
                    "guided story validator passed after {} round(s): {}",
                    validator_rounds,
                    lesson.relative_display(&lesson.guided_story_manifest_path())
                );
            } else {
                eprintln!(
                    "guided story validator did not pass after {} round(s); using latest executor output with scores {}",
                    validator_rounds,
                    scores
                );
            }
        }

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

        let manifest = Self::write_guided_story_steps(lesson, &final_story_json)?;
        Self::write_pretty_json(&lesson.guided_story_manifest_path(), &manifest)?;
        Ok(())
    }

    fn generate_question_bank(
        lesson: &LessonPaths,
        llm: &mut Option<LlmReadyConfig>,
        force: bool,
        target_language: &str,
        plain_text: &str,
        related_important: &str,
    ) -> Result<bool, MvpError> {
        let repo = lesson.repo();
        let course_id = lesson.course_id().unwrap_or("");
        let course_title = lesson.course_title().unwrap_or("");
        let chapter_id = lesson.chapter_id().unwrap_or("");
        let chapter_title = lesson.chapter_title().unwrap_or("");
        let system = Self::read_prompt(repo, "question_bank_system.md")?;
        let manifest = Self::read_required_text(&lesson.guided_story_manifest_path())?;
        let step_refs = Self::read_manifest_step_refs(lesson)?;
        let step_count = step_refs.len();
        let steps = Self::read_guided_story_steps(lesson, &step_refs)?;
        let source_outline = Self::source_outline_from_plain_text(plain_text);
        let prompt = Self::read_prompt(repo, "question_bank_user.md")?;
        let gate_system = Self::read_prompt(repo, "question_bank_gate_system.md")?;
        let gate_prompt = Self::read_prompt(repo, "question_bank_gate_user.md")?;
        let mut generated_any = false;

        for step_ref in step_refs {
            let current_step = Self::read_required_text(&step_ref.path)?;
            if !force && Self::question_bank_ready(&step_ref.question_bank_path) {
                if Self::question_bank_gate_ready(&step_ref.question_bank_path) {
                    eprintln!(
                        "reusing question bank: {}",
                        lesson.relative_display(&step_ref.question_bank_path)
                    );
                    continue;
                }

                eprintln!(
                    "gating existing question bank: {}",
                    lesson.relative_display(&step_ref.question_bank_path)
                );
                let llm = Self::ready_llm(repo, llm)?;
                let mut question_bank =
                    Self::read_json(&step_ref.question_bank_path, Stage::QuestionBank)?;
                Self::normalize_question_bank_json(
                    lesson,
                    &step_ref.sequence_id,
                    &mut question_bank,
                );
                Self::gate_and_write_question_bank(
                    lesson,
                    llm,
                    &gate_system,
                    &gate_prompt,
                    &step_ref,
                    step_count,
                    target_language,
                    course_id,
                    course_title,
                    chapter_id,
                    chapter_title,
                    &current_step,
                    &mut question_bank,
                )?;
                generated_any = true;
                continue;
            }

            let user = Self::render_prompt(
                &prompt,
                &[
                    ("target_language", target_language),
                    ("course_id", course_id),
                    ("course_title", course_title),
                    ("chapter_id", chapter_id),
                    ("chapter_title", chapter_title),
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
            let llm = Self::ready_llm(repo, llm)?;
            let response = Self::call_chat_completion_with_audit(
                lesson,
                llm,
                Stage::QuestionBank,
                &audit_stage,
                &system,
                &user,
            )?;
            let content = Self::strip_code_fence(&response.content, Some("json"));
            let mut question_bank =
                Self::parse_llm_json(lesson, Stage::QuestionBank, &audit_stage, &content)?;
            Self::normalize_question_bank_json(lesson, &step_ref.sequence_id, &mut question_bank);

            Self::gate_and_write_question_bank(
                lesson,
                llm,
                &gate_system,
                &gate_prompt,
                &step_ref,
                step_count,
                target_language,
                course_id,
                course_title,
                chapter_id,
                chapter_title,
                &current_step,
                &mut question_bank,
            )?;
            generated_any = true;
        }

        Ok(generated_any)
    }

    fn gate_question_bank(
        lesson: &LessonPaths,
        llm: &LlmReadyConfig,
        system: &str,
        prompt: &str,
        audit_stage: &str,
        target_language: &str,
        course_id: &str,
        course_title: &str,
        chapter_id: &str,
        chapter_title: &str,
        current_step_id: &str,
        current_step: &str,
        question_bank: &mut Value,
    ) -> Result<Value, MvpError> {
        let candidate_question_bank =
            serde_json::to_string_pretty(question_bank).map_err(|source| MvpError::JsonWrite {
                path: lesson.audit_stage_dir(audit_stage),
                source,
            })?;
        let user = Self::render_prompt(
            prompt,
            &[
                ("target_language", target_language),
                ("course_id", course_id),
                ("course_title", course_title),
                ("chapter_id", chapter_id),
                ("chapter_title", chapter_title),
                ("lesson_id", lesson.lesson_id()),
                ("current_step_id", current_step_id),
                ("current_step", current_step),
                ("candidate_question_bank", &candidate_question_bank),
            ],
        );
        let response = Self::call_chat_completion_with_audit(
            lesson,
            llm,
            Stage::QuestionBankGate,
            audit_stage,
            system,
            &user,
        )?;
        let content = Self::strip_code_fence(&response.content, Some("json"));
        let report: GateReport = serde_json::from_value(Self::parse_llm_json(
            lesson,
            Stage::QuestionBankGate,
            audit_stage,
            &content,
        )?)
        .map_err(|source| MvpError::JsonParse {
            stage: Stage::QuestionBankGate.as_str(),
            source,
        })?;

        Ok(QuestionBankGate::apply(question_bank, report))
    }

    fn gate_and_write_question_bank(
        lesson: &LessonPaths,
        llm: &LlmReadyConfig,
        gate_system: &str,
        gate_prompt: &str,
        step_ref: &StepRef,
        step_count: usize,
        target_language: &str,
        course_id: &str,
        course_title: &str,
        chapter_id: &str,
        chapter_title: &str,
        current_step: &str,
        question_bank: &mut Value,
    ) -> Result<(), MvpError> {
        if let Some(parent) = step_ref.question_bank_path.parent() {
            fs::create_dir_all(parent).map_err(|source| MvpError::Write {
                path: parent.to_path_buf(),
                source,
            })?;
        }
        Self::write_pretty_json(
            &Self::candidate_question_bank_path(&step_ref.question_bank_path),
            question_bank,
        )?;

        let gate_audit_stage = if step_count == 1 {
            "question_bank_gate".to_owned()
        } else {
            format!("question_bank_gate/{}", step_ref.sequence_id)
        };
        let rejected = Self::gate_question_bank(
            lesson,
            llm,
            gate_system,
            gate_prompt,
            &gate_audit_stage,
            target_language,
            course_id,
            course_title,
            chapter_id,
            chapter_title,
            &step_ref.sequence_id,
            current_step,
            question_bank,
        )?;
        eprintln!(
            "question bank gate kept {}, rejected {}: {}",
            QuestionBankGate::count_question_families(question_bank),
            rejected
                .get("rejected_family_ids")
                .and_then(Value::as_array)
                .map(Vec::len)
                .unwrap_or(0),
            lesson.relative_display(&step_ref.question_bank_path)
        );
        Self::write_pretty_json(
            &Self::rejected_question_bank_path(&step_ref.question_bank_path),
            &rejected,
        )?;
        Self::write_pretty_json(&step_ref.question_bank_path, question_bank)
    }

    fn generate_textbook(
        lesson: &LessonPaths,
        llm: &LlmReadyConfig,
        target_language: &str,
        plain_text: &str,
        related_important: &str,
    ) -> Result<(), MvpError> {
        let repo = lesson.repo();
        let course_id = lesson.course_id().unwrap_or("");
        let course_title = lesson.course_title().unwrap_or("");
        let chapter_id = lesson.chapter_id().unwrap_or("");
        let chapter_title = lesson.chapter_title().unwrap_or("");
        let system = Self::read_prompt(repo, "textbook_system.md")?;
        let manifest = Self::read_required_text(&lesson.guided_story_manifest_path())?;
        let step_refs = Self::read_manifest_step_refs(lesson)?;
        let steps = Self::read_guided_story_steps(lesson, &step_refs)?;
        let question_bank = Self::read_step_question_banks(lesson)?;
        let source_outline = Self::source_outline_from_plain_text(plain_text);
        let user = Self::render_prompt(
            &Self::read_prompt(repo, "textbook_user.md")?,
            &[
                ("target_language", target_language),
                ("course_id", course_id),
                ("course_title", course_title),
                ("chapter_id", chapter_id),
                ("chapter_title", chapter_title),
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
        let response = Self::call_chat_completion(lesson, llm, Stage::Textbook, &system, &user)?;
        let content =
            Self::sanitize_textbook_mdx(&Self::strip_code_fence(&response.content, Some("mdx")));

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

    fn sanitize_textbook_mdx(source: &str) -> String {
        source
            .lines()
            .map(Self::strip_heading_anchor)
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn strip_heading_anchor(line: &str) -> String {
        let trimmed = line.trim_end();
        if !trimmed.starts_with('#') || !trimmed.ends_with('}') {
            return line.to_owned();
        }

        let Some(anchor_start) = trimmed.rfind(" {#") else {
            return line.to_owned();
        };

        let anchor = &trimmed[anchor_start + 2..];
        if anchor.len() < 4 || !anchor.starts_with("{#") || !anchor.ends_with('}') {
            return line.to_owned();
        }

        trimmed[..anchor_start].to_owned()
    }

    fn call_chat_completion(
        lesson: &LessonPaths,
        llm: &LlmReadyConfig,
        stage: Stage,
        system: &str,
        user: &str,
    ) -> Result<LlmResponse, MvpError> {
        Self::call_chat_completion_with_audit(lesson, llm, stage, stage.as_str(), system, user)
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
            match Self::call_chat_completion_once(lesson, llm, stage, audit_stage, system, user) {
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

        let client = LlmClient::new(llm);
        let request = client.request(system, user, 0.2);
        Self::write_pretty_json(&audit_dir.join("request.json"), &request)?;

        let response = client
            .send(&request)
            .map_err(|error| Self::map_llm_error(stage, error))?;
        fs::write(audit_dir.join("raw_response.txt"), &response.raw_text).map_err(|source| {
            MvpError::Write {
                path: audit_dir.join("raw_response.txt"),
                source,
            }
        })?;
        Self::write_pretty_json(&audit_dir.join("raw_response.json"), &response.raw_json)?;
        fs::write(audit_dir.join("content.txt"), &response.content).map_err(|source| {
            MvpError::Write {
                path: audit_dir.join("content.txt"),
                source,
            }
        })?;

        Ok(LlmResponse {
            content: response.content,
        })
    }

    fn map_llm_error(stage: Stage, error: LlmClientError) -> MvpError {
        match error {
            LlmClientError::Request(source) => MvpError::LlmRequest {
                stage: stage.as_str(),
                source,
            },
            LlmClientError::Status { status, body } => MvpError::LlmStatus {
                stage: stage.as_str(),
                status,
                body,
            },
            LlmClientError::Json(source) => MvpError::JsonParse {
                stage: stage.as_str(),
                source,
            },
            LlmClientError::MissingContent => MvpError::LlmMissingContent {
                stage: stage.as_str(),
            },
        }
    }
}

#[derive(Clone, Copy)]
enum Stage {
    GuidedStory,
    GuidedStoryDraft,
    GuidedStoryPlanner,
    GuidedStoryReviewer,
    GuidedStoryExecutor,
    GuidedStoryValidator,
    QuestionBank,
    QuestionBankGate,
    Textbook,
}

impl Stage {
    fn as_str(self) -> &'static str {
        match self {
            Self::GuidedStory => "guided_story",
            Self::GuidedStoryDraft => "guided_story_draft",
            Self::GuidedStoryPlanner => "guided_story_planner",
            Self::GuidedStoryReviewer => "guided_story_reviewer",
            Self::GuidedStoryExecutor => "guided_story_executor",
            Self::GuidedStoryValidator => "guided_story_validator",
            Self::QuestionBank => "question_bank",
            Self::QuestionBankGate => "question_bank_gate",
            Self::Textbook => "textbook",
        }
    }

    fn supports_json_latex_recovery(self) -> bool {
        matches!(
            self,
            Self::GuidedStoryDraft
                | Self::GuidedStoryPlanner
                | Self::GuidedStoryReviewer
                | Self::GuidedStoryExecutor
                | Self::GuidedStoryValidator
                | Self::QuestionBank
                | Self::QuestionBankGate
        )
    }
}

struct LlmResponse {
    content: String,
}

const QUESTION_FAMILY_FIELDS: &[&str] =
    &["flashcard_families", "quiz_families", "longform_families"];

#[derive(Clone, Debug, Deserialize, Serialize)]
struct GateReport {
    decisions: Vec<GateDecision>,
    #[serde(default)]
    summary: Value,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct GateDecision {
    family_id: String,
    decision: String,
    #[serde(default)]
    reason: String,
    #[serde(default)]
    practice_target: String,
}

#[derive(Clone, Debug)]
struct StepRef {
    sequence_id: String,
    path: PathBuf,
    question_bank_path: PathBuf,
}

impl MvpRunner {
    fn read_prompt(repo: &crate::paths::RepoPaths, file_name: &str) -> Result<String, MvpError> {
        Self::read_required_text(&repo.prompts_dir().join(file_name))
    }

    fn write_guided_story_steps(
        lesson: &LessonPaths,
        story_json: &Value,
    ) -> Result<Value, MvpError> {
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
                Self::normalize_step_json(lesson, &sequence_id, &mut step_json);

                let step_path = lesson
                    .guided_story_dir()
                    .join(&sequence_id)
                    .join("step.json");
                OutputValidator::validate_guided_story_step_exercise_kinds(&step_path, &step_json)?;
                Self::write_pretty_json(&step_path, &step_json)?;
                manifest_steps.push(json!({
                    "sequence_id": sequence_id,
                    "file": format!("{sequence_id}/step.json"),
                    "concept": concept
                }));
            }
        }

        if manifest_steps.is_empty() {
            let mut step_json = story_json.clone();
            Self::normalize_step_json(lesson, "step1", &mut step_json);
            OutputValidator::validate_guided_story_step_exercise_kinds(
                &lesson.guided_story_step_path(1),
                &step_json,
            )?;
            Self::write_pretty_json(&lesson.guided_story_step_path(1), &step_json)?;
            manifest_steps.push(json!({
                "sequence_id": "step1",
                "file": "step1/step.json",
                "concept": "Guided story step"
            }));
        }

        let mut manifest = json!({
            "lesson_id": lesson.lesson_id(),
            "mode": "guided_story",
            "steps": manifest_steps
        });
        if let Some(object) = manifest.as_object_mut() {
            if let Some(course_id) = lesson.course_id() {
                object.insert("course_id".to_owned(), json!(course_id));
            }
            if let Some(chapter_id) = lesson.chapter_id() {
                object.insert("chapter_id".to_owned(), json!(chapter_id));
            }
            if let Some(course_title) = lesson.course_title() {
                object.insert("course_title".to_owned(), json!(course_title));
            }
            if let Some(chapter_title) = lesson.chapter_title() {
                object.insert("chapter_title".to_owned(), json!(chapter_title));
            }
        }

        Ok(manifest)
    }

    fn normalize_step_json(lesson: &LessonPaths, sequence_id: &str, step_json: &mut Value) {
        let Some(object) = step_json.as_object_mut() else {
            return;
        };
        object
            .entry("lesson_id")
            .or_insert_with(|| json!(lesson.lesson_id()));
        if let Some(course_id) = lesson.course_id() {
            object
                .entry("course_id")
                .or_insert_with(|| json!(course_id));
        }
        if let Some(chapter_id) = lesson.chapter_id() {
            object
                .entry("chapter_id")
                .or_insert_with(|| json!(chapter_id));
        }
        object
            .entry("sequence_id")
            .or_insert_with(|| json!(sequence_id));
        object
            .entry("mode")
            .or_insert_with(|| json!("guided_story"));
        object.entry("term_catalog").or_insert_with(|| json!({}));
    }

    fn normalize_question_bank_json(
        lesson: &LessonPaths,
        sequence_id: &str,
        question_bank: &mut Value,
    ) {
        let Some(object) = question_bank.as_object_mut() else {
            return;
        };
        object
            .entry("lesson_id")
            .or_insert_with(|| json!(lesson.lesson_id()));
        if let Some(course_id) = lesson.course_id() {
            object
                .entry("course_id")
                .or_insert_with(|| json!(course_id));
        }
        if let Some(chapter_id) = lesson.chapter_id() {
            object
                .entry("chapter_id")
                .or_insert_with(|| json!(chapter_id));
        }
        object
            .entry("sequence_id")
            .or_insert_with(|| json!(sequence_id));
    }
}

struct QuestionBankGate;

impl QuestionBankGate {
    fn apply(question_bank: &mut Value, report: GateReport) -> Value {
        let decisions_by_id = report
            .decisions
            .iter()
            .cloned()
            .map(|decision| (decision.family_id.clone(), decision))
            .collect::<BTreeMap<_, _>>();
        let mut rejected_ids = BTreeSet::new();
        let mut kept_family_ids = Vec::new();
        let mut rejected_by_field = serde_json::Map::new();

        for field in QUESTION_FAMILY_FIELDS {
            let mut rejected_families = Vec::new();

            if let Some(families) = question_bank.get_mut(*field).and_then(Value::as_array_mut) {
                let mut kept_families = Vec::new();

                for family in std::mem::take(families) {
                    let family_id = family
                        .get("family_id")
                        .and_then(Value::as_str)
                        .unwrap_or("<missing_family_id>")
                        .to_owned();
                    let mut decision =
                        decisions_by_id
                            .get(&family_id)
                            .cloned()
                            .unwrap_or_else(|| GateDecision {
                                family_id: family_id.clone(),
                                decision: "uncertain".to_owned(),
                                reason: "gate did not return a decision for this family".to_owned(),
                                practice_target: String::new(),
                            });

                    if family_id == "<missing_family_id>" {
                        decision.decision = "fail".to_owned();
                        decision.reason = "family is missing family_id".to_owned();
                    } else if Self::family_is_marked_meta(&family) {
                        decision.decision = "fail".to_owned();
                        decision.reason =
                            "candidate marked this family as meta about course or material"
                                .to_owned();
                    }

                    if decision.decision.eq_ignore_ascii_case("pass") {
                        kept_family_ids.push(Value::String(family_id));
                        kept_families.push(family);
                    } else {
                        rejected_ids.insert(family_id);
                        rejected_families.push(Self::family_with_gate_decision(family, &decision));
                    }
                }

                *families = kept_families;
            }

            rejected_by_field.insert((*field).to_owned(), Value::Array(rejected_families));
        }

        Self::remove_rejected_coverage_refs(question_bank, &rejected_ids);

        json!({
            "gate": "question_bank_gate",
            "decisions": report.decisions,
            "summary": report.summary,
            "kept_family_ids": kept_family_ids,
            "rejected_family_ids": rejected_ids.into_iter().collect::<Vec<_>>(),
            "rejected_families": Value::Object(rejected_by_field)
        })
    }

    fn family_is_marked_meta(family: &Value) -> bool {
        family
            .get("is_meta_about_course_or_material")
            .and_then(Value::as_bool)
            .unwrap_or(false)
    }

    fn family_with_gate_decision(mut family: Value, decision: &GateDecision) -> Value {
        if let Some(object) = family.as_object_mut() {
            object.insert(
                "_gate_decision".to_owned(),
                json!({
                    "decision": decision.decision,
                    "reason": decision.reason,
                    "practice_target": decision.practice_target
                }),
            );
        }
        family
    }

    fn remove_rejected_coverage_refs(question_bank: &mut Value, rejected_ids: &BTreeSet<String>) {
        if rejected_ids.is_empty() {
            return;
        }

        let Some(items) = question_bank
            .get_mut("coverage_map")
            .and_then(Value::as_array_mut)
        else {
            return;
        };

        for item in items {
            if let Some(covered_by) = item.get_mut("covered_by").and_then(Value::as_array_mut) {
                covered_by.retain(|family_id| {
                    family_id
                        .as_str()
                        .map_or(true, |family_id| !rejected_ids.contains(family_id))
                });
            }
        }
    }

    fn count_question_families(question_bank: &Value) -> usize {
        QUESTION_FAMILY_FIELDS
            .iter()
            .filter_map(|field| question_bank.get(*field).and_then(Value::as_array))
            .map(Vec::len)
            .sum()
    }
}

impl MvpRunner {
    fn candidate_question_bank_path(question_bank_path: &Path) -> PathBuf {
        question_bank_path.with_file_name("candidate_question_bank.json")
    }

    fn rejected_question_bank_path(question_bank_path: &Path) -> PathBuf {
        question_bank_path.with_file_name("question_bank.rejected.json")
    }

    fn read_manifest_step_refs(lesson: &LessonPaths) -> Result<Vec<StepRef>, MvpError> {
        let manifest = Self::read_json(&lesson.guided_story_manifest_path(), Stage::GuidedStory)?;
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
            let path = Self::resolve_manifest_file(lesson, file);
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
            let content = Self::read_required_text(&step_ref.path)?;
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
        let content = Self::read_required_text(path)?;
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
                banks.push(Self::read_required_text(&question_bank_path)?);
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

    fn to_pretty_json_string(path: &Path, value: &Value) -> Result<String, MvpError> {
        let mut payload =
            serde_json::to_string_pretty(value).map_err(|source| MvpError::JsonWrite {
                path: path.to_path_buf(),
                source,
            })?;
        payload.push('\n');
        Ok(payload)
    }

    fn guided_story_validator_pass(report: &Value) -> bool {
        report.get("pass").and_then(Value::as_bool).unwrap_or(false)
            || [
                "question_chain",
                "exam_coverage",
                "contract_compliance",
                "exercise_quality",
                "definition_drift",
            ]
            .iter()
            .all(|field| {
                report
                    .get("scores")
                    .and_then(|scores| scores.get(*field))
                    .and_then(Value::as_i64)
                    .unwrap_or(0)
                    >= 4
            })
    }

    fn parse_llm_json(
        lesson: &LessonPaths,
        stage: Stage,
        audit_stage: &str,
        content: &str,
    ) -> Result<Value, MvpError> {
        match serde_json::from_str(content) {
            Ok(value) => Ok(value),
            Err(source) => {
                if stage.supports_json_latex_recovery() {
                    if let Some(recovered) = Self::recover_unescaped_latex_json(content) {
                        match serde_json::from_str(&recovered) {
                            Ok(value) => {
                                let recovered_path = lesson
                                    .audit_stage_dir(audit_stage)
                                    .join("content.recovered.txt");
                                fs::write(&recovered_path, &recovered).map_err(|write_source| {
                                    MvpError::Write {
                                        path: recovered_path.clone(),
                                        source: write_source,
                                    }
                                })?;
                                eprintln!(
                                    "{} recovered JSON by escaping legacy LaTeX backslashes: {}",
                                    stage.as_str(),
                                    lesson.relative_display(&recovered_path)
                                );
                                return Ok(value);
                            }
                            Err(_) => {}
                        }
                    }
                }

                Err(MvpError::JsonParse {
                    stage: stage.as_str(),
                    source,
                })
            }
        }
    }

    fn recover_unescaped_latex_json(content: &str) -> Option<String> {
        let chars = content.chars().collect::<Vec<_>>();
        let mut recovered = String::with_capacity(content.len() + 32);
        let mut in_string = false;
        let mut escaped = false;
        let mut changed = false;
        let mut index = 0usize;

        while index < chars.len() {
            let ch = chars[index];

            if !in_string {
                if ch == '"' {
                    in_string = true;
                }
                recovered.push(ch);
                index += 1;
                continue;
            }

            if escaped {
                recovered.push(ch);
                escaped = false;
                index += 1;
                continue;
            }

            match ch {
                '\\' => {
                    let next = chars.get(index + 1).copied();
                    if let Some(next) = next {
                        if Self::should_escape_legacy_latex_backslash(
                            next,
                            chars.get(index + 2).copied(),
                        ) {
                            recovered.push('\\');
                            recovered.push('\\');
                            recovered.push(next);
                            changed = true;
                            index += 2;
                            continue;
                        }
                    }
                    recovered.push(ch);
                    escaped = true;
                    index += 1;
                }
                '"' => {
                    in_string = false;
                    recovered.push(ch);
                    index += 1;
                }
                _ => {
                    recovered.push(ch);
                    index += 1;
                }
            }
        }

        changed.then_some(recovered)
    }

    fn should_escape_legacy_latex_backslash(next: char, after_next: Option<char>) -> bool {
        match next {
            '"' | '\\' | '/' => false,
            'u' => !after_next.is_some_and(|ch| ch.is_ascii_hexdigit()),
            'n' | 'r' | 't' | 'b' | 'f' => after_next.is_some_and(|ch| ch.is_ascii_alphabetic()),
            '(' | ')' | '[' | ']' => true,
            _ => true,
        }
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

        let mut payload =
            serde_json::to_vec_pretty(value).map_err(|source| MvpError::JsonWrite {
                path: path.to_path_buf(),
                source,
            })?;
        payload.push(b'\n');
        fs::write(path, payload).map_err(|source| MvpError::Write {
            path: path.to_path_buf(),
            source,
        })
    }
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
