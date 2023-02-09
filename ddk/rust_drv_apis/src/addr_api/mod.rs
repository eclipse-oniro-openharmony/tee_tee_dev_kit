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
mod addr_api_ffi;
use crate::framework::defines::VsrootInfoT;

#[repr(C)]
pub struct AddrApi {}

impl AddrApi {
    pub fn virt_to_phys(addr: usize) -> u64 {
        unsafe { addr_api_ffi::drv_virt_to_phys(addr) }
    }

    pub fn get_vsrootinfo() -> Result<VsrootInfoT, i32> {
        let mut info = VsrootInfoT {
            pgd: 0,
            pud: 0,
            asid: 0,
        };
        let ret = unsafe { addr_api_ffi::drv_get_vsrootinfo(&mut info) };
        if ret == 0 {
            return Ok(info);
        }
        Err(ret)
    }
}
