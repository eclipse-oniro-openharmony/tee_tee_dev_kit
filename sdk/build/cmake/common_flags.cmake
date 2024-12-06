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

list(APPEND COMMON_LDFLAGS
    "-s"
    "SHELL:-z text"
    "SHELL:-z now"
    "SHELL:-z relro"
    "SHELL:-z noexecstack"
    "-shared"
)
if ("${CONFIG_GCC}" STREQUAL "n")
    list(APPEND COMMON_LDFLAGS
        "SHELL:-z max-page-size=4096"
    )
if ("${TARGET_IS_ARM64}" STREQUAL "y")
    list(APPEND COMMON_LDFLAGS "-execute-only")
endif()
endif()
set(COMMON_LDFLAGS ${COMMON_LDFLAGS} CACHE INTERNAL "")

list(APPEND COMMON_CFLAGS
    -Wall
    -Werror
    -fno-short-enums
    -fno-omit-frame-pointer
    -Wstack-protector
    --param ssp-buffer-size=4
    -Wextra
    -nostdinc
    -march=armv8-a
    -fPIC
    -fno-common
    -fsigned-char
)
if ("${CONFIG_GCC}" STREQUAL "y")
    list(APPEND COMMON_CFLAGS
        -W
        -fstack-protector
        -Os
        -fno-peephole
        -fno-peephole2
    )
else()
    list(APPEND COMMON_CFLAGS
        -fstack-protector-strong
        -funwind-tables
        -Oz
        -munaligned-access
        -fmax-type-align=1
        -flto
        -fvisibility=default
        -fsanitize=cfi
        -fno-exceptions
    )
    if ("${TARGET_IS_ARM64}" STREQUAL "n")
        list(APPEND COMMON_CFLAGS
            --target=arm-linux-gnu
            -mfloat-abi=soft
        )
    else()
        list(APPEND COMMON_CFLAGS
            --target=aarch64-linux-gnu
        )
    endif()
endif()
set(COMMON_CFLAGS ${COMMON_CFLAGS} CACHE INTERNAL "")
