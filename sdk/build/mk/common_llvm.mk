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

# config llvm compiler
CC      := clang
CXX     := clang++
AS      := llvm-as
LD      := ld.lld
CPP     := clang-cpp
# disable ar creating debug
AR      := llvm-ar 2>/dev/null
NM      := llvm-nm
OBJCOPY := llvm-objcopy
READELF := llvm-readelf
STRIP   := llvm-strip
RANLIB  := llvm-ranlib
