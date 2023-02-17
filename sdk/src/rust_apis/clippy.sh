#/usr/bin/env bash

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

