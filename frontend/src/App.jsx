// src/App.jsx
import React, { useState } from 'react';
import FileUpload from './components/FileUpload';
import LatexView from './components/LatexView';
import PdfView from './components/PdfView';
import ErrorMessage from './components/ErrorMessage';

function App() {
  const [currentStep, setCurrentStep] = useState('upload');
  const [fileId, setFileId] = useState(null);
  const [latex, setLatex] = useState('');
  const [isMultiPage, setIsMultiPage] = useState(false);
  const [error, setError] = useState('');

  const handleUploadSuccess = (id, isMulti) => {
    setFileId(id);
    setIsMultiPage(isMulti);
    setCurrentStep('latex');
    setError('');
  };

  const handleLatexGenerated = (latexCode) => {
    setLatex(latexCode);
  };

  const handleBackToUpload = () => {
    setCurrentStep('upload');
    setFileId(null);
    setLatex('');
    setError('');
  };

  const handlePdfGeneration = () => {
    setCurrentStep('pdf');
    setError('');
  };

  const handleError = (message) => {
    setError(message);
  };

  return (
    <div className="app-container">
      <header>
        <h1>noteforge</h1>
        <p>converts hand-written math notes to latex and then to PDF</p>
      </header>

      {error && <ErrorMessage message={error} />}

      {currentStep === 'upload' && (
        <FileUpload 
          onUploadSuccess={handleUploadSuccess} 
          onError={handleError}
        />
      )}

      {currentStep === 'latex' && (
        <LatexView
          fileId={fileId}
          isMultiPage={isMultiPage}
          onLatexGenerated={handleLatexGenerated}
          onBackClick={handleBackToUpload}
          onGeneratePdf={handlePdfGeneration}
          onError={handleError}
        />
      )}

      {currentStep === 'pdf' && (
        <PdfView
          fileId={fileId}
          onBackClick={() => setCurrentStep('latex')}
          onBackToUpload={handleBackToUpload}
          onError={handleError}
        />
      )}
    </div>
  );
}

export default App;