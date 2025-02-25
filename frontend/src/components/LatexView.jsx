import React, { useState, useEffect } from 'react';
import { convertToLatex } from '../services/api';

function LatexView({ 
  fileId, 
  isMultiPage, 
  onLatexGenerated, 
  onBackClick, 
  onGeneratePdf,
  onError 
}) {
  const [latex, setLatex] = useState('');
  const [loading, setLoading] = useState(true);
  const [editing, setEditing] = useState(false);
  const [editedLatex, setEditedLatex] = useState('');
  
  const [hasFetched, setHasFetched] = useState(false);

  useEffect(() => {
    // Only run once after initial mount
    if (hasFetched) return;
    
    async function fetchLatex() {
      try {
        const response = await convertToLatex(fileId, isMultiPage);
        setLatex(response.content);
        setEditedLatex(response.content);
        setLoading(false);
        
        // Mark as fetched BEFORE calling the callback to prevent loops
        setHasFetched(true);
        
        // Now call the callback
        onLatexGenerated(response.content);
      } catch (error) {
        onError('Failed to convert: ' + error.message);
        setLoading(false);
        setHasFetched(true);
      }
    }

    fetchLatex();
  }, [hasFetched]); // Only depend on hasFetched

  const handleCopyLatex = () => {
    navigator.clipboard.writeText(editing ? editedLatex : latex)
      .then(() => {
        alert('LaTeX code copied to clipboard');
      })
      .catch(err => {
        onError('Failed to copy: ' + err.message);
      });
  };

  const handleSaveEdit = () => {
    setLatex(editedLatex);
    
    // We can still call this on user actions
    onLatexGenerated(editedLatex);
    
    setEditing(false);
  };

  const handleCancelEdit = () => {
    setEditedLatex(latex);
    setEditing(false);
  };

  return (
    <div className="latex-container">
      <h2>generated LaTeX</h2>
      
      {loading ? (
        <div className="loading">Converting to LaTeX...</div>
      ) : (
        <>
          <div className="latex-content">
            {editing ? (
              <textarea
                value={editedLatex}
                onChange={(e) => setEditedLatex(e.target.value)}
                className="latex-editor"
              />
            ) : (
              <pre className="latex-display">{latex}</pre>
            )}
          </div>

          <div className="action-buttons">
            {editing ? (
              <>
                <button 
                  className="action-button" 
                  onClick={handleSaveEdit}
                >
                  save changes
                </button>
                <button 
                  className="action-button secondary" 
                  onClick={handleCancelEdit}
                >
                  cancel
                </button>
              </>
            ) : (
              <>
                <button 
                  className="action-button" 
                  onClick={() => setEditing(true)}
                >
                  edit LaTeX
                </button>
                <button 
                  className="action-button" 
                  onClick={handleCopyLatex}
                >
                  copy LaTeX
                </button>
                <button 
                  className="action-button" 
                  onClick={onGeneratePdf}
                >
                  generate PDF
                </button>
                <button 
                  className="action-button secondary" 
                  onClick={onBackClick}
                >
                  back to upload
                </button>
              </>
            )}
          </div>
        </>
      )}
    </div>
  );
}

export default LatexView;