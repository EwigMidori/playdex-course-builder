use std::{
    env, fs,
    io::{Read, Write},
    net::TcpListener,
    path::{Path, PathBuf},
    process,
    process::Command,
    sync::atomic::{AtomicU64, Ordering},
    thread,
    time::{SystemTime, UNIX_EPOCH},
};

static NEXT_TEMP_REPO_ID: AtomicU64 = AtomicU64::new(0);

const ENV_KEYS: &[&str] = &[
    "COURSEGEN_LLM_API_KEY",
    "COURSEGEN_LLM_BASE_URL",
    "COURSEGEN_LLM_MODEL",
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

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crate directory should live under the repository root")
        .to_path_buf()
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
        write_file(&root.join(".ci/agent/AGENTS.md"), "test harness\n");
        write_file(&root.join("docs/progress.json"), "{}\n");
        write_file(&root.join("research/README.md"), "test harness\n");

        Self { root }
    }

    fn root(&self) -> &Path {
        &self.root
    }

    fn write(&self, relative: impl AsRef<Path>, contents: &str) {
        write_file(&self.root.join(relative), contents);
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
        self.write("research/prompts/textbook_system.md", "textbook system\n");
        self.write(
            "research/prompts/textbook_user.md",
            "textbook {{target_language}} {{lesson_id}} {{question_bank}} {{plain_text}}\n",
        );
    }
}

impl Drop for TempRepo {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

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

struct StubLlmServer {
    base_url: String,
    handle: thread::JoinHandle<()>,
}

impl StubLlmServer {
    fn start(contents: Vec<String>) -> Self {
        let listener =
            TcpListener::bind("127.0.0.1:0").expect("stub server should bind a local port");
        let addr = listener
            .local_addr()
            .expect("stub server should report its bound address");
        let base_url = format!("http://{addr}/v1");

        let handle = thread::spawn(move || {
            for content in contents {
                let (mut stream, _) = listener
                    .accept()
                    .expect("stub server should accept an LLM request");
                let mut buffer = [0_u8; 8192];
                let _ = stream.read(&mut buffer);
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

        Self { base_url, handle }
    }

    fn base_url(&self) -> &str {
        &self.base_url
    }

    fn finish(self) {
        self.handle
            .join()
            .expect("stub server should finish cleanly");
    }
}

#[test]
fn validate_fails_clearly_outside_the_repository_root() {
    let cwd = repo_root().join("research");
    let output = coursegen_command()
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

    let output = coursegen_command()
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

    let output = coursegen_command()
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

    let output = coursegen_command()
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
    let textbook =
        r#"---\nlessonId: L2\n---\n# L2\n<QuestionFamily familyId="family_a" />\n<QuestionRef id="question_a" />\n"#.to_owned();
    let server = StubLlmServer::start(vec![guided_story, question_bank, textbook]);

    let output = coursegen_command()
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
        String::from_utf8_lossy(&output.stdout).contains("mvp complete: lesson=L2"),
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
    for stage in ["guided_story", "question_bank", "textbook"] {
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
        read_text(audit_root.join("guided_story/user.md")).contains("zh-CN"),
        "rendered prompt should include target language"
    );
    assert!(
        read_text(textbook_path).contains(r#"<QuestionRef id=\"question_a\" />"#)
            || read_text(repo.root().join("research/pipeline/5-textbook/L2.mdx"))
                .contains(r#"<QuestionRef id="question_a" />"#),
        "textbook should contain the stubbed question reference"
    );

    let validate = coursegen_command()
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
