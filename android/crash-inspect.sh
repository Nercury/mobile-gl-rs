#!/usr/bin/env bash

ADB="/Users/nercury/Library/Android/sdk/platform-tools/adb"
NDKSTACK="/Users/nercury/Library/Android/sdk/ndk-bundle/ndk-stack"
PROJECTROOT="`pwd`/android"

${ADB} logcat | ${NDKSTACK} -sym ${PROJECTROOT}