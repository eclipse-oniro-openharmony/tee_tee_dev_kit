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

name="rust_helloworld" # same as cargo package name
std="core" # std value: 1) core 2) std
api_level=1
features=""

# target ta sec file in target/aarch64-unknown-teeos/release after build
bash ../../../build/pack-TA/build_rust_ta.sh $name $std $api_level $features
