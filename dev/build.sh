#!/usr/bin/env bash

cd deps/inl
/usr/local/bin/multirust override android

echo "building rust for platform ${1}"
/usr/local/bin/cargo build --target=${1} --verbose