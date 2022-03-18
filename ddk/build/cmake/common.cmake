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

# compile flags

list(APPEND INC_DIR
    ${TEE_BUILD_PATH}/include
)

if ("${TARGET_IS_ARM64}" STREQUAL "n")
    set(LIBC_INC "libc_32")
    set(LIBC_ARCH "arm")
else()
    set(LIBC_INC "libc")
    set(LIBC_ARCH "aarch64")
endif()
list(APPEND COMMON_INCLUDES
    ${TEE_BUILD_PATH}/thirdparty/open_source/musl/${LIBC_INC}
    ${TEE_BUILD_PATH}/thirdparty/open_source/musl/${LIBC_INC}/hm
    ${TEE_BUILD_PATH}/thirdparty/open_source/musl/${LIBC_INC}/arch/generic
    ${TEE_BUILD_PATH}/thirdparty/open_source/musl/${LIBC_INC}/arch/${LIBC_ARCH}
    ${TEE_BUILD_PATH}/thirdparty/open_source/musl/${LIBC_INC}/arch/${LIBC_ARCH}/bits
    ${TEE_BUILD_PATH}/thirdparty/open_source/bounds_checking_function/include
)

include (${TEE_BUILD_PATH}/build/cmake/common_flags.cmake)

#############################################
#compile_drv : main function (c to so)
#target : so_name (libcombine.so)
#SOURCE_FILE : c files (demo.c)
#INC_DIR : -Iflags
#FLAGS : -Dflags
#LD_FLAGS : -Wl,-z,now
#############################################
function(compile_drv SOURCE_FILE FLAGS INC_DIR LD_FLAGS)

    set(FLAGS ${FLAGS} CACHE INTERNAL "")
    set(INC_DIR ${INC_DIR} CACHE INTERNAL "")
    set(LD_FLAGS ${LD_FLAGS} CACHE INTERNAL "")

    list(APPEND FLAGS
        ${COMMON_CFLAGS}
    )
    list(APPEND INC_DIR
        ${COMMON_INCLUDES}
    )
    list(APPEND LD_FLAGS
        ${COMMON_LDFLAGS}
    )

    set(c_flags)
    foreach (f IN LISTS FLAGS)
        if(NOT c_flags)
            set(c_flags "${f}")
        else()
            set(c_flags "${c_flags} ${f}")
        endif()
    endforeach()

    set(c_includes)
    foreach (i IN LISTS INC_DIR)
        if (NOT c_includes)
            set(c_includes "-I${i}")
        else()
            set(c_includes "${c_includes} -I${i}")
        endif()
    endforeach()

    set_source_files_properties(${SOURCE_FILE} PROPERTIES COMPILE_FLAGS "${c_includes} ${c_flags}")
    add_library(combine SHARED ${SOURCE_FILE})
    target_link_options(combine PRIVATE ${LD_FLAGS})

endfunction(compile_drv)
