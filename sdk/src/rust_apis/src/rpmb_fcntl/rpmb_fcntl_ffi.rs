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
use core::ffi::c_char;

extern "C" {
    /// Partition initialization, perform RPMB Key writing and formatting operations
    ///
    /// # Safety
    ///
    /// This function only needs to be executed once
    ///
    /// # Errors
    ///
    /// TEE_SUCCESS  Indicates that the function was executed successfully
    /// TEE_ERROR_RPMB_GENERIC  RPMB controller general error
    /// TEE_ERROR_RPMB_MAC_FAIL  RPMB controller MAC check TEE_ERROR_RPMB_GENERIC
    /// TEE_ERROR_RPMB_RESP_UNEXPECT_MAC  RPMB response data MAC check error
    pub fn TEE_RPMB_FS_Init() -> TeeResult;
}

extern "C" {
    /// RPMB secure storage fully formatted operation
    ///
    /// # Errors
    ///
    /// TEE_SUCCESS  Indicates that the function was executed successfully
    /// TEE_ERROR_RPMB_GENERIC  RPMB controller general error
    /// TEE_ERROR_RPMB_MAC_FAIL  RPMB controller MAC check error
    /// TEE_ERROR_RPMB_RESP_UNEXPECT_MAC  RPMB response data MAC check error
    pub fn TEE_RPMB_FS_Format() -> TeeResult;
}

extern "C" {
    /// Write files to RPMB
    ///
    /// # Safety
    ///
    /// If you want to improve the performance of writing files, you need to
    /// define the heap size in TA's manifest to be at least 3 times the file size plus 256KB,
    /// For example: To write a file with a size of 100KB, the defined heap size is at least
    /// 556KB (3*100+256). If the heap size cannot be satisfied, the file writing will still succeed,
    /// but the performance will be poor.
    ///
    /// # Params
    ///
    /// * filename IN  The file name of the data to be written, the maximum length is 64 bytes
    /// * buf IN  Buffer for writing data
    /// * size IN  The size of the written data, the maximum size is 160KB
    ///
    /// # Errors
    ///
    /// * TEE_SUCCESS  Indicates that the function was executed successfully
    /// * TEE_ERROR_BAD_PARAMETERS The input parameter is incorrect, or the file name is longer than 64 bytes
    /// * TEE_ERROR_RPMB_NOSPC Insufficient disk space on RPMB partition
    pub fn TEE_RPMB_FS_Write(filename: *const c_char, buf: *const u8, size: usize) -> TeeResult;
}

extern "C" {
    /// Read file from RPMB
    ///
    /// # Attention
    ///
    /// If you want to improve the performance of reading files, you need to
    /// define the heap size in TA's manifest to be at least 3 times the file size plus 256KB,
    /// For example: To write a file with a size of 100KB, the defined heap size is at least
    /// 556KB (3*100+256). If the heap size cannot be satisfied, the file reading will still succeed,
    /// but the performance will be poor.
    ///
    /// # Params
    ///
    /// filename IN  The file name of the data to be read, the maximum length is 64 bytes
    /// buf IN  Buffer for reading data
    /// size IN  Read data size
    /// count OUT  The size of the actual read
    ///
    /// # Errors
    ///
    /// TEE_SUCCESS  Indicates that the function was executed successfully
    /// TEE_ERROR_BAD_PARAMETERS  The input parameter is incorrect, or the file name is longer than 64 bytes
    /// TEE_ERROR_RPMB_FILE_NOT_FOUND  File does not exist
    pub fn TEE_RPMB_FS_Read(
        filename: *const c_char,
        buf: *mut u8,
        size: usize,
        count: *mut u32,
    ) -> TeeResult;
}

extern "C" {
    /// Rename file in RPMB
    ///
    /// # Params
    ///
    /// old_name IN  Old file name
    /// new_name IN  New file name
    ///
    /// # Errors
    ///
    /// TEE_SUCCESS  Indicates that the function was executed successfully
    /// TEE_ERROR_BAD_PARAMETERS  The input parameter is incorrect, or the file name is longer than 64 bytes
    /// TEE_ERROR_RPMB_FILE_NOT_FOUND Old file does not exist
    pub fn TEE_RPMB_FS_Rename(old_name: *const c_char, new_name: *const c_char) -> TeeResult;
}

extern "C" {
    /// Delete files in RPMB
    ///
    /// # Params
    ///
    /// filename IN  File name to be deleted
    ///
    /// # Errors
    ///
    /// TEE_SUCCESS  Indicates that the function was executed successfully
    /// TEE_ERROR_BAD_PARAMETERS  The input parameter is incorrect, or the file name is longer than 64 bytes
    /// TEE_ERROR_RPMB_FILE_NOT_FOUND  File does not exist
    pub fn TEE_RPMB_FS_Rm(filename: *const c_char) -> TeeResult;
}

/// File status stored in RPMB partition, used in TEE_RPMB_FS_Stat function
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RpmbFsStat {
    /// File size
    pub size: u32,
    pub reserved: u32,
}
extern "C" {
    /// Get file status in RPMB
    ///
    /// # Params
    ///
    /// filename IN  File name
    /// stat OUT  File status information obtained
    ///
    /// # Errors
    ///
    /// TEE_SUCCESS  Indicates that the function was executed successfully
    /// TEE_ERROR_BAD_PARAMETERS  The input parameter is incorrect, or the file name is longer than 64 bytes
    /// TEE_ERROR_RPMB_FILE_NOT_FOUND  File does not exist
    pub fn TEE_RPMB_FS_Stat(filename: *const c_char, stat: *mut RpmbFsStat) -> TeeResult;
}

/// Disk status stored in RPMB partition, used in TEE_RPMB_FS_StatDisk function
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RpmbFsStatdisk {
    /// Total size of RPMB partition
    pub disk_size: u32,
    /// TA used size
    pub ta_used_size: u32,
    /// RPMB partition free size
    pub free_size: u32,
    pub reserved: u32,
}
extern "C" {
    /// Get disk status
    ///
    /// # Params
    ///
    /// stat OUT  Disk status information obtained
    ///
    /// # Errors
    ///
    /// TEE_SUCCESS  Indicates that the function was executed successfully
    /// TEE_ERROR_BAD_PARAMETERS  The input parameter is incorrect, or the file name is longer than 64 bytes
    /// TeeResult TEE_RPMB_FS_StatDisk(struct RpmbFsStatdisk *stat);
    pub fn TEE_RPMB_FS_StatDisk(stat: *mut RpmbFsStatdisk) -> TeeResult;
}

/// File attribute definition, which means that the file cannot be erased during the factory reset
pub const TEE_RPMB_FMODE_NON_ERASURE: u32 = 1;
/// File attribute definition, which means the attribute value of the cleared file
pub const TEE_RPMB_FMODE_CLEAR: u32 = 0;

extern "C" {
    /// Set file attributes
    ///
    /// # Params
    ///
    /// filename IN  File name
    /// fmode IN File attributes, currently supports TEE_RPMB_FMODE_NON_ERASURE and
    /// TEE_RPMB_FMODE_CLEAR two attributes, other values will return TEE_ERROR_BAD_PARAMETERS
    ///
    /// # Errors
    ///
    /// TEE_SUCCESS Indicates that the function was executed successfully
    /// TEE_ERROR_BAD_PARAMETERS Incorrect input parameters
    /// TEE_ERROR_RPMB_FILE_NOT_FOUND  File does not exist
    pub fn TEE_RPMB_FS_SetAttr(filename: *const c_char, fmode: u32) -> TeeResult;
}
extern "C" {
    /// Format, delete file attribute is erasable file, Keep the file attribute is an inerasable file
    ///
    /// # Errors
    ///
    /// TEE_SUCCESS  Indicates that the function was executed successfully
    /// TEE_ERROR_RPMB_GENERIC  RPMB controller general error
    /// TEE_ERROR_RPMB_MAC_FAIL  RPMB controller MAC check error
    /// TEE_ERROR_RPMB_RESP_UNEXPECT_MAC  RPMB response data MAC check error
    pub fn TEE_RPMB_FS_Erase() -> TeeResult;
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TeeRpmbKeyStatus(pub u32);

impl TeeRpmbKeyStatus {
    pub const INVALID: Self = Self(0);
    /// RPMB Key has been programmed and matched correctly
    pub const SUCCESS: Self = Self(1);
    /// RPMB Key is not programmed
    pub const NOT_PROGRAM: Self = Self(2);
    /// RPMB Key has been programmed but failed to match
    pub const NOT_MATCH: Self = Self(3);
}

extern "C" {
    /// Obtain RPMB Key status
    pub fn TEE_RPMB_KEY_Status() -> TeeRpmbKeyStatus;
}

extern "C" {
    /// Current TA version information
    ///
    /// # Params
    ///
    /// ta_version TA version
    ///
    /// # Errors
    ///
    /// TEE_SUCCESS  Indicates that the function was executed successfully
    /// TEE_ERROR_BAD_PARAMETERS  Incorrect input parameters
    /// TEE_ERROR_GENERIC  Processing failed
    pub fn TEE_RPMB_TAVERSION_Process(ta_version: u32) -> TeeResult;
}
