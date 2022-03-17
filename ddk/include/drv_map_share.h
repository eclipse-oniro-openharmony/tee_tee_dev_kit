/*
 * Copyright (C) 2022 Huawei Technologies Co., Ltd.
 * Licensed under the Mulan PSL v2.
 * You can use this software according to the terms and conditions of the Mulan PSL v2.
 * You may obtain a copy of Mulan PSL v2 at:
 *     http://license.coscl.org.cn/MulanPSL2
 * THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND, EITHER EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR
 * PURPOSE.
 * See the Mulan PSL v2 for more details.
 */

/**
 * @file drv_map_share.h
 *
 * @brief 共享内存接口
 *
 * @since 1
 */

#ifndef LIBDRV_MAP_SHARED_H
#define LIBDRV_MAP_SHARED_H

#include <stdint.h>
typedef enum {
cache = 1,
non_cache = 0,
cache_mode_device = 2
} cache_mode_type;
typedef uint64_t paddr_t;

int32_t tee_map_secure(paddr_t paddr, uint64_t size, uintptr_t *vaddr, cache_mode_type cache_mode);
int32_t tee_map_nonsecure(paddr_t paddr, uint64_t size, uintptr_t *vaddr, cache_mode_type cache_mode);

#endif
