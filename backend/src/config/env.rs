use crate::errors::{ApiError, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub claude_api_key: String,
    // ... other config fields
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenv::dotenv().ok();

        let claude_api_key = std::env::var("CLAUDE_API_KEY").map_err(|_| {
            ApiError::InternalServerError(anyhow::anyhow!("CLAUDE_API_KEY not set"))
        })?;

        Ok(Config {
            claude_api_key,
            // ... other fields
        })
    }
}
