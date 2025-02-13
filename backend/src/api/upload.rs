use std::io::Write;
use std::path::Path;

use axum::extract::Multipart;
use axum::response::Json;
use serde_json::json;
use tracing::info;
use uuid::Uuid;

use crate::errors::{ApiError, Result};

const UPLOAD_DIR: &str = "uploads";
const MAX_FILE_SIZE: usize = 10 * 1024 * 1024; // 10MB
const ALLOWED_TYPES: [&str; 3] = ["image/jpeg", "image/png", "image/webp"]; // MIME types

pub async fn handle_upload(mut multipart: Multipart) -> Result<Json<serde_json::Value>> {
    // Ensure upload directory exists
    std::fs::create_dir_all(UPLOAD_DIR)
        .map_err(|e| ApiError::FileError(format!("Failed to create upload directory: {}", e)))?;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| ApiError::FileError(format!("Failed to process multipart form: {}", e)))?
    {
        // Extract and store content type as String before consuming field
        let content_type = field
            .content_type()
            .ok_or_else(|| ApiError::ValidationError("Missing content type".to_string()))?
            .to_string();

        // Validate file type
        if !ALLOWED_TYPES.contains(&content_type.as_str()) {
            return Err(ApiError::ValidationError(format!(
                "Unsupported file type: {}. Allowed types: {:?}",
                content_type, ALLOWED_TYPES
            )));
        }

        // Get file data
        let data = field
            .bytes()
            .await
            .map_err(|e| ApiError::FileError(format!("Failed to read file data: {}", e)))?;

        // Check file size
        if data.len() > MAX_FILE_SIZE {
            return Err(ApiError::ValidationError(format!(
                "File too large. Maximum size is {} bytes",
                MAX_FILE_SIZE
            )));
        }

        // Generate unique filename
        let file_id = Uuid::new_v4();
        let extension = mime_to_extension(&content_type)
            .ok_or_else(|| ApiError::ValidationError("Invalid mime type".to_string()))?;
        let filename = format!("{}.{}", file_id, extension);
        let filepath = Path::new(UPLOAD_DIR).join(&filename);

        // Save file
        let mut file = std::fs::File::create(&filepath)
            .map_err(|e| ApiError::FileError(format!("Failed to create file: {}", e)))?;

        file.write_all(&data)
            .map_err(|e| ApiError::FileError(format!("Failed to write file: {}", e)))?;

        info!("File uploaded successfully: {}", filename);
        return Ok(Json(json!({
            "status": "success",
            "file_id": file_id.to_string(),
            "filename": filename
        })));
    }

    Err(ApiError::ValidationError("No file provided".to_string()))
}

fn mime_to_extension(content_type: &str) -> Option<&str> {
    match content_type {
        "image/jpeg" => Some("jpg"),
        "image/png" => Some("png"),
        "image/webp" => Some("webp"),
        _ => None,
    }
}
