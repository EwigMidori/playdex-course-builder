use std::{
    collections::BTreeMap,
    env,
    error::Error,
    fmt, fs, io,
    num::ParseIntError,
    path::{Path, PathBuf},
};

use crate::paths::RepoPaths;

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
const COURSEGEN_LLM_PROVIDER: &str = "COURSEGEN_LLM_PROVIDER";
const COURSEGEN_LLM_MAX_TOKENS: &str = "COURSEGEN_LLM_MAX_TOKENS";
const COURSEGEN_LLM_TIMEOUT_SECONDS: &str = "COURSEGEN_LLM_TIMEOUT_SECONDS";
const DEEPSEEK_API_KEY: &str = "DEEPSEEK_API_KEY";
const DEEPSEEK_API_KEYS_PATH: &str = "research/API_KEYS.md";
const DEEPSEEK_BASE_URL: &str = "DEEPSEEK_BASE_URL";
const DEEPSEEK_MODEL: &str = "DEEPSEEK_MODEL";
const DEEPSEEK_MAX_TOKENS: &str = "DEEPSEEK_MAX_TOKENS";
const DEEPSEEK_TIMEOUT_SECONDS: &str = "DEEPSEEK_TIMEOUT_SECONDS";

#[derive(Clone, Debug)]
pub struct Config {
    pub mineru: MineruConfig,
    pub llm: LlmConfig,
}

#[derive(Clone, Debug)]
pub struct MineruConfig {
    pub api_base_url: String,
    pub token_file: PathBuf,
    pub model_version: String,
    pub language: String,
    pub enable_formula: bool,
    pub enable_table: bool,
    pub is_ocr: bool,
    pub fail_on_empty_text: bool,
    pub request_timeout_seconds: u64,
    pub upload_timeout_seconds: u64,
    pub result_timeout_seconds: u64,
    pub poll_interval_seconds: u64,
    pub download_timeout_seconds: u64,
    inline_token: Option<String>,
}

#[derive(Clone, Debug)]
pub struct LlmConfig {
    pub provider: ApiProvider,
    pub api_key: Option<String>,
    pub base_url: String,
    pub model: Option<String>,
    pub max_tokens: u64,
    pub timeout_seconds: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ApiProvider {
    OpenAiCompatible,
    DeepSeek,
}

impl Config {
    pub fn load(repo: &RepoPaths) -> Result<Self, ConfigError> {
        let env = env::vars_os()
            .filter_map(|(key, value)| {
                let key = key.into_string().ok()?;
                let value = value.into_string().ok()?;
                Some((key, value))
            })
            .collect::<BTreeMap<_, _>>();

        Self::from_env_map(repo, &env)
    }

    pub fn from_env_map(
        repo: &RepoPaths,
        process_env: &BTreeMap<String, String>,
    ) -> Result<Self, ConfigError> {
        let dotenv = ConfigEnvResolver::load_dotenv_map(&repo.dotenv_path())?;
        let merged = ConfigEnvResolver::merge_env(dotenv, process_env);

        let provider = ApiProvider::from_config_value(ConfigEnvResolver::resolve_optional_string(
            &merged,
            COURSEGEN_LLM_PROVIDER,
        ))?;
        let llm = LlmConfig::from_env(repo, &merged, provider)?;

        Ok(Self {
            mineru: MineruConfig {
                api_base_url: ConfigEnvResolver::resolve_string_with_default(
                    &merged,
                    MINERU_API_BASE_URL,
                    "https://mineru.net",
                ),
                token_file: ConfigEnvResolver::resolve_token_file(repo, &merged),
                model_version: ConfigEnvResolver::resolve_string_with_default(
                    &merged,
                    MINERU_MODEL_VERSION,
                    "vlm",
                ),
                language: ConfigEnvResolver::resolve_string_with_default(
                    &merged,
                    MINERU_LANGUAGE,
                    "ch",
                ),
                enable_formula: ConfigEnvResolver::resolve_bool_with_default(
                    &merged,
                    MINERU_ENABLE_FORMULA,
                    true,
                ),
                enable_table: ConfigEnvResolver::resolve_bool_with_default(
                    &merged,
                    MINERU_ENABLE_TABLE,
                    true,
                ),
                is_ocr: ConfigEnvResolver::resolve_bool_with_default(&merged, MINERU_IS_OCR, true),
                fail_on_empty_text: ConfigEnvResolver::resolve_bool_with_default(
                    &merged,
                    MINERU_FAIL_ON_EMPTY_TEXT,
                    false,
                ),
                request_timeout_seconds: ConfigEnvResolver::resolve_u64_with_default(
                    &merged,
                    MINERU_REQUEST_TIMEOUT_SECONDS,
                    120,
                )?,
                upload_timeout_seconds: ConfigEnvResolver::resolve_u64_with_default(
                    &merged,
                    MINERU_UPLOAD_TIMEOUT_SECONDS,
                    600,
                )?,
                result_timeout_seconds: ConfigEnvResolver::resolve_u64_with_default(
                    &merged,
                    MINERU_RESULT_TIMEOUT_SECONDS,
                    3600,
                )?,
                poll_interval_seconds: ConfigEnvResolver::resolve_u64_with_default(
                    &merged,
                    MINERU_POLL_INTERVAL_SECONDS,
                    15,
                )?,
                download_timeout_seconds: ConfigEnvResolver::resolve_u64_with_default(
                    &merged,
                    MINERU_DOWNLOAD_TIMEOUT_SECONDS,
                    600,
                )?,
                inline_token: ConfigEnvResolver::resolve_optional_string(&merged, MINERU_API_TOKEN),
            },
            llm,
        })
    }
}

impl MineruConfig {
    pub fn read_token(&self) -> Result<String, ConfigError> {
        if let Some(token) = &self.inline_token {
            return Ok(token.clone());
        }

        ConfigEnvResolver::read_trimmed_file(&self.token_file)
    }
}

impl LlmConfig {
    fn from_env(
        repo: &RepoPaths,
        env: &BTreeMap<String, String>,
        provider: ApiProvider,
    ) -> Result<Self, ConfigError> {
        match provider {
            ApiProvider::OpenAiCompatible => Ok(Self {
                provider,
                api_key: ConfigEnvResolver::resolve_first_string(
                    env,
                    &["COURSEGEN_LLM_API_KEY", "OPENAI_API_KEY"],
                ),
                base_url: ConfigEnvResolver::resolve_string_with_default_from_keys(
                    env,
                    &["COURSEGEN_LLM_BASE_URL", "OPENAI_BASE_URL"],
                    "https://api.openai.com/v1",
                ),
                model: ConfigEnvResolver::resolve_first_string(
                    env,
                    &["COURSEGEN_LLM_MODEL", "OPENAI_MODEL"],
                ),
                max_tokens: ConfigEnvResolver::resolve_u64_with_default_from_keys(
                    env,
                    &[COURSEGEN_LLM_MAX_TOKENS],
                    393_216,
                )?,
                timeout_seconds: ConfigEnvResolver::resolve_u64_with_default_from_keys(
                    env,
                    &[COURSEGEN_LLM_TIMEOUT_SECONDS],
                    3600,
                )?,
            }),
            ApiProvider::DeepSeek => Ok(Self {
                provider,
                api_key: match ConfigEnvResolver::resolve_first_string(
                    env,
                    &[DEEPSEEK_API_KEY, "COURSEGEN_LLM_API_KEY"],
                ) {
                    Some(api_key) => Some(api_key),
                    None => ConfigEnvResolver::read_deepseek_key_file(repo)?,
                },
                base_url: ConfigEnvResolver::resolve_string_with_default_from_keys(
                    env,
                    &[DEEPSEEK_BASE_URL, "COURSEGEN_LLM_BASE_URL"],
                    "https://api.deepseek.com",
                ),
                model: ConfigEnvResolver::resolve_first_string(
                    env,
                    &[DEEPSEEK_MODEL, "COURSEGEN_LLM_MODEL"],
                )
                .or_else(|| Some("deepseek-chat".to_owned())),
                max_tokens: ConfigEnvResolver::resolve_u64_with_default_from_keys(
                    env,
                    &[DEEPSEEK_MAX_TOKENS, COURSEGEN_LLM_MAX_TOKENS],
                    393_216,
                )?,
                timeout_seconds: ConfigEnvResolver::resolve_u64_with_default_from_keys(
                    env,
                    &[DEEPSEEK_TIMEOUT_SECONDS, COURSEGEN_LLM_TIMEOUT_SECONDS],
                    3600,
                )?,
            }),
        }
    }

    pub fn require_ready(&self) -> Result<LlmReadyConfig, ConfigError> {
        let api_key = self.api_key.clone().ok_or(ConfigError::MissingLlmApiKey)?;
        let model = self.model.clone().ok_or(ConfigError::MissingLlmModel)?;

        Ok(LlmReadyConfig {
            provider: self.provider,
            api_key,
            base_url: self.base_url.clone(),
            model,
            max_tokens: self.max_tokens,
            timeout_seconds: self.timeout_seconds,
        })
    }
}

impl ApiProvider {
    fn from_config_value(value: Option<String>) -> Result<Self, ConfigError> {
        let Some(value) = value else {
            return Ok(Self::OpenAiCompatible);
        };

        match value.to_ascii_lowercase().as_str() {
            "openai" | "openai-compatible" | "openai_compatible" => Ok(Self::OpenAiCompatible),
            "deepseek" => Ok(Self::DeepSeek),
            _ => Err(ConfigError::InvalidLlmProvider { value }),
        }
    }

    pub fn chat_completions_path(self) -> &'static str {
        match self {
            Self::OpenAiCompatible | Self::DeepSeek => "chat/completions",
        }
    }
}

#[derive(Clone, Debug)]
pub struct LlmReadyConfig {
    pub provider: ApiProvider,
    pub api_key: String,
    pub base_url: String,
    pub model: String,
    pub max_tokens: u64,
    pub timeout_seconds: u64,
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
    MissingMineruToken {
        path: PathBuf,
    },
    MineruTokenRead {
        path: PathBuf,
        source: io::Error,
    },
    InvalidLlmProvider {
        value: String,
    },
    MissingLlmApiKey,
    MissingLlmModel,
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
            Self::MissingMineruToken { path } => write!(
                f,
                "missing MinerU API token: set {MINERU_API_TOKEN} or provide a non-empty token file at {}",
                path.display()
            ),
            Self::MineruTokenRead { path, source } => {
                write!(f, "failed to read MinerU token file {}: {source}", path.display())
            }
            Self::InvalidLlmProvider { value } => write!(
                f,
                "invalid LLM provider '{value}': expected openai-compatible or deepseek"
            ),
            Self::MissingLlmApiKey => write!(
                f,
                "missing LLM API key: set COURSEGEN_LLM_API_KEY or OPENAI_API_KEY; for DeepSeek set COURSEGEN_LLM_PROVIDER=deepseek with DEEPSEEK_API_KEY or add DEEPSEEK: ... to {DEEPSEEK_API_KEYS_PATH}"
            ),
            Self::MissingLlmModel => write!(
                f,
                "missing LLM model: set COURSEGEN_LLM_MODEL or OPENAI_MODEL"
            ),
        }
    }
}

impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::DotenvRead { source, .. } => Some(source),
            Self::InvalidInteger { source, .. } => Some(source),
            Self::MineruTokenRead { source, .. } => Some(source),
            Self::InvalidLlmProvider { .. }
            | Self::MissingMineruToken { .. }
            | Self::MissingLlmApiKey
            | Self::MissingLlmModel => None,
        }
    }
}

struct ConfigEnvResolver;

impl ConfigEnvResolver {
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
            if !key.is_empty() {
                env.insert(
                    key.to_owned(),
                    Self::strip_surrounding_quotes(raw_value.trim()).to_owned(),
                );
            }
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

    fn resolve_token_file(repo: &RepoPaths, env: &BTreeMap<String, String>) -> PathBuf {
        let token_file = Self::resolve_optional_string(env, MINERU_TOKEN_FILE)
            .map(PathBuf::from)
            .unwrap_or_else(|| repo.default_mineru_token_file_path());

        if token_file.is_absolute() {
            token_file
        } else {
            repo.root().join(token_file)
        }
    }

    fn resolve_string_with_default(
        env: &BTreeMap<String, String>,
        key: &'static str,
        default: &str,
    ) -> String {
        Self::resolve_optional_string(env, key)
            .filter(|value| !value.is_empty())
            .unwrap_or_else(|| default.to_owned())
    }

    fn resolve_optional_string(env: &BTreeMap<String, String>, key: &str) -> Option<String> {
        env.get(key)
            .map(|value| value.trim().to_owned())
            .filter(|value| !value.is_empty())
    }

    fn resolve_first_string(env: &BTreeMap<String, String>, keys: &[&str]) -> Option<String> {
        keys.iter()
            .find_map(|key| Self::resolve_optional_string(env, key))
    }

    fn resolve_string_with_default_from_keys(
        env: &BTreeMap<String, String>,
        keys: &[&str],
        default: &str,
    ) -> String {
        Self::resolve_first_string(env, keys).unwrap_or_else(|| default.to_owned())
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

    fn resolve_u64_with_default_from_keys(
        env: &BTreeMap<String, String>,
        keys: &[&'static str],
        default: u64,
    ) -> Result<u64, ConfigError> {
        for key in keys {
            if env
                .get(*key)
                .map(|value| !value.trim().is_empty())
                .unwrap_or(false)
            {
                return Self::resolve_u64_with_default(env, key, default);
            }
        }

        Ok(default)
    }

    fn read_deepseek_key_file(repo: &RepoPaths) -> Result<Option<String>, ConfigError> {
        let path = repo.root().join(DEEPSEEK_API_KEYS_PATH);
        let contents = match fs::read_to_string(&path) {
            Ok(contents) => contents,
            Err(source) if source.kind() == io::ErrorKind::NotFound => return Ok(None),
            Err(source) => {
                return Err(ConfigError::DotenvRead { path, source });
            }
        };

        Ok(contents.lines().find_map(|line| {
            let (key, value) = line.split_once(':')?;
            if key.trim().eq_ignore_ascii_case("DEEPSEEK") {
                let value = value.trim();
                if !value.is_empty() {
                    return Some(value.to_owned());
                }
            }
            None
        }))
    }

    fn read_trimmed_file(path: &Path) -> Result<String, ConfigError> {
        match fs::read_to_string(path) {
            Ok(contents) => {
                let token = contents.trim().to_owned();
                if token.is_empty() {
                    Err(ConfigError::MissingMineruToken {
                        path: path.to_path_buf(),
                    })
                } else {
                    Ok(token)
                }
            }
            Err(source) if source.kind() == io::ErrorKind::NotFound => {
                Err(ConfigError::MissingMineruToken {
                    path: path.to_path_buf(),
                })
            }
            Err(source) => Err(ConfigError::MineruTokenRead {
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
}
