# Use the official Rust image as the build environment
FROM rust:1.87 AS builder

# Set the working directory inside the container
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies (this will be cached if Cargo.toml doesn't change)
RUN cargo build --release
RUN rm src/main.rs

# Copy the actual source code
COPY src ./src

# Build the application with optimizations
RUN cargo build --release --locked

# Use a smaller base image for the runtime
FROM debian:bookworm-slim

# Install runtime dependencies and security updates
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && apt-get upgrade -y \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

# Create a non-root user for security
RUN groupadd -r appgroup && useradd -r -g appgroup -s /bin/false appuser

# Set the working directory
WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/log-server /app/log-server

# Create a directory for logs and set proper permissions
RUN mkdir -p /app/logs \
    && chown -R appuser:appgroup /app \
    && chmod 755 /app \
    && chmod 755 /app/logs

# Switch to the non-root user
USER appuser

# Expose the port the app runs on
EXPOSE 8080

# Create a volume for persistent log storage
VOLUME ["/app/logs"]

# Set environment variables
ENV RUST_LOG=info \
    LOG_PATH=/app/logs/server.log

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Run the application
CMD ["./log-server"]
