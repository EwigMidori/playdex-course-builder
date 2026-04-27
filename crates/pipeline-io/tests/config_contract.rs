use std::{
    collections::BTreeMap,
    env, fs,
    path::{Path, PathBuf},
    process,
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

use pipeline_core::RepoPaths;
use pipeline_io::{load_config_from_env_map, ConfigError, TokenSourceState};
use serde_json::Value;

static NEXT_TEMP_REPO_ID: AtomicU64 = AtomicU64::new(0);

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
        let root = env::temp_dir().join(format!("pipeline-io-config-tests-{unique_id}"));

        fs::create_dir_all(root.join(".git")).expect("temp repo should create .git directory");
        write_file(&root.join(".ci/agent/AGENTS.md"), "test harness\n");
        write_file(&root.join("docs/progress.json"), "{}\n");
        write_file(&root.join("research/README.md"), "test harness\n");

        Self { root }
    }

    fn paths(&self) -> RepoPaths {
        RepoPaths::from_root(self.root.clone()).expect("temp repo root should resolve")
    }

    fn write(&self, relative: impl AsRef<Path>, contents: &str) {
        write_file(&self.root.join(relative), contents);
    }

    fn root(&self) -> &Path {
        &self.root
    }
}

impl Drop for TempRepo {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn write_file(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("parent directories should be created");
    }
    fs::write(path, contents).expect("file should be written");
}

fn env_map(entries: &[(&str, &str)]) -> BTreeMap<String, String> {
    entries
        .iter()
        .map(|(key, value)| ((*key).to_owned(), (*value).to_owned()))
        .collect()
}

fn parse_sanitized_json(raw: &str) -> Value {
    serde_json::from_str(raw).expect("sanitized config should be valid json")
}

#[test]
fn env_values_override_dotenv_and_blank_values_fall_back_to_defaults() {
    let repo = TempRepo::new();
    repo.write(
        ".env",
        concat!(
            "MINERU_LANGUAGE=dotenv-language\n",
            "MINERU_API_BASE_URL=https://dotenv.example\n",
            "MINERU_MODEL_VERSION=dotenv-model\n",
            "MINERU_ENABLE_TABLE=no\n",
            "MINERU_REQUEST_TIMEOUT_SECONDS=77\n",
        ),
    );

    let config = load_config_from_env_map(
        &repo.paths(),
        &env_map(&[
            ("MINERU_LANGUAGE", "  process-language  "),
            ("MINERU_MODEL_VERSION", "   "),
            ("MINERU_ENABLE_TABLE", "   "),
            ("MINERU_REQUEST_TIMEOUT_SECONDS", "42"),
        ]),
    )
    .expect("config should load");

    assert!(config.dotenv_loaded());
    assert_eq!(config.dotenv_path(), repo.root().join(".env"));
    assert_eq!(config.mineru().language(), "process-language");
    assert_eq!(config.mineru().api_base_url(), "https://dotenv.example");
    assert_eq!(config.mineru().model_version(), "vlm");
    assert!(config.mineru().enable_table());
    assert_eq!(config.mineru().request_timeout_seconds(), 42);
    assert_eq!(
        config.mineru().token_file(),
        repo.root().join("mineru_token.txt")
    );
}

#[test]
fn missing_dotenv_uses_hardcoded_defaults() {
    let repo = TempRepo::new();

    let config =
        load_config_from_env_map(&repo.paths(), &BTreeMap::new()).expect("config should load");

    assert!(!config.dotenv_loaded());
    assert_eq!(config.mineru().api_base_url(), "https://mineru.net");
    assert_eq!(config.mineru().model_version(), "vlm");
    assert_eq!(config.mineru().language(), "ch");
    assert!(config.mineru().enable_formula());
    assert!(config.mineru().enable_table());
    assert!(config.mineru().is_ocr());
    assert!(!config.mineru().fail_on_empty_text());
    assert_eq!(config.mineru().request_timeout_seconds(), 120);
    assert_eq!(config.mineru().upload_timeout_seconds(), 600);
    assert_eq!(config.mineru().result_timeout_seconds(), 3600);
    assert_eq!(config.mineru().poll_interval_seconds(), 15);
    assert_eq!(config.mineru().download_timeout_seconds(), 600);
    assert_eq!(
        config.mineru().token_source_state(),
        TokenSourceState::Missing
    );
}

#[test]
fn invalid_integer_errors_name_the_key_and_trimmed_value() {
    let repo = TempRepo::new();

    let error = load_config_from_env_map(
        &repo.paths(),
        &env_map(&[("MINERU_RESULT_TIMEOUT_SECONDS", "  not-a-number  ")]),
    )
    .expect_err("invalid integers should fail");

    match error {
        ConfigError::InvalidInteger { key, value, .. } => {
            assert_eq!(key, "MINERU_RESULT_TIMEOUT_SECONDS");
            assert_eq!(value, "not-a-number");
        }
        other => panic!("expected InvalidInteger, got {other:?}"),
    }
}

#[test]
fn inline_token_wins_over_token_file_and_sanitized_dump_stays_redacted() {
    let repo = TempRepo::new();
    repo.write("secrets/mineru.txt", "file-secret\n");

    let config = load_config_from_env_map(
        &repo.paths(),
        &env_map(&[
            ("MINERU_API_TOKEN", "  inline-secret  "),
            ("MINERU_TOKEN_FILE", "secrets/mineru.txt"),
        ]),
    )
    .expect("config should load");

    assert_eq!(config.mineru().token_source_state(), TokenSourceState::Env);
    assert_eq!(
        config.read_mineru_token().expect("inline token should win"),
        "inline-secret"
    );

    let sanitized = config.sanitized_json();
    let json = parse_sanitized_json(&sanitized);

    assert_eq!(json["dotenv"]["loaded"], false);
    assert_eq!(json["dotenv"]["path"], ".env");
    assert_eq!(json["mineru"]["token_source"], "env");
    assert_eq!(json["mineru"]["token_file"], "secrets/mineru.txt");
    assert_eq!(json["mineru"]["timeouts_seconds"]["request"], 120);
    assert!(!sanitized.contains("inline-secret"));
    assert!(!sanitized.contains("file-secret"));
}

#[test]
fn token_file_is_used_when_inline_token_is_missing() {
    let repo = TempRepo::new();
    repo.write("config/tokens/mineru.txt", "  file-token  \n");

    let config = load_config_from_env_map(
        &repo.paths(),
        &env_map(&[("MINERU_TOKEN_FILE", "config/tokens/mineru.txt")]),
    )
    .expect("config should load");

    assert_eq!(config.mineru().token_source_state(), TokenSourceState::File);
    assert_eq!(
        config.mineru().token_file(),
        repo.root().join("config/tokens/mineru.txt")
    );
    assert_eq!(
        config
            .read_mineru_token()
            .expect("token file should be read"),
        "file-token"
    );
}

#[test]
fn missing_token_reports_a_repository_owned_error_without_secret_leakage() {
    let repo = TempRepo::new();
    repo.write("empty-token.txt", " \n");

    let config = load_config_from_env_map(
        &repo.paths(),
        &env_map(&[("MINERU_TOKEN_FILE", "empty-token.txt")]),
    )
    .expect("config loading should succeed without a token");

    assert_eq!(
        config.mineru().token_source_state(),
        TokenSourceState::Missing
    );

    match config
        .read_mineru_token()
        .expect_err("empty token files should count as missing")
    {
        ConfigError::MissingToken { path } => {
            assert_eq!(path, repo.root().join("empty-token.txt"));
        }
        other => panic!("expected MissingToken, got {other:?}"),
    }

    let sanitized = config.sanitized_json();
    assert!(!sanitized.contains("Bearer"));
}
