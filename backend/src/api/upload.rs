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
const MAX_FILES: usize = 5;

pub async fn handle_upload(mut multipart: Multipart) -> Result<Json<serde_json::Value>> {
    // Ensure upload directory exists
    std::fs::create_dir_all(UPLOAD_DIR)
        .map_err(|e| ApiError::FileError(format!("Failed to create upload directory: {}", e)))?;

    // Generate a single file_id for this upload batch
    let file_id = Uuid::new_v4();
    let mut uploaded_files = Vec::new();
    let mut is_multi_page = false;
    let mut file_counter = 0;

    // Process each field in the multipart form
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| ApiError::FileError(format!("Failed to process multipart form: {}", e)))?
    {
        let field_name = field.name().unwrap_or("unknown").to_string();
        
        // Check if this is the is_multi_page flag
        if field_name == "is_multi_page" {
            let value = field.text().await
                .map_err(|e| ApiError::FileError(format!("Failed to read field data: {}", e)))?;
            is_multi_page = value == "true";
            continue;
        }
        
        // Limit number of files
        file_counter += 1;
        if file_counter > MAX_FILES {
            return Err(ApiError::ValidationError(
                format!("Too many files. Maximum is {}", MAX_FILES)
            ));
        }

        // Extract content type
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

        // Generate filename - for multi-page, we'll append a suffix
        let extension = mime_to_extension(&content_type)
            .ok_or_else(|| ApiError::ValidationError("Invalid mime type".to_string()))?;
        
        // For multi-page, name files with sequence: file_id_0.jpg, file_id_1.jpg, etc.
        let filename = if is_multi_page || file_counter > 1 {
            format!("{}_{}.{}", file_id, uploaded_files.len(), extension)
        } else {
            format!("{}.{}", file_id, extension)
        };
        
        let filepath = Path::new(UPLOAD_DIR).join(&filename);

        // Save file
        let mut file = std::fs::File::create(&filepath)
            .map_err(|e| ApiError::FileError(format!("Failed to create file: {}", e)))?;

        file.write_all(&data)
            .map_err(|e| ApiError::FileError(format!("Failed to write file: {}", e)))?;

        uploaded_files.push(filename.clone());
        info!("File uploaded successfully: {}", filename);
    }

    if uploaded_files.is_empty() {
        return Err(ApiError::ValidationError("No files provided".to_string()));
    }

    Ok(Json(json!({
        "status": "success",
        "file_id": file_id.to_string(),
        "filenames": uploaded_files,
        "is_multi_page": is_multi_page || uploaded_files.len() > 1
    })))
}

fn mime_to_extension(content_type: &str) -> Option<&str> {
    match content_type {
        "image/jpeg" => Some("jpg"),
        "image/png" => Some("png"),
        "image/webp" => Some("webp"),
        _ => None,
    }
}