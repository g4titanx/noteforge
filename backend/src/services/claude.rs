use crate::config::env::Config;
use crate::errors::{ApiError, Result};
use base64::{engine::general_purpose::STANDARD as base64, Engine};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, Serialize)]
struct ClaudeRequest {
    model: String,
    max_tokens: usize,
    messages: Vec<Message>,
}

#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: Vec<MessageContent>,
}

#[derive(Debug, Serialize)]
struct MessageContent {
    #[serde(rename = "type")]
    content_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<ImageSource>,
}

#[derive(Debug, Serialize)]
struct ImageSource {
    #[serde(rename = "type")]
    source_type: String,
    media_type: String,
    data: String,
}

#[derive(Debug, Deserialize)]
struct ClaudeResponse {
    id: String,
    #[serde(rename = "type")]
    message_type: String,
    role: String,
    content: Vec<ContentItem>,
}

#[derive(Debug, Deserialize)]
struct ContentItem {
    #[serde(rename = "type")]
    content_type: String,
    text: String,
}

enum PageType {
    Single,
    First,
    Middle,
    Last,
}

pub struct ClaudeService {
    client: Client,
    api_key: String,
    model: String,
}

impl ClaudeService {
    pub fn new(config: &Config) -> Self {
        Self {
            client: Client::new(),
            api_key: config.claude_api_key.clone(),
            model: "claude-3-5-sonnet-20241022".to_string(),
        }
    }

    pub async fn convert_single_page(&self, image_path: &Path) -> Result<String> {
        self.process_with_prompt(image_path, PageType::Single).await
    }

    pub async fn convert_multiple_pages(&self, image_paths: &[PathBuf]) -> Result<String> {
        let mut combined_latex = String::new();
        let paths_len = image_paths.len();

        for (index, path) in image_paths.iter().enumerate() {
            let page_type = match index {
                0 => PageType::First,
                i if i == paths_len - 1 => PageType::Last,
                _ => PageType::Middle,
            };

            let content = self.process_with_prompt(path, page_type).await?;
            if index > 0 {
                combined_latex.push_str("\\newpage\n");
            }
            combined_latex.push_str(&content);
        }

        Ok(combined_latex)
    }

    async fn process_with_prompt(&self, image_path: &Path, page_type: PageType) -> Result<String> {
        let image_data = tokio::fs::read(image_path)
            .await
            .map_err(|e| ApiError::FileError(format!("Failed to read image: {}", e)))?;

        let base64_image = base64.encode(image_data);

        let prompt = match page_type {
            PageType::Single => {
                "Convert this mathematical content to a complete LaTeX document:
                1. Document Structure:
                   - Must start with \\documentclass{article}
                   - Include necessary packages (amsmath, amssymb)
                   - Must have \\begin{document} and \\end{document}
                2. Mathematical Content:
                   - Use align* for equations
                   - Format all special symbols correctly
                   - Preserve spacing and layout
                Do not include ```latex or ``` markers. Return only the raw LaTeX code."
            }
            PageType::First => {
                "Convert this mathematical content to the start of a LaTeX document:
                1. Document Structure:
                   - Must start with \\documentclass{article}
                   - Include necessary packages
                   - Begin document
                2. Mathematical Content:
                   - Format equations properly
                Do not include \\end{document}.
                Do not include ```latex or ``` markers. Return only the raw LaTeX code."
            }
            PageType::Middle => {
                "Convert this mathematical content to LaTeX:
                Format all equations and preserve layout.
                Do not include preamble or \\end{document}.
                Do not include ```latex or ``` markers. Return only the raw LaTeX code."
            }
            PageType::Last => {
                "Convert this mathematical content to LaTeX:
                Format all equations and end with \\end{document}.
                Do not include ```latex or ``` markers. Return only the raw LaTeX code."
            }
        };

        let messages = vec![Message {
            role: "user".to_string(),
            content: vec![
                MessageContent {
                    content_type: "text".to_string(),
                    text: Some(prompt.to_string()),
                    source: None,
                },
                MessageContent {
                    content_type: "image".to_string(),
                    text: None,
                    source: Some(ImageSource {
                        source_type: "base64".to_string(),
                        media_type: "image/png".to_string(),
                        data: base64_image,
                    }),
                },
            ],
        }];

        let request = ClaudeRequest {
            model: self.model.clone(),
            max_tokens: 1024,
            messages,
        };

        let response = self
            .client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| ApiError::ClaudeError(format!("Failed to send request: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(ApiError::ClaudeError(format!(
                "API request failed: {}",
                error_text
            )));
        }

        let claude_response: ClaudeResponse = response
            .json()
            .await
            .map_err(|e| ApiError::ClaudeError(format!("Failed to parse response: {}", e)))?;

        let latex_content = claude_response
            .content
            .first()
            .ok_or_else(|| ApiError::ClaudeError("No content in response".to_string()))?
            .text
            .clone();

        Ok(latex_content)
    }
}
