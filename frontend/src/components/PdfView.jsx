
import React, { useState, useEffect } from 'react';
import { getPdfUrl } from '../services/api';

function PdfView({ fileId, onBackClick, onBackToUpload, onError }) {
  const [pdfUrl, setPdfUrl] = useState('');
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const [errorDetails, setErrorDetails] = useState(null);

  useEffect(() => {
    setLoading(true);
    setError(null);
    setErrorDetails(null);
    
    // Generate the PDF URL directly from the fileId
    const url = getPdfUrl(fileId);
    setPdfUrl(url);
    
    // Test the URL - this will only complete after the PDF is generated
    fetch(url)
      .then(response => {
        if (!response.ok) {
          return response.json().then(err => {
            throw err;
          });
        }
        setLoading(false);
      })
      .catch(err => {
        setLoading(false);
        setError('Failed to generate PDF');
        // Check if the error has detailed information
        if (err.error && err.error.message) {
          // Look for LaTeX errors in the message
          const message = err.error.message;
          
          // Check for common LaTeX errors
          if (message.includes('LaTeX Error') || 
              message.includes('Undefined control sequence') ||
              message.includes('Emergency stop')) {
                
            // These are typical LaTeX errors
            setErrorDetails('There was an error in the LaTeX code. This might be due to complex mathematical notation or an environment that needs additional packages.');
          } else {
            setErrorDetails(message);
          }
        }
        onError('PDF generation failed');
      });
  }, [fileId, onError]);

  const handleRegeneratePdf = () => {
    setLoading(true);
    setError(null);
    setErrorDetails(null);
    // Reloading the page triggers regeneration
    window.location.reload();
  };

  // Render different content based on state
  const renderContent = () => {
    if (loading) {
      return <div className="loading">Generating PDF...</div>;
    }

    if (error) {
      return (
        <div className="pdf-error">
          <h3>{error}</h3>
          {errorDetails && (
            <div className="error-details">
              <p>{errorDetails}</p>
              
              {/* Only show if it's a LaTeX error */}
              {errorDetails.includes('LaTeX') && (
                <p>this typically happens with complex environments like theorems. 
                try editing the LaTeX to fix the issue or regenerate.</p>
              )}
            </div>
          )}
          <div className="action-buttons">
            <button 
              className="action-button" 
              onClick={handleRegeneratePdf}
            >
              regenerate PDF
            </button>
            <button 
              className="action-button" 
              onClick={onBackClick}
            >
              edit LaTeX
            </button>
            <button 
              className="action-button secondary" 
              onClick={onBackToUpload}
            >
              back to upload
            </button>
          </div>
        </div>
      );
    }

    return (
      <>
        <div className="pdf-viewer">
          <iframe
            src={pdfUrl}
            title="PDF Viewer"
            width="100%"
            height="100%"
            style={{ border: 'none' }}
          />
        </div>

        <div className="action-buttons">
          <button 
            className="action-button" 
            onClick={() => {
              const link = document.createElement('a');
              link.href = pdfUrl;
              link.download = `mathNotes-${fileId}.pdf`;
              document.body.appendChild(link);
              link.click();
              document.body.removeChild(link);
            }}
          >
            download PDF
          </button>
          <button 
            className="action-button" 
            onClick={onBackClick}
          >
            back to LaTeX
          </button>
          <button 
            className="action-button secondary" 
            onClick={onBackToUpload}
          >
            back to upload
          </button>
        </div>
      </>
    );
  };

  return (
    <div className="pdf-container">
      <h2>Generated PDF</h2>
      {renderContent()}
    </div>
  );
}

export default PdfView;
