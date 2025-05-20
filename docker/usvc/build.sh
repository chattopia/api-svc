#!/usr/bin/env bash

set -e

REPOSITORY_ROOT="$(git rev-parse --show-toplevel)"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

VERSION=$(grep -v '^\s*$' "${SCRIPT_DIR}/VERSION" | head -n 1 | xargs)

IMAGE_NAME="chattopia/api-svc"
IMAGE_TAG="${IMAGE_NAME}:${VERSION}"

docker build -f "${SCRIPT_DIR}/Dockerfile" -t "${IMAGE_TAG}" \
             --label org.opencontainers.image.version="${VERSION}" \
             --label org.opencontainers.image.revision="$(git rev-parse HEAD)" \
             --label org.opencontainers.image.created="$(date -u +'%Y-%m-%dT%H:%M:%SZ')" \
             "${REPOSITORY_ROOT}"

