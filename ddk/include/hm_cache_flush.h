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
 * @file hm_cache_flush.h
 *
 * @brief cache接口
 *
 * @since 1
 */

#ifndef  LIBMMGR_HM_CACHE_FLUSH_H
#define  LIBMMGR_HM_CACHE_FLUSH_H

#include <stdint.h>

void dma_flush_range(uint64_t start, uint64_t end);
void dma_inv_range(uint64_t start, uint64_t end);
void dma_map_area(uint64_t start, uint64_t size, int32_t dir);
void dma_unmap_area(uint64_t start, uint64_t size, int32_t dir);
void dma_clean_range(uint64_t start, uint64_t end);

#endif /* LIBMMGR_HM_CACHE_FLUSH_H */