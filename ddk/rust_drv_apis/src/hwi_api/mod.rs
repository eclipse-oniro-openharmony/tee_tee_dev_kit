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
mod hwi_api_ffi;
pub use self::hwi_api_ffi::HwiProcFunc;

#[repr(C)]
pub struct HwiApi {
    pub _hwi_num: u32,
}

impl HwiApi {
    pub fn create(&self, hwi_prio: u16, mode: u16, handler: HwiProcFunc, args: u32) -> u32 {
        unsafe { hwi_api_ffi::sys_hwi_create(self._hwi_num, hwi_prio, mode, handler, args) }
    }

    pub fn resume(&self, hwi_prio: u16, mode: u16) -> u32 {
        unsafe { hwi_api_ffi::sys_hwi_resume(self._hwi_num, hwi_prio, mode) }
    }

    pub fn delete(&self) -> u32 {
        unsafe { hwi_api_ffi::sys_hwi_delete(self._hwi_num) }
    }

    pub fn disable(&self) -> u32 {
        unsafe { hwi_api_ffi::sys_hwi_disable(self._hwi_num) }
    }

    pub fn enable(&self) -> u32 {
        unsafe { hwi_api_ffi::sys_hwi_enable(self._hwi_num) }
    }

    pub fn notify(&self) -> u32 {
        unsafe { hwi_api_ffi::sys_hwi_notify(self._hwi_num) }
    }
}
