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

// no_std means that we do not need to use std::*;
#![no_std]

use core::ffi::c_void;
use core::ffi::CStr;
use core::num::NonZeroU32;

use rust_apis::{
    tlogi,
    {
        error::{FfiResult, TeeError},
        parameters::{MemrefSlice, RustParameter, RustParameters},
        security::add_caller_ca_exec,
        tee_defines::{TeeParam, TeeParamTypes, TeeResult},
    },
};

/// Length of the [TeeParam](crate::c_bindings::TEE_Param) array passed to e.g. [TA_OpenSessionEntryPoint].
const PARAM_COUNT: usize = 4;

#[repr(u32)]
enum CmdVariants {
    CmdGetTaVersion = 1,
}

impl From<CmdVariants> for u32 {
    fn from(cmd: CmdVariants) -> u32 {
        cmd as u32
    }
}

impl TryFrom<u32> for CmdVariants {
    type Error = ();

    fn try_from(cmd: u32) -> core::result::Result<Self, Self::Error> {
        match cmd {
            x if x == Self::CmdGetTaVersion.into() => Ok(Self::CmdGetTaVersion),
            _ => Err(()),
        }
    }
}

fn get_ta_version(param: &mut MemrefSlice<u8>) -> FfiResult {
    let version = CStr::from_bytes_with_nul(b"demo_20220321\0").unwrap();
    let version_as_bytes = version.to_bytes_with_nul();

    param
        .get_slice_mut()
        .get_mut(0..version_as_bytes.len())
        .ok_or(NonZeroU32::new(0xffff0010).unwrap())?
        .copy_from_slice(version_as_bytes);

    let res = param.set_slice_len(version_as_bytes.len());
    match res {
        Ok(()) => Ok(()),
        Err(_e) => Err(NonZeroU32::new(0xffff0010).unwrap()),
    }
}

// tee functions here..
#[no_mangle]
pub extern "C" fn TA_CreateEntryPoint() -> TeeResult {
    tlogi!("----- Rust TA entry point -----");
    let exe_name = CStr::from_bytes_with_nul(b"/vendor/bin/test_rust_hello_world\0").unwrap();
    match add_caller_ca_exec(exe_name, 0) {
        Ok(_) => TeeResult::TEE_SUCCESS,
        Err(e) => TeeResult(u32::from(e)),
    }
}

#[no_mangle]
pub extern "C" fn TA_OpenSessionEntryPoint(
    _param_type: TeeParamTypes,
    // Option<&mut x>:None means NULL ptr, Some(&mut x) means ptr not NULL
    _params: Option<&mut [TeeParam; PARAM_COUNT]>,
    // > The Trusted Application instance can register a session data pointer by setting
    // > *sessionContext. The framework SHALL ensure that sessionContext is a valid address
    // > of a pointer, and that it is unique per TEE Client session.
    // From GPD Internal Core API 1.2.  framwork must make sure that session_context is not NULL
    _session_context: &mut *mut c_void,
) -> TeeResult {
    tlogi!("---- TA open session --------");
    let world = "World";
    tlogi!("Hello {}", world);
    let a = 4_u8;
    let b = 0xFF_u64;
    tlogi!("Print 4 (u8): {}, FF (u64) {}", a, b);
    TeeResult::TEE_SUCCESS
}

#[no_mangle]
pub extern "C" fn TA_InvokeCommandEntryPoint(
    _session_context: Option<&mut c_void>,
    cmd: u32,
    param_type: TeeParamTypes,
    params: Option<&mut [TeeParam; PARAM_COUNT]>,
) -> TeeResult {
    tlogi!("---- TA invoke command --------");

    if let Ok(command) = cmd.try_into() {
        match command {
            CmdVariants::CmdGetTaVersion => {
                let res =
                    RustParameters::<u8, u8, u8, u8>::convert_from_tee_params(param_type, params);
                match res {
                    Ok(param) => {
                        if let RustParameter::MemrefOutput(mut memref) = param.p3 {
                            get_ta_version(&mut memref).into()
                        } else {
                            TeeError::BadParameters.into()
                        }
                    }
                    Err(_e) => TeeError::BadParameters.into(),
                }
            }
        }
    } else {
        tlogi!("Error: Unknown command: {}", cmd);
        TeeResult::TEE_ERROR_BAD_PARAMETERS
    }
}

#[no_mangle]
pub extern "C" fn TA_CloseSessionEntryPoint(_session_context: Option<&mut c_void>) {
    tlogi!("---- close session ----- \n");
}

#[no_mangle]
pub extern "C" fn TA_DestroyEntryPoint() {
    tlogi!("---- destroy TA (new macro) -----");
}
