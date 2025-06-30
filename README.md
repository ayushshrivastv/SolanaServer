# Solana HTTP Server

A Rust-based HTTP server providing Solana blockchain functionality through REST API endpoints.

## Endpoints

### POST /keypair
Generate a new Solana keypair.

### POST /message/sign
Sign a message using a private key.

### POST /message/verify
Verify a signed message.

### POST /token/create
Create a new SPL token initialize mint instruction.

### POST /token/mint
Create a mint-to instruction for SPL tokens.

### POST /send/sol
Create a SOL transfer instruction.

### POST /send/token
Create an SPL token transfer instruction.

## Development

```bash
cargo run
```

## Docker

```bash
docker build -t solana-http-server .
docker run -p 3000:3000 solana-http-server
```

## Deployment

Deploy to Railway, Render, or any container platform that supports Dockerfile.

Environment variable: `PORT` (defaults to 3000) 
# Force deployment - Mon Jun 30 22:59:57 IST 2025
