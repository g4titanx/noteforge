use crate::errors::{ApiError, Result};
use std::path::PathBuf;
use tectonic::driver::ProcessingSessionBuilder;
use tectonic::status::termcolor::ColorChoice;
use tokio::fs;

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
        let temp_dir = tempfile::Builder::new()
            .prefix("noteforge-latex")
            .tempdir()
            .map_err(|e| ApiError::LaTeXError(format!("Failed to create temp dir: {}", e)))?;

        // Write LaTeX content to a temporary file
        let latex_path = temp_dir.path().join("output.tex");
        fs::write(&latex_path, latex_content)
            .await
            .map_err(|e| ApiError::LaTeXError(format!("Failed to write LaTeX file: {}", e)))?;

        // Set up Tectonic processing
        let auto_create_config_file = false;
        let only_cached = false;
        let bundle = tectonic::Bundle::new(only_cached).map_err(|e| {
            ApiError::LaTeXError(format!("Failed to initialize Tectonic bundle: {}", e))
        })?;

        let format_cache_path = tectonic::FormatCache::default();

        // Create processing session
        let mut session = ProcessingSessionBuilder::new()
            .bundle(bundle)
            .primary_input_path(latex_path.to_str().unwrap())
            .format_cache_path(format_cache_path)
            .keep_logs(false)
            .keep_intermediates(false)
            .do_not_write_output_files()
            .build()
            .map_err(|e| {
                ApiError::LaTeXError(format!("Failed to create processing session: {}", e))
            })?;

        // Run the processing
        if let Err(e) = session.run(auto_create_config_file) {
            return Err(ApiError::LaTeXError(format!(
                "PDF generation failed: {}",
                e
            )));
        }

        // Get the PDF data
        let pdf_data = session
            .get_pdf_data()
            .map_err(|e| ApiError::LaTeXError(format!("Failed to get PDF data: {}", e)))?
            .ok_or_else(|| ApiError::LaTeXError("No PDF data generated".to_string()))?;

        // Write to output path if provided
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent).await.map_err(|e| {
                ApiError::FileError(format!("Failed to create output directory: {}", e))
            })?;

            fs::write(output_path, &pdf_data)
                .await
                .map_err(|e| ApiError::FileError(format!("Failed to write PDF file: {}", e)))?;
        }

        Ok(pdf_data)
    }
}
