FROM alpine:3.22.0

RUN addgroup -g 1001 jig && \
	adduser -D -u 1001 -G jig jig

USER jig

HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
	CMD echo "healthy" || exit 1

CMD ["echo", "Hello, World!"]
