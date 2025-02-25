FROM rust:latest as builder
WORKDIR /app
COPY . .

# Build with the modified Cargo.toml that doesn't require OCR libraries
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app

# Install only LaTeX dependencies
RUN apt-get update && apt-get install -y \
    texlive-latex-base \
    texlive-latex-extra \
    texlive-fonts-recommended \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/backend /app/backend

# Create directories needed by the app
RUN mkdir -p /app/uploads /app/latex /app/pdf

EXPOSE 3000
CMD ["/app/backend"]