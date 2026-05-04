use std::{
    env, fs,
    io::{Read, Write},
    net::TcpListener,
    path::{Path, PathBuf},
    process,
    process::Command,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex,
    },
    thread,
    time::{SystemTime, UNIX_EPOCH},
};

static NEXT_TEMP_REPO_ID: AtomicU64 = AtomicU64::new(0);

const ENV_KEYS: &[&str] = &[
    "COURSEGEN_LLM_API_KEY",
    "COURSEGEN_LLM_BASE_URL",
    "COURSEGEN_LLM_MODEL",
    "COURSEGEN_LLM_MAX_TOKENS",
    "COURSEGEN_LLM_PROVIDER",
    "COURSEGEN_LLM_TIMEOUT_SECONDS",
    "DEEPSEEK_API_KEY",
    "DEEPSEEK_BASE_URL",
    "DEEPSEEK_MAX_TOKENS",
    "DEEPSEEK_MODEL",
    "DEEPSEEK_TIMEOUT_SECONDS",
    "OPENAI_API_KEY",
    "OPENAI_BASE_URL",
    "OPENAI_MODEL",
    "MINERU_API_TOKEN",
    "MINERU_TOKEN_FILE",
    "MINERU_API_BASE_URL",
    "MINERU_MODEL_VERSION",
    "MINERU_LANGUAGE",
    "MINERU_ENABLE_FORMULA",
    "MINERU_ENABLE_TABLE",
    "MINERU_IS_OCR",
    "MINERU_FAIL_ON_EMPTY_TEXT",
    "MINERU_REQUEST_TIMEOUT_SECONDS",
    "MINERU_UPLOAD_TIMEOUT_SECONDS",
    "MINERU_RESULT_TIMEOUT_SECONDS",
    "MINERU_POLL_INTERVAL_SECONDS",
    "MINERU_DOWNLOAD_TIMEOUT_SECONDS",
];

struct TestSupport;

impl TestSupport {
    fn repo_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(2)
            .expect("crate directory should live under the repository root")
            .to_path_buf()
    }
}

struct TempRepo {
    root: PathBuf,
}

impl TempRepo {
    fn new() -> Self {
        let unique_id = format!(
            "{}-{}-{}",
            process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system clock should be after unix epoch")
                .as_nanos(),
            NEXT_TEMP_REPO_ID.fetch_add(1, Ordering::Relaxed)
        );
        let root = env::temp_dir().join(format!("coursegen-mvp-tests-{unique_id}"));

        fs::create_dir_all(root.join(".git")).expect("temp repo should create .git directory");
        TestSupport::write_file(&root.join(".ci/agent/AGENTS.md"), "test harness\n");
        TestSupport::write_file(&root.join("docs/progress.json"), "{}\n");
        TestSupport::write_file(&root.join("research/README.md"), "test harness\n");
        let repo = Self { root };
        repo.seed_course_index();
        repo
    }

    fn root(&self) -> &Path {
        &self.root
    }

    fn write(&self, relative: impl AsRef<Path>, contents: &str) {
        TestSupport::write_file(&self.root.join(relative), contents);
    }

    fn seed_prompts(&self) {
        self.write("research/prompts/guided_story_system.md", "guided system\n");
        self.write(
            "research/prompts/guided_story_user.md",
            "guided {{target_language}} {{plain_text}} {{related_important}}\n",
        );
        self.write(
            "research/prompts/question_bank_system.md",
            "question system\n",
        );
        self.write(
            "research/prompts/question_bank_user.md",
            "question {{target_language}} {{lesson_id}} {{guided_story_manifest}} {{guided_story_steps}}\n",
        );
        self.write(
            "research/prompts/question_bank_gate_system.md",
            "gate system\n",
        );
        self.write(
            "research/prompts/question_bank_gate_user.md",
            "gate {{target_language}} {{lesson_id}} {{current_step_id}} {{candidate_question_bank}}\n",
        );
        self.write("research/prompts/textbook_system.md", "textbook system\n");
        self.write(
            "research/prompts/textbook_user.md",
            "textbook {{target_language}} {{lesson_id}} {{question_bank}} {{plain_text}}\n",
        );
    }

    fn seed_relevance_inputs(&self) {
        self.write(
            "research/pipeline/1-plain/L2/plain.txt",
            "A lesson about execution, questions, and review.\n",
        );
        self.write(
            "research/pipeline/2-related_important/course_desc.md",
            "Course description with assessment outcomes.\n",
        );
        self.write(
            "research/pipeline/3-guided_story/L2/manifest.json",
            r#"{"lesson_id":"L2","course_id":"comp7415a","chapter_id":"lecture-2","mode":"guided_story","steps":[{"sequence_id":"step1","file":"research/pipeline/3-guided_story/L2/step1/step.json","concept":"Execution"}]}"#,
        );
        self.write(
            "research/pipeline/3-guided_story/L2/step1/step.json",
            r#"{"lesson_id":"L2","course_id":"comp7415a","chapter_id":"lecture-2","sequence_id":"step1","mode":"guided_story","screens":[{"id":"s1","body":"Execution matters."}],"term_catalog":{}}"#,
        );
        self.write(
            "research/pipeline/3-guided_story/L2/step1/question_bank.json",
            r#"{"lesson_id":"L2","course_id":"comp7415a","chapter_id":"lecture-2","sequence_id":"step1","flashcard_families":[{"family_id":"family_a","linked_steps":["step1"],"variants":[{"question_id":"question_a","linked_steps":["step1"],"stem":"A?","answer":0}]}],"longform_families":[]}"#,
        );
        self.write(
            "research/pipeline/5-textbook/L2.mdx",
            "# Execution\n\nExecution textbook content.\n\n## Review\n\n<QuestionFamily familyId=\"family_a\" />\n",
        );
        self.write(
            "research/pipeline/0-raw/exam/sample.txt",
            "exam sample text\n",
        );
        self.write("research/pipeline/0-raw/exam/sample.pdf", "%PDF test\n");
        self.write(
            "research/pipeline/1-plain/exam/sample/plain.txt",
            "cached exam PDF text about assessed execution tradeoffs\n",
        );
    }

    fn seed_course_index(&self) {
        self.write(
            "research/pipeline/course-index.json",
            r#"{
  "version": 1,
  "courses": [
    {
      "courseId": "comp7415a",
      "title": "COMP7415A Algorithmic Trading",
      "relatedImportantPath": "research/pipeline/2-related_important/course_desc.md",
      "examRawDir": "research/pipeline/0-raw/exam",
      "examPlainRoot": "research/pipeline/1-plain/exam",
      "chapters": [
        {
          "chapterId": "lecture-2",
          "title": "Data Scraping and Database Management",
          "lessonId": "L2",
          "guidedStoryDir": "research/pipeline/3-guided_story/L2",
          "textbookPath": "research/pipeline/5-textbook/L2.mdx",
          "rawPdfPath": "research/pipeline/0-raw/L2.pdf",
          "plainOutputDir": "research/pipeline/1-plain/L2",
          "metaDir": "research/pipeline/meta/L2"
        }
      ]
    }
  ]
}
"#,
        );
    }
}

impl Drop for TempRepo {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

impl TestSupport {
    fn coursegen_command() -> Command {
        let mut command = Command::new(env!("CARGO_BIN_EXE_coursegen"));
        for key in ENV_KEYS {
            command.env_remove(key);
        }
        command
    }

    fn write_file(path: &Path, contents: &str) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("parent directories should be created");
        }
        fs::write(path, contents).expect("file should be written");
    }

    fn read_text(path: impl AsRef<Path>) -> String {
        fs::read_to_string(path).expect("file should be readable")
    }

    fn gate_pass_report(family_ids: &[&str]) -> String {
        let decisions = family_ids
            .iter()
            .map(|family_id| {
                serde_json::json!({
                    "family_id": family_id,
                    "decision": "pass",
                    "reason": "Checks practice content.",
                    "practice_target": "Practice content."
                })
            })
            .collect::<Vec<_>>();

        serde_json::json!({
            "decisions": decisions,
            "summary": {
                "passed": family_ids.len(),
                "failed": 0,
                "uncertain": 0
            }
        })
        .to_string()
    }
}

struct StubLlmServer {
    base_url: String,
    handle: thread::JoinHandle<()>,
    requests: Arc<Mutex<Vec<String>>>,
}

impl StubLlmServer {
    fn start(contents: Vec<String>) -> Self {
        let listener =
            TcpListener::bind("127.0.0.1:0").expect("stub server should bind a local port");
        let addr = listener
            .local_addr()
            .expect("stub server should report its bound address");
        let base_url = format!("http://{addr}/v1");
        let requests = Arc::new(Mutex::new(Vec::new()));
        let request_sink = Arc::clone(&requests);

        let handle = thread::spawn(move || {
            for content in contents {
                let (mut stream, _) = listener
                    .accept()
                    .expect("stub server should accept an LLM request");
                let request = Self::read_http_request(&mut stream);
                request_sink
                    .lock()
                    .expect("request sink should not be poisoned")
                    .push(request);
                let body = serde_json::json!({
                    "choices": [{
                        "message": {
                            "role": "assistant",
                            "content": content
                        }
                    }]
                })
                .to_string();
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    body.len(),
                    body
                );
                stream
                    .write_all(response.as_bytes())
                    .expect("stub server should write response");
                stream.flush().expect("stub server should flush response");
            }
        });

        Self {
            base_url,
            handle,
            requests,
        }
    }

    fn base_url(&self) -> &str {
        &self.base_url
    }

    fn finish(self) -> Vec<String> {
        self.handle
            .join()
            .expect("stub server should finish cleanly");
        Arc::try_unwrap(self.requests)
            .expect("stub server request sink should have one owner")
            .into_inner()
            .expect("request sink should not be poisoned")
    }

    fn read_http_request(stream: &mut impl Read) -> String {
        let mut buffer = Vec::new();
        let mut chunk = [0_u8; 4096];
        let header_end = loop {
            let bytes_read = stream
                .read(&mut chunk)
                .expect("stub server should read request bytes");
            if bytes_read == 0 {
                break buffer.len();
            }

            buffer.extend_from_slice(&chunk[..bytes_read]);
            if let Some(header_end) = Self::find_header_end(&buffer) {
                break header_end;
            }
        };

        let headers = String::from_utf8_lossy(&buffer[..header_end]);
        let content_length = headers
            .lines()
            .find_map(|line| {
                let (key, value) = line.split_once(':')?;
                if key.eq_ignore_ascii_case("content-length") {
                    return value.trim().parse::<usize>().ok();
                }
                None
            })
            .unwrap_or(0);

        while buffer.len().saturating_sub(header_end) < content_length {
            let bytes_read = stream
                .read(&mut chunk)
                .expect("stub server should read request body bytes");
            if bytes_read == 0 {
                break;
            }
            buffer.extend_from_slice(&chunk[..bytes_read]);
        }

        String::from_utf8_lossy(&buffer).into_owned()
    }

    fn find_header_end(buffer: &[u8]) -> Option<usize> {
        buffer
            .windows(b"\r\n\r\n".len())
            .position(|window| window == b"\r\n\r\n")
            .map(|position| position + b"\r\n\r\n".len())
    }
}

#[test]
fn validate_fails_clearly_outside_the_repository_root() {
    let cwd = TestSupport::repo_root().join("research");
    let output = TestSupport::coursegen_command()
        .current_dir(&cwd)
        .args(["validate", "L2"])
        .output()
        .expect("coursegen should run");

    assert!(
        !output.status.success(),
        "stdout: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    assert!(
        output.stdout.is_empty(),
        "stdout: {}",
        String::from_utf8_lossy(&output.stdout)
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("repository root required: run this command from the repository root"),
        "stderr: {stderr}"
    );
    assert!(
        stderr.contains(&cwd.display().to_string()),
        "stderr: {stderr}"
    );
}

#[test]
fn run_reports_missing_raw_pdf_when_plain_text_is_missing() {
    let repo = TempRepo::new();

    let output = TestSupport::coursegen_command()
        .current_dir(repo.root())
        .args(["run", "L2", "--target-language", "zh-CN"])
        .output()
        .expect("coursegen should run");

    assert!(
        !output.status.success(),
        "stdout: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("missing raw PDF for lesson 'L2': research/pipeline/0-raw/L2.pdf"),
        "stderr: {stderr}"
    );
    assert!(
        stderr.contains("configured MinerU setup"),
        "stderr: {stderr}"
    );
}

#[test]
fn run_reports_missing_llm_api_key_before_prompts_or_network() {
    let repo = TempRepo::new();
    repo.write(
        "research/pipeline/1-plain/L2/plain.txt",
        "plain text is already available\n",
    );

    let output = TestSupport::coursegen_command()
        .current_dir(repo.root())
        .env("COURSEGEN_LLM_MODEL", "test-model")
        .args(["run", "L2"])
        .output()
        .expect("coursegen should run");

    assert!(
        !output.status.success(),
        "stdout: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("missing LLM API key: set COURSEGEN_LLM_API_KEY or OPENAI_API_KEY"),
        "stderr: {stderr}"
    );
}

#[test]
fn run_reports_missing_llm_model_before_prompts_or_network() {
    let repo = TempRepo::new();
    repo.write(
        "research/pipeline/1-plain/L2/plain.txt",
        "plain text is already available\n",
    );

    let output = TestSupport::coursegen_command()
        .current_dir(repo.root())
        .env("COURSEGEN_LLM_API_KEY", "test-token")
        .args(["run", "L2"])
        .output()
        .expect("coursegen should run");

    assert!(
        !output.status.success(),
        "stdout: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("missing LLM model: set COURSEGEN_LLM_MODEL or OPENAI_MODEL"),
        "stderr: {stderr}"
    );
}

#[test]
fn score_relevance_reports_missing_llm_api_key_before_network() {
    let repo = TempRepo::new();
    repo.seed_relevance_inputs();

    let output = TestSupport::coursegen_command()
        .current_dir(repo.root())
        .env("COURSEGEN_LLM_MODEL", "test-model")
        .args(["score-relevance", "L2"])
        .output()
        .expect("coursegen should run");

    assert!(
        !output.status.success(),
        "stdout: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("missing LLM API key: set COURSEGEN_LLM_API_KEY or OPENAI_API_KEY"),
        "stderr: {stderr}"
    );
}

#[test]
fn score_relevance_reports_missing_llm_model_before_network() {
    let repo = TempRepo::new();
    repo.seed_relevance_inputs();

    let output = TestSupport::coursegen_command()
        .current_dir(repo.root())
        .env("COURSEGEN_LLM_API_KEY", "test-token")
        .args(["score-relevance", "L2"])
        .output()
        .expect("coursegen should run");

    assert!(
        !output.status.success(),
        "stdout: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("missing LLM model: set COURSEGEN_LLM_MODEL or OPENAI_MODEL"),
        "stderr: {stderr}"
    );
}

#[test]
fn score_relevance_writes_report_and_audit_outputs_with_local_llm_stub() {
    let repo = TempRepo::new();
    repo.seed_relevance_inputs();
    let report = serde_json::json!({
        "lesson_id": "L2",
        "target_language": "zh-CN",
        "exam_signal": {
            "summary": "sample exam signal",
            "notes": ["exam_pdf_text_cached"]
        },
        "step_scores": [{
            "sequence_id": "step1",
            "importance": 0.9,
            "course_relevance": 0.9,
            "exam_relevance": 0.5,
            "recommended_treatment": "keep",
            "reason": "Core lesson step."
        }],
        "question_family_scores": [{
            "sequence_id": "step1",
            "family_id": "family_a",
            "importance": 0.8,
            "course_relevance": 0.9,
            "exam_relevance": 0.4,
            "recommended_treatment": "keep",
            "reason": "Checks the step objective."
        }],
        "textbook_section_scores": [{
            "section_id": "section_1",
            "title": "Execution",
            "importance": 0.8,
            "course_relevance": 0.9,
            "exam_relevance": 0.4,
            "recommended_treatment": "keep",
            "reason": "Relevant section."
        }],
        "coverage_scores": [{
            "coverage_id": "coverage_1",
            "title": "Execution",
            "importance": 0.8,
            "course_relevance": 0.9,
            "exam_relevance": 0.4,
            "recommended_treatment": "keep",
            "reason": "Likely assessable."
        }]
    })
    .to_string();
    let server = StubLlmServer::start(vec![report]);

    let output = TestSupport::coursegen_command()
        .current_dir(repo.root())
        .env("COURSEGEN_LLM_API_KEY", "test-token")
        .env("COURSEGEN_LLM_MODEL", "test-model")
        .env("COURSEGEN_LLM_BASE_URL", server.base_url())
        .args(["score-relevance", "L2", "--target-language", "zh-CN"])
        .output()
        .expect("coursegen should run");

    server.finish();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        String::from_utf8_lossy(&output.stdout)
            .contains("relevance scoring complete: target=comp7415a/lecture-2 (lesson L2)"),
        "stdout: {}",
        String::from_utf8_lossy(&output.stdout)
    );

    let audit_root = repo.root().join("research/pipeline/meta/L2/relevance");
    let report_path = audit_root.join("report.json");
    assert!(report_path.is_file(), "relevance report should be written");
    for file_name in [
        "system.md",
        "user.md",
        "request.json",
        "raw_response.txt",
        "raw_response.json",
        "content.txt",
    ] {
        assert!(
            audit_root.join(file_name).is_file(),
            "{file_name} should be audited"
        );
    }

    let report: serde_json::Value =
        serde_json::from_str(&TestSupport::read_text(report_path)).expect("report should parse");
    assert!(
        report["step_scores"]
            .as_array()
            .is_some_and(|scores| !scores.is_empty()),
        "step scores should be present"
    );
    assert!(
        report["question_family_scores"]
            .as_array()
            .is_some_and(|scores| !scores.is_empty()),
        "question family scores should be present"
    );
    assert!(
        report["textbook_section_scores"]
            .as_array()
            .is_some_and(|scores| !scores.is_empty()),
        "textbook section scores should be present"
    );
    assert!(
        report["coverage_scores"]
            .as_array()
            .is_some_and(|scores| !scores.is_empty()),
        "coverage scores should be present"
    );

    let user_prompt = TestSupport::read_text(audit_root.join("user.md"));
    assert!(
        user_prompt.contains("cached exam PDF text about assessed execution tradeoffs"),
        "cached PDF exam text should be included in scorer input"
    );
    assert!(
        user_prompt.contains("research/pipeline/1-plain/exam/sample/plain.txt"),
        "cached PDF exam text path should be included in scorer input"
    );
    assert!(
        user_prompt.contains("sample.txt"),
        "exam text files should be included in scorer input"
    );
}

#[test]
fn score_relevance_uses_deepseek_provider_defaults_and_api_keys_file() {
    let repo = TempRepo::new();
    repo.seed_relevance_inputs();
    repo.write("research/API_KEYS.md", "DEEPSEEK: test-token\n");
    let report = serde_json::json!({
        "lesson_id": "L2",
        "target_language": "zh-CN",
        "exam_signal": {"summary": "sample", "notes": []},
        "step_scores": [],
        "question_family_scores": [],
        "textbook_section_scores": [],
        "coverage_scores": []
    })
    .to_string();
    let server = StubLlmServer::start(vec![report]);

    let output = TestSupport::coursegen_command()
        .current_dir(repo.root())
        .env("COURSEGEN_LLM_PROVIDER", "deepseek")
        .env("DEEPSEEK_BASE_URL", server.base_url())
        .args(["score-relevance", "L2", "--target-language", "zh-CN"])
        .output()
        .expect("coursegen should run");

    let requests = server.finish();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        requests.len(),
        1,
        "DeepSeek provider should send one request"
    );
    let request = &requests[0];
    assert!(
        request.starts_with("POST /v1/chat/completions HTTP/1.1"),
        "request should use the provider chat completions path: {request}"
    );
    assert!(
        request
            .to_ascii_lowercase()
            .contains("authorization: bearer test-token"),
        "request should use the DeepSeek key from research/API_KEYS.md"
    );
    let body = request
        .split_once("\r\n\r\n")
        .map(|(_, body)| body)
        .expect("request should include a JSON body");
    let request_json: serde_json::Value =
        serde_json::from_str(body).expect("request body should parse as JSON");
    assert_eq!(request_json["model"], "deepseek-chat");
    assert_eq!(request_json["max_tokens"].as_u64(), Some(8192));
}

#[test]
fn run_writes_prompt_audit_outputs_and_validates_with_local_llm_stub() {
    let repo = TempRepo::new();
    repo.seed_prompts();
    repo.write(
        "research/pipeline/1-plain/L2/plain.txt",
        "A lesson about execution, questions, and review.\n",
    );
    repo.write(
        "research/pipeline/2-related_important/course_desc.md",
        "Related course note.\n",
    );

    let guided_story = r#"{"lesson_id":"L2","sequence_id":"step1","mode":"guided_story","screens":[],"term_catalog":{}}"#.to_owned();
    let question_bank = r#"{"lesson_id":"L2","sequence_id":"step1","flashcard_families":[{"family_id":"family_a","linked_steps":["step1"],"variants":[{"question_id":"question_a","linked_steps":["step1"],"stem":"A?","answer":0}]}],"longform_families":[]}"#.to_owned();
    let gate_report = TestSupport::gate_pass_report(&["family_a"]);
    let textbook =
        r#"---\nlessonId: L2\n---\n# L2\n<QuestionFamily familyId="family_a" />\n<QuestionRef id="question_a" />\n"#.to_owned();
    let server = StubLlmServer::start(vec![guided_story, question_bank, gate_report, textbook]);

    let output = TestSupport::coursegen_command()
        .current_dir(repo.root())
        .env("COURSEGEN_LLM_API_KEY", "test-token")
        .env("COURSEGEN_LLM_MODEL", "test-model")
        .env("COURSEGEN_LLM_BASE_URL", server.base_url())
        .args(["run", "L2", "--target-language", "zh-CN"])
        .output()
        .expect("coursegen should run");

    server.finish();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        String::from_utf8_lossy(&output.stdout)
            .contains("mvp complete: target=comp7415a/lecture-2 (lesson L2)"),
        "stdout: {}",
        String::from_utf8_lossy(&output.stdout)
    );

    let manifest_path = repo
        .root()
        .join("research/pipeline/3-guided_story/L2/manifest.json");
    let step_path = repo
        .root()
        .join("research/pipeline/3-guided_story/L2/step1/step.json");
    let question_path = repo
        .root()
        .join("research/pipeline/3-guided_story/L2/step1/question_bank.json");
    let textbook_path = repo.root().join("research/pipeline/5-textbook/L2.mdx");
    assert!(manifest_path.is_file(), "manifest should be written");
    assert!(step_path.is_file(), "guided step should be written");
    assert!(question_path.is_file(), "question bank should be written");
    assert!(textbook_path.is_file(), "textbook should be written");

    let audit_root = repo.root().join("research/pipeline/meta/L2/mvp");
    for stage in [
        "guided_story",
        "question_bank",
        "question_bank_gate",
        "textbook",
    ] {
        assert!(
            audit_root.join(stage).join("system.md").is_file(),
            "{stage} system prompt should be audited"
        );
        assert!(
            audit_root.join(stage).join("user.md").is_file(),
            "{stage} user prompt should be audited"
        );
        assert!(
            audit_root.join(stage).join("raw_response.json").is_file(),
            "{stage} raw response should be audited"
        );
    }

    assert!(
        TestSupport::read_text(audit_root.join("guided_story/user.md")).contains("zh-CN"),
        "rendered prompt should include target language"
    );
    assert!(
        TestSupport::read_text(textbook_path).contains(r#"<QuestionRef id=\"question_a\" />"#)
            || TestSupport::read_text(repo.root().join("research/pipeline/5-textbook/L2.mdx"))
                .contains(r#"<QuestionRef id="question_a" />"#),
        "textbook should contain the stubbed question reference"
    );

    let validate = TestSupport::coursegen_command()
        .current_dir(repo.root())
        .args(["validate", "L2"])
        .output()
        .expect("coursegen should validate");
    assert!(
        validate.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&validate.stderr)
    );
}

#[test]
fn validate_catalog_writes_ready_status_for_complete_course() {
    let repo = TempRepo::new();
    repo.seed_relevance_inputs();

    let output = TestSupport::coursegen_command()
        .current_dir(repo.root())
        .args(["validate-catalog", "--write"])
        .output()
        .expect("coursegen should validate catalog");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        String::from_utf8_lossy(&output.stdout).contains("ready=1 blocked=0"),
        "stdout: {}",
        String::from_utf8_lossy(&output.stdout)
    );

    let index: serde_json::Value = serde_json::from_str(&TestSupport::read_text(
        repo.root().join("research/pipeline/course-index.json"),
    ))
    .expect("course index should parse");
    let course = &index["courses"][0];
    assert_eq!(course["status"], "ready");
    assert!(
        course.get("validationErrors").is_none(),
        "ready courses should not carry stale validation errors"
    );
}

#[test]
fn validate_catalog_writes_blocked_status_for_missing_assets() {
    let repo = TempRepo::new();

    let output = TestSupport::coursegen_command()
        .current_dir(repo.root())
        .args(["validate-catalog", "--write"])
        .output()
        .expect("coursegen should validate catalog");

    assert!(
        !output.status.success(),
        "incomplete catalog validation should fail"
    );
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("ready=0 blocked=1"),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let index: serde_json::Value = serde_json::from_str(&TestSupport::read_text(
        repo.root().join("research/pipeline/course-index.json"),
    ))
    .expect("course index should parse");
    let course = &index["courses"][0];
    let errors = course["validationErrors"]
        .as_array()
        .expect("blocked course should carry validation errors");
    assert_eq!(course["status"], "blocked");
    assert!(
        errors
            .iter()
            .any(|error| error["code"] == "MISSING_MANIFEST"),
        "errors should include missing manifest: {errors:?}"
    );
}

#[test]
fn run_filters_question_bank_with_gate_and_writes_rejected_artifact() {
    let repo = TempRepo::new();
    repo.seed_prompts();
    repo.write(
        "research/pipeline/1-plain/L2/plain.txt",
        "A lesson about execution quality and course orientation.\n",
    );
    repo.write(
        "research/pipeline/2-related_important/course_desc.md",
        "Related course note.\n",
    );

    let guided_story = r#"{"lesson_id":"L2","sequence_id":"step1","mode":"guided_story","screens":[],"term_catalog":{}}"#.to_owned();
    let question_bank = r#"{
      "lesson_id": "L2",
      "sequence_id": "step1",
      "coverage_map": [
        {"coverage_tag":"execution_quality","covered_by":["family_keep"]},
        {"coverage_tag":"course_orientation","covered_by":["family_meta"]}
      ],
      "flashcard_families": [
        {
          "family_id":"family_keep",
          "linked_steps":["step1"],
          "practice_target":"Students recall the execution concept.",
          "is_meta_about_course_or_material":false,
          "variants":[{"question_id":"question_keep","linked_steps":["step1"],"front":"A?","back":"A"}]
        }
      ],
      "quiz_families": [
        {
          "family_id":"family_meta",
          "linked_steps":["step1"],
          "practice_target":"Students recall the course order.",
          "is_meta_about_course_or_material":false,
          "variants":[{"question_id":"question_meta","linked_steps":["step1"],"stem":"Which topic comes later?","answer":0}]
        }
      ],
      "longform_families": []
    }"#.to_owned();
    let gate_report = serde_json::json!({
        "decisions": [
            {
                "family_id": "family_keep",
                "decision": "pass",
                "reason": "Trains practice content.",
                "practice_target": "Execution concept."
            },
            {
                "family_id": "family_meta",
                "decision": "fail",
                "reason": "Mainly asks about course organization.",
                "practice_target": "Course order."
            }
        ],
        "summary": {"passed": 1, "failed": 1, "uncertain": 0}
    })
    .to_string();
    let textbook =
        r#"---\nlessonId: L2\n---\n# L2\n<QuestionFamily familyId="family_keep" />\n<QuestionRef id="question_keep" />\n"#.to_owned();
    let server = StubLlmServer::start(vec![guided_story, question_bank, gate_report, textbook]);

    let output = TestSupport::coursegen_command()
        .current_dir(repo.root())
        .env("COURSEGEN_LLM_API_KEY", "test-token")
        .env("COURSEGEN_LLM_MODEL", "test-model")
        .env("COURSEGEN_LLM_BASE_URL", server.base_url())
        .args(["run", "L2", "--target-language", "zh-CN"])
        .output()
        .expect("coursegen should run");

    server.finish();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let step_dir = repo
        .root()
        .join("research/pipeline/3-guided_story/L2/step1");
    let main_bank: serde_json::Value =
        serde_json::from_str(&TestSupport::read_text(step_dir.join("question_bank.json")))
            .expect("main question bank should parse");
    let candidate: serde_json::Value = serde_json::from_str(&TestSupport::read_text(
        step_dir.join("candidate_question_bank.json"),
    ))
    .expect("candidate question bank should parse");
    let rejected: serde_json::Value = serde_json::from_str(&TestSupport::read_text(
        step_dir.join("question_bank.rejected.json"),
    ))
    .expect("rejected question bank should parse");

    assert_eq!(
        main_bank["flashcard_families"][0]["family_id"],
        "family_keep"
    );
    assert!(
        main_bank["quiz_families"]
            .as_array()
            .is_some_and(Vec::is_empty),
        "rejected quiz family should not remain in the main bank"
    );
    assert_eq!(
        main_bank["coverage_map"][1]["covered_by"]
            .as_array()
            .map(Vec::len),
        Some(0),
        "coverage refs should drop rejected family ids"
    );
    assert_eq!(
        candidate["quiz_families"][0]["family_id"], "family_meta",
        "candidate artifact should preserve generated families"
    );
    assert_eq!(
        rejected["rejected_families"]["quiz_families"][0]["family_id"], "family_meta",
        "rejected artifact should preserve removed families"
    );
}

#[test]
fn run_writes_question_banks_for_each_generated_step() {
    let repo = TempRepo::new();
    repo.seed_prompts();
    repo.write(
        "research/pipeline/1-plain/L2/plain.txt",
        "A lesson with two natural learning chunks.\n",
    );

    let guided_story = r#"{
      "lesson_id": "L2",
      "mode": "guided_story",
      "steps": [
        {
          "sequence_id": "step1",
          "concept": "First chunk",
          "step": {"lesson_id":"L2","sequence_id":"step1","mode":"guided_story","screens":[{"id":"s1","lines":["First"]}],"term_catalog":{}}
        },
        {
          "sequence_id": "step2",
          "concept": "Second chunk",
          "step": {"lesson_id":"L2","sequence_id":"step2","mode":"guided_story","screens":[{"id":"s1","lines":["Second"]}],"term_catalog":{}}
        }
      ]
    }"#.to_owned();
    let question_bank_1 = r#"{"lesson_id":"L2","sequence_id":"step1","flashcard_families":[{"family_id":"family_step1","linked_steps":["step1"],"variants":[{"question_id":"question_step1","linked_steps":["step1"],"front":"A?","back":"A"}]}],"quiz_families":[],"longform_families":[]}"#.to_owned();
    let question_bank_2 = r#"{"lesson_id":"L2","sequence_id":"step2","flashcard_families":[{"family_id":"family_step2","linked_steps":["step2"],"variants":[{"question_id":"question_step2","linked_steps":["step2"],"front":"B?","back":"B"}]}],"quiz_families":[],"longform_families":[]}"#.to_owned();
    let gate_report_1 = TestSupport::gate_pass_report(&["family_step1"]);
    let gate_report_2 = TestSupport::gate_pass_report(&["family_step2"]);
    let textbook = r#"---
lessonId: L2
---
# L2
<QuestionFamily familyId="family_step1" />
<QuestionRef id="question_step1" />
<QuestionFamily familyId="family_step2" />
<QuestionRef id="question_step2" />
"#
    .to_owned();
    let server = StubLlmServer::start(vec![
        guided_story,
        question_bank_1,
        gate_report_1,
        question_bank_2,
        gate_report_2,
        textbook,
    ]);

    let output = TestSupport::coursegen_command()
        .current_dir(repo.root())
        .env("COURSEGEN_LLM_API_KEY", "test-token")
        .env("COURSEGEN_LLM_MODEL", "test-model")
        .env("COURSEGEN_LLM_BASE_URL", server.base_url())
        .args(["run", "L2", "--target-language", "zh-CN"])
        .output()
        .expect("coursegen should run");

    server.finish();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let manifest: serde_json::Value = serde_json::from_str(&TestSupport::read_text(
        repo.root()
            .join("research/pipeline/3-guided_story/L2/manifest.json"),
    ))
    .expect("manifest should parse");
    assert_eq!(manifest["steps"].as_array().map(Vec::len), Some(2));

    for step in ["step1", "step2"] {
        assert!(
            repo.root()
                .join(format!(
                    "research/pipeline/3-guided_story/L2/{step}/step.json"
                ))
                .is_file(),
            "{step} step should be written"
        );
        assert!(
            repo.root()
                .join(format!(
                    "research/pipeline/3-guided_story/L2/{step}/question_bank.json"
                ))
                .is_file(),
            "{step} question bank should be written"
        );
        assert!(
            repo.root()
                .join(format!(
                    "research/pipeline/meta/L2/mvp/question_bank/{step}/user.md"
                ))
                .is_file(),
            "{step} question bank prompt should be audited"
        );
        assert!(
            repo.root()
                .join(format!(
                    "research/pipeline/meta/L2/mvp/question_bank_gate/{step}/user.md"
                ))
                .is_file(),
            "{step} question bank gate prompt should be audited"
        );
    }
}

#[test]
fn run_accepts_course_and_chapter_selectors_from_course_index() {
    let repo = TempRepo::new();
    repo.seed_prompts();
    repo.seed_course_index();
    repo.write(
        "research/pipeline/1-plain/L2/plain.txt",
        "A lesson about execution, questions, and review.\n",
    );
    repo.write(
        "research/pipeline/2-related_important/course_desc.md",
        "Related course note.\n",
    );

    let guided_story = r#"{"lesson_id":"L2","sequence_id":"step1","mode":"guided_story","screens":[],"term_catalog":{}}"#.to_owned();
    let question_bank = r#"{"lesson_id":"L2","sequence_id":"step1","flashcard_families":[{"family_id":"family_a","linked_steps":["step1"],"variants":[{"question_id":"question_a","linked_steps":["step1"],"stem":"A?","answer":0}]}],"longform_families":[]}"#.to_owned();
    let gate_report = TestSupport::gate_pass_report(&["family_a"]);
    let textbook =
        r#"---\nlessonId: L2\n---\n# L2\n<QuestionFamily familyId="family_a" />\n<QuestionRef id="question_a" />\n"#.to_owned();
    let server = StubLlmServer::start(vec![guided_story, question_bank, gate_report, textbook]);

    let output = TestSupport::coursegen_command()
        .current_dir(repo.root())
        .env("COURSEGEN_LLM_API_KEY", "test-token")
        .env("COURSEGEN_LLM_MODEL", "test-model")
        .env("COURSEGEN_LLM_BASE_URL", server.base_url())
        .args([
            "run",
            "--course",
            "comp7415a",
            "--chapter",
            "lecture-2",
            "--target-language",
            "zh-CN",
        ])
        .output()
        .expect("coursegen should run");

    server.finish();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        String::from_utf8_lossy(&output.stdout)
            .contains("mvp complete: target=comp7415a/lecture-2 (lesson L2)"),
        "stdout: {}",
        String::from_utf8_lossy(&output.stdout)
    );

    let manifest: serde_json::Value = serde_json::from_str(&TestSupport::read_text(
        repo.root()
            .join("research/pipeline/3-guided_story/L2/manifest.json"),
    ))
    .expect("manifest should parse");
    assert_eq!(manifest["course_id"], "comp7415a");
    assert_eq!(manifest["chapter_id"], "lecture-2");

    let step: serde_json::Value = serde_json::from_str(&TestSupport::read_text(
        repo.root()
            .join("research/pipeline/3-guided_story/L2/step1/step.json"),
    ))
    .expect("step should parse");
    assert_eq!(step["course_id"], "comp7415a");
    assert_eq!(step["chapter_id"], "lecture-2");

    let question_bank: serde_json::Value = serde_json::from_str(&TestSupport::read_text(
        repo.root()
            .join("research/pipeline/3-guided_story/L2/step1/question_bank.json"),
    ))
    .expect("question bank should parse");
    assert_eq!(question_bank["course_id"], "comp7415a");
    assert_eq!(question_bank["chapter_id"], "lecture-2");
}

#[test]
fn run_resumes_from_missing_step_question_bank_without_regenerating_prior_steps() {
    let repo = TempRepo::new();
    repo.seed_prompts();
    repo.write(
        "research/pipeline/1-plain/L2/plain.txt",
        "A lesson with resumable generated chunks.\n",
    );
    repo.write(
        "research/pipeline/3-guided_story/L2/manifest.json",
        r#"{"lesson_id":"L2","course_id":"comp7415a","chapter_id":"lecture-2","mode":"guided_story","steps":[{"sequence_id":"step1","file":"research/pipeline/3-guided_story/L2/step1/step.json","concept":"First"},{"sequence_id":"step2","file":"research/pipeline/3-guided_story/L2/step2/step.json","concept":"Second"}]}"#,
    );
    repo.write(
        "research/pipeline/3-guided_story/L2/step1/step.json",
        r#"{"lesson_id":"L2","course_id":"comp7415a","chapter_id":"lecture-2","sequence_id":"step1","mode":"guided_story","screens":[{"id":"s1","lines":["First"]}],"term_catalog":{}}"#,
    );
    repo.write(
        "research/pipeline/3-guided_story/L2/step2/step.json",
        r#"{"lesson_id":"L2","course_id":"comp7415a","chapter_id":"lecture-2","sequence_id":"step2","mode":"guided_story","screens":[{"id":"s1","lines":["Second"]}],"term_catalog":{}}"#,
    );
    repo.write(
        "research/pipeline/3-guided_story/L2/step1/question_bank.json",
        r#"{"lesson_id":"L2","course_id":"comp7415a","chapter_id":"lecture-2","sequence_id":"step1","flashcard_families":[{"family_id":"family_step1","linked_steps":["step1"],"variants":[{"question_id":"question_step1","linked_steps":["step1"],"front":"A?","back":"A"}]}],"quiz_families":[],"longform_families":[]}"#,
    );

    let gate_report_1 = TestSupport::gate_pass_report(&["family_step1"]);
    let question_bank_2 = r#"{"lesson_id":"L2","course_id":"comp7415a","chapter_id":"lecture-2","sequence_id":"step2","flashcard_families":[{"family_id":"family_step2","linked_steps":["step2"],"variants":[{"question_id":"question_step2","linked_steps":["step2"],"front":"B?","back":"B"}]}],"quiz_families":[],"longform_families":[]}"#.to_owned();
    let gate_report_2 = TestSupport::gate_pass_report(&["family_step2"]);
    let textbook = r#"---
lessonId: L2
---
# L2
<QuestionFamily familyId="family_step1" />
<QuestionRef id="question_step1" />
<QuestionFamily familyId="family_step2" />
<QuestionRef id="question_step2" />
"#
    .to_owned();
    let server = StubLlmServer::start(vec![
        gate_report_1,
        question_bank_2,
        gate_report_2,
        textbook,
    ]);

    let output = TestSupport::coursegen_command()
        .current_dir(repo.root())
        .env("COURSEGEN_LLM_API_KEY", "test-token")
        .env("COURSEGEN_LLM_MODEL", "test-model")
        .env("COURSEGEN_LLM_BASE_URL", server.base_url())
        .args(["run", "L2", "--target-language", "zh-CN"])
        .output()
        .expect("coursegen should run");

    server.finish();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("reusing guided story"),
        "stderr should show guided story reuse: {stderr}"
    );
    assert!(
        stderr.contains(
            "gating existing question bank: research/pipeline/3-guided_story/L2/step1/question_bank.json"
        ),
        "stderr should show step1 existing question bank gate: {stderr}"
    );
    assert!(
        !repo
            .root()
            .join("research/pipeline/meta/L2/mvp/guided_story/request.json")
            .exists(),
        "guided story should not be requested during resume"
    );
    assert!(
        !repo
            .root()
            .join("research/pipeline/meta/L2/mvp/question_bank/step1/request.json")
            .exists(),
        "existing step1 question bank should not be requested again"
    );
    assert!(
        repo.root()
            .join("research/pipeline/meta/L2/mvp/question_bank_gate/step1/request.json")
            .is_file(),
        "existing step1 question bank should be gated"
    );
    assert!(
        repo.root()
            .join("research/pipeline/meta/L2/mvp/question_bank/step2/request.json")
            .is_file(),
        "missing step2 question bank should be requested"
    );
}
