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
ifeq ($(TEE_BUILD_PATH), )
    TEE_BUILD_PATH=${CUR_DIR}/../..
endif

include $(TEE_BUILD_PATH)/build/mk/common_flags.mk

# set header directory
ifeq ($(TARGET_IS_ARM64), n)
	LIBC_INC := libc_32
	LIBC_ARCH := arm
else
	LIBC_INC := libc
	LIBC_ARCH := aarch64
endif

# set header directory
INCLUDEDIR += -I$(TEE_BUILD_PATH)/include
INCLUDEDIR += -I$(TEE_BUILD_PATH)/thirdparty/open_source/musl/$(LIBC_INC)
INCLUDEDIR += -I$(TEE_BUILD_PATH)/thirdparty/open_source/musl/$(LIBC_INC)/hm
INCLUDEDIR += -I$(TEE_BUILD_PATH)/thirdparty/open_source/musl/$(LIBC_INC)/arch/generic
INCLUDEDIR += -I$(TEE_BUILD_PATH)/thirdparty/open_source/musl/$(LIBC_INC)/arch/$(LIBC_ARCH)
INCLUDEDIR += -I$(TEE_BUILD_PATH)/thirdparty/open_source/musl/$(LIBC_INC)/arch/$(LIBC_ARCH)/bits
INCLUDEDIR += -I$(TEE_BUILD_PATH)/thirdparty/open_source/bounds_checking_function/include

# choose compile method
ifeq ($(CONFIG_GCC), y)
    include $(TEE_BUILD_PATH)/build/mk/common_gcc.mk
else
    include $(TEE_BUILD_PATH)/build/mk/common_llvm.mk
endif

BUILD_DIR := ../output
SRC_DIR := ../src
OBJS := $(patsubst $(SRC_DIR)/%.c, $(BUILD_DIR)/%.o, $(SRC))

$(OBJS): $(BUILD_DIR)/%.o:$(SRC_DIR)/%.c
	@echo "$@"
	@test -d $(dir $@) || mkdir -p $(dir $@)
	$(CC) $(CFLAGS) $(INCLUDEDIR) -c $< -o $@

combine: $(OBJS)
	$(LD) $(LDFLAGS) $(OBJS) $(EXTRAO) -o $(BUILD_DIR)/libcombine.so
	@rm -f $(BUILD_DIR)/*.o
