use crate::errors::{ApiError, Result};
use std::path::PathBuf;
use tokio::fs;
use tokio::process::Command;

pub struct PdfService;

impl PdfService {
    pub fn new() -> Self {
        Self
    }

    pub async fn generate_pdf(
        &self,
        latex_content: &str,
        output_path: &PathBuf,
    ) -> Result<Vec<u8>> {
        // Create a temporary directory for processing
        let temp_dir = std::env::temp_dir().join(format!("noteforge-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&temp_dir)
            .await
            .map_err(|e| ApiError::LaTeXError(format!("Failed to create temp dir: {}", e)))?;

        // Write LaTeX content to a temporary file
        let latex_path = temp_dir.join("output.tex");
        fs::write(&latex_path, latex_content)
            .await
            .map_err(|e| ApiError::LaTeXError(format!("Failed to write LaTeX file: {}", e)))?;

        // Run pdflatex
        let output = Command::new("pdflatex")
            .args([
                "-interaction=nonstopmode",
                "-output-directory",
                temp_dir.to_str().unwrap(),
                latex_path.to_str().unwrap(),
            ])
            .output()
            .await
            .map_err(|e| ApiError::LaTeXError(format!("Failed to run pdflatex: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            return Err(ApiError::LaTeXError(format!(
                "PDF generation failed: {}\n{}",
                stderr, stdout
            )));
        }

        // Read the generated PDF
        let pdf_path = temp_dir.join("output.pdf");
        let pdf_data = fs::read(&pdf_path)
            .await
            .map_err(|e| ApiError::LaTeXError(format!("Failed to read PDF file: {}", e)))?;

        // Write to output path if provided
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent).await.map_err(|e| {
                ApiError::FileError(format!("Failed to create output directory: {}", e))
            })?;

            fs::write(output_path, &pdf_data)
                .await
                .map_err(|e| ApiError::FileError(format!("Failed to write PDF file: {}", e)))?;
        }

        // Clean up temporary directory
        tokio::spawn(async move {
            let _ = fs::remove_dir_all(temp_dir).await;
        });

        Ok(pdf_data)
    }
}
