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
mod mem_copy_api_ffi;

#[repr(C)]
pub struct MemCopyApi {}

impl MemCopyApi {
    pub fn copy_from_client(src: u64, src_size: u32, dst: &mut [u8]) -> i32 {
        unsafe {
            mem_copy_api_ffi::copy_from_client(src, src_size, dst.as_mut_ptr() as _, dst.len() as _)
        }
    }

    pub fn copy_to_client(src: &[u8], dst: u64, dst_size: u32) -> i32 {
        unsafe {
            mem_copy_api_ffi::copy_to_client(src.as_ptr() as _, src.len() as _, dst, dst_size)
        }
    }
}
