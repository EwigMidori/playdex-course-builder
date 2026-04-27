use std::{
    env, fs,
    path::{Path, PathBuf},
    process,
    process::Command,
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

static NEXT_TEMP_REPO_ID: AtomicU64 = AtomicU64::new(0);

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
}

#[test]
fn convert_reports_missing_token_before_any_network_work() {
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
}
