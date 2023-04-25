#!/bin/bash
set -ex

DOCKER_IMG_NAME=${1:-"driver-did-sov"}
VERSION=${2:-"0.0.1"}

SCRIPT_DIR="$( cd "$(dirname "$0")" ; pwd -P )"
DOCKER_IMAGE_DRIVER_DID_SOV="$DOCKER_IMG_NAME:$VERSION"

docker build -f ./ci/Dockerfile.prod -t $DOCKER_IMAGE_DRIVER_DID_SOV $SCRIPT_DIR/..
docker-compose -f ./ci/docker-compose.yml up -d
