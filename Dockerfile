# Using cargo-chef to help speed up build times
FROM lukemathwalker/cargo-chef:latest-rust-1.65.0 as chef 
WORKDIR /app 
RUN apt update && install lld clang -y

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
# bulid our project dependencies, but not our application
RUN cargo chef cook --release --recipe-path recipe.json 
# up to this point, if our dependencies stay the same, then all layers should be cached
# Use this version of Rust as the base image
COPY . .

# Uses sqlx offline mode for query validation
ENV SQLX_OFFLINE true
# Build the binary using the release profile for SPEEEEED
RUN cargo build --release --bin zero2prod

# Runtime stage now
FROM debian:bullseye-slim AS runtime

WORKDIR /app 
# Install OpenSSL - it is dynaamically linked by some of our dependencies
# Install ca-certificates - it is needed to verify TLS certificates
# when establishing https connections
RUN apt-get update -y \
    && apt-get install --no-install-recommends openssl ca-certificates \ 
    # clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
# Copy the compiled binary from the builder environment
COPY --from=builder /app/target/release/zero2prod zero2prod
# Grab the config file at runtime
COPY configuration configuration 
ENV APP_ENVIRONMENT production
# When `docker run` is executed, launch the binary
ENTRYPOINT ["./zero2prod"]