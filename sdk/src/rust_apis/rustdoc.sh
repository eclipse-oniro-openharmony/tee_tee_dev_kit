#/usr/bin/env bash

SCRIPT_DIR="$( cd -- "$( dirname -- "${BASH_SOURCE[0]:-$0}"; )" &> /dev/null && pwd 2> /dev/null; )";
export CARGO_NET_GIT_FETCH_WITH_CLI=true

RUSTC_BOOTSTRAP=1 cargo doc \
    --no-deps \
    --workspace \

rm -rf ./target
