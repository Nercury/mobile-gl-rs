#!/usr/bin/env bash

cd deps/inl
/usr/local/bin/multirust override android

echo "building rust for platform ${1}"

BUILD_TYPE=""
if [ "$2" == "release" ]; then
    BUILD_TYPE="--release"
fi

/usr/local/bin/cargo build --target=${1} ${BUILD_TYPE} --verbose