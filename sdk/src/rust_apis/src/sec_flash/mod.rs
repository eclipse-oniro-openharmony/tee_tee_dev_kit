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
use crate::error::{FfiResult, FfiTeeError};
use crate::sec_flash::sec_flash_ffi::FfiSecFlashStatus;

mod sec_flash_ffi;

pub struct SecFlash;

#[derive(Eq, PartialEq)]
pub enum SecFlashStatus {
    Absence,
    Exist,
    NxpExist,
    StExist,
    Unknown(FfiSecFlashStatus),
}

impl SecFlashStatus {
    pub fn raw(&self) -> u32 {
        match self {
            SecFlashStatus::Absence => FfiSecFlashStatus::ABSENT.0,
            SecFlashStatus::Exist => FfiSecFlashStatus::EXISTS.0,
            SecFlashStatus::NxpExist => FfiSecFlashStatus::NXP_EXIST.0,
            SecFlashStatus::StExist => FfiSecFlashStatus::ST_EXIST.0,
            SecFlashStatus::Unknown(s) => s.0,
        }
    }
}

impl From<FfiSecFlashStatus> for SecFlashStatus {
    fn from(value: FfiSecFlashStatus) -> Self {
        match value {
            FfiSecFlashStatus::ABSENT => Self::Absence,
            FfiSecFlashStatus::EXISTS => Self::Exist,
            FfiSecFlashStatus::NXP_EXIST => Self::NxpExist,
            FfiSecFlashStatus::ST_EXIST => Self::StExist,
            unknown_status => Self::Unknown(unknown_status),
        }
    }
}

pub enum ResetType {
    Soft,
    Hard,
}

impl From<ResetType> for u32 {
    fn from(val: ResetType) -> Self {
        match val {
            ResetType::Soft => sec_flash_ffi::SECFLASH_RESET_TYPE_SOFT,
            ResetType::Hard => sec_flash_ffi::SECFLASH_RESET_TYPE_HARD,
        }
    }
}

impl SecFlash {
    pub fn status() -> Result<SecFlashStatus, FfiTeeError> {
        let mut status = FfiSecFlashStatus(0);
        let res: FfiResult =
            unsafe { sec_flash_ffi::TEE_EXT_SecFlashIsAvailable(&mut status as _) }.into();
        res?;
        Ok(status.into())
    }

    pub fn factory_recovery(flags: u32) -> crate::error::FfiResult {
        unsafe { sec_flash_ffi::TEE_EXT_SecFlashFactoryRecovery(flags) }.into()
    }

    pub fn power_saving() -> crate::error::FfiResult {
        unsafe { sec_flash_ffi::TEE_EXT_SecFlashPowerSaving() }.into()
    }

    pub fn reset(reset_type: ResetType) -> crate::error::FfiResult {
        unsafe { sec_flash_ffi::TEE_EXT_SecFlashReset(reset_type.into()) }.into()
    }

    pub fn get_binding_key(key_buff: &mut [u8]) -> crate::error::FfiResult {
        unsafe {
            sec_flash_ffi::TEE_EXT_SecFlashGetBindingKey(
                key_buff.as_mut_ptr() as _,
                key_buff.len() as _,
            )
        }
        .into()
    }

    pub fn write_lock_enable(op: WriteLockOperation) -> FfiResult {
        unsafe { sec_flash_ffi::TEE_EXT_SecFlashWriteLockEnable(op.into()) }.into()
    }
}

pub enum WriteLockOperation {
    Get,
    Set,
}

impl From<WriteLockOperation> for i32 {
    /// Map to `c_bool`
    fn from(op: WriteLockOperation) -> Self {
        match op {
            WriteLockOperation::Set => true as Self,
            WriteLockOperation::Get => false as Self,
        }
    }
}
