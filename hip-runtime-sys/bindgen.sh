#!/bin/bash

set -eu
set -x # debugging

# Update the Rust bindings to local HIP runtime. This script must be run
# whenever the GPU code changes.

# This script requires bindgen. This can be provided by a package manager or
# installed with "cargo install bindgen-cli".

# https://stackoverflow.com/questions/4774054/reliable-way-for-a-bash-script-to-get-the-full-path-to-itself
SCRIPTPATH="$(cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P)"

# build up extra arguments from checking rocm path.
export extra="${extra:-}"

function check_libclang_path() {
    local path=$1
    [ -z "$path" ] && export LAST_ERR="LIBCLANG_PATH not set" && return 1
    [ ! -d "$path" ] && export LAST_ERR="LIBCLANG_PATH not found: $path" && return 1
    [ ! -f "$path/libclang.so" ] && export LAST_ERR="libclang.so not found: $path/libclang.so" && return 1
    [ ! -x "$path/libclang.so" ] && export LAST_ERR="warning: libclang.so not executable: $path/libclang.so"
    return 0
}

function check_hip_path() {
    local path=$1
    [ -z "$path" ] && export LAST_ERR="HIP_PATH not set" && return 1
    [ ! -d "$path" ] && export LAST_ERR="HIP_PATH not found: $path" && return 1
    [ ! -f "$path/bin/hipcc" ] && export LAST_ERR="hipcc not found: $path/bin/hipcc" && return 1
    [ ! -x "$path/bin/hipcc" ] && export LAST_ERR="warning: hipcc not executable: $path/bin/hipcc"
    [ ! -d "$path/include/hip" ] && export LAST_ERR="hip include not found: $path/include/hip" && return 1
    return 0
}

function check_rocm_path() {
    local path=$1
    [ -z "$path" ] && (export LAST_ERR="ROCM_PATH not set"; return 1)
    [ ! -d "$path" ] && (export LAST_ERR="ROCM_PATH not found: $path"; return 1)

    for libclang_suffix in 'llvm/lib' 'lib/llvm/lib'; do
        if check_libclang_path "${path}/${libclang_suffix}"; then
            export LIBCLANG_PATH="${path}/${libclang_suffix}"
            echo "LIBCLANG_PATH found: $LIBCLANG_PATH"
            break
        fi
    done
    [ -z "${LIBCLANG_PATH:-}" ] && (echo "LIBCLANG_PATH not found ${LAST_ERR:-}"; return 1)

    [ ! -d "${path}/include/hipify/" ] && export LAST_ERR "\${path}/include/hipify/ not found: ${path}/include/hipify/" && return 1
    [ ! -f "${path}/include/hipify/stddef.h" ] && export LAST_ERR="stddef.h not found in ${path}/include/hipify/stddef.h" && return 1

    for hip_suffix in '/' 'hip'; do
        if check_hip_path "${path}/${hip_suffix}"; then
            export HIP_PATH="${path}/${hip_suffix}"
            echo "HIP_PATH found: $HIP_PATH"
            break
        fi
    done
    [ -z "${HIP_PATH:-}" ] && (echo "HIP_PATH not found. ${LAST_ERR:-}"; return 1)

    return 0
}

if [ -z "${ROCM_PATH:-}" ]; then
    for path in /opt/rocm* ; do
        if check_rocm_path $path; then
            export ROCM_PATH="$path"
            echo "ROCM_PATH found: $ROCM_PATH"
            break
        fi
    done
    if [ -z "$ROCM_PATH" ]; then
        export LAST_ERR="ROCM_PATH not set"
        exit 1
    fi
else
    check_rocm_path $ROCM_PATH
fi

[ -d "$ROCM_PATH/include" ] && export extra="${extra} -I$ROCM_PATH/include/"
[ -d "$ROCM_PATH/include/hipify" ] && export extra="${extra} -I$ROCM_PATH/include/hipify/"
# export HIPIFY_INCLUDE="${HIPIFY_INCLUDE:="-I${ROCM_PATH}/include/hipify/"}"
# [ ! -z "${HIPIFY_INCLUDE:-}" ] && export extra="${extra} ${HIPIFY_INCLUDE}"

# The rocm include path may need to be adjusted
bindgen "${SCRIPTPATH}"/wrapper.h \
    --raw-line "#![allow(non_camel_case_types)]" \
    --raw-line "#![allow(non_upper_case_globals)]" \
    --raw-line "#![allow(non_snake_case)]" \
    --rustified-enum "hip.*" \
    --generate-block \
    --ctypes-prefix "::libc" \
    --with-derive-default \
    --with-derive-eq \
    --with-derive-ord \
    --with-derive-hash \
    -o "${SCRIPTPATH}"/src/bindings.rs \
    -- $extra -D__HIP_PLATFORM_AMD__
