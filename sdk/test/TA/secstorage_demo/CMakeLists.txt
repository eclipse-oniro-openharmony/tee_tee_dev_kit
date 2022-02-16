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
cmake_minimum_required(VERSION 3.16 FATAL_ERROR)
project(tee_sdk C)

if (PROJECT_SOURCE_DIR STREQUAL PROJECT_BINARY_DIR)
    message(FATAL_ERROR "Forbid compiling in the source tree")
endif()

include(${CMAKE_CURRENT_SOURCE_DIR}/config.cmake)
include($ENV{TEE_BUILD_PATH}/build/cmake/common.cmake)

set(LIBRARY_OUTPUT_PATH ${CMAKE_CURRENT_SOURCE_DIR})
set(CURRENT_TARGET_SO "combine")

set(SDK_C_SOURCES
    ${SDK_C_SOURCES}
    secstorage_demo.c
)

add_library(${CURRENT_TARGET_SO} SHARED ${SDK_C_SOURCES})
target_include_directories(${CURRENT_TARGET_SO} PUBLIC ${COMMON_INCLUDES})
target_compile_options(${CURRENT_TARGET_SO} PRIVATE ${COMMON_CFLAGS})
target_link_options(${CURRENT_TARGET_SO} PRIVATE ${COMMON_LDFLAGS})

add_custom_command(
    TARGET ${CURRENT_TARGET_SO} POST_BUILD
    COMMAND sh $ENV{TEE_BUILD_PATH}/build/tools/ta_entry_check.sh ${CMAKE_READELF} ${CMAKE_CURRENT_SOURCE_DIR}/libcombine.so n y ${TARGET_IS_ARM64}
    COMMAND python3 -B $ENV{TEE_BUILD_PATH}/build/signtools/signtool_sec.py ${CMAKE_CURRENT_SOURCE_DIR} ${CMAKE_CURRENT_SOURCE_DIR} --privateCfg $ENV{TEE_BUILD_PATH}/build/config_tee_private_sample.ini --publicCfg ${CMAKE_CURRENT_SOURCE_DIR}/config_ta_public.ini
)
