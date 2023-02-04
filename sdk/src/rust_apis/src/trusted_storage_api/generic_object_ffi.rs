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
use crate::TeeResult;

use super::TeeObjectAttribute;
#[cfg(any(feature = "api_level2", feature = "api_level3"))]
use super::TeeObjectInfo;

#[repr(C)]
pub struct __TeeObjectHandle {
    _unused: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

extern "C" {
    /// Obtain the TEE_object_info of the object and copy it to the space pointed
    /// to by the parameter object_info, which is pre-allocated by the user
    ///
    /// ### Params
    /// - object (IN) Source
    /// - object_info (OUT) Pointer to the structure used to store the TEE_object_info
    ///
    /// ### Return
    /// - TEE_SUCCESS  Indicates that the function was executed successfully
    /// - TEE_ERROR_CORRUPT_OBJECT  The file is damaged and the file handle will be closed
    /// - TEE_ERROR_STORAGE_NOT_AVAILABLE  Cannot access the storage area where the file is located
    #[cfg(any(feature = "api_level2", feature = "api_level3"))]
    pub fn TEE_GetObjectInfo1(
        object: *const __TeeObjectHandle,
        object_info: *mut TeeObjectInfo,
    ) -> TeeResult;

    /// Limit the object_usage bit of the object. This bit determines the usage of the key in the object.
    /// The value range is Usage_Constants. For the flag bit of the parameter object_usage:
    ///   * If this bit is set to 1, the use flag of object will not be changed
    ///   * If this bit is set to 0, the corresponding object usage flag of the object is cleared
    ///
    /// The newly created object will contain all Usage_Constants, and the usage flag can
    /// only be cleared, not set
    /// ### Params
    /// - object (IN) Need to restrict
    /// - object_usage (IN) object_usage users want to change
    ///
    /// ### Return
    /// - TEE_SUCCESS  Indicates that the function was executed successfully
    /// - TEE_ERROR_CORRUPT_OBJECT  The file is damaged and the file handle will be closed
    /// - TEE_ERROR_STORAGE_NOT_AVAILABLE  Cannot access the storage area where the file is located
    #[cfg(any(feature = "api_level2", feature = "api_level3"))]
    pub fn TEE_RestrictObjectUsage1(object: *mut __TeeObjectHandle, object_usage: u32)
        -> TeeResult;

    /// Get the buffer content of the union in the TeeAttribute structure of the object pointed
    /// to by usize, and the union member must be ref
    ///
    /// The union member in the TeeAttribute structure must be ref. If the TeeAttribute is private,
    /// the Usage_Constants of the object must include TEE_USAGE_EXTRACTABLE
    /// ### Params
    /// - object (IN)  The source
    /// - attribute_id (IN)  The Attribute ID you want to obtain, such as TEE_ObjectAttribute, can also be customized
    /// - buffer (OUT)  Pointer, the buffer pointed to is used to store the contents of the obtained buffer
    /// - size INOUT  Pointer, storing content byte length
    ///
    /// ### Return
    /// - TEE_SUCCESS Indicates that the function was executed successfully
    /// - TEE_ERROR_ITEM_NOT_FOUND The TeeAttribute you are looking for is not found in the object,
    /// or the object is not initialized
    /// - TEE_ERROR_SHORT_BUFFER The provided buffer is too small to store the acquired content
    ///
    /// ### Panic Reasons
    /// * If object is not a valid opened object handle.
    /// * If the object is not initialized.
    /// * If Bit 29 of attribute_id is not set to 0, so the attribute is not a buffer attribute.
    /// * If Bit 28 of attribute_id is set to 0, denoting a protected attribute, and the object
    ///   usage does not contain the TEE_USAGE_EXTRACTABLE flag.
    /// * If the implementation detects any other error associated with this function that is not
    ///   explicitly associated with a defined return code for this function.
    pub fn TEE_GetObjectBufferAttribute(
        object: *const __TeeObjectHandle,
        attribute_id: TeeObjectAttribute,
        buffer: *mut u8,
        size: *mut usize,
    ) -> TeeResult;

    /// Get the value of the union in the TeeAttribute in the object, and the union member must be the value
    ///
    /// The member of the union in the TeeAttribute structure must be value. If the TeeAttribute is private,
    /// the Usage_Constants of the object must include TEE_USAGE_EXTRACTABLE
    /// ### Params
    /// - object (IN)  The source
    /// - attribute_id (IN)  The Attribute ID you want to obtain, such as TEE_ObjectAttribute, can also be customized
    /// - a (OUT)  Pointer, the space pointed to is used to store a
    /// - b (OUT)  Pointer, the space pointed to is used to store b
    ///
    /// ### Return
    /// - TEE_SUCCESS Indicates that the function was executed successfully
    /// - TEE_ERROR_ITEM_NOT_FOUND The TeeAttribute you are looking for is not found in the object,
    /// or the object is not initialized
    /// - TEE_ERROR_ACCESS_DENIED Attempt to obtain a private TeeAttribute but did not set TEE_USAGE_EXTRACTABLE
    pub fn TEE_GetObjectValueAttribute(
        object: *const __TeeObjectHandle,
        attribute_id: TeeObjectAttribute,
        a: *mut u32,
        b: *mut u32,
    ) -> TeeResult;

    /// Close the opened object. the object can be a persistent object or a transient object
    ///
    /// ### Params
    /// - object to be closed - takes ownership of the object.
    pub fn TEE_CloseObject(object: *mut __TeeObjectHandle);
}
