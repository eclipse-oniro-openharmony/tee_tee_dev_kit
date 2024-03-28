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
export TEE_OH_PATH=${ABS_SOURCE_PATH}/../../../
export TEE_OH_BUILD_PATH=${TEE_OH_PATH}/build/teekit/
export TEE_SRC_PATH=${TEE_OH_BUILD_PATH}/TA_demo/
SIGNTOOL_DIR=${TEE_OH_BUILD_PATH}/script

build_clean()
{
    [ -d "output" ] && rm -rf output #clean
    make clean
}

get_config()
{
    while read line;do
        eval "$line"
    done < ${TEE_SRC_PATH}/defconfig
}

cmake_build()
{
    echo "start cmake build ${CONFIG_GCC} ${TARGET_IS_ARM64} target"
    build_clean

    mkdir -p output #create cmake_build file
    cd output

    if [ "${CONFIG_GCC}" == "y" ];then
        if [ "${TARGET_IS_ARM64}" == "y" ];then
            CMAKE_TOOLCHAIN_FILE=${TEE_OH_BUILD_PATH}/cmake/aarch64_toolchain.cmake
        elif [ "${TARGET_IS_ARM64}" == "n" ];then
            CMAKE_TOOLCHAIN_FILE=${TEE_OH_BUILD_PATH}/cmake/arm_toolchain.cmake
        fi
    elif [ "${CONFIG_GCC}" == "n" ];then
        if [ "${TARGET_IS_ARM64}" == "y" ];then
            CMAKE_TOOLCHAIN_FILE=${TEE_OH_BUILD_PATH}/cmake/llvm_toolchain.cmake
        else
            echo "LLVM only support 64bit TA.";
            exit -1
        fi
    else
        echo "Invalid tool chain";
        exit -1
    fi

    cmake \
          -DCMAKE_VERBOSE_MAKEFILE=on \
          -DCMAKE_TOOLCHAIN_FILE=${CMAKE_TOOLCHAIN_FILE} \
          -DTARGET_IS_ARM64=${TARGET_IS_ARM64} \
          -DTEE_OH_PATH=${TEE_OH_PATH} \
          -DTEE_OH_BUILD_PATH=${TEE_OH_BUILD_PATH} \
          ${TEE_SRC_PATH}

    cmake --build . -j8
}

mk_build()
{
    echo "start make build ${CONFIG_GCC} ${TARGET_IS_ARM64} ${SIGNATURE_VER} target"
    build_clean

    export TARGET_IS_ARM64=${TARGET_IS_ARM64}
    export CONFIG_GCC=${CONFIG_GCC}

    make V=3 libcombine -j4
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

    python3 ${SIGNTOOL_DIR}/ta_check_undefined_symbol.py libcombine.so
    if [ ${SIGNATURE_VER} == "V3" ];then
        python3 -B ${SIGNTOOL_DIR}/signtool_sec.py --in_path ${TEE_SRC_PATH} --out_path ${TEE_SRC_PATH} \
            --privateCfg ${TEE_OH_BUILD_PATH}/config/ta_sign_algo_config.ini
    elif [ ${SIGNATURE_VER} == "V3_1" ];then
        python3 -B ${SIGNTOOL_DIR}/signtool_sec.py --in_path ${TEE_SRC_PATH} --out_path ${TEE_SRC_PATH} \
            --privateCfg ${TEE_OH_BUILD_PATH}/config/ta_sign_algo_config.ini \
            --publicCfg ${TEE_OH_BUILD_PATH}/config/config_ta_public.ini
    else
        echo "Invalid signature version.";
        exit -1
    fi
fi
