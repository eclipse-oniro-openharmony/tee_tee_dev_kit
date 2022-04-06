# cmake<a name="EN_US_TOPIC_0000001078026808"></a>

## Introduction

SDK cmake support using both LLVM and GCC toolchain to build Trusted Application.

before building Trusted Application please install target toolchain, and add the toolchain path to system environment.<br>
for example:<br>
```
export PATH="/path/to/toolchains/clang+llvm/bin/":$PATH
```
or
```
export PATH="/path/to/gcc-linaro-aarch64-linux-gnu/bin/":$PATH
```


we suggest to use LLVM as it support more security compile options, in this SDK it assume LLVM is used by default.

if you need to use GCC to build your Trusted Application,
please use aarch64_toolchain.cmake in your shell script, as:
```
cmake -DCMAKE_TOOLCHAIN_FILE=${TEE_BUILD_PATH}/build/cmake/arm_toolchain.cmake CMakeListsFilePath
```
