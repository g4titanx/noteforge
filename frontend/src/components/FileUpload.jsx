import React, { useState } from 'react';
import { uploadFiles } from '../services/api';

function FileUpload({ onUploadSuccess, onError }) {
  const [files, setFiles] = useState([]);
  const [uploading, setUploading] = useState(false);
  const MAX_FILES = 5;

  const handleFileChange = (e) => {
    if (e.target.files) {
      const selectedFiles = Array.from(e.target.files);
      
      // Check if too many files are selected
      if (selectedFiles.length > MAX_FILES) {
        onError(`You can only upload up to ${MAX_FILES} images at once`);
        return;
      }

      setFiles(selectedFiles);
    }
  };

  const handleUpload = async () => {
    if (files.length === 0) {
      onError('Please select at least one file');
      return;
    }

    setUploading(true);
    
    try {
      // We'll treat single and multiple files the same way on the backend
      const isMultiPage = files.length > 1;
      const response = await uploadFiles(files);
      onUploadSuccess(response.file_id, isMultiPage);
    } catch (error) {
      onError('Upload failed: ' + error.message);
    } finally {
      setUploading(false);
    }
  };

  return (
    <div className="upload-container">
      <div className="file-input-container">
        <label className="file-input-label">
          select image files (max 5)
          <input
            type="file"
            accept="image/*"
            multiple
            onChange={handleFileChange}
            className="file-input"
          />
        </label>
        
        <div className="selected-files">
          {files.length > 0 ? (
            <>
              <p>{files.length} file(s) selected:</p>
              <ul>
                {files.map((file, index) => (
                  <li key={index}>{file.name}</li>
                ))}
              </ul>
            </>
          ) : (
            <p>no files selected</p>
          )}
        </div>
      </div>

      <div className="upload-info">
        {files.length > 1 && (
          <p className="multi-page-info">
            these images will be processed as pages of a single note, in the order shown above.
          </p>
        )}
      </div>

      <button 
        className="action-button" 
        onClick={handleUpload}
        disabled={uploading || files.length === 0}
      >
        {uploading ? 'Uploading...' : 'convert to LaTeX'}
      </button>
    </div>
  );
}

export default FileUpload;