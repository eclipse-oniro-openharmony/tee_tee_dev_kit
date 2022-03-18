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
set(CMAKE_SYSTEM_PROCESSOR arm)

set(CMAKE_C_COMPILER clang CACHE PATH "llvm compiler")
set(CMAKE_CXX_COMPILER clang++ CACHE PATH "GCC CXX32 compiler")
set(CMAKE_AS llvm-as CACHE STRING "Cross AS")
set(CMAKE_LINKER ld.lld CACHE STRING "Cross linker")
set(CMAKE_CPP clang-cpp CACHE STRING "Cross CPP")
set(CMAKE_AR llvm-ar 2>/dev/null)
set(CMAKE_NM llvm-nm)
set(CMAKE_OBJCOPY llvm-objcopy CACHE STRING "Cross OBJCOPY")
set(CMAKE_READELF llvm-readelf CACHE STRING "Cross READELF")
set(CMAKE_STRIP llvm-strip CACHE STRING "Cross STRIP")
set(CMAKE_RANLIB llvm-ranlib)

set(CMAKE_SKIP_BUILD_RPATH TRUE CACHE BOOL "" FORCE)
set(CMAKE_C_CREATE_SHARED_LIBRARY "<CMAKE_LINKER> <LANGUAGE_COMPILE_FLAGS> <LINK_FLAGS> -o <TARGET> <OBJECTS> <LINK_LIBRARIES>")

set(CMAKE_C_COMPILER_WORKS TRUE)
set(CMAKE_C_ARCHIVE_FINISH "<CMAKE_RANLIB> <TARGET>")
