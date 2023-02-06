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
use core::ffi::CStr;

use crate::{
    error::FfiResult,
    ext_api::{AddCaller_CA_apk, AddCaller_CA_exec, AddCaller_TA_all, TEE_EXT_GetCallerInfo},
};

/// Allow a binary-executable CA to call this TA.
///
/// ## Arguments
/// * `ca_name`: The process name of the CA.
/// * `ca_uid`: The uid of the CA.
pub fn add_caller_ca_exec(ca_name: &CStr, ca_uid: u32) -> FfiResult {
    // SAFETY: `ca_name` lives at least as long as the function, and `AddCaller_CA_exec()`
    // promises to not modify the data behind the pointer. Additionally the CA will not store
    // the pointer after the `AddCaller_CA_exec` has returned.
    unsafe { AddCaller_CA_exec(ca_name.as_ptr() as _, ca_uid).into() }
}

/// Allow a APK to call this TA.
///
/// ## Arguments
/// * `ca_name`: CA caller's packagename.
/// * `modulus`: CA caller's modulus
/// * `pub_exponent`: CA caller's pub_exponent
pub fn add_caller_ca_apk(ca_name: &CStr, modulus: &CStr, pub_exponent: &CStr) -> FfiResult {
    // SAFETY: `ca_name` `modulus` `pub_exponent` lives at least as long as the function, and
    // `AddCaller_CA_apk()` promises to not modify the data behind the pointer.
    // Additionally the CA will not store the pointer after the `AddCaller_CA_apk` has returned.
    unsafe {
        AddCaller_CA_apk(
            ca_name.as_ptr() as _,
            modulus.as_ptr() as _,
            pub_exponent.as_ptr() as _,
        )
        .into()
    }
}

/// Allow all TA to call this CA
///
/// ## Arguments
/// none
pub fn add_caller_ta_all() -> FfiResult {
    // SAFETY: no data in or out
    unsafe { AddCaller_TA_all() }.into()
}

pub fn tee_ext_get_caller_info(caller_info_data: &mut [u8]) -> FfiResult {
    unsafe {
        TEE_EXT_GetCallerInfo(
            caller_info_data.as_ptr() as _,
            caller_info_data.len() as u32,
        )
    }
    .into()
}
