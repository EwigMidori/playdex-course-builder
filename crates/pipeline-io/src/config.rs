use std::{
    collections::BTreeMap,
    env,
    error::Error,
    fmt, fs, io,
    num::ParseIntError,
    path::{Path, PathBuf},
};

use pipeline_core::RepoPaths;

const MINERU_API_TOKEN: &str = "MINERU_API_TOKEN";
const MINERU_TOKEN_FILE: &str = "MINERU_TOKEN_FILE";
const MINERU_API_BASE_URL: &str = "MINERU_API_BASE_URL";
const MINERU_MODEL_VERSION: &str = "MINERU_MODEL_VERSION";
const MINERU_LANGUAGE: &str = "MINERU_LANGUAGE";
const MINERU_ENABLE_FORMULA: &str = "MINERU_ENABLE_FORMULA";
const MINERU_ENABLE_TABLE: &str = "MINERU_ENABLE_TABLE";
const MINERU_IS_OCR: &str = "MINERU_IS_OCR";
const MINERU_FAIL_ON_EMPTY_TEXT: &str = "MINERU_FAIL_ON_EMPTY_TEXT";
const MINERU_REQUEST_TIMEOUT_SECONDS: &str = "MINERU_REQUEST_TIMEOUT_SECONDS";
const MINERU_UPLOAD_TIMEOUT_SECONDS: &str = "MINERU_UPLOAD_TIMEOUT_SECONDS";
const MINERU_RESULT_TIMEOUT_SECONDS: &str = "MINERU_RESULT_TIMEOUT_SECONDS";
const MINERU_POLL_INTERVAL_SECONDS: &str = "MINERU_POLL_INTERVAL_SECONDS";
const MINERU_DOWNLOAD_TIMEOUT_SECONDS: &str = "MINERU_DOWNLOAD_TIMEOUT_SECONDS";

pub fn load_process_config(repo: &RepoPaths) -> Result<ResolvedConfig, ConfigError> {
    let env = env::vars_os()
        .filter_map(|(key, value)| {
            let key = key.into_string().ok()?;
            let value = value.into_string().ok()?;
            Some((key, value))
        })
        .collect::<BTreeMap<_, _>>();

    load_config_from_env_map(repo, &env)
}

pub fn load_config_from_env_map(
    repo: &RepoPaths,
    env: &BTreeMap<String, String>,
) -> Result<ResolvedConfig, ConfigError> {
    let dotenv_path = repo.dotenv_path();
    let dotenv_loaded = dotenv_path.is_file();
    let dotenv = load_dotenv_map(&dotenv_path)?;
    let merged = merge_env(dotenv, env);

    Ok(ResolvedConfig {
        repo_root: repo.root().to_path_buf(),
        dotenv_loaded,
        dotenv_path,
        mineru: MineruConfig {
            api_base_url: resolve_string_with_default(
                &merged,
                MINERU_API_BASE_URL,
                "https://mineru.net",
            ),
            token_file: resolve_token_file(repo, &merged),
            model_version: resolve_string_with_default(&merged, MINERU_MODEL_VERSION, "vlm"),
            language: resolve_string_with_default(&merged, MINERU_LANGUAGE, "ch"),
            enable_formula: resolve_bool_with_default(&merged, MINERU_ENABLE_FORMULA, true),
            enable_table: resolve_bool_with_default(&merged, MINERU_ENABLE_TABLE, true),
            is_ocr: resolve_bool_with_default(&merged, MINERU_IS_OCR, true),
            fail_on_empty_text: resolve_bool_with_default(
                &merged,
                MINERU_FAIL_ON_EMPTY_TEXT,
                false,
            ),
            request_timeout_seconds: resolve_u64_with_default(
                &merged,
                MINERU_REQUEST_TIMEOUT_SECONDS,
                120,
            )?,
            upload_timeout_seconds: resolve_u64_with_default(
                &merged,
                MINERU_UPLOAD_TIMEOUT_SECONDS,
                600,
            )?,
            result_timeout_seconds: resolve_u64_with_default(
                &merged,
                MINERU_RESULT_TIMEOUT_SECONDS,
                3600,
            )?,
            poll_interval_seconds: resolve_u64_with_default(
                &merged,
                MINERU_POLL_INTERVAL_SECONDS,
                15,
            )?,
            download_timeout_seconds: resolve_u64_with_default(
                &merged,
                MINERU_DOWNLOAD_TIMEOUT_SECONDS,
                600,
            )?,
            inline_token: resolve_optional_string(&merged, MINERU_API_TOKEN),
        },
    })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TokenSourceState {
    Env,
    File,
    Missing,
}

pub struct ResolvedConfig {
    repo_root: PathBuf,
    dotenv_loaded: bool,
    dotenv_path: PathBuf,
    mineru: MineruConfig,
}

impl ResolvedConfig {
    pub fn mineru(&self) -> &MineruConfig {
        &self.mineru
    }

    pub fn dotenv_loaded(&self) -> bool {
        self.dotenv_loaded
    }

    pub fn dotenv_path(&self) -> &Path {
        &self.dotenv_path
    }

    pub fn sanitized_json(&self) -> String {
        let api_base_url = json_string(self.mineru.api_base_url());
        let token_source = json_string(self.mineru.token_source_state().as_str());
        let token_file = json_string(&path_to_sanitized_string(
            &self.repo_root,
            self.mineru.token_file(),
        ));
        let model_version = json_string(self.mineru.model_version());
        let language = json_string(self.mineru.language());

        format!(
            concat!(
                "{{\n",
                "  \"dotenv\": {{\n",
                "    \"loaded\": {},\n",
                "    \"path\": \".env\"\n",
                "  }},\n",
                "  \"mineru\": {{\n",
                "    \"api_base_url\": {},\n",
                "    \"token_source\": {},\n",
                "    \"token_file\": {},\n",
                "    \"model_version\": {},\n",
                "    \"language\": {},\n",
                "    \"enable_formula\": {},\n",
                "    \"enable_table\": {},\n",
                "    \"is_ocr\": {},\n",
                "    \"fail_on_empty_text\": {},\n",
                "    \"timeouts_seconds\": {{\n",
                "      \"request\": {},\n",
                "      \"upload\": {},\n",
                "      \"result\": {},\n",
                "      \"poll_interval\": {},\n",
                "      \"download\": {}\n",
                "    }}\n",
                "  }}\n",
                "}}"
            ),
            self.dotenv_loaded(),
            api_base_url,
            token_source,
            token_file,
            model_version,
            language,
            self.mineru.enable_formula(),
            self.mineru.enable_table(),
            self.mineru.is_ocr(),
            self.mineru.fail_on_empty_text(),
            self.mineru.request_timeout_seconds(),
            self.mineru.upload_timeout_seconds(),
            self.mineru.result_timeout_seconds(),
            self.mineru.poll_interval_seconds(),
            self.mineru.download_timeout_seconds(),
        )
    }

    pub fn read_mineru_token(&self) -> Result<String, ConfigError> {
        self.mineru.read_token()
    }
}

impl fmt::Debug for ResolvedConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.sanitized_json())
    }
}

pub struct MineruConfig {
    api_base_url: String,
    token_file: PathBuf,
    model_version: String,
    language: String,
    enable_formula: bool,
    enable_table: bool,
    is_ocr: bool,
    fail_on_empty_text: bool,
    request_timeout_seconds: u64,
    upload_timeout_seconds: u64,
    result_timeout_seconds: u64,
    poll_interval_seconds: u64,
    download_timeout_seconds: u64,
    inline_token: Option<String>,
}

impl MineruConfig {
    pub fn api_base_url(&self) -> &str {
        &self.api_base_url
    }

    pub fn token_source_state(&self) -> TokenSourceState {
        if self.inline_token.is_some() {
            return TokenSourceState::Env;
        }

        match read_trimmed_file(&self.token_file) {
            Ok(token) if !token.is_empty() => TokenSourceState::File,
            Ok(_) | Err(ConfigError::MissingToken { .. }) | Err(ConfigError::TokenRead { .. }) => {
                TokenSourceState::Missing
            }
            Err(_) => TokenSourceState::Missing,
        }
    }

    pub fn token_file(&self) -> &Path {
        &self.token_file
    }

    pub fn model_version(&self) -> &str {
        &self.model_version
    }

    pub fn language(&self) -> &str {
        &self.language
    }

    pub fn enable_formula(&self) -> bool {
        self.enable_formula
    }

    pub fn enable_table(&self) -> bool {
        self.enable_table
    }

    pub fn is_ocr(&self) -> bool {
        self.is_ocr
    }

    pub fn fail_on_empty_text(&self) -> bool {
        self.fail_on_empty_text
    }

    pub fn request_timeout_seconds(&self) -> u64 {
        self.request_timeout_seconds
    }

    pub fn upload_timeout_seconds(&self) -> u64 {
        self.upload_timeout_seconds
    }

    pub fn result_timeout_seconds(&self) -> u64 {
        self.result_timeout_seconds
    }

    pub fn poll_interval_seconds(&self) -> u64 {
        self.poll_interval_seconds
    }

    pub fn download_timeout_seconds(&self) -> u64 {
        self.download_timeout_seconds
    }

    fn read_token(&self) -> Result<String, ConfigError> {
        if let Some(token) = &self.inline_token {
            return Ok(token.clone());
        }

        read_trimmed_file(&self.token_file)
    }
}

impl fmt::Debug for MineruConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MineruConfig")
            .field("api_base_url", &self.api_base_url)
            .field("token_source_state", &self.token_source_state())
            .field("token_file", &self.token_file)
            .field("model_version", &self.model_version)
            .field("language", &self.language)
            .field("enable_formula", &self.enable_formula)
            .field("enable_table", &self.enable_table)
            .field("is_ocr", &self.is_ocr)
            .field("fail_on_empty_text", &self.fail_on_empty_text)
            .field("request_timeout_seconds", &self.request_timeout_seconds)
            .field("upload_timeout_seconds", &self.upload_timeout_seconds)
            .field("result_timeout_seconds", &self.result_timeout_seconds)
            .field("poll_interval_seconds", &self.poll_interval_seconds)
            .field("download_timeout_seconds", &self.download_timeout_seconds)
            .finish()
    }
}

#[derive(Debug)]
pub enum ConfigError {
    DotenvRead {
        path: PathBuf,
        source: io::Error,
    },
    InvalidInteger {
        key: &'static str,
        value: String,
        source: ParseIntError,
    },
    MissingToken {
        path: PathBuf,
    },
    TokenRead {
        path: PathBuf,
        source: io::Error,
    },
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DotenvRead { path, source } => {
                write!(f, "failed to read dotenv file {}: {source}", path.display())
            }
            Self::InvalidInteger { key, value, .. } => {
                write!(f, "invalid integer for {key}: '{value}'")
            }
            Self::MissingToken { path } => write!(
                f,
                "missing MinerU API token: set {MINERU_API_TOKEN} or provide a non-empty token file at {}",
                path.display()
            ),
            Self::TokenRead { path, source } => {
                write!(f, "failed to read MinerU token file {}: {source}", path.display())
            }
        }
    }
}

impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::DotenvRead { source, .. } => Some(source),
            Self::InvalidInteger { source, .. } => Some(source),
            Self::TokenRead { source, .. } => Some(source),
            Self::MissingToken { .. } => None,
        }
    }
}

fn load_dotenv_map(path: &Path) -> Result<BTreeMap<String, String>, ConfigError> {
    let mut env = BTreeMap::new();

    let contents = match fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(source) if source.kind() == io::ErrorKind::NotFound => return Ok(env),
        Err(source) => {
            return Err(ConfigError::DotenvRead {
                path: path.to_path_buf(),
                source,
            });
        }
    };

    for raw_line in contents.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let Some((raw_key, raw_value)) = line.split_once('=') else {
            continue;
        };

        let key = raw_key.trim();
        if key.is_empty() {
            continue;
        }

        env.insert(
            key.to_owned(),
            strip_surrounding_quotes(raw_value.trim()).to_owned(),
        );
    }

    Ok(env)
}

fn merge_env(
    dotenv: BTreeMap<String, String>,
    process_env: &BTreeMap<String, String>,
) -> BTreeMap<String, String> {
    let mut merged = dotenv;
    for (key, value) in process_env {
        merged.insert(key.clone(), value.clone());
    }
    merged
}

fn resolve_string_with_default(
    env: &BTreeMap<String, String>,
    key: &'static str,
    default: &str,
) -> String {
    resolve_optional_string(env, key)
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| default.to_owned())
}

fn resolve_optional_string(env: &BTreeMap<String, String>, key: &'static str) -> Option<String> {
    env.get(key)
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}

fn resolve_bool_with_default(
    env: &BTreeMap<String, String>,
    key: &'static str,
    default: bool,
) -> bool {
    let Some(value) = env.get(key) else {
        return default;
    };

    let value = value.trim();
    if value.is_empty() {
        return default;
    }

    matches!(
        value.to_ascii_lowercase().as_str(),
        "1" | "true" | "yes" | "on"
    )
}

fn resolve_u64_with_default(
    env: &BTreeMap<String, String>,
    key: &'static str,
    default: u64,
) -> Result<u64, ConfigError> {
    let Some(value) = env.get(key) else {
        return Ok(default);
    };

    let value = value.trim();
    if value.is_empty() {
        return Ok(default);
    }

    value
        .parse::<u64>()
        .map_err(|source| ConfigError::InvalidInteger {
            key,
            value: value.to_owned(),
            source,
        })
}

fn resolve_token_file(repo: &RepoPaths, env: &BTreeMap<String, String>) -> PathBuf {
    let token_file = resolve_optional_string(env, MINERU_TOKEN_FILE)
        .map(PathBuf::from)
        .unwrap_or_else(|| repo.default_mineru_token_file_path());

    if token_file.is_absolute() {
        token_file
    } else {
        repo.root().join(token_file)
    }
}

fn read_trimmed_file(path: &Path) -> Result<String, ConfigError> {
    match fs::read_to_string(path) {
        Ok(contents) => {
            let token = contents.trim().to_owned();
            if token.is_empty() {
                Err(ConfigError::MissingToken {
                    path: path.to_path_buf(),
                })
            } else {
                Ok(token)
            }
        }
        Err(source) if source.kind() == io::ErrorKind::NotFound => Err(ConfigError::MissingToken {
            path: path.to_path_buf(),
        }),
        Err(source) => Err(ConfigError::TokenRead {
            path: path.to_path_buf(),
            source,
        }),
    }
}

fn strip_surrounding_quotes(value: &str) -> &str {
    if value.len() >= 2 {
        let bytes = value.as_bytes();
        let first = bytes[0];
        let last = bytes[value.len() - 1];
        if (first == b'"' && last == b'"') || (first == b'\'' && last == b'\'') {
            return &value[1..value.len() - 1];
        }
    }

    value
}

fn path_to_sanitized_string(repo_root: &Path, path: &Path) -> String {
    path.strip_prefix(repo_root)
        .ok()
        .map(path_to_forward_slashes)
        .unwrap_or_else(|| path_to_forward_slashes(path))
}

fn path_to_forward_slashes(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

impl TokenSourceState {
    fn as_str(self) -> &'static str {
        match self {
            Self::Env => "env",
            Self::File => "file",
            Self::Missing => "missing",
        }
    }
}

fn json_string(value: &str) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| "\"<serialization-error>\"".to_owned())
}
