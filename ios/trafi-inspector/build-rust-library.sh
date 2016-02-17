#!/bin/bash

cd ../inl
multirust override ios

LIB=libinl.a

if [ "$OPTIMIZATION_LEVEL"=="0" ]; then
    echo "building debug version"
    FLAGS=""
else
    echo "building release version"
    FLAGS="--release"
fi

cargo build ${FLAGS} --target i386-apple-ios
cargo build ${FLAGS} --target x86_64-apple-ios
cargo build ${FLAGS} --target armv7-apple-ios
cargo build ${FLAGS} --target armv7s-apple-ios
cargo build ${FLAGS} --target aarch64-apple-ios

lipo -create -o target/ios-combined-${LIB} \
    target/i386-apple-ios/debug/${LIB} \
    target/x86_64-apple-ios/debug/${LIB} \
    target/armv7-apple-ios/debug/${LIB} \
    target/armv7s-apple-ios/debug/${LIB} \
    target/aarch64-apple-ios/debug/${LIB}
