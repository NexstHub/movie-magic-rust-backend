# Use the official Rust image to build the project
FROM rust:latest AS builder

# Set the working directory
WORKDIR /usr/src/app

# Copy the project files into the container
COPY . .

# Build the application in release mode
RUN cargo build --release

# Use a smaller, minimal base image for the final container
FROM debian:bookworm-slim

# Copy the compiled binary from the builder image
COPY --from=builder /usr/src/app/target/release/rust-backend /usr/local/bin/rust-backend

# Install dependencies for running the application
RUN apt-get update && apt-get install -y libssl-dev ca-certificates libc6 curl && rm -rf /var/lib/apt/lists/*

# Expose the port the backend will run on
EXPOSE 3000

# Run the binary
CMD ["rust-backend"]

