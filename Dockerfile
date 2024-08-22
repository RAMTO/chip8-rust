# Use the official Rust image as the base image
FROM rust:latest AS builder

# Create a new directory for the app
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files to this directory
COPY Cargo.toml Cargo.lock ./

# This dummy build step will cache the dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Now copy the actual source code
COPY src ./src

# Build the application
RUN cargo build --release

# Use a newer Debian image that supports GLIBC 2.33
FROM rust:latest

# Copy the compiled binary from the builder image
COPY --from=builder /usr/src/app/target/release/chip8_emulator /usr/local/bin/chip8_emulator

# Set the startup command to run the binary
CMD ["sh", "-c", "chip8_emulator && tail -f /dev/null"]
