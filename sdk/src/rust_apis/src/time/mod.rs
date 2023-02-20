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
mod time_ffi;
use crate::tee_defines::TeeResult;

pub use time_ffi::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TeeTime {
    pub seconds: u32,
    pub millis: u32,
}

pub enum TeeTimeError {
    TimeErrorCancel,
    TimeErrorTimeNotSet,
    TimeErrorNeedsReset,
    TimeErrorOverflow,
    TimeErrorTimeOutOfMemory,
    TimeErrorStorageNoSpace,
}

impl TeeTime {
    pub fn new() -> Self {
        TeeTime {
            seconds: 0,
            millis: 0,
        }
    }

    pub fn get_system_time(&mut self) {
        unsafe {
            TEE_GetSystemTime(self as *mut _ as _);
        }
    }

    pub fn get_ree_time(&mut self) {
        unsafe {
            TEE_GetREETime(self as *mut _ as _);
        }
    }

    pub fn wait(timeout: u32) -> Result<(), TeeTimeError> {
        match unsafe { TEE_Wait(timeout) } {
            TeeResult::TEE_SUCCESS => Ok(()),
            TeeResult::TEE_ERROR_CANCEL => Err(TeeTimeError::TimeErrorCancel),
            _ => unreachable!("GPD specification"),
        }
    }

    pub fn get_ta_persistent_time(&mut self) -> crate::error::FfiResult {
        unsafe { TEE_GetTAPersistentTime(self as *mut _ as _) }.into()
    }

    pub fn set_ta_persistent_time(&mut self) -> crate::error::FfiResult {
        unsafe { TEE_SetTAPersistentTime(self as *mut _ as _) }.into()
    }
}

impl Default for TeeTime {
    fn default() -> Self {
        TeeTime::new()
    }
}
