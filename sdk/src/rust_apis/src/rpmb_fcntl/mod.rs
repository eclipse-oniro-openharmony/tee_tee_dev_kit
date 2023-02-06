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
use core::mem::MaybeUninit;
use core::num::NonZeroU32;

use core::ffi::CStr;

pub use rpmb_fcntl_ffi::{RpmbFsStat, RpmbFsStatdisk};

use crate::error::{FfiResult, FfiTeeError, TeeError};
use crate::rpmb_fcntl::rpmb_fcntl_ffi::TeeRpmbKeyStatus;
use crate::TeeResult;

mod rpmb_fcntl_ffi;

pub type RpmbKeyResult = Result<(), RpmbKeyError>;

pub struct RpmbFs;

/// RPMB Key status, used in TEE_RPMB_KEY_Status function
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RpmbKeyError {
    Invalid,
    /// RPMB Key is not programmed
    NotProgram,
    /// RPMB Key has been programmed but failed to match
    NotMatch,
    /// Unknown Error code
    Unknown(TeeRpmbKeyStatus),
}

impl From<RpmbKeyError> for u32 {
    fn from(e: RpmbKeyError) -> Self {
        match e {
            RpmbKeyError::Invalid => TeeRpmbKeyStatus::INVALID.0,
            RpmbKeyError::NotProgram => TeeRpmbKeyStatus::NOT_PROGRAM.0,
            RpmbKeyError::NotMatch => TeeRpmbKeyStatus::NOT_MATCH.0,
            RpmbKeyError::Unknown(err_code) => err_code.0,
        }
    }
}

impl From<TeeRpmbKeyStatus> for RpmbKeyResult {
    fn from(status: TeeRpmbKeyStatus) -> Self {
        match status {
            TeeRpmbKeyStatus::SUCCESS => Ok(()),
            TeeRpmbKeyStatus::INVALID => Err(RpmbKeyError::Invalid),
            TeeRpmbKeyStatus::NOT_PROGRAM => Err(RpmbKeyError::NotProgram),
            TeeRpmbKeyStatus::NOT_MATCH => Err(RpmbKeyError::NotMatch),
            err_code => Err(RpmbKeyError::Unknown(err_code)),
        }
    }
}

#[derive(Copy, Clone)]
pub enum FileMode {
    NonErasure,
    Clear,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum FsError {
    /// TEE_ERROR_RPMB_GENERIC  RPMB controller general error
    Generic,
    /// TEE_ERROR_RPMB_MAC_FAIL  RPMB controller MAC check error
    MacFail,
    /// TEE_ERROR_RPMB_RESP_UNEXPECT_MAC  RPMB response data MAC check error
    UnexpectedMac,
    Unknown(FfiTeeError),
}

impl From<FfiTeeError> for FsError {
    fn from(e: FfiTeeError) -> Self {
        let tee_error: Result<TeeError, FfiTeeError> = e.try_into();
        match tee_error {
            Ok(TeeError::RpmbGeneric) => Self::Generic,
            Ok(TeeError::RpmbMacFail) => Self::MacFail,
            Ok(TeeError::RpmbRespUnexpectMac) => Self::UnexpectedMac,
            _ => Self::Unknown(e),
        }
    }
}

impl From<TeeResult> for Result<(), FsError> {
    fn from(res: TeeResult) -> Self {
        let r: FfiResult = res.into();
        r?;
        Ok(())
    }
}

impl FsError {
    pub fn raw_err(&self) -> NonZeroU32 {
        match self {
            FsError::Generic => TeeError::RpmbGeneric.into(),
            FsError::MacFail => TeeError::RpmbMacFail.into(),
            FsError::UnexpectedMac => TeeError::RpmbRespUnexpectMac.into(),
            FsError::Unknown(e) => *e,
        }
    }
}

pub enum RwError {
    /// The input parameter is incorrect, e.g. the file name is longer than 64 bytes
    BadParameters,
    /// Insufficient disk space on RPMB partition
    RpmbNoSpace,
    /// File does not exist
    FileNotFound,
    /// Unknown error code
    Unknown(FfiTeeError),
}

impl From<FfiTeeError> for RwError {
    fn from(e: FfiTeeError) -> Self {
        let tee_error: Result<TeeError, FfiTeeError> = e.try_into();
        match tee_error {
            Ok(TeeError::BadParameters) => Self::BadParameters,
            Ok(TeeError::RpmbNospc) => Self::RpmbNoSpace,
            Ok(TeeError::RpmbFileNotFound) => Self::FileNotFound,
            _ => Self::Unknown(e),
        }
    }
}

impl From<TeeResult> for Result<(), RwError> {
    fn from(res: TeeResult) -> Self {
        let r: FfiResult = res.into();
        r?;
        Ok(())
    }
}

impl From<FileMode> for u32 {
    fn from(val: FileMode) -> Self {
        match val {
            FileMode::NonErasure => rpmb_fcntl_ffi::TEE_RPMB_FMODE_NON_ERASURE,
            FileMode::Clear => rpmb_fcntl_ffi::TEE_RPMB_FMODE_CLEAR,
        }
    }
}

impl RpmbFs {
    pub fn init() -> Result<(), FsError> {
        let res: FfiResult = unsafe { rpmb_fcntl_ffi::TEE_RPMB_FS_Init() }.into();
        res?;
        Ok(())
    }

    pub fn format() -> Result<(), FsError> {
        let res: FfiResult = unsafe { rpmb_fcntl_ffi::TEE_RPMB_FS_Format() }.into();
        res?;
        Ok(())
    }

    pub fn write_file(file_name: &CStr, data: &[u8]) -> Result<(), RwError> {
        unsafe {
            rpmb_fcntl_ffi::TEE_RPMB_FS_Write(file_name.as_ptr() as _, data.as_ptr(), data.len())
        }
        .into()
    }

    pub fn read_file(file_name: &CStr, data: &mut [u8]) -> Result<u32, RwError> {
        let mut count: u32 = 0;
        let result: FfiResult = unsafe {
            rpmb_fcntl_ffi::TEE_RPMB_FS_Read(
                file_name.as_ptr() as _,
                data.as_mut_ptr() as _,
                data.len() as _,
                &mut count as _,
            )
        }
        .into();
        result?;
        Ok(count)
    }

    pub fn rename_file(old_name: &CStr, new_name: &CStr) -> Result<(), FsError> {
        unsafe {
            rpmb_fcntl_ffi::TEE_RPMB_FS_Rename(old_name.as_ptr() as _, new_name.as_ptr() as _)
        }
        .into()
    }

    pub fn delete_file(file_name: &CStr) -> Result<(), RwError> {
        unsafe { rpmb_fcntl_ffi::TEE_RPMB_FS_Rm(file_name.as_ptr() as _) }.into()
    }

    pub fn get_file_status(file_name: &CStr) -> Result<RpmbFsStat, RwError> {
        let mut status = MaybeUninit::<RpmbFsStat>::uninit();
        unsafe {
            let res: FfiResult =
                rpmb_fcntl_ffi::TEE_RPMB_FS_Stat(file_name.as_ptr() as _, status.as_mut_ptr())
                    .into();
            res?;
            Ok(status.assume_init())
        }
    }

    pub fn get_disk_status() -> Result<RpmbFsStatdisk, RwError> {
        let mut status = MaybeUninit::<RpmbFsStatdisk>::uninit();
        unsafe {
            let res: FfiResult = rpmb_fcntl_ffi::TEE_RPMB_FS_StatDisk(status.as_mut_ptr()).into();
            res?;
            Ok(status.assume_init())
        }
    }

    pub fn set_file_attribute(file_name: &CStr, fmode: FileMode) -> Result<(), RwError> {
        unsafe { rpmb_fcntl_ffi::TEE_RPMB_FS_SetAttr(file_name.as_ptr() as _, fmode.into()) }.into()
    }

    pub fn erase() -> Result<(), FsError> {
        let res: FfiResult = unsafe { rpmb_fcntl_ffi::TEE_RPMB_FS_Erase() }.into();
        res?;
        Ok(())
    }

    pub fn get_key_status() -> RpmbKeyResult {
        unsafe { rpmb_fcntl_ffi::TEE_RPMB_KEY_Status() }.into()
    }

    pub fn set_ta_version(ta_version: u32) -> crate::error::FfiResult {
        unsafe { rpmb_fcntl_ffi::TEE_RPMB_TAVERSION_Process(ta_version) }.into()
    }
}
