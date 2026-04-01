#!/usr/bin/env bash

# fail asap
set -e

# Check if an argument was provided
if [ $# -eq 0 ]; then
    echo "No arguments provided"
    echo "Usage: scripts/publish-debug-image.sh 20230826-1 true"
    echo ""
    echo "Last argument specifies whether we should have a debug build as opposed to release build."
    exit 1
fi

DEBUG=$2
if [ "$DEBUG" = "true" ]; then
  echo "[profile.release]" >> Cargo.toml
  echo "debug = true" >> Cargo.toml
fi

TAG=$1-debug
echo "Building images, will tag for ghcr.io with $TAG!"
docker build -t legitservices/stoked-base:latest -f Dockerfile.useCurrentArch .
docker build -t legitservices/stoked-server:$TAG - < crates/delta/Dockerfile
docker build -t legitservices/stoked-bonfire:$TAG - < crates/bonfire/Dockerfile
docker build -t legitservices/stoked-autumn:$TAG - < crates/services/autumn/Dockerfile
docker build -t legitservices/stoked-january:$TAG - < crates/services/january/Dockerfile
docker build -t legitservices/stoked-gifbox:$TAG - < crates/services/gifbox/Dockerfile
docker build -t legitservices/stoked-crond:$TAG - < crates/daemons/crond/Dockerfile
docker build -t legitservices/stoked-pushd:$TAG - < crates/daemons/pushd/Dockerfile
docker build -t legitservices/stoked-voice-ingress:$TAG - < crates/daemons/voice-ingress/Dockerfile

if [ "$DEBUG" = "true" ]; then
  git restore Cargo.toml
fi

docker push legitservices/stoked-server:$TAG
docker push legitservices/stoked-bonfire:$TAG
docker push legitservices/stoked-autumn:$TAG
docker push legitservices/stoked-january:$TAG
docker push legitservices/stoked-gifbox:$TAG
docker push legitservices/stoked-crond:$TAG
docker push legitservices/stoked-pushd:$TAG
docker push legitservices/stoked-voice-ingress:$TAG
