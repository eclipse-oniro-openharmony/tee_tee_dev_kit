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

include $(TEE_OH_BUILD_PATH)/mk/common_flags.mk

# set header directory
INCLUDEDIR += -I$(TEE_OH_PATH)/sysroot/usr/include/tee \
    -I$(TEE_OH_PATH)/thirdparty/open_source/bounds_checking_function/include

CFLAGS += -Wstack-protector --param ssp-buffer-size=4

ifneq ($(TARGET_IS_ARM64), y)
    CFLAGS += -marm
endif

ifneq ($(TARGET_IS_ARM64),y)
    INCLUDEDIR += -I$(TEE_OH_PATH)/thirdparty/open_source/musl/libc_32 \
                  -I$(TEE_OH_PATH)/thirdparty/open_source/musl/libc_32/arch/arm \
                  -I$(TEE_OH_PATH)/thirdparty/open_source/musl/libc_32/arch/arm/bits \
                  -I$(TEE_OH_PATH)/thirdparty/open_source/musl/libc_32/arch/generic
else
    INCLUDEDIR += -I$(TEE_OH_PATH)/thirdparty/open_source/musl/libc \
                  -I$(TEE_OH_PATH)/thirdparty/open_source/musl/libc/arch/aarch64 \
                  -I$(TEE_OH_PATH)/thirdparty/open_source/musl/libc/arch/aarch64/bits \
                  -I$(TEE_OH_PATH)/thirdparty/open_source/musl/libc/arch/generic
endif

ifneq ($(TARGET_IS_ARM64),y)
    LDFLAGS += -T$(TEE_OH_BUILD_PATH)/ld/ta_link.ld

else
    LDFLAGS += -T$(TEE_OH_BUILD_PATH)/ld/ta_link_64.ld
endif

ifeq ($(CONFIG_GCC),y)
    include $(TEE_OH_BUILD_PATH)/mk/common_gcc.mk
else
    ifneq ($(TARGET_IS_ARM64),y)
        $(error "llvm only support 64bit TA")
    else
        include $(TEE_OH_BUILD_PATH)/mk/common_llvm.mk
