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

set(CMAKE_SYSTEM_NAME Linux)
set(CMAKE_SYSTEM_PROCESSOR aarch64)

set(CMAKE_C_COMPILER aarch64-linux-gnu-gcc CACHE PATH "GCC 64 compiler")
set(CMAKE_CXX_COMPILER aarch64-linux-gnu-g++ CACHE PATH "GCC CXX64 compiler")
set(CMAKE_LINKER aarch64-linux-gnu-ld CACHE PATH "GCC C64 LD")
set(CMAKE_READELF aarch64-linux-gnu-readelf CACHE PATH "GCC C64 READELF")

set(CMAKE_SKIP_BUILD_RPATH TRUE CACHE BOOL "" FORCE)
set(CMAKE_C_CREATE_SHARED_LIBRARY "<CMAKE_LINKER> <LANGUAGE_COMPILE_FLAGS> <LINK_FLAGS> -o <TARGET> <OBJECTS> <LINK_LIBRARIES>")
