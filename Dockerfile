# Build stage
FROM rust:1.88.0-alpine3.22 AS builder

# Install build dependencies
RUN apk add --no-cache musl-dev openssl-dev perl-dev alpine-sdk

# Create a new empty project
WORKDIR /usr/src/jig
COPY . .

# Build the application in release mode
RUN cargo build --release

# Runtime stage
FROM alpine:3.22.0

# Install runtime dependencies if needed
RUN apk add --no-cache libgcc

# Create non-root user
RUN addgroup -g 1001 jig && \
    adduser -D -u 1001 -G jig jig

# Copy the binary from the build stage
COPY --from=builder /usr/src/jig/target/release/jig /jig
RUN chown jig:jig /jig && chmod +x /jig

USER jig
WORKDIR /home/jig

HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD test -x /jig || exit 1

# Set the binary as the entrypoint
ENTRYPOINT ["/jig"]
# Default command arguments if none provided
# Note: The application requires a path argument, so we provide /home/jig as default
CMD ["--help"]
