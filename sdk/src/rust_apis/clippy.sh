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

SCRIPT_DIR="$( cd -- "$( dirname -- "${BASH_SOURCE[0]:-$0}"; )" &> /dev/null && pwd 2> /dev/null; )";
export CARGO_NET_GIT_FETCH_WITH_CLI=true 

set -e

deny_list=(warnings unsafe_op_in_unsafe_fn unused_must_use)
allow_list=(clippy::result_unit_err)

# Note: We currently can't run cargo clippy for all packages at once, since we only enable alloc for some crates...
pushd "$SCRIPT_DIR"
echo "Running cargo clippy for rust_apis..."
RUSTC_BOOTSTRAP=1 cargo clippy -p rust_apis -- ${deny_lints[@]/#/--deny=}  ${allow_list[@]/#/--allow=}
rm -rf ./target
popd

