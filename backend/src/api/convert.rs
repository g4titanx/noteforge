use crate::{
    config::env::Config,
    errors::{ApiError, Result},
    models::document::Document,
    services::{claude::ClaudeService, pdf::PdfService},
    utils::headers::HeaderMap,
};
use axum::{
    extract::{Path, Query},
    response::{IntoResponse, Response},
    Json,
};
use serde::Deserialize;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct ConvertParams {
    is_multi_page: bool,
}

// Store converted LaTeX for later PDF generation
async fn store_latex(file_id: &Uuid, content: &str) -> Result<()> {
    let latex_dir = PathBuf::from("latex");
    tokio::fs::create_dir_all(&latex_dir)
        .await
        .map_err(|e| ApiError::FileError(format!("Failed to create latex directory: {}", e)))?;

    let latex_path = latex_dir.join(format!("{}.tex", file_id));
    tokio::fs::write(&latex_path, content)
        .await
        .map_err(|e| ApiError::FileError(format!("Failed to write LaTeX file: {}", e)))?;

    Ok(())
}

// Retrieve stored LaTeX content
async fn get_latex(file_id: &Uuid) -> Result<String> {
    let latex_path = PathBuf::from("latex").join(format!("{}.tex", file_id));
    tokio::fs::read_to_string(&latex_path)
        .await
        .map_err(|e| ApiError::NotFound(format!("LaTeX file not found: {}", e)))
}

pub async fn convert_to_text(
    Path(file_id): Path<Uuid>,
    Query(params): Query<ConvertParams>,
) -> Result<Json<Document>> {
    let config = Config::from_env()?;
    let claude_service = ClaudeService::new(&config);

    let upload_dir = PathBuf::from("uploads");

    let content = if params.is_multi_page {
        let files: Vec<PathBuf> = std::fs::read_dir(&upload_dir)
            .map_err(|e| ApiError::FileError(format!("Failed to read directory: {}", e)))?
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry
                    .path()
                    .file_stem()
                    .and_then(|stem| stem.to_str())
                    .map(|name| name.starts_with(&file_id.to_string()))
                    .unwrap_or(false)
            })
            .map(|entry| entry.path())
            .collect();

        if files.is_empty() {
            return Err(ApiError::NotFound(format!(
                "No files found for ID {}",
                file_id
            )));
        }

        claude_service.convert_multiple_pages(&files).await?
    } else {
        let file_path = upload_dir.join(format!("{}.png", file_id));
        if !file_path.exists() {
            return Err(ApiError::NotFound(format!("File not found: {}", file_id)));
        }
        claude_service.convert_single_page(&file_path).await?
    };

    // Store the LaTeX content for later PDF generation
    store_latex(&file_id, &content).await?;

    Ok(Json(Document {
        id: file_id,
        filename: format!("{}.tex", file_id),
        content,
        created_at: chrono::Utc::now(),
    }))
}

pub async fn generate_pdf(Path(file_id): Path<Uuid>) -> Result<impl IntoResponse> {
    // Get the stored LaTeX content
    let latex_content = get_latex(&file_id).await?;

    // Generate PDF
    let pdf_service = PdfService::new();
    let output_dir = PathBuf::from("pdf");
    tokio::fs::create_dir_all(&output_dir)
        .await
        .map_err(|e| ApiError::FileError(format!("Failed to create PDF directory: {}", e)))?;

    let output_path = output_dir.join(format!("{}.pdf", file_id));
    let pdf_data = pdf_service
        .generate_pdf(&latex_content, &output_path)
        .await?;

    // Create response headers
    let mut headers = HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        "application/pdf".parse().unwrap(),
    );
    headers.insert(
        axum::http::header::CONTENT_DISPOSITION,
        format!("attachment; filename=\"{}.pdf\"", file_id)
            .parse()
            .unwrap(),
    );

    Ok((headers, pdf_data))
}
