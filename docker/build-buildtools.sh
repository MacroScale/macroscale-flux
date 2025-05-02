PROJECT_ROOT=$(git rev-parse --show-toplevel)
docker buildx build -t flux-buildtools -f $PROJECT_ROOT/docker/buildtools.Dockerfile $PROJECT_ROOT
