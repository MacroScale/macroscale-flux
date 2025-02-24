# !/bin/bash

function build(){
    cmake -S . -B build
    cmake --build build
}

PROJECT_ROOT=$(git rev-parse --show-toplevel)
docker run --rm -v "$PROJECT_ROOT:$PROJECT_ROOT" -w "$PROJECT_ROOT/captureInterface" -it flux-buildtools bash -c "$(declare -f build); build"
