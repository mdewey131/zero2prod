# Use this version of Rust as the base image
FROM rust:1.65.0

# Switch our working directory to `app`. This folder will be created if it doesn't already exist
WORKDIR /app

# Install the required system dependencies for our linking configuration
RUN apt update && apt install lld clang -y 

# Copy all files from the working environment to the Docker image
COPY . . 

# Uses sqlx offline mode for query validation
ENV SQLX_OFFLINE true
# Build the binary using the release profile for SPEEEEED
RUN cargo build --release
ENV APP_ENVIRONMENT production
# When `docker run` is executed, launch the binary
ENTRYPOINT ["./target/release/zero2prod"]