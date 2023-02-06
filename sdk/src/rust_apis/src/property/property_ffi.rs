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
use super::PseudoPropSetHandle;
use crate::{
    tee_defines::{TeeIdentity, TeeUuid},
    TeeResult,
};

extern "C" {
    /// performs a lookup in a property set to retrieve an individual property
    /// and convert its value into a printable string
    ///
    /// # FunParameters
    ///
    /// + `propsetOrEnumerator` One of the TEE_PROPSET_XXX pseudo-handles or a handle on a property enumerator
    /// + `name` Pointer to the zero-terminated string containing name of the property to retrieve
    /// + `valueBuffer` Output buffer for the property value
    /// + `valueBufferLen` Output buffer length
    ///
    /// # Return
    ///
    /// + `TEE_SUCCESS` operation success
    /// + `TEE_ERROR_ITEM_NOT_FOUND` cannot find target property
    /// + `TEE_ERROR_SHORT_BUFFER` the value buffer is not large enough to hold the whole property value
    #[allow(dead_code)]
    pub fn TEE_GetPropertyAsString(
        propsetOrEnumerator: PseudoPropSetHandle,
        name: *const core::ffi::c_char,
        valueBuffer: *mut core::ffi::c_char,
        valueBufferLen: *mut usize,
    ) -> TeeResult;

    /// retrieves a single property in a property set and converts its value to a Boolean
    ///
    /// # FunParameters
    ///
    /// + `propsetOrEnumerator` One of the TEE_PROPSET_XXX pseudo-handles or a handle on a property enumerator
    /// + `name` Pointer to the zero-terminated string containing name of the property to retrieve
    /// + `value` A pointer to the variable that will contain the value of the property
    ///
    /// # Return
    ///
    /// + `TEE_SUCCESS` operation success
    /// + `TEE_ERROR_ITEM_NOT_FOUND` cannot find target property
    pub fn TEE_GetPropertyAsBool(
        propsetOrEnumerator: PseudoPropSetHandle,
        name: *const core::ffi::c_char,
        value: *mut i32,
    ) -> TeeResult;

    /// retrieves a single property in a property set and converts its value to a 32-bits unsigned integer
    ///
    /// # FunParameters
    ///
    /// + `propsetOrEnumerator` One of the TEE_PROPSET_XXX pseudo-handles or a handle on a property enumerator
    /// + `name` Pointer to the zero-terminated string containing name of the property to retrieve
    /// + `value` A pointer to the variable that will contain the value of the property
    ///
    /// # Return
    ///
    /// + `TEE_SUCCESS` operation success
    /// + `TEE_ERROR_ITEM_NOT_FOUND` cannot find target property
    pub fn TEE_GetPropertyAsU32(
        propsetOrEnumerator: PseudoPropSetHandle,
        name: *const core::ffi::c_char,
        value: *mut u32,
    ) -> TeeResult;

    /// retrieves a single property in a property set and converts its value to a 64-bits unsigned integer
    ///
    /// # FunParameters
    ///
    /// + `propsetOrEnumerator` One of the TEE_PROPSET_XXX pseudo-handles or a handle on a property enumerator
    /// + `name` Pointer to the zero-terminated string containing name of the property to retrieve
    /// + `value` A pointer to the variable that will contain the value of the property
    ///
    /// # Return
    ///
    /// + `TEE_SUCCESS` operation success
    /// + `TEE_ERROR_ITEM_NOT_FOUND` cannot find target property
    pub fn TEE_GetPropertyAsU64(
        propsetOrEnumerator: PseudoPropSetHandle,
        name: *const core::ffi::c_char,
        value: *mut u64,
    ) -> TeeResult;

    /// retrieves an individual property and converts its value into a binary block
    ///
    /// # FunParameters
    ///
    /// + `propsetOrEnumerator` One of the TEE_PROPSET_XXX pseudo-handles or a handle on a property enumerator
    /// + `name` Pointer to the zero-terminated string containing name of the property to retrieve
    /// + `valueBuffer` Output buffer for the property value
    /// + `valueBufferLen` Output buffer length
    ///
    /// # Return
    ///
    /// + `TEE_SUCCESS` operation success
    /// + `TEE_ERROR_ITEM_NOT_FOUND` cannot find target property
    /// + `TEE_ERROR_SHORT_BUFFER` the value buffer is not large enough to hold the whole property value
    pub fn TEE_GetPropertyAsBinaryBlock(
        propsetOrEnumerator: PseudoPropSetHandle,
        name: *const core::ffi::c_char,
        valueBuffer: *mut core::ffi::c_void,
        valueBufferLen: *mut usize,
    ) -> TeeResult;

    /// retrieves a single property in a property set and converts its value to TeeUuid struct
    ///
    /// # FunParameters
    ///
    /// + `propsetOrEnumerator` One of the TEE_PROPSET_XXX pseudo-handles or a handle on a property enumerator
    /// + `name` Pointer to the zero-terminated string containing name of the property to retrieve
    /// + `value` A pointer to the variable that will contain the value of the property
    ///
    /// # Return
    ///
    /// + `TEE_SUCCESS` operation success
    /// + `TEE_ERROR_ITEM_NOT_FOUND` cannot find target property
    pub fn TEE_GetPropertyAsUUID(
        propsetOrEnumerator: PseudoPropSetHandle,
        name: *const core::ffi::c_char,
        value: *mut TeeUuid,
    ) -> TeeResult;

    /// retrieves a single property in a property set and converts its value to TeeIdentity struct
    ///
    /// # FunParameters
    ///
    /// + `propsetOrEnumerator` One of the TEE_PROPSET_XXX pseudo-handles or a handle on a property enumerator
    /// + `name` Pointer to the zero-terminated string containing name of the property to retrieve
    /// + `value` A pointer to the variable that will contain the value of the property
    ///
    /// # Return
    ///
    /// + `TEE_SUCCESS` operation success
    /// + `TEE_ERROR_ITEM_NOT_FOUND` cannot find target property
    pub fn TEE_GetPropertyAsIdentity(
        propsetOrEnumerator: PseudoPropSetHandle,
        name: *const core::ffi::c_char,
        value: *mut TeeIdentity,
    ) -> TeeResult;

    /// allocates a property enumerator object
    ///
    /// # FunParameters
    ///
    /// + `enumerator` A pointer filled with an opaque handle on the property enumerator
    ///
    /// # Return
    ///
    /// + `TEE_SUCCESS` operation success
    /// + `TEE_ERROR_OUT_OF_MEMORY` not enough resources to allocate the property enumerator
    pub fn TEE_AllocatePropertyEnumerator(enumerator: *mut PseudoPropSetHandle) -> TeeResult;

    /// deallocates a property enumerator object
    ///
    /// # FunParameters
    ///
    /// + `enumerator` A handle on the enumerator to free
    pub fn TEE_FreePropertyEnumerator(enumerator: PseudoPropSetHandle);

    /// starts to enumerate the properties in an enumerator
    ///
    /// # FunParameters
    ///
    /// + `enumerator` A handle on the enumerator
    /// + `propSet` A pseudo-handle on the property set to enumerate
    pub fn TEE_StartPropertyEnumerator(
        enumerator: PseudoPropSetHandle,
        propSet: PseudoPropSetHandle,
    );

    /// resets a property enumerator to its state immediately after allocation
    ///
    /// # FunParameters
    ///
    /// `enumerator` A handle on the enumerator to reset
    pub fn TEE_ResetPropertyEnumerator(enumerator: PseudoPropSetHandle);

    /// gets the name of the current property in an enumerator
    ///
    /// # FunParameters
    ///
    /// + `enumerator` A handle on the enumerator
    /// + `nameBuffer` The buffer to be filled with the name
    /// + `nameBufferLen` The length of buffer to be filled
    ///
    /// # Return
    ///
    /// + `TEE_SUCCESS` operation success
    /// + `TEE_ERROR_ITEM_NOT_FOUND` no current property either because the enumerator has not started
    /// or because it has reached the end of the property set
    /// + `TEE_ERROR_SHORT_BUFFER` if the name buffer is not large enough to contain the property name
    pub fn TEE_GetPropertyName(
        enumerator: PseudoPropSetHandle,
        nameBuffer: *mut core::ffi::c_char,
        nameBufferLen: *mut usize,
    ) -> TeeResult;

    /// advances the enumerator to the next property
    ///
    /// # FunParameters
    ///
    /// `enumerator` A handle on the enumerator
    ///
    /// # Return
    ///
    /// `TEE_SUCCESS` operation success
    /// `TEE_ERROR_ITEM_NOT_FOUND` enumerator has reached the end of the property set or if it has not started
    #[allow(dead_code)]
    pub fn TEE_GetNextProperty(enumerator: PseudoPropSetHandle) -> TeeResult;
}
