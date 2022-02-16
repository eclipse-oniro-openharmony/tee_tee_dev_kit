# make<a name="EN_US_TOPIC_0000001078026808"></a>

## Introduction

SDK support using both LLVM and GCC toolchain to build Trusted Application.

before building Trusted Application please install target toolchain, and add the toolchain path to system environment.<br>
for example:<br>
```
export PATH=$PATH:"/path/to/toolchains/clang+llvm/bin/"
```
or
```
export PATH=$PATH:"/path/to/gcc-linaro-aarch64-linux-gnu/bin/"
```

we suggust to use LLVM as it support more security compile options, in this SDK it assume LLVM is used by default.

if you need to use GCC to build your Trusted Application, 
please add CONFIG_GCC to your system environment before build your Trusted Application.<br>
using command:<br>
```
export CONFIG_GCC=y
```