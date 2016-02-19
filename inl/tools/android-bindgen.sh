#!/usr/bin/env bash

export DYLD_LIBRARY_PATH=/Library/Developer/CommandLineTools/usr/lib

PATH="/Users/nercury/Library/Android/sdk/ndk-bundle/platforms/android-21/arch-x86_64/usr"

export C_INCLUDE_PATH=${PATH}/include

# echo "#include <android/native_activity.h>" > gen.h
# /Users/nercury/.multirust/toolchains/nightly/cargo/bin/bindgen \
#     -match android/native_activity.h \
#     -o src/android/ffi/android/native_activity.rs \
#     gen.h

# echo "#include <android/input.h>" > gen.h
# /Users/nercury/.multirust/toolchains/nightly/cargo/bin/bindgen \
#     -match android/input.h \
#     -o src/android/ffi/android/input.rs \
#     gen.h

# echo "#include <android/looper.h>" > gen.h
# /Users/nercury/.multirust/toolchains/nightly/cargo/bin/bindgen \
#     -match android/looper.h \
#     -o src/android/ffi/android/looper.rs \
#     gen.h

#echo "#include <android/window.h>" > gen.h
#/Users/nercury/.multirust/toolchains/nightly/cargo/bin/bindgen \
#    -match android/window.h \
#    -o src/android/ffi/android/window.rs \
#    gen.h

echo "#include <android/log.h>" > gen.h
/Users/nercury/.multirust/toolchains/nightly/cargo/bin/bindgen \
    -match android/log.h \
    -o src/android/ffi/android/log.rs \
    gen.h

# echo "#include <android/native_window.h>" > gen.h
# /Users/nercury/.multirust/toolchains/nightly/cargo/bin/bindgen \
#     -match native_window.h \
#     -o src/android/ffi/android/native_window.rs \
#     gen.h

# echo "#include <android/rect.h>" > gen.h
# /Users/nercury/.multirust/toolchains/nightly/cargo/bin/bindgen \
#     -match android/rect.h \
#     -o src/android/ffi/android/rect.rs \
#     gen.h

# echo "#include <android/asset_manager.h>" > gen.h
# /Users/nercury/.multirust/toolchains/nightly/cargo/bin/bindgen \
#     -match android/asset_manager.h \
#     -o src/android/ffi/android/asset_manager.rs \
#     gen.h

# echo "#include <jni.h>" > gen.h
# /Users/nercury/.multirust/toolchains/nightly/cargo/bin/bindgen \
#     -match jni.h \
#     -o src/android/ffi/jni.rs \
#     gen.h
