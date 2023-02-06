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
pub use crate::tee_defines::*;

pub const _SF_BINDING_KEY_LEN_IN_BYTES: u32 = 48;

#[repr(transparent)]
#[derive(Eq, PartialEq)]
pub struct FfiSecFlashStatus(pub u32);

/// For Baltimore, we just use SECFLASH_IS_EXIST_MAGIC and SECFLASH_IS_ABSENCE_MAGIC
impl FfiSecFlashStatus {
    /// See `SECFLASH_IS_ABSENCE_MAGIC`
    pub const ABSENT: Self = Self(0x70eb2c2d);
    /// See `SECFLASH_IS_EXIST_MAGIC`
    pub const EXISTS: Self = Self(0x8f14d3d2);
    /// See `SECFLASH_NXP_EXIST_MAGIC`
    pub const NXP_EXIST: Self = Self(0xa5c89cea);
    /// See `SECFLASH_ST_EXIST_MAGIC`
    pub const ST_EXIST: Self = Self(0xe59a6b89);
}

pub const SECFLASH_RESET_TYPE_SOFT: u32 = 0;
pub const SECFLASH_RESET_TYPE_HARD: u32 = 1;

extern "C" {
    pub fn TEE_EXT_SecFlashIsAvailable(status_info: *mut FfiSecFlashStatus) -> TeeResult;

    pub fn TEE_EXT_SecFlashFactoryRecovery(flags: u32) -> TeeResult;

    pub fn TEE_EXT_SecFlashPowerSaving() -> TeeResult;

    pub fn TEE_EXT_SecFlashReset(reset_type: u32) -> TeeResult;

    pub fn TEE_EXT_SecFlashGetBindingKey(key_buf: *mut u8, buf_len: u32) -> TeeResult;

    pub fn TEE_EXT_SecFlashWriteLockEnable(is_set_operation: i32) -> TeeResult;
}
