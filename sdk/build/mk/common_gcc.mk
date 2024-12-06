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

# set gcc compiler
ifneq ($(TARGET_IS_ARM64), y)
    CC = arm-linux-gnueabi-gcc
    LD = arm-linux-gnueabi-ld
    READELF = arm-linux-gnueabi-readelf
else
    CC = aarch64-linux-gnu-gcc
    LD = aarch64-linux-gnu-ld
    READELF = aarch64-linux-gnu-readelf
endif
