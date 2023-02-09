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
mod map_api_ffi;
pub use map_api_ffi::CacheModeType;
pub use map_api_ffi::PaddrT;

#[repr(C)]
pub struct MapApi {}

impl MapApi {
    pub fn tee_map_secure<'a>(
        paddr: PaddrT,
        size: u64,
        cache_mode: CacheModeType,
    ) -> Result<&'a mut [u8], i32> {
        let mut vaddr: usize = 0;
        let ret = unsafe { map_api_ffi::tee_map_secure(paddr, size, &mut vaddr, cache_mode) };
        if ret == 0 {
            // safety: if ret == 0, tee promise the vaddr is valid
            let slice = unsafe { core::slice::from_raw_parts_mut(vaddr as _, size as _) };
            return Ok(slice);
        }
        Err(ret)
    }

    pub fn tee_map_nonsecure<'a>(
        paddr: PaddrT,
        size: u64,
        cache_mode: CacheModeType,
    ) -> Result<&'a mut [u8], i32> {
        let mut vaddr: usize = 0;
        let ret = unsafe { map_api_ffi::tee_map_nonsecure(paddr, size, &mut vaddr, cache_mode) };
        if ret == 0 {
            // safety: if ret == 0, tee promise the vaddr is valid
            let slice = unsafe { core::slice::from_raw_parts_mut(vaddr as _, size as _) };
            return Ok(slice);
        }
        Err(ret)
    }
}
