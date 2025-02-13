use std::path::Path;

use tesseract::Tesseract;
use tracing::info;

use crate::errors::{ApiError, Result};

pub struct OcrService {
    tesseract: Option<Tesseract>,
}

impl OcrService {
    pub fn new() -> Result<Self> {
        let tesseract = Tesseract::new(None, Some("eng"))
            .map_err(|e| ApiError::OcrError(format!("Failed to initialize Tesseract: {}", e)))?;

        Ok(Self {
            tesseract: Some(tesseract),
        })
    }

    pub fn process_image(&mut self, image_path: &Path) -> Result<String> {
        info!("Processing image for OCR: {:?}", image_path);

        let path_str = image_path
            .to_str()
            .ok_or_else(|| ApiError::OcrError("Invalid path".to_string()))?;

        let mut tesseract = self
            .tesseract
            .take()
            .ok_or_else(|| ApiError::OcrError("Tesseract instance already in use".to_string()))?;

        let result = {
            let mut instance = tesseract
                .set_image(path_str)
                .map_err(|e| ApiError::OcrError(format!("Failed to set image: {}", e)))?;
            let text = instance
                .get_text()
                .map_err(|e| ApiError::OcrError(format!("Failed to get text: {}", e)))?;
            tesseract = instance;
            Ok::<_, ApiError>(text)
        };

        self.tesseract = Some(tesseract);

        result
    }
}
