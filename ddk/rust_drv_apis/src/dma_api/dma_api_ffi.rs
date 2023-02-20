// Copyright (C) 2023 Huawei Technologies Co., Ltd.
// Licensed under the Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2.
// You may obtain a copy of Mulan PSL v2 at:
//     http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

extern "C" {
    ///
    /// flush cache range
    ///
    /// # Parameters
    /// start: cache start
    /// end: cache end
    pub fn dma_flush_range(start: u64, end: u64);

    ///
    /// inv cache range
    ///
    /// # Parameters
    /// start: cache start
    /// end: cache end
    pub fn dma_inv_range(start: u64, end: u64);

    ///
    /// map cache range
    ///
    /// # Parameters
    /// start: cache start
    /// size: cache size
    /// dir: cache dir
    pub fn dma_map_area(start: u64, size: u64, dir: i32);

    ///
    /// unmap cache range
    ///
    /// # Parameters
    /// start: cache start
    /// size: cache size
    /// dir: cache dir
    pub fn dma_unmap_area(start: u64, size: u64, dir: i32);

    ///
    /// clean cache range
    ///
    /// # Parameters
    /// start: cache start
    /// end: cache end
    pub fn dma_clean_range(start: u64, end: u64);
}
