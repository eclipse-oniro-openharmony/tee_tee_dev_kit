#!/bin/bash
# Copyright Huawei Technologies Co., Ltd. 2010-2023. All rights reserved.

set -e

target_dir=$1
features=$2
cur_dir=$(cd "$(dirname "$0")"; pwd)
toolchain_dir="$cur_dir/../../../open_source/rust_toolchains"

export PATH="$toolchain_dir/bin:$PATH"
export LD_LIBRARY_PATH="$toolchain_dir/lib/"
export LIBRARY_PATH="toolchain_dir/lib/"

cd $cur_dir
cargo clean
cargo build --features="$features" --target="aarch64-unknown-teeos" --release

cp $cur_dir/target/aarch64-unknown-teeos/release/librust_huk_service.a $target_dir

