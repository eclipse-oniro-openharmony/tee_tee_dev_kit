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

# common compile parameters
CFLAGS += -Wall -Werror
CFLAGS += -Wdate-time -Wfloat-equal -Wshadow
CFLAGS += -fno-short-enums
CFLAGS += -fno-omit-frame-pointer
CFLAGS += -Wextra -nostdinc
CFLAGS += -march=armv8-a -fPIC
CFLAGS += -pipe
CFLAGS += -fno-common -fsigned-char
CFLAGS += -ftrivial-auto-var-init=zero
CFLAGS += -enable-trivial-auto-var-init-zero-knowing-it-will-be-removed-from-clang

ifeq ($(CONFIG_GCC),y)
    CFLAGS += -W
    CFLAGS += -fstack-protector
    CFLAGS += -Os -fno-peephole -fno-peephole2
else
    # set compile parameters
    CFLAGS += -fstack-protector-strong
    CFLAGS += -funwind-tables
    CFLAGS += -Oz
    CFLAGS += -munaligned-access -fmax-type-align=1
    CFLAGS += -flto -fvisibility=default -fsanitize=cfi -fno-sanitize-cfi-cross-dso
    CFLAGS += -fno-exceptions
    ifeq ($(TARGET_IS_ARM64),y)
        CFLAGS += --target=aarch64-linux-gnu
    else
        CFLAGS += --target=arm-linux-gnu
        CFLAGS += -mfloat-abi=soft
    endif
    # set LD flags
    LDFLAGS += -z max-page-size=4096
    # enable XOM
    ifeq ($(TARGET_IS_ARM64),y)
        LDFLAGS += --execute-only
    endif
endif

# common LDFLAGS
LDFLAGS += -s -z text -z now -z relro -z noexecstack -shared
