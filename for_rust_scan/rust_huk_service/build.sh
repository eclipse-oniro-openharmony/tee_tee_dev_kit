#!/bin/bash
# Copyright (C) 2023 Huawei Technologies Co., Ltd.
# Licensed under the Mulan PSL v2.
# You can use this software according to the terms and conditions of the Mulan
# PSL v2.
# You may obtain a copy of Mulan PSL v2 at:
#     http://license.coscl.org.cn/MulanPSL2
# THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
# KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
# NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.

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

