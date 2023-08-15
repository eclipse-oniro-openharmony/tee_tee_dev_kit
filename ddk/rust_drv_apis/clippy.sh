#/usr/bin/env bash
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

export CARGO_NET_GIT_FETCH_WITH_CLI=true 

set -e

deny_list=(warnings unsafe_op_in_unsafe_fn unused_must_use)
allow_list=(clippy::result_unit_err)

pushd "$SCRIPT_DIR"
echo "Running cargo clippy for rust_drv_apis..."
RUSTC_BOOTSTRAP=1 cargo clippy -p rust_drv_apis -- ${deny_lints[@]/#/--deny=}  ${allow_list[@]/#/--allow=}
rm -rf ./target
popd

