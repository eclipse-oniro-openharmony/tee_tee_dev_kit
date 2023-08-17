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

CUR_DIR=$(shell pwd)
INTERNAL_DIR=$(CUR_DIR)/../../../../internal/
IS_INTERNAL_TA=$(shell if [ -d $(INTERNAL_DIR) ]; then echo "y"; else echo "n"; fi)

include $(TEE_BUILD_PATH)/build/mk/common_flags.mk
ifeq ($(IS_INTERNAL_TA), y)
    include $(INTERNAL_DIR)/mk/internal_common.mk
endif

# set header directory
INCLUDEDIR += -I$(TEE_BUILD_PATH)/include/TA \
    -I$(TEE_BUILD_PATH)/include/TA/ext \
    -I$(TEE_BUILD_PATH)/thirdparty/open_source/bounds_checking_function/include

CFLAGS += -Wstack-protector --param ssp-buffer-size=4

ifneq ($(TARGET_IS_ARM64), y)
    CFLAGS += -marm
endif

ifneq ($(TARGET_IS_ARM64),y)
    INCLUDEDIR += -I$(TEE_BUILD_PATH)/thirdparty/open_source/musl/libc_32 \
                  -I$(TEE_BUILD_PATH)/thirdparty/open_source/musl/libc_32/arch/arm \
                  -I$(TEE_BUILD_PATH)/thirdparty/open_source/musl/libc_32/arch/arm/bits \
                  -I$(TEE_BUILD_PATH)/thirdparty/open_source/musl/libc_32/arch/generic
else
    INCLUDEDIR += -I$(TEE_BUILD_PATH)/thirdparty/open_source/musl/libc \
                  -I$(TEE_BUILD_PATH)/thirdparty/open_source/musl/libc/arch/aarch64 \
                  -I$(TEE_BUILD_PATH)/thirdparty/open_source/musl/libc/arch/aarch64/bits \
                  -I$(TEE_BUILD_PATH)/thirdparty/open_source/musl/libc/arch/generic
endif

ifneq ($(TARGET_IS_ARM64),y)
    ifeq ($(IS_INTERNAL_TA), y)
	LDFLAGS += -T$(INTERNAL_DIR)/ld/ta_link.ld
    else
        LDFLAGS += -T$(TEE_BUILD_PATH)/build/ld/ta_link.ld
    endif
else
    LDFLAGS += -T$(TEE_BUILD_PATH)/build/ld/ta_link_64.ld
endif

include $(TEE_BUILD_PATH)/build/mk/common_llvm.mk
