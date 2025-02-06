# Copyright (C) 2024 Huawei Technologies Co., Ltd.
# Licensed under the Mulan PSL v2.
# You can use this software according to the terms and conditions of the Mulan
# PSL v2.
# You may obtain a copy of Mulan PSL v2 at:
#     http://license.coscl.org.cn/MulanPSL2
# THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
# KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
# NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
# See the Mulan PSL v2 for more details.

set(TEE_OH_PATH $ENV{TEE_OH_PATH})
set(TEE_OH_BUILD_PATH $ENV{TEE_OH_BUILD_PATH})

include (${TEE_OH_BUILD_PATH}/cmake/common_flags.cmake)

set(COMMON_INCLUDES
    ${COMMON_INCLUDES}
    ${TEE_OH_PATH}/sysroot/usr/include/tee
    ${TEE_OH_PATH}/thirdparty/open_source/bounds_checking_function/include
)

list(APPEND COMMON_CFLAGS
    -D__LITEOS__)

if (NOT "${TARGET_IS_ARM64}" STREQUAL "y")
    list(APPEND COMMON_INCLUDES
        ${TEE_OH_PATH}/thirdparty/open_source/musl/libc_32
        ${TEE_OH_PATH}/thirdparty/open_source/musl/libc_32/arch/generic
        ${TEE_OH_PATH}/thirdparty/open_source/musl/libc_32/arch/arm/bits
        ${TEE_OH_PATH}/thirdparty/open_source/musl/libc_32/arch/arm
    )

    list(APPEND COMMON_CFLAGS
        -marm
    )

    list(APPEND COMMON_LDFLAGS
        "-T${TEE_OH_BUILD_PATH}/ld/ta_link.ld"
    )

else()
    list(APPEND COMMON_INCLUDES
        ${TEE_OH_PATH}/thirdparty/open_source/musl/libc
        ${TEE_OH_PATH}/thirdparty/open_source/musl/libc/arch/generic
        ${TEE_OH_PATH}/thirdparty/open_source/musl/libc/arch/aarch64/bits
        ${TEE_OH_PATH}/thirdparty/open_source/musl/libc/arch/aarch64
    )

    list(APPEND COMMON_LDFLAGS
        "-T${TEE_OH_BUILD_PATH}/ld/ta_link_64.ld"
    )
endif()
