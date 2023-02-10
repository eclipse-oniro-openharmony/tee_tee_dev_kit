#/usr/bin/env bash

export CARGO_NET_GIT_FETCH_WITH_CLI=true

RUSTC_BOOTSTRAP=1 cargo doc \
    --no-deps \
    --workspace \

rm -rf ./target
