#!/bin/bash

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

set -e

THIRD_PARTY_ORIGIN=$(pwd)/../../../../../third_party

MUSL_CODE=${THIRD_PARTY_ORIGIN}/musl
EXPORT_MUSL_HEADER_PATH=$(pwd)/musl

BOUNDS_CHECK_CODE=${THIRD_PARTY_ORIGIN}/bounds_checking_function
EXPORT_BOUNDS_CHECK_HEADER_PATH=$(pwd)/bounds_checking_function

function do_copy_musl_header_file() {
    if [[ "$1" == "ARM32" ]]; then
        ARCH=arm
        LIBC_HEADER=libc_32
    else
        ARCH=aarch64
        LIBC_HEADER=libc
    fi

    LIBC_HEADER_PATH=${EXPORT_MUSL_HEADER_PATH}/${LIBC_HEADER}
    if [[ -d "${LIBC_HEADER_PATH}" ]]; then
        rm -rf ${LIBC_HEADER_PATH}
    fi

    mkdir ${LIBC_HEADER_PATH}
    mkdir -p ${LIBC_HEADER_PATH}/sys
    mkdir -p ${LIBC_HEADER_PATH}/netinet
    mkdir -p ${LIBC_HEADER_PATH}/arpa
    mkdir -p ${LIBC_HEADER_PATH}/arch/generic
    
    LIBC_ARCH_PATH=${LIBC_HEADER_PATH}/arch
    mkdir -p ${LIBC_ARCH_PATH}

    cp ${MUSL_CODE}/include/semaphore.h ${LIBC_HEADER_PATH}
    cp ${MUSL_CODE}/include/signal.h ${LIBC_HEADER_PATH}
    cp ${MUSL_CODE}/include/stdalign.h ${LIBC_HEADER_PATH}
    cp ${MUSL_CODE}/include/stdbool.h ${LIBC_HEADER_PATH}
    cp ${MUSL_CODE}/include/malloc.h ${LIBC_HEADER_PATH}
    cp ${MUSL_CODE}/include/dlfcn.h ${LIBC_HEADER_PATH}
    cp ${MUSL_CODE}/include/byteswap.h ${LIBC_HEADER_PATH}
    cp ${MUSL_CODE}/include/pthread.h ${LIBC_HEADER_PATH}
    cp ${MUSL_CODE}/include/stdlib.h ${LIBC_HEADER_PATH}
    cp ${MUSL_CODE}/include/stdio.h ${LIBC_HEADER_PATH}
    cp ${MUSL_CODE}/include/errno.h ${LIBC_HEADER_PATH}
    cp ${MUSL_CODE}/include/inttypes.h ${LIBC_HEADER_PATH}
    cp ${MUSL_CODE}/include/elf.h ${LIBC_HEADER_PATH}
    cp ${MUSL_CODE}/include/assert.h ${LIBC_HEADER_PATH}
    cp ${MUSL_CODE}/include/string.h ${LIBC_HEADER_PATH}
    cp ${MUSL_CODE}/include/strings.h ${LIBC_HEADER_PATH}
    cp ${MUSL_CODE}/include/stdarg.h ${LIBC_HEADER_PATH}
    cp ${MUSL_CODE}/include/setjmp.h ${LIBC_HEADER_PATH}
    cp ${MUSL_CODE}/include/fcntl.h ${LIBC_HEADER_PATH}
    cp ${MUSL_CODE}/include/endian.h ${LIBC_HEADER_PATH}
    cp ${MUSL_CODE}/include/time.h ${LIBC_HEADER_PATH}
    cp ${MUSL_CODE}/include/limits.h ${LIBC_HEADER_PATH}
    cp ${MUSL_CODE}/include/unistd.h ${LIBC_HEADER_PATH}
    cp ${MUSL_CODE}/include/locale.h ${LIBC_HEADER_PATH}
    cp ${MUSL_CODE}/include/wchar.h ${LIBC_HEADER_PATH}
    cp ${MUSL_CODE}/include/sched.h ${LIBC_HEADER_PATH}
    cp ${MUSL_CODE}/include/ctype.h ${LIBC_HEADER_PATH}
    cp ${MUSL_CODE}/include/math.h ${LIBC_HEADER_PATH}
    cp ${MUSL_CODE}/include/wctype.h ${LIBC_HEADER_PATH}
    cp ${MUSL_CODE}/include/alloca.h ${LIBC_HEADER_PATH}
    cp ${MUSL_CODE}/include/stddef.h ${LIBC_HEADER_PATH}
    cp ${MUSL_CODE}/include/stdint.h ${LIBC_HEADER_PATH}
    cp ${MUSL_CODE}/include/alloca.h ${LIBC_HEADER_PATH}
    cp ${MUSL_CODE}/include/netdb.h ${LIBC_HEADER_PATH}
    cp ${MUSL_CODE}/include/poll.h ${LIBC_HEADER_PATH}
    cp ${MUSL_CODE}/include/syscall.h ${LIBC_HEADER_PATH}
    cp ${MUSL_CODE}/include/features.h ${LIBC_HEADER_PATH}
    cp ${MUSL_CODE}/include/sys/mman.h ${LIBC_HEADER_PATH}/sys
    cp ${MUSL_CODE}/include/sys/syscall.h ${LIBC_HEADER_PATH}/sys
    cp ${MUSL_CODE}/include/sys/procfs.h ${LIBC_HEADER_PATH}/sys
    cp ${MUSL_CODE}/include/sys/user.h ${LIBC_HEADER_PATH}/sys
    cp ${MUSL_CODE}/include/sys/stat.h ${LIBC_HEADER_PATH}/sys
    cp ${MUSL_CODE}/include/sys/param.h ${LIBC_HEADER_PATH}/sys
    cp ${MUSL_CODE}/include/sys/time.h ${LIBC_HEADER_PATH}/sys
    cp ${MUSL_CODE}/include/sys/types.h ${LIBC_HEADER_PATH}/sys
    cp ${MUSL_CODE}/include/sys/select.h ${LIBC_HEADER_PATH}/sys
    cp ${MUSL_CODE}/include/sys/sysmacros.h ${LIBC_HEADER_PATH}/sys
    cp ${MUSL_CODE}/include/sys/resource.h ${LIBC_HEADER_PATH}/sys
    cp ${MUSL_CODE}/include/sys/auxv.h ${LIBC_HEADER_PATH}/sys
    cp ${MUSL_CODE}/include/sys/uio.h ${LIBC_HEADER_PATH}/sys
    cp ${MUSL_CODE}/include/sys/socket.h ${LIBC_HEADER_PATH}/sys
    cp ${MUSL_CODE}/include/sys/un.h ${LIBC_HEADER_PATH}/sys
    cp ${MUSL_CODE}/include/sys/ioctl.h ${LIBC_HEADER_PATH}/sys
    cp ${MUSL_CODE}/include/sys/times.h ${LIBC_HEADER_PATH}/sys
    cp ${MUSL_CODE}/include/netinet/in.h ${LIBC_HEADER_PATH}/netinet
    cp ${MUSL_CODE}/include/netinet/tcp.h ${LIBC_HEADER_PATH}/netinet
    cp ${MUSL_CODE}/include/arpa/inet.h ${LIBC_HEADER_PATH}/arpa
    cp -r ${MUSL_CODE}/arch/generic/* ${LIBC_HEADER_PATH}/arch/generic/
    cp -r ${MUSL_CODE}/arch/${ARCH} ${LIBC_ARCH_PATH}

    cp -r ${MUSL_CODE}/arch/${ARCH}/bits/syscall.h.in ${LIBC_ARCH_PATH}/${ARCH}/bits/syscall.h
    sed -f ${MUSL_CODE}/tools/mkalltypes.sed ${MUSL_CODE}/arch/${ARCH}/bits/alltypes.h.in ${MUSL_CODE}/include/alltypes.h.in > ${LIBC_ARCH_PATH}/${ARCH}/bits/alltypes.h
    sed -n -e s/__NR_/SYS_/p < ${MUSL_CODE}/arch/${ARCH}/bits/alltypes.h.in >> ${LIBC_ARCH_PATH}/${ARCH}/bits/alltypes.h
}

function copy_musl_header_file() {
    mkdir -p ${EXPORT_MUSL_HEADER_PATH}
    do_copy_musl_header_file ARM32
    do_copy_musl_header_file ARM64
}

function copy_bounds_check_header_file() {
    mkdir -p ${EXPORT_BOUNDS_CHECK_HEADER_PATH}
    mkdir -p ${EXPORT_BOUNDS_CHECK_HEADER_PATH}/include
    cp -r ${BOUNDS_CHECK_CODE}/include/* ${EXPORT_BOUNDS_CHECK_HEADER_PATH}/include/
}

copy_musl_header_file
copy_bounds_check_header_file
