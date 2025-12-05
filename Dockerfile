# Multi-architecture Dockerfile using prebuilt binaries
FROM --platform=$BUILDPLATFORM alpine:3.23.0 AS binary-selector

# Map platform to target architecture
ARG TARGETPLATFORM
RUN case "$TARGETPLATFORM" in \
  "linux/amd64") echo "x86_64-unknown-linux-gnu" > /tmp/target ;; \
  "linux/arm64") echo "aarch64-unknown-linux-gnu" > /tmp/target ;; \
  *) echo "Unsupported platform: $TARGETPLATFORM" && exit 1 ;; \
  esac

# Copy all binaries from the build context
COPY docker-context/ /binaries/

# Select the correct binary for the target platform
RUN TARGET=$(cat /tmp/target) && \
    echo "Selecting binary for target: $TARGET" && \
    ls -la /binaries/ && \
    cp "/binaries/binary-$TARGET/jig" /jig && \
    chmod +x /jig

# Runtime stage
FROM alpine:3.23.0

# Install runtime dependencies
# gcompat allows us to run glibc stuff on Alpine
RUN apk add --no-cache gcompat=1.1.0-r4 libgcc=14.2.0-r6 && \
  rm -rf /var/cache/apk/*

# Create non-root user
RUN addgroup -g 1001 jig && \
    adduser -D -u 1001 -G jig jig

# Copy the binary from the selector stage
COPY --from=binary-selector /jig /jig
RUN chown jig:jig /jig && chmod +x /jig

USER jig
WORKDIR /home/jig

HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD test -x /jig || exit 1

# Set the binary as the entrypoint
ENTRYPOINT ["/jig"]
# Default command arguments if none provided
CMD ["--help"]
