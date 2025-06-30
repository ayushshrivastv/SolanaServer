# Use the official Rust image as the base
FROM rust:1.75 as builder

# Set the working directory
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Build the application
RUN cargo build --release

# Use a minimal image for the runtime
FROM debian:bookworm-slim

# Install OpenSSL and CA certificates
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/solana-http-server /usr/local/bin/solana-http-server

# Expose the port
EXPOSE 3000

# Set environment variable for port
ENV PORT=3000

# Run the application
CMD ["solana-http-server"] 
