#!/usr/bin/env bash

name="rust_drv_test_module" # same as cargo package name
std="core" # std value: 1) core 2) std
features=""

# target ta sec file in target/aarch64-unknown-teeos/release after build
bash ../../../build/build_rust_drv.sh $name $std $features
