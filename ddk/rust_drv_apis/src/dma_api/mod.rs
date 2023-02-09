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
mod dma_api_ffi;

#[repr(C)]
pub struct DmaApi {}

impl DmaApi {
    pub fn flush_range(start: u64, end: u64) {
        unsafe {
            dma_api_ffi::dma_flush_range(start, end);
        }
    }

    pub fn inv_range(start: u64, end: u64) {
        unsafe {
            dma_api_ffi::dma_inv_range(start, end);
        }
    }

    pub fn map_area(start: u64, size: u64, dir: i32) {
        unsafe {
            dma_api_ffi::dma_map_area(start, size, dir);
        }
    }

    pub fn unmap_area(start: u64, size: u64, dir: i32) {
        unsafe {
            dma_api_ffi::dma_unmap_area(start, size, dir);
        }
    }

    pub fn clean_range(start: u64, end: u64) {
        unsafe {
            dma_api_ffi::dma_clean_range(start, end);
        }
    }
}
