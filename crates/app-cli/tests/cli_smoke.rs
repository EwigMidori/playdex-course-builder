use std::{
    env, fs,
    io::{Read, Write},
    net::TcpListener,
    path::{Path, PathBuf},
    process,
    process::Command,
    sync::atomic::{AtomicU64, Ordering},
    sync::mpsc,
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

static NEXT_TEMP_REPO_ID: AtomicU64 = AtomicU64::new(0);
const RESUME_UNAVAILABLE_MESSAGE: &str =
    "resume requested but no valid prior passed convert run is available for existing output";
const WATCH_TIMEOUT: Duration = Duration::from_secs(5);

const MINERU_ENV_KEYS: &[&str] = &[
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
        let root = env::temp_dir().join(format!("app-cli-smoke-tests-{unique_id}"));

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

    fn write_bytes(&self, relative: impl AsRef<Path>, contents: &[u8]) {
        write_bytes(&self.root.join(relative), contents);
    }
}

impl Drop for TempRepo {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn coursegen_command() -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_coursegen"));
    for key in MINERU_ENV_KEYS {
        command.env_remove(key);
    }
    command
}

fn write_file(path: &Path, contents: &str) {
    write_bytes(path, contents.as_bytes());
}

fn write_bytes(path: &Path, contents: &[u8]) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("parent directories should be created");
    }
    fs::write(path, contents).expect("file should be written");
}

fn run_ids(repo: &TempRepo, lesson_id: &str) -> Vec<String> {
    let run_root = repo
        .root()
        .join("research/pipeline/meta")
        .join(lesson_id)
        .join("runs");
    let mut run_ids = fs::read_dir(&run_root)
        .expect("run root should exist")
        .map(|entry| {
            entry
                .expect("run dir entry should be readable")
                .file_name()
                .into_string()
                .expect("run dir name should be valid utf-8")
        })
        .collect::<Vec<_>>();
    run_ids.sort();
    run_ids
}

fn single_run_id(repo: &TempRepo, lesson_id: &str) -> String {
    let run_ids = run_ids(repo, lesson_id);
    assert_eq!(
        run_ids.len(),
        1,
        "expected exactly one run, got {run_ids:?}"
    );
    run_ids.into_iter().next().expect("single run should exist")
}

fn run_json_path(repo: &TempRepo, lesson_id: &str, run_id: &str) -> PathBuf {
    repo.root()
        .join("research/pipeline/meta")
        .join(lesson_id)
        .join("runs")
        .join(run_id)
        .join("run.json")
}

fn stage_json_path(repo: &TempRepo, lesson_id: &str, run_id: &str) -> PathBuf {
    repo.root()
        .join("research/pipeline/meta")
        .join(lesson_id)
        .join("runs")
        .join(run_id)
        .join("stages/convert.json")
}

fn read_text(path: &Path) -> String {
    fs::read_to_string(path).expect("file should be readable")
}

fn assert_all_contains(text: &str, snippets: &[&str]) {
    for snippet in snippets {
        assert!(
            text.contains(snippet),
            "missing snippet {snippet:?} in:\n{text}"
        );
    }
}

fn seed_passed_convert_run(repo: &TempRepo, lesson_id: &str, run_id: &str) {
    repo.write(
        format!("research/pipeline/meta/{lesson_id}/runs/{run_id}/run.json"),
        &format!(
            concat!(
                "{{\n",
                "  \"schema_version\": 1,\n",
                "  \"run_id\": \"{}\",\n",
                "  \"lesson_id\": \"{}\",\n",
                "  \"command\": \"convert\",\n",
                "  \"resume_requested\": false,\n",
                "  \"resumed_from_run_id\": null,\n",
                "  \"status\": \"passed\",\n",
                "  \"created_at_epoch_ms\": 1,\n",
                "  \"updated_at_epoch_ms\": 2\n",
                "}}\n"
            ),
            run_id, lesson_id
        ),
    );
    repo.write(
        format!("research/pipeline/meta/{lesson_id}/runs/{run_id}/stages/convert.json"),
        &format!(
            concat!(
                "{{\n",
                "  \"schema_version\": 1,\n",
                "  \"run_id\": \"{}\",\n",
                "  \"lesson_id\": \"{}\",\n",
                "  \"stage\": \"convert\",\n",
                "  \"status\": \"passed\",\n",
                "  \"mode\": \"executed\",\n",
                "  \"output_dir\": \"research/pipeline/1-plain/{}\",\n",
                "  \"reused_from_run_id\": null,\n",
                "  \"error\": null,\n",
                "  \"updated_at_epoch_ms\": 2\n",
                "}}\n"
            ),
            run_id, lesson_id, lesson_id
        ),
    );
}

fn seed_passed_convert_output(repo: &TempRepo, lesson_id: &str) {
    repo.write(
        format!("research/pipeline/1-plain/{lesson_id}/plain.txt"),
        "seed plain text\n",
    );
}

struct StubFailureServer {
    base_url: String,
    handle: thread::JoinHandle<()>,
}

impl StubFailureServer {
    fn start(response_delay: Duration) -> Self {
        let listener =
            TcpListener::bind("127.0.0.1:0").expect("stub server should bind a local port");
        let addr = listener
            .local_addr()
            .expect("stub server should report its bound address");
        let base_url = format!("http://{addr}");

        let handle = thread::spawn(move || {
            let (mut stream, _) = listener
                .accept()
                .expect("stub server should accept the upload-url request");
            stream
                .set_read_timeout(Some(Duration::from_secs(2)))
                .expect("stub server should set read timeout");

            let mut buffer = [0_u8; 4096];
            let _ = stream.read(&mut buffer);
            thread::sleep(response_delay);
            stream
                .write_all(
                    b"HTTP/1.1 500 Internal Server Error\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
                )
                .expect("stub server should write a failure response");
            stream.flush().expect("stub server should flush");
        });

        Self { base_url, handle }
    }

    fn base_url(&self) -> &str {
        &self.base_url
    }

    fn finish(self) {
        self.handle
            .join()
            .expect("stub server should complete the failure response");
    }
}

#[cfg(unix)]
struct PermissionRestore {
    path: PathBuf,
    mode: u32,
}

#[cfg(unix)]
impl Drop for PermissionRestore {
    fn drop(&mut self) {
        if self.path.exists() {
            let _ = fs::set_permissions(&self.path, fs::Permissions::from_mode(self.mode));
        }
    }
}

#[cfg(unix)]
struct RunningStageLockWatcher {
    receiver: mpsc::Receiver<Result<PermissionRestore, String>>,
    handle: thread::JoinHandle<()>,
}

#[cfg(unix)]
impl RunningStageLockWatcher {
    fn start(repo: &TempRepo, lesson_id: &str) -> Self {
        let run_root = repo
            .root()
            .join("research/pipeline/meta")
            .join(lesson_id)
            .join("runs");
        let (tx, rx) = mpsc::channel();

        let handle = thread::spawn(move || {
            let deadline = Instant::now() + WATCH_TIMEOUT;
            loop {
                if Instant::now() >= deadline {
                    tx.send(Err(
                        "timed out waiting for running stage metadata".to_owned()
                    ))
                    .expect("watcher should report timeout");
                    return;
                }

                if let Ok(entries) = fs::read_dir(&run_root) {
                    for entry in entries.flatten() {
                        let stage_dir = entry.path().join("stages");
                        let stage_json = stage_dir.join("convert.json");
                        let stage_text = match fs::read_to_string(&stage_json) {
                            Ok(stage_text) => stage_text,
                            Err(_) => continue,
                        };
                        if !stage_text.contains("\"status\": \"running\"") {
                            continue;
                        }

                        let original_mode = fs::metadata(&stage_dir)
                            .expect("stage dir metadata should be readable");
                        let original_mode = original_mode.permissions().mode();
                        fs::set_permissions(&stage_dir, fs::Permissions::from_mode(0o555))
                            .expect("stage dir should become read-only");
                        tx.send(Ok(PermissionRestore {
                            path: stage_dir,
                            mode: original_mode,
                        }))
                        .expect("watcher should report permission change");
                        return;
                    }
                }

                thread::sleep(Duration::from_millis(2));
            }
        });

        Self {
            receiver: rx,
            handle,
        }
    }

    fn finish(self) -> PermissionRestore {
        let restore = self
            .receiver
            .recv_timeout(WATCH_TIMEOUT)
            .expect("watcher should respond before timeout")
            .expect("watcher should find the running stage");
        self.handle
            .join()
            .expect("watcher thread should finish cleanly");
        restore
    }
}

#[test]
fn run_prints_the_placeholder_line_from_repo_root() {
    let output = coursegen_command()
        .current_dir(repo_root())
        .args(["run", "L2"])
        .output()
        .expect("coursegen should run");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "placeholder: command=run stage=- lesson=L2 raw_pdf=research/pipeline/0-raw/L2.pdf\n"
    );
    assert!(
        output.stderr.is_empty(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn run_fails_clearly_outside_the_repository_root() {
    let cwd = repo_root().join("research");
    let output = coursegen_command()
        .current_dir(&cwd)
        .args(["run", "L2"])
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
fn convert_fails_clearly_outside_the_repository_root() {
    let cwd = repo_root().join("research");
    let output = coursegen_command()
        .current_dir(&cwd)
        .args(["convert", "L2"])
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
fn convert_reports_missing_raw_pdf_before_loading_config() {
    let repo = TempRepo::new();
    repo.write(".env", "MINERU_RESULT_TIMEOUT_SECONDS=not-a-number\n");

    let output = coursegen_command()
        .current_dir(repo.root())
        .args(["convert", "L2"])
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
        stderr.contains("missing raw PDF for lesson 'L2': research/pipeline/0-raw/L2.pdf"),
        "stderr: {stderr}"
    );
    assert!(
        !stderr.contains("invalid integer for MINERU_RESULT_TIMEOUT_SECONDS"),
        "stderr: {stderr}"
    );

    let run_id = single_run_id(&repo, "L2");
    let run_json = read_text(&run_json_path(&repo, "L2", &run_id));
    let stage_json = read_text(&stage_json_path(&repo, "L2", &run_id));

    assert_all_contains(
        &run_json,
        &[
            "\"schema_version\": 1",
            &format!("\"run_id\": \"{run_id}\""),
            "\"lesson_id\": \"L2\"",
            "\"command\": \"convert\"",
            "\"resume_requested\": false",
            "\"resumed_from_run_id\": null",
            "\"status\": \"failed\"",
        ],
    );
    assert_all_contains(
        &stage_json,
        &[
            "\"schema_version\": 1",
            &format!("\"run_id\": \"{run_id}\""),
            "\"lesson_id\": \"L2\"",
            "\"stage\": \"convert\"",
            "\"status\": \"failed\"",
            "\"mode\": \"executed\"",
            "\"output_dir\": \"research/pipeline/1-plain/L2\"",
            "\"reused_from_run_id\": null",
            "\"error\": \"missing raw PDF for lesson 'L2': research/pipeline/0-raw/L2.pdf\"",
        ],
    );
}

#[test]
fn convert_records_failed_metadata_for_config_load_errors() {
    let repo = TempRepo::new();
    repo.write_bytes("research/pipeline/0-raw/L2.pdf", b"%PDF-1.7\nstub pdf\n");
    repo.write(".env", "MINERU_RESULT_TIMEOUT_SECONDS=not-a-number\n");

    let output = coursegen_command()
        .current_dir(repo.root())
        .args(["convert", "L2"])
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
        stderr.contains("invalid integer for MINERU_RESULT_TIMEOUT_SECONDS: 'not-a-number'"),
        "stderr: {stderr}"
    );

    let run_id = single_run_id(&repo, "L2");
    let run_json = read_text(&run_json_path(&repo, "L2", &run_id));
    let stage_json = read_text(&stage_json_path(&repo, "L2", &run_id));

    assert_all_contains(
        &run_json,
        &[
            "\"resume_requested\": false",
            "\"resumed_from_run_id\": null",
            "\"status\": \"failed\"",
        ],
    );
    assert_all_contains(
        &stage_json,
        &[
            "\"stage\": \"convert\"",
            "\"status\": \"failed\"",
            "\"mode\": \"executed\"",
            "\"error\": \"invalid integer for MINERU_RESULT_TIMEOUT_SECONDS: 'not-a-number'\"",
        ],
    );
}

#[test]
fn convert_reports_missing_token_before_any_network_work_and_records_failed_metadata() {
    let repo = TempRepo::new();
    repo.write_bytes("research/pipeline/0-raw/L2.pdf", b"%PDF-1.7\nstub pdf\n");
    repo.write("empty-token.txt", "  \n");

    let output = coursegen_command()
        .current_dir(repo.root())
        .env("MINERU_TOKEN_FILE", "empty-token.txt")
        .env("MINERU_API_BASE_URL", "http://127.0.0.1:1")
        .args(["convert", "L2"])
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
        stderr.contains("missing MinerU API token"),
        "stderr: {stderr}"
    );
    assert!(
        stderr.contains(&repo.root().join("empty-token.txt").display().to_string()),
        "stderr: {stderr}"
    );
    assert!(!stderr.contains("127.0.0.1:1"), "stderr: {stderr}");
    assert!(
        !repo.root().join("research/pipeline/1-plain/L2").exists(),
        "final output directory should not be created on missing-token failure"
    );

    let run_id = single_run_id(&repo, "L2");
    let run_json = read_text(&run_json_path(&repo, "L2", &run_id));
    let stage_json = read_text(&stage_json_path(&repo, "L2", &run_id));

    assert_all_contains(
        &run_json,
        &[
            "\"resume_requested\": false",
            "\"resumed_from_run_id\": null",
            "\"status\": \"failed\"",
        ],
    );
    assert_all_contains(
        &stage_json,
        &[
            "\"stage\": \"convert\"",
            "\"status\": \"failed\"",
            "\"mode\": \"executed\"",
            "\"output_dir\": \"research/pipeline/1-plain/L2\"",
            "\"reused_from_run_id\": null",
            "\"error\": \"missing MinerU API token:",
        ],
    );
    assert!(
        stage_json.contains(&repo.root().join("empty-token.txt").display().to_string()),
        "stage metadata should record the repository-owned missing-token message: {stage_json}"
    );
}

#[test]
fn convert_resume_reuses_prior_passed_run_with_fresh_metadata_and_skips_config_and_raw_checks() {
    let repo = TempRepo::new();
    let prior_run_id = "0000000000001-1";
    seed_passed_convert_run(&repo, "L2", prior_run_id);
    seed_passed_convert_output(&repo, "L2");
    repo.write(".env", "MINERU_RESULT_TIMEOUT_SECONDS=not-a-number\n");

    let output = coursegen_command()
        .current_dir(repo.root())
        .args(["convert", "L2", "--resume"])
        .output()
        .expect("coursegen should run");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        output.stdout.is_empty(),
        "stdout: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    assert!(
        output.stderr.is_empty(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let run_ids = run_ids(&repo, "L2");
    assert_eq!(run_ids.len(), 2, "expected prior and fresh runs");
    let fresh_run_id = run_ids
        .iter()
        .find(|run_id| run_id.as_str() != prior_run_id)
        .expect("fresh run id should exist")
        .clone();
    assert_ne!(fresh_run_id, prior_run_id);

    let fresh_run_json = read_text(&run_json_path(&repo, "L2", &fresh_run_id));
    let fresh_stage_json = read_text(&stage_json_path(&repo, "L2", &fresh_run_id));

    assert_all_contains(
        &fresh_run_json,
        &[
            &format!("\"run_id\": \"{fresh_run_id}\""),
            "\"resume_requested\": true",
            &format!("\"resumed_from_run_id\": \"{prior_run_id}\""),
            "\"status\": \"passed\"",
        ],
    );
    assert_all_contains(
        &fresh_stage_json,
        &[
            "\"stage\": \"convert\"",
            "\"status\": \"passed\"",
            "\"mode\": \"reused\"",
            "\"output_dir\": \"research/pipeline/1-plain/L2\"",
            &format!("\"reused_from_run_id\": \"{prior_run_id}\""),
            "\"error\": null",
        ],
    );
    assert!(
        !fresh_stage_json.contains("missing raw PDF"),
        "resume path should skip raw-pdf validation: {fresh_stage_json}"
    );
    assert!(
        !fresh_stage_json.contains("invalid integer for MINERU_RESULT_TIMEOUT_SECONDS"),
        "resume path should skip config loading: {fresh_stage_json}"
    );
}

#[test]
fn convert_resume_ignores_stale_passed_metadata_and_falls_back_to_executed_path() {
    let repo = TempRepo::new();
    let prior_run_id = "0000000000001-1";
    seed_passed_convert_run(&repo, "L2", prior_run_id);
    repo.write_bytes("research/pipeline/0-raw/L2.pdf", b"%PDF-1.7\nstub pdf\n");
    repo.write(".env", "MINERU_RESULT_TIMEOUT_SECONDS=not-a-number\n");

    let output = coursegen_command()
        .current_dir(repo.root())
        .args(["convert", "L2", "--resume"])
        .output()
        .expect("coursegen should run");

    assert!(
        !output.status.success(),
        "stdout: {}",
        String::from_utf8_lossy(&output.stdout)
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("invalid integer for MINERU_RESULT_TIMEOUT_SECONDS: 'not-a-number'"),
        "stderr: {stderr}"
    );
    assert!(
        !stderr.contains(RESUME_UNAVAILABLE_MESSAGE),
        "stale metadata should fall back to executed path instead of resume-unavailable: {stderr}"
    );

    let run_ids = run_ids(&repo, "L2");
    assert_eq!(run_ids.len(), 2, "expected prior and fresh runs");
    let fresh_run_id = run_ids
        .iter()
        .find(|run_id| run_id.as_str() != prior_run_id)
        .expect("fresh run id should exist")
        .clone();

    let fresh_run_json = read_text(&run_json_path(&repo, "L2", &fresh_run_id));
    let fresh_stage_json = read_text(&stage_json_path(&repo, "L2", &fresh_run_id));
    assert_all_contains(
        &fresh_run_json,
        &[
            "\"resume_requested\": true",
            "\"resumed_from_run_id\": null",
            "\"status\": \"failed\"",
        ],
    );
    assert_all_contains(
        &fresh_stage_json,
        &[
            "\"status\": \"failed\"",
            "\"mode\": \"executed\"",
            "\"reused_from_run_id\": null",
            "\"error\": \"invalid integer for MINERU_RESULT_TIMEOUT_SECONDS: 'not-a-number'\"",
        ],
    );
}

#[test]
fn convert_resume_unavailable_records_failed_reused_metadata_and_returns_top_level_error() {
    let repo = TempRepo::new();
    repo.write(
        "research/pipeline/1-plain/L2/plain.txt",
        "existing plain text\n",
    );
    repo.write(".env", "MINERU_RESULT_TIMEOUT_SECONDS=not-a-number\n");

    let output = coursegen_command()
        .current_dir(repo.root())
        .args(["convert", "L2", "--resume"])
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
        stderr.contains(&format!("{RESUME_UNAVAILABLE_MESSAGE} for lesson 'L2'")),
        "stderr: {stderr}"
    );
    assert!(!stderr.contains("missing raw PDF"), "stderr: {stderr}");
    assert!(
        !stderr.contains("invalid integer for MINERU_RESULT_TIMEOUT_SECONDS"),
        "stderr: {stderr}"
    );

    let run_id = single_run_id(&repo, "L2");
    let run_json = read_text(&run_json_path(&repo, "L2", &run_id));
    let stage_json = read_text(&stage_json_path(&repo, "L2", &run_id));

    assert_all_contains(
        &run_json,
        &[
            "\"resume_requested\": true",
            "\"resumed_from_run_id\": null",
            "\"status\": \"failed\"",
        ],
    );
    assert_all_contains(
        &stage_json,
        &[
            "\"stage\": \"convert\"",
            "\"status\": \"failed\"",
            "\"mode\": \"reused\"",
            "\"output_dir\": \"research/pipeline/1-plain/L2\"",
            "\"reused_from_run_id\": null",
            &format!("\"error\": \"{RESUME_UNAVAILABLE_MESSAGE}\""),
        ],
    );
}

#[cfg(unix)]
#[test]
fn convert_reports_a_composite_error_when_business_failure_and_failed_metadata_write_both_happen() {
    let repo = TempRepo::new();
    repo.write_bytes("research/pipeline/0-raw/L2.pdf", b"%PDF-1.7\nstub pdf\n");

    let server = StubFailureServer::start(Duration::from_millis(250));
    let permission_watcher = RunningStageLockWatcher::start(&repo, "L2");

    let output = coursegen_command()
        .current_dir(repo.root())
        .env("MINERU_API_TOKEN", "test-token")
        .env("MINERU_API_BASE_URL", server.base_url())
        .args(["convert", "L2"])
        .output()
        .expect("coursegen should run");

    let permission_restore = permission_watcher.finish();
    server.finish();

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
        stderr.contains("operation failed and run metadata persistence also failed:"),
        "stderr: {stderr}"
    );
    assert!(
        stderr.contains("operation: failed to request MinerU upload URL:"),
        "stderr: {stderr}"
    );
    assert!(
        stderr.contains("metadata: failed to write stage metadata"),
        "stderr: {stderr}"
    );

    drop(permission_restore);

    let run_id = single_run_id(&repo, "L2");
    let run_json = read_text(&run_json_path(&repo, "L2", &run_id));
    let stage_json = read_text(&stage_json_path(&repo, "L2", &run_id));
    assert_all_contains(
        &run_json,
        &[
            "\"resume_requested\": false",
            "\"resumed_from_run_id\": null",
            "\"status\": \"running\"",
        ],
    );
    assert_all_contains(
        &stage_json,
        &[
            "\"status\": \"running\"",
            "\"mode\": \"executed\"",
            "\"error\": null",
        ],
    );
}
