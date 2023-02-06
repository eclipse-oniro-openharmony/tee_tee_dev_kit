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

use super::{TeeAttribute, TeeObjectAttribute, TeeObjectType, __TeeObjectHandle};

extern "C" {
    /// Allocate an uninitialized object to store the key, in which object_type and max_object_size
    /// must be specified to pre-allocate
    ///
    /// ### Params
    /// - object_type (IN)  The type of the object to be created, the value can be TEE_object_type
    /// - max_object_size (IN)  Maximum bytes of object
    /// - object (OUT)  Pointer to the handle of the newly created object
    ///
    /// ### Return
    /// - TEE_SUCCESS Indicates that the function was executed successfully
    /// - TEE_ERROR_OUT_OF_MEMORY Not enough  to allocate
    /// - TEE_ERROR_NOT_SUPPORTED The bytes provided by the object is not supported
    pub fn TEE_AllocateTransientObject(
        object_type: TeeObjectType,
        max_object_size: u32,
        object: &mut *mut __TeeObjectHandle,
    ) -> TeeResult;

    /// Release a transient object that has allocated.
    ///
    /// After the function is called, the handle becomes invalid,
    /// and all allocated  are released. Paired with TEE_AllocateTransientObject
    ///
    /// ### Params
    /// - object to be released. Takes ownership.
    #[allow(dead_code)]
    pub fn TEE_FreeTransientObject(object: *mut __TeeObjectHandle);

    /// Reset the transient object to initial state, that is, the state just after the allocate.
    /// The uninitialized object that has allocated but has not stored the key can be reused to store the key
    ///
    /// ### Params
    /// - object (IN/OUT) to be reset
    ///
    /// ### Return
    /// - void
    pub fn TEE_ResetTransientObject(object: *mut __TeeObjectHandle);

    /// This function assigns the attributes in the parameter attrs to an uninitialized transient object,
    /// and the parameter attrs is provided by the trusted application (TA)
    ///
    /// # Attention
    ///
    /// Ensure that the object is still uninitialized
    ///
    /// ### Params
    /// - object (IN/OUT) created but not initialized
    /// - attrs (IN)  object attribute array, can be one or more TeeAttribute
    /// - attr_count (IN)  Number of array members
    ///
    /// ### Return
    /// - TEE_SUCCESS Indicates that the function was executed successfully
    /// - TEE_ERROR_BAD_PARAMETERS The attribute is incorrect or inconsistent.
    ///
    /// # Panics
    ///
    /// * If object is not a valid opened object handle that is transient and uninitialized
    /// * If some mandatory attribute is missing.
    /// * If attrs includes an attribute that is not defined for the object’s type.
    /// * If an attribute value is too big to fit within the maximum object size specified when the
    ///   object was created.
    /// * If the implementation detects any other error associated with this function that is not
    ///   explicitly associated with a defined return code for this function.
    pub fn TEE_PopulateTransientObject(
        object: *mut __TeeObjectHandle,
        attrs: *const TeeAttribute,
        attr_count: u32,
    ) -> TeeResult;

    /// Initialize a buffer type TeeAttribute, that is, the member of union in the TeeAttribute structure must be ref
    ///
    /// ### Params
    /// - attr (OUT)  TeeAttribute to be initialized
    /// - attribute_id (IN)  ID assigned to TeeAttribute
    /// - buffer (IN)  The buffer stores the content to be assigned
    /// - length (IN)  The byte length of the assignment content
    ///
    /// ### Return
    /// - void
    pub fn TEE_InitRefAttribute(
        attr: *mut TeeAttribute,
        attribute_id: TeeObjectAttribute,
        buffer: *const u8,
        length: usize,
    );

    /// Initialize a value type TeeAttribute
    ///
    /// ### Params
    /// - attr (OUT)  TeeAttribute to be initialized
    /// - attribute_id (IN)  ID assigned to TeeAttribute
    /// - a (IN)  Assign the value to the member value a of the union in the TeeAttribute
    /// - b (IN)  Assign the value to the member value b of the union in the TeeAttribute
    ///
    /// ### Return
    /// - void
    pub fn TEE_InitValueAttribute(
        attr: *mut TeeAttribute,
        attribute_id: TeeObjectAttribute,
        a: u32,
        b: u32,
    );

    /// This function generates a random key or key-pair and assigns it to the transient object
    ///
    /// ### Params
    /// - object (IN)  Transient object, used to store the generated key
    /// - key_size (IN)  The bytes of the required key
    /// - params (IN)  FunParameters required for key generation
    /// - param_count (IN)  The number of parameters required to generate the key
    ///
    /// ### Return
    /// - TEE_SUCCESS  Indicates that the function was executed successfully
    /// - TEE_ERROR_BAD_PARAMETERS  The generated key is inconsistent with the key type
    /// that the transient object can store
    pub fn TEE_GenerateKey(
        object: *mut __TeeObjectHandle,
        key_size: u32,
        params: *const TeeAttribute,
        param_count: u32,
    ) -> TeeResult;

    /// Get the information of the object data part, the total length of the data part and the current
    /// position of the data stream
    ///
    /// ### Params
    /// - object (IN)  to be obtained
    /// - pos (OUT)  Data stream position
    /// - len (IN)  Data stream length
    ///
    /// ### Return
    /// - void
    pub fn TEE_InfoObjectData(
        object: *const __TeeObjectHandle,
        pos: *mut u32,
        len: *mut u32,
    ) -> TeeResult;

    /// This function uses an initialized object to assign TeeAttribute to an uninitialized object,
    /// which is equivalent to copying the TeeAttribute of src_object to dest_object
    ///
    /// The TeeAttribute type and number of the two objects must match
    /// ### Params
    /// - dest_object (IN/OUT)  The uninitialized to be assigned
    /// - src_object (IN)  The initialized is used to assign a value to another object
    ///
    /// ### Return
    /// - TEE_SUCCESS  Indicates that the function was executed successfully
    /// - TEE_ERROR_CORRUPT_OBJECT  The file is damaged and the file handle will be closed
    /// - TEE_ERROR_STORAGE_NOT_AVAILABLE  Cannot access the storage area where the file is located
    #[cfg(any(feature = "api_level2", feature = "api_level3"))]
    pub fn TEE_CopyObjectAttributes1(
        dest_object: *mut __TeeObjectHandle,
        src_object: *const __TeeObjectHandle,
    ) -> TeeResult;

}
