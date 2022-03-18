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
project(tee_ddk C)

include (${TEE_BUILD_PATH}/build/cmake/common.cmake)

list(APPEND SOURCE_FILE
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/drv_test_module.c
)

list(APPEND INC_DIR
    ${CMAKE_CURRENT_SOURCE_DIR}/../include
)

list(APPEND FLAGS
    -DCONFIG_SAMPLE
)

list(APPEND LINK_FLAGS
    -Wl,-z,relro
)

compile_drv("${SOURCE_FILE}" "${FLAGS}" "${INC_DIR}" "${LD_FLAGS}")
