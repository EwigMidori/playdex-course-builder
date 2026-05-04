use std::time::Duration;

use reqwest::blocking::Client;
use serde::Serialize;
use serde_json::Value;

use crate::config::LlmReadyConfig;

pub struct LlmClient<'a> {
    config: &'a LlmReadyConfig,
}

impl<'a> LlmClient<'a> {
    pub fn new(config: &'a LlmReadyConfig) -> Self {
        Self { config }
    }

    pub fn request(
        &'a self,
        system: &'a str,
        user: &'a str,
        temperature: f32,
    ) -> ChatCompletionRequest<'a> {
        ChatCompletionRequest {
            model: &self.config.model,
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
            temperature,
            max_tokens: self.config.max_tokens,
        }
    }

    pub fn send(
        &self,
        request: &ChatCompletionRequest<'_>,
    ) -> Result<ChatCompletionResponse, LlmClientError> {
        let client = Client::builder()
            .http1_only()
            .timeout(Duration::from_secs(self.config.timeout_seconds))
            .build()
            .map_err(LlmClientError::Request)?;
        let url = format!(
            "{}/{}",
            self.config.base_url.trim_end_matches('/'),
            self.config.provider.chat_completions_path()
        );
        let response = client
            .post(url)
            .header("Accept-Encoding", "identity")
            .bearer_auth(&self.config.api_key)
            .json(request)
            .send()
            .map_err(LlmClientError::Request)?;
        let status = response.status();
        let raw_bytes = response.bytes().map_err(LlmClientError::Request)?;
        let raw_text = String::from_utf8_lossy(&raw_bytes).into_owned();
        if !status.is_success() {
            return Err(LlmClientError::Status {
                status: status.as_u16(),
                body: raw_text,
            });
        }

        let raw_json: Value = serde_json::from_str(&raw_text).map_err(LlmClientError::Json)?;
        let content = raw_json
            .pointer("/choices/0/message/content")
            .and_then(Value::as_str)
            .or_else(|| raw_json.pointer("/choices/0/text").and_then(Value::as_str))
            .ok_or(LlmClientError::MissingContent)?
            .to_owned();

        Ok(ChatCompletionResponse {
            raw_text,
            raw_json,
            content,
        })
    }
}

#[derive(Serialize)]
pub struct ChatCompletionRequest<'a> {
    model: &'a str,
    messages: Vec<ChatMessage<'a>>,
    temperature: f32,
    max_tokens: u64,
}

#[derive(Serialize)]
struct ChatMessage<'a> {
    role: &'a str,
    content: &'a str,
}

pub struct ChatCompletionResponse {
    pub raw_text: String,
    pub raw_json: Value,
    pub content: String,
}

pub enum LlmClientError {
    Request(reqwest::Error),
    Status { status: u16, body: String },
    Json(serde_json::Error),
    MissingContent,
}
