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

//!
//! the IoApi is for io memory copy
//!

use crate::framework::defines::MAP_FAILED;

mod io_api_ffi;

pub const PROT_NONE: u32 = 0;

pub const PROT_READ: u32 = 1;

pub const PROT_WRITE: u32 = 2;

pub const PROT_EXEC: u32 = 4;

pub const PROT_GROWSDOWN: u32 = 0x01000000;

pub const PROT_GROWSUP: u32 = 0x02000000;

#[repr(C)]
pub struct IoApi {}

impl IoApi {
    pub fn read(dst: &mut [u8], src: usize) {
        unsafe {
            io_api_ffi::read_from_io(dst.as_mut_ptr() as _, src as _, dst.len() as _);
        }
    }

    pub fn write(dst: usize, src: &[u8]) {
        unsafe {
            io_api_ffi::write_to_io(dst as _, src.as_ptr() as _, src.len() as _);
        }
    }

    pub fn ioremap<'a>(paddr: usize, size: usize, prot: i32) -> Option<&'a mut [u8]> {
        let res = unsafe { io_api_ffi::ioremap(paddr, size, prot) as *mut u8 };
        if res as u64 == MAP_FAILED {
            return None;
        }
        // safety: if ioremap success, tee promise the res is valid
        unsafe { Some(core::slice::from_raw_parts_mut(res, size)) }
    }

    pub fn iounmap(paddr: usize, addr: usize) -> i32 {
        unsafe { io_api_ffi::iounmap(paddr, addr as _) }
    }
}
