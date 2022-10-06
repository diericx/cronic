docker buildx create --use
docker buildx build --push --platform=linux/amd64,linux/arm64 -t diericx/cronic:latest -t "diericx/cronic:$(git rev-parse HEAD)" -f ./build/Dockerfile .
