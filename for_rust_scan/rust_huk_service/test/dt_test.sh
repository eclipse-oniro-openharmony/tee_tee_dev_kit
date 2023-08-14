#!/bin/bash

rustup default 1.68.0-x86_64-unknown-linux-gnu

cd ../

#clear
cargo clean
rm -rf ./default_*.profraw
rm -rf ./report
rm -rf ./default.profdata

# compile and run test
CARGO_INCREMENTAL=0 RUSTC_BOOTSTRAP=1 RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort" cargo test --target=x86_64-unknown-linux-gnu

# profdata
# "$HOME/.rustup/toolchains/1.68.0-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-profdata" merge --sparse ./*.profraw -o ./default.profdata

# gen report
grcov . -s ./src --binary-path . -t html --branch --excl-br-line "^\s*((debug_)?assert(_eq|_ne)?!|#\[derive\()" --ignore-not-existing -o ./report
