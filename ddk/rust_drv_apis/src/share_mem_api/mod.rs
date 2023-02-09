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
mod share_mem_api_ffi;
use core::ffi::CStr;

use crate::framework::tee_defines::TeeUuid;

pub const MAX_TAG_LEN: usize = 32;
pub const TLV_SHAREDMEM_SUCCESS: i32 = 0;

#[repr(C)]
pub struct ShareMemApi<'a> {
    addr: &'a mut [u8],
}

impl ShareMemApi<'_> {
    pub fn get_tlv_sharedmem(
        t_type: &CStr,
        buffer: &mut [u8],
        clear_flag: bool,
    ) -> Result<u32, i32> {
        let mut buffer_size = buffer.len() as u32;
        let type_size = t_type.to_bytes().len() + 1;
        let ret = unsafe {
            share_mem_api_ffi::get_tlv_sharedmem(
                t_type.as_ptr() as _,
                type_size as _,
                buffer.as_mut_ptr() as _,
                &mut buffer_size,
                clear_flag,
            )
        };
        if ret == TLV_SHAREDMEM_SUCCESS {
            return Ok(buffer_size);
        }
        Err(ret)
    }

    pub fn get(&self) -> &[u8] {
        self.addr
    }

    pub fn get_mut(&mut self) -> &mut [u8] {
        self.addr
    }

    pub fn alloc(uuid: &TeeUuid, size: u32) -> Result<ShareMemApi, ()> {
        let ret = unsafe { share_mem_api_ffi::tee_alloc_sharemem_aux(uuid, size) as *mut u8 };
        if ret.is_null() {
            return Err(());
        }
        let raddr = unsafe { core::slice::from_raw_parts_mut(ret, size as _) };
        Ok(ShareMemApi { addr: raddr })
    }
}

impl Drop for ShareMemApi<'_> {
    fn drop(&mut self) {
        let _ = unsafe {
            share_mem_api_ffi::tee_free_sharemem(self.addr.as_mut_ptr() as _, self.addr.len() as _)
        };
    }
}
