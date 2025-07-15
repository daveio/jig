# Build stage
FROM rust:1.88.0-alpine3.22 AS builder

# Install build dependencies
RUN apk add --no-cache \
  alpine-sdk=1.1-r0 \
  openssl-dev=3.5.1-r0 \
  perl-dev=5.40.2-r0

# Create a new empty project
WORKDIR /usr/src/jig
COPY . .

# Build the application in release mode
RUN cargo build --release

# Runtime stage
FROM alpine:3.22.0

# Install runtime dependencies if needed
# libgcc comes as standard with alpine

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
