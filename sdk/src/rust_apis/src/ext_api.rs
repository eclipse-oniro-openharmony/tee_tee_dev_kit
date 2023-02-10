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
use crate::security::TaCallerInfo;
pub use crate::tee_defines::{TeeParam, TeeResult};
use core::ffi::c_char;

/*
 * below definitions are defined by Platform SDK released previously
 * for compatibility:
 * don't make any change to the content below
*/
pub const TEE_INFO_RESERVED: u32 = 0xFFFFFFFF;
pub const TEE_RETURN_AGENT_BUFFER: u32 = 0x99;
pub const TEE_INVALID_AGENT: u32 = 0x66;
pub const TEE_AGENT_LOCK: u32 = 0x33;
pub const TEE_GET_REEINFO_SUCCESS: u32 = 0;
pub const TEE_GET_REEINFO_FAILED: u32 = 1;
pub const RESERVED_BUF_SIZE: u32 = 32;
pub const SESSION_FROM_CA: u32 = 0;
pub const SESSION_FROM_TA: u32 = 1;
pub const SESSION_FROM_UNKNOWN: u32 = 255;

extern "C" {
    #[doc = " Get caller info of current session, refer caller_info struct for more details"]
    #[doc = ""]
    #[doc = " @param caller_info_data OUT caller info to be returned"]
    #[doc = " @param length           IN sizeof struct caller_info"]
    #[doc = ""]
    #[doc = " return TEE_SUCCESS operation success"]
    #[doc = " return others failed to get caller info"]
    pub fn TEE_EXT_GetCallerInfo(caller_info_data: *mut TaCallerInfo, length: u32) -> TeeResult;
}

extern "C" {
    /// TA can call this API to add caller's info,  which is allowed to call this TA.
    ///
    /// This API is for CA in form of a binary-executable file. On success
    /// [TEE_SUCCESS](super::tee_defines::TEE_Result_Value::TEE_SUCCESS) is returned.
    ///
    /// ## Arguments
    /// * `ca_name`: Read-only - A pointer to a C-string containing the CA caller's process name.
    ///              Ownerships remains the callers.
    /// * `ca_uid`: CA caller's uid
    pub(crate) fn AddCaller_CA_exec(ca_name: *const c_char, ca_uid: u32) -> TeeResult;
}

extern "C" {
    #[doc = " TA can call this API to add caller's info,"]
    #[doc = " which is allowed to call this TA."]
    #[doc = " this API is for CA in form of APK."]
    #[doc = ""]
    #[doc = " @param ca_name       IN        CA caller's packagename"]
    #[doc = " @param modulus       IN        CA caller's modulus"]
    #[doc = " @param pub_exponent  IN        CA caller's pub_exponent"]
    #[doc = ""]
    #[doc = " return TEE_SUCCESS operation"]
    #[doc = " return others failed to add caller info for target CA"]
    pub fn AddCaller_CA_apk(
        ca_name: *const c_char,
        modulus: *const c_char,
        pub_exponent: *const c_char,
    ) -> TeeResult;
}

extern "C" {
    #[doc = " TA call this API allow others TA open session with itself"]
    #[doc = ""]
    #[doc = " return TEE_SUCCESS operation success"]
    #[doc = " result others operation failed"]
    pub fn AddCaller_TA_all() -> TeeResult;
}
