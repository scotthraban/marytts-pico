docker buildx build \
  --platform linux/arm64/v8,linux/amd64 \
  --push \
  --tag docker-registry.hraban.com/marytts-pico:latest \
  --tag docker-registry.hraban.com/marytts-pico:1.5 \
  .
