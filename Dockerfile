FROM golang:1.24-alpine AS builder

WORKDIR /app

# Copy go.mod and go.sum files
COPY go.mod go.sum ./

# Download dependencies
RUN go mod download

# Copy the source code
COPY . .

# Build the application
RUN CGO_ENABLED=0 GOOS=linux go build -o /belt -ldflags="-s -w -X github.com/daveio/belt/src/internal/version.Version=$(git describe --tags --always) -X github.com/daveio/belt/src/internal/version.Commit=$(git rev-parse HEAD) -X github.com/daveio/belt/src/internal/version.Date=$(date -u +%Y-%m-%dT%H:%M:%SZ)" ./src

# Use a minimal alpine image for the final stage
FROM alpine:latest

# Install CA certificates for HTTPS requests
RUN apk --no-cache add ca-certificates

WORKDIR /root/

# Copy the binary from the builder stage
COPY --from=builder /belt /usr/local/bin/belt

# Create a directory for belt configuration
RUN mkdir -p /root/.config/belt

# Set the entrypoint
ENTRYPOINT ["belt"]

# Default command (shows help)
CMD ["--help"]
