const API_BASE_URL = 'https://backend-billowing-waterfall-6116.fly.dev';

export async function uploadFiles(files) {
  const formData = new FormData();
  
  // If we have multiple files, append each one with a unique identifier
  if (files.length > 1) {
    files.forEach((file, index) => {
      formData.append(`file_${index}`, file);
    });
    // Add a flag to indicate this is a multi-page upload
    formData.append('is_multi_page', 'true');
  } else {
    // Single file upload
    formData.append('file', files[0]);
    formData.append('is_multi_page', 'false');
  }

  const response = await fetch(`${API_BASE_URL}/upload`, {
    method: 'POST',
    body: formData,
  });

  if (!response.ok) {
    const errorData = await response.json();
    throw new Error(errorData.error?.message || 'Upload failed');
  }

  return response.json();
}

export async function convertToLatex(fileId, isMultiPage) {
  const response = await fetch(
    `${API_BASE_URL}/convert/${fileId}?is_multi_page=${isMultiPage}`, 
    {
      method: 'GET',
    }
  );

  if (!response.ok) {
    const errorData = await response.json();
    throw new Error(errorData.error?.message || 'Conversion failed');
  }

  return response.json();
}

export function getPdfUrl(fileId) {
  return `${API_BASE_URL}/pdf/${fileId}`;
}