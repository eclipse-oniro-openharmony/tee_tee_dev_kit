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
use crate::print::{tee_print, LogLevel};
pub use crate::tee_defines::*;
use core::{
    ffi::{c_void, CStr},
    ptr::null,
};

mod drv_client_api_ffi;

#[repr(transparent)]
pub struct DrvHandle {
    fd: i64,
}

impl DrvHandle {
    pub fn open(drv_name: &CStr, param: Option<&[u8]>) -> Result<DrvHandle, i64> {
        let (addr, len) = match param {
            Some(p) => (p.as_ptr() as *const c_void, p.len() as u32),
            None => (null(), 0u32),
        };
        let rfd = unsafe { drv_client_api_ffi::tee_drv_open(drv_name.as_ptr() as _, addr, len) };
        if rfd <= 0 {
            Err(rfd)
        } else {
            Ok(DrvHandle { fd: rfd })
        }
    }

    pub fn ioctl(&self, cmd_id: u32, param: Option<&[u8]>) -> i64 {
        let (addr, len) = match param {
            Some(p) => (p.as_ptr() as *const c_void, p.len() as u32),
            None => (null(), 0u32),
        };
        unsafe { drv_client_api_ffi::tee_drv_ioctl(self.fd, cmd_id, addr, len) }
    }
}

impl Drop for DrvHandle {
    fn drop(&mut self) {
        if self.fd > 0 {
            let ret = unsafe { drv_client_api_ffi::tee_drv_close(self.fd) };
            if ret != 0 {
                unsafe {
                    tee_print(
                        LogLevel::ERROR,
                        "close drv failed ret %lld\0".as_ptr() as _,
                        ret,
                    );
                }
            }
        }
    }
}
