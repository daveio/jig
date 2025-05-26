.PHONY: build test clean lint fmt vet install

BINARY_NAME=hubbit
GO=go
GOFLAGS=-v

build:
	$(GO) build $(GOFLAGS) -o $(BINARY_NAME) .

test:
	$(GO) test $(GOFLAGS) -race -coverprofile=coverage.txt -covermode=atomic ./...

clean:
	$(GO) clean
	rm -f $(BINARY_NAME)
	rm -f coverage.txt

lint:
	golangci-lint run

fmt:
	$(GO) fmt ./...

vet:
	$(GO) vet ./...

install:
	$(GO) install $(GOFLAGS) .

deps:
	$(GO) mod download
	$(GO) mod tidy

check: fmt vet lint test

all: clean deps build test