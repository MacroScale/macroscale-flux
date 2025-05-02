# !/bin/bash

PROJECT_ROOT=$(git rev-parse --show-toplevel)
docker run --rm -v "$PROJECT_ROOT:$PROJECT_ROOT" -w "$PROJECT_ROOT/captureInterface" -it flux-buildtools
