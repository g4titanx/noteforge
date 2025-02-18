// use axum::Json;
// use serde::{Deserialize, Serialize};
// use crate::services::claude::ClaudeService;
// use crate::errors::Result;
// use crate::config::env::Config;

// #[derive(Deserialize)]
// pub struct TestRequest {
//     text: String,
// }

// #[derive(Serialize)]
// pub struct TestResponse {
//     latex: String,
// }

// pub async fn test_claude(
//     Json(payload): Json<TestRequest>,
// ) -> Result<Json<TestResponse>> {
//     let config = Config::from_env()?;
//     let claude_service = ClaudeService::new(&config);

//     let latex = claude_service.process_math_text(&payload.text).await?;

//     Ok(Json(TestResponse { latex }))
// }
