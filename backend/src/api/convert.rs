use std::path::PathBuf;

use axum::extract::Path;
use axum::response::Json;
use chrono::Utc;
use uuid::Uuid;

use crate::errors::{ApiError, Result};
use crate::models::document::Document;
use crate::services::ocr::OcrService;

pub async fn convert_to_text(Path(file_id): Path<Uuid>) -> Result<Json<Document>> {
    let upload_dir = PathBuf::from("uploads");

    // Find the uploaded file
    let entries = std::fs::read_dir(&upload_dir)
        .map_err(|e| ApiError::FileError(format!("Failed to read upload directory: {}", e)))?;

    let file_path = entries
        .filter_map(|entry| entry.ok())
        .find(|entry| {
            entry
                .path()
                .file_stem()
                .and_then(|stem| stem.to_str())
                .map(|name| name == file_id.to_string())
                .unwrap_or(false)
        })
        .ok_or_else(|| ApiError::NotFound(format!("File with ID {} not found", file_id)))?
        .path();

    // Initialize OCR service and process the image
    let mut ocr_service = OcrService::new()?;
    let content = ocr_service.process_image(&file_path)?;

    let document = Document {
        id: file_id,
        filename: file_path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| ApiError::FileError("Invalid filename".to_string()))?
            .to_string(),
        content,
        created_at: Utc::now(),
    };

    Ok(Json(document))
}
