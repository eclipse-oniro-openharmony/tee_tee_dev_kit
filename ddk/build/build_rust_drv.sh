#!/usr/bin/env bash

# Copyright (C) 2023 Huawei Technologies Co., Ltd.
# Licensed under the Mulan PSL v2.
# You can use this software according to the terms and conditions of the Mulan
# PSL v2.
# You may obtain a copy of Mulan PSL v2 at:
#     http://license.coscl.org.cn/MulanPSL2
# THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
# KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
# NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
# See the Mulan PSL v2 for more details.

set -e

export CARGO_NET_GIT_FETCH_WITH_CLI=true
set -o pipefail
target="aarch64-unknown-teeos"
os="teeos"
dyn_perm_file="dyn_perm.xml"
auth_config_file="auth_config.xml"

cur_dir=$(cd "$(dirname "$0")"; pwd)
echo "build_rust_drv.sh is at dir: $cur_dir"
name=$1
std=$2
features=$3

function clean() {
    rm -rf ./target
}

function build_rust_drv() {
    if [ -z "$features" ]; then
        echo "no features"
    else
        features="--features=$features"
    fi
    echo "features = $features"

    echo "start compile"
    # compile
    release_path="target/$target/release"
    rust_so="$release_path/lib$name.so"
    combile_so="$release_path/libcombine.so"
    case "$std" in
    "core") cargo +$os build $features --target=$target --release
        ;;
    "std") cargo +$os build $features --target=$target --release
        ;;
    *) echo "std value error"
        ;;
    esac
    echo "compile finish"

    # sign
    echo "start sign"
    echo "release path : $release_path/"
    cp "${rust_so}" "${combile_so}"
    if [ -f "$dyn_perm_file" ]; then
        cp $dyn_perm_file $release_path/
    fi
    cp *.csv $release_path/ || true
    cp defconfig $release_path/ || true
    if [ -f "$auth_config_file" ]; then
        cp $auth_config_file $release_path/
    fi
    cp manifest.txt "${release_path}"
}

build_rust_drv
