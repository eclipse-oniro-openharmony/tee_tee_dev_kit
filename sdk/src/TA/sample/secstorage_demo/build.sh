#!/bin/bash

# Copyright (C) 2022 Huawei Technologies Co., Ltd.
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

export SOURCE_PATH=$(dirname $0)
export ABS_SOURCE_PATH=$(cd ${SOURCE_PATH};pwd)
export TEE_BUILD_PATH=${ABS_SOURCE_PATH}/../../../

build_clean()
{
    [ -d "output" ] && rm -rf output #clean
    make clean
}

get_config()
{
    while read line;do
        eval "$line"
    done < defconfig
}

cmake_build()
{
    echo "start cmake build ${TARGET_IS_ARM64} target"
    build_clean

    mkdir -p output #create cmake_build file
    cd output

    CMAKE_TOOLCHAIN_FILE=${TEE_BUILD_PATH}/build/cmake/llvm_toolchain.cmake

    cmake \
          -DCMAKE_VERBOSE_MAKEFILE=on \
          -DTARGET_IS_ARM64=${TARGET_IS_ARM64} \
          -DCMAKE_TOOLCHAIN_FILE=${CMAKE_TOOLCHAIN_FILE} \
          -DTEE_BUILD_PATH=${TEE_BUILD_PATH} \
          ${ABS_SOURCE_PATH}

    cmake --build . -j8
}

mk_build()
{
    echo "start make build ${TARGET_IS_ARM64} target"
    build_clean
    TEE_BUILD_PATH=${TEE_BUILD_PATH} \
    TARGET_IS_ARM64=${TARGET_IS_ARM64} \
    make V=3 sec_binary -j4
}

if [ "${1}" = "clean" ]; then
    build_clean
else
    get_config

    case "$CONFIG_BUILD_TOOL" in
        "") echo "can not find config" ;;
        "cmake")
            echo "do cmake compile"
            cmake_build ;;
        "make")
            echo "do make compile"
            mk_build ;;
        *) echo "do make compile" && exit -1 ;;
    esac
fi
