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

set(TEE_BUILD_PATH $ENV{TEE_BUILD_PATH})

include (${TEE_BUILD_PATH}/build/cmake/common_flags.cmake)

if (NOT "${TARGET_IS_ARM64}" STREQUAL "y")
    set(SDK_C_SOURCES ${TEE_BUILD_PATH}/src/TA/ta_magic.c)
endif()

set(COMMON_INCLUDES
    ${COMMON_INCLUDES}
    ${TEE_BUILD_PATH}/include/TA
    ${TEE_BUILD_PATH}/include/TA/ext
    ${TEE_BUILD_PATH}/thirdparty/open_source/bounds_checking_function/include
)

if (NOT "${TARGET_IS_ARM64}" STREQUAL "y")
    list(APPEND COMMON_INCLUDES
        ${TEE_BUILD_PATH}/thirdparty/open_source/musl/libc_32
        ${TEE_BUILD_PATH}/thirdparty/open_source/musl/libc_32/arch/generic
        ${TEE_BUILD_PATH}/thirdparty/open_source/musl/libc_32/arch/arm/bits
        ${TEE_BUILD_PATH}/thirdparty/open_source/musl/libc_32/arch/arm
    )

    list(APPEND COMMON_CFLAGS
        -marm
    )

    list(APPEND COMMON_LDFLAGS
        "-T${TEE_BUILD_PATH}/build/tools/ta_link.ld"
    )
else()
    list(APPEND COMMON_INCLUDES
        ${TEE_BUILD_PATH}/thirdparty/open_source/musl/libc
        ${TEE_BUILD_PATH}/thirdparty/open_source/musl/libc/arch/generic
        ${TEE_BUILD_PATH}/thirdparty/open_source/musl/libc/arch/aarch64/bits
        ${TEE_BUILD_PATH}/thirdparty/open_source/musl/libc/arch/aarch64
    )

    list(APPEND COMMON_LDFLAGS
        "-T${TEE_BUILD_PATH}/build/tools/ta_link_64.ld"
    )
endif()
