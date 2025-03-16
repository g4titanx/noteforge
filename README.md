# noteforge
converts hand-written math notes in (jpeg, png, webp formats) -> latex -> PDF (or some other doc format, in a later version)

## basic-system architecture

Client (Browser)
   |
   | (HTTP/WebSocket)
   |
Server (Rust)
   |
   |---> PDF Processing Module
   |---> OCR Service (external API) (havent decided if this is necessary or not)
   |---> Claude API Integration
   |---> LaTeX Generation Module
   |---> PDF geenration
   
eventually i aim to have 
- backend structure with basic endpoints and
- simple frontend for file uploads and displaying results

Single Page:
Upload → Convert → Preview LaTeX → Generate PDF → Download

Multiple Pages:
Upload All Pages → Convert Sequentially → Combine LaTeX → Preview → Generate PDF → Download

Frontend:
Upload interface with single/multi page toggle
Preview window for LaTeX
"Generate PDF" button

Backend:

Modified upload endpoint to handle multiple files
Different prompts for single vs multi-page
PDF generation endpoint
