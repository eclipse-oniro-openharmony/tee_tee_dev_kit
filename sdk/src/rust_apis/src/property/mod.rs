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
use crate::error::{FfiResult, FfiTeeError, TeeError};
use core::ffi::CStr;

use super::tee_defines::{TeeIdentity, TeeUuid};

mod property_ffi;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct PseudoPropSetHandle(u32);

impl PseudoPropSetHandle {
    pub const TEE_PROPSET_UNKNOW: PseudoPropSetHandle = PseudoPropSetHandle(0);
    pub const TEE_PROPSET_TEE_IMPLEMENTATION: PseudoPropSetHandle = PseudoPropSetHandle(0xFFFFFFFD);
    pub const TEE_PROPSET_CURRENT_CLIENT: PseudoPropSetHandle = PseudoPropSetHandle(0xFFFFFFFE);
    pub const TEE_PROPSET_CURRENT_TA: PseudoPropSetHandle = PseudoPropSetHandle(0xFFFFFFFF);
}

/// defines a set of functions to access individual properties in a property set, to convert them into a
/// variety of types (printable strings, integers, Booleans, binary blocks, etc.), and to enumerate the properties in
/// a property set. These functions can be used to access TA Configuration Properties, Client Properties, and
/// Implementation Properties.
pub trait PropertySet {
    fn get_property_as_string(
        &self,
        name: Option<&CStr>,
        buf: &mut [u8],
        len: &mut usize,
    ) -> FfiResult;

    fn get_property_as_bool(&self, name: Option<&CStr>) -> Result<bool, FfiTeeError>;

    fn get_property_as_u32(&self, name: Option<&CStr>) -> Result<u32, FfiTeeError>;

    fn get_property_as_u64(&self, name: Option<&CStr>) -> Result<u64, FfiTeeError>;

    fn get_property_as_binary_block(
        &self,
        name: Option<&CStr>,
        value: &mut [u8],
    ) -> Result<usize, FfiTeeError>;

    fn get_property_as_uuid(&self, name: Option<&CStr>, value: &mut TeeUuid) -> FfiResult;

    fn get_property_as_identity(&self, name: Option<&CStr>, value: &mut TeeIdentity) -> FfiResult;
}

pub struct PseudoPropSet(PseudoPropSetHandle);
impl PropertySet for PseudoPropSet {
    fn get_property_as_string(
        &self,
        name: Option<&CStr>,
        buf: &mut [u8],
        len: &mut usize,
    ) -> FfiResult {
        match name {
            Some(property_name) => {
                let res: FfiResult = unsafe {
                    property_ffi::TEE_GetPropertyAsString(
                        self.0,
                        property_name.as_ptr() as _,
                        buf.as_mut_ptr() as _,
                        len as _,
                    )
                }
                .into();
                res
            }
            None => Err(TeeError::BadParameters.into()),
        }
    }

    fn get_property_as_bool(&self, name: Option<&CStr>) -> Result<bool, FfiTeeError> {
        match name {
            Some(property_name) => {
                let mut value: i32 = false as _;
                let res: FfiResult = unsafe {
                    property_ffi::TEE_GetPropertyAsBool(
                        self.0,
                        property_name.as_ptr() as _,
                        &mut value,
                    )
                }
                .into();
                res?;
                if value == 0 {
                    Ok(false)
                } else {
                    Ok(true)
                }
            }
            None => Err(TeeError::BadParameters.into()),
        }
    }

    fn get_property_as_u32(&self, name: Option<&CStr>) -> Result<u32, FfiTeeError> {
        let mut value = u32::default();
        match name {
            Some(property_name) => {
                let res: FfiResult = unsafe {
                    property_ffi::TEE_GetPropertyAsU32(
                        self.0,
                        property_name.as_ptr() as _,
                        &mut value,
                    )
                }
                .into();
                res?;
                Ok(value)
            }
            None => Err(TeeError::BadParameters.into()),
        }
    }

    fn get_property_as_u64(&self, name: Option<&CStr>) -> Result<u64, FfiTeeError> {
        let mut value = u64::default();
        match name {
            Some(property_name) => {
                let res: FfiResult = unsafe {
                    property_ffi::TEE_GetPropertyAsU64(
                        self.0,
                        property_name.as_ptr() as _,
                        &mut value,
                    )
                }
                .into();
                res?;
                Ok(value)
            }
            None => Err(TeeError::BadParameters.into()),
        }
    }

    fn get_property_as_binary_block(
        &self,
        name: Option<&CStr>,
        value: &mut [u8],
    ) -> Result<usize, FfiTeeError> {
        let mut value_len = value.len();
        match name {
            Some(property_name) => {
                let res: FfiResult = unsafe {
                    property_ffi::TEE_GetPropertyAsBinaryBlock(
                        self.0,
                        property_name.as_ptr() as _,
                        value.as_mut_ptr() as _,
                        &mut value_len,
                    )
                }
                .into();
                res?;
                Ok(value_len)
            }
            None => Err(TeeError::BadParameters.into()),
        }
    }

    fn get_property_as_uuid(&self, name: Option<&CStr>, value: &mut TeeUuid) -> FfiResult {
        match name {
            Some(property_name) => unsafe {
                property_ffi::TEE_GetPropertyAsUUID(self.0, property_name.as_ptr() as _, value as _)
            }
            .into(),
            None => Err(TeeError::BadParameters.into()),
        }
    }

    fn get_property_as_identity(&self, name: Option<&CStr>, value: &mut TeeIdentity) -> FfiResult {
        match name {
            Some(property_name) => unsafe {
                property_ffi::TEE_GetPropertyAsIdentity(
                    self.0,
                    property_name.as_ptr() as _,
                    value as _,
                )
            }
            .into(),
            None => Err(TeeError::BadParameters.into()),
        }
    }
}

impl PseudoPropSet {
    pub fn new(property_set: PseudoPropSetHandle) -> Self {
        PseudoPropSet(property_set)
    }
}

pub struct EnumrablePropertySet(PseudoPropSetHandle);
impl PropertySet for EnumrablePropertySet {
    fn get_property_as_string(
        &self,
        name: Option<&CStr>,
        buf: &mut [u8],
        len: &mut usize,
    ) -> FfiResult {
        let property_name = match name {
            Some(x) => x.as_ptr(),
            None => core::ptr::null(),
        };
        let res: FfiResult = unsafe {
            property_ffi::TEE_GetPropertyAsString(
                self.0,
                property_name as _,
                buf.as_mut_ptr() as _,
                len as _,
            )
        }
        .into();
        res
    }

    fn get_property_as_bool(&self, name: Option<&CStr>) -> Result<bool, FfiTeeError> {
        let mut value: i32 = false as _;
        let property_name = match name {
            Some(x) => x.as_ptr(),
            None => core::ptr::null(),
        };
        let res: FfiResult =
            unsafe { property_ffi::TEE_GetPropertyAsBool(self.0, property_name as _, &mut value) }
                .into();
        res?;
        if value == 0 {
            Ok(false)
        } else {
            Ok(true)
        }
    }

    fn get_property_as_u32(&self, name: Option<&CStr>) -> Result<u32, FfiTeeError> {
        let mut value = u32::default();
        let property_name = match name {
            Some(x) => x.as_ptr(),
            None => core::ptr::null(),
        };
        let res: FfiResult =
            unsafe { property_ffi::TEE_GetPropertyAsU32(self.0, property_name as _, &mut value) }
                .into();
        res?;
        Ok(value)
    }

    fn get_property_as_u64(&self, name: Option<&CStr>) -> Result<u64, FfiTeeError> {
        let mut value = u64::default();
        let property_name = match name {
            Some(x) => x.as_ptr(),
            None => core::ptr::null(),
        };
        let res: FfiResult =
            unsafe { property_ffi::TEE_GetPropertyAsU64(self.0, property_name as _, &mut value) }
                .into();
        res?;
        Ok(value)
    }

    fn get_property_as_binary_block(
        &self,
        name: Option<&CStr>,
        value: &mut [u8],
    ) -> Result<usize, FfiTeeError> {
        let mut value_len = value.len();
        let property_name = match name {
            Some(x) => x.as_ptr(),
            None => core::ptr::null(),
        };
        let res: FfiResult = unsafe {
            property_ffi::TEE_GetPropertyAsBinaryBlock(
                self.0,
                property_name as _,
                value.as_mut_ptr() as _,
                &mut value_len,
            )
        }
        .into();
        res?;
        Ok(value_len)
    }

    fn get_property_as_uuid(&self, name: Option<&CStr>, value: &mut TeeUuid) -> FfiResult {
        let property_name = match name {
            Some(x) => x.as_ptr(),
            None => core::ptr::null(),
        };
        unsafe { property_ffi::TEE_GetPropertyAsUUID(self.0, property_name as _, value as _) }
            .into()
    }

    fn get_property_as_identity(&self, name: Option<&CStr>, value: &mut TeeIdentity) -> FfiResult {
        let property_name = match name {
            Some(x) => x.as_ptr(),
            None => core::ptr::null(),
        };
        unsafe { property_ffi::TEE_GetPropertyAsIdentity(self.0, property_name as _, value as _) }
            .into()
    }
}

impl EnumrablePropertySet {
    pub fn new() -> Result<EnumrablePropertySet, FfiTeeError> {
        let mut enumerator = EnumrablePropertySet(PseudoPropSetHandle::TEE_PROPSET_UNKNOW);
        let res: FfiResult =
            unsafe { property_ffi::TEE_AllocatePropertyEnumerator(&mut enumerator.0) }.into();
        res?;
        Ok(enumerator)
    }
    pub fn start(&mut self, property_set: PseudoPropSetHandle) {
        unsafe { property_ffi::TEE_StartPropertyEnumerator(self.0, property_set) }
    }
    pub fn reset(&mut self) {
        unsafe { property_ffi::TEE_ResetPropertyEnumerator(self.0) }
    }

    pub fn get_property_name(&self, name: &mut [u8]) -> Result<(), FfiTeeError> {
        let mut name_buffer_len: usize = name.len();
        let res: Result<(), FfiTeeError> = unsafe {
            property_ffi::TEE_GetPropertyName(
                self.0,
                name.as_mut_ptr() as _,
                &mut name_buffer_len as _,
            )
        }
        .into();
        res?;
        Ok(())
    }
}

impl Iterator for EnumrablePropertySet {
    type Item = EnumrablePropertySet;
    fn next(&mut self) -> Option<Self::Item> {
        let ret: FfiResult = unsafe { property_ffi::TEE_GetNextProperty(self.0) }.into();
        match ret {
            Ok(()) => Some(EnumrablePropertySet(self.0)),
            _ => None,
        }
    }
}

impl Drop for EnumrablePropertySet {
    fn drop(&mut self) {
        unsafe { property_ffi::TEE_FreePropertyEnumerator(self.0) };
    }
}
