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

use super::{
    DataFlagConstants, ObjectStorageConstants, TeeObjectInfo, TeeWhence, __TeeObjectHandle,
};

#[repr(C)]
pub struct __TeeObjectEnumHandle {
    _unused: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

extern "C" {
    /// Create a new persistent object, you can directly initialize the data stream and TeeAttribute,
    /// the user can use the returned handle to access the object's TeeAttribute and data stream
    ///
    /// # FunParameters
    ///
    /// + `storage_id` \[IN\]  Corresponding to a separate storage space for each application,
    /// the value is [ObjectStorageConstants]
    /// + `object_id` \[IN\] Object identifier, the name of the object to be created
    /// + `object_id_len` \[IN\] The length of the object identifier by byte, no more than 128 bytes
    /// + `flags` \[IN\] Flags after object creation, the value can be one or more of [DataFlagConstants]
    /// + `attributes` \[IN\] The TEE_ObjectHandle of the transient object, used to initialize the
    /// TeeAttribute of the object, can be TEE_HANDLE_NULL
    /// + `initial_data` \[IN\] Initial data, used to initialize data stream data
    /// + `initial_data_len` \[IN\] InitialData length in byte
    /// + `object` \[OUT\] TEE_ObjectHandle returned after the function is successfully executed
    ///
    /// # Return
    ///
    /// + `TEE_SUCCESS` Indicates that the function was executed successfully
    /// + `TEE_ERROR_ITEM_NOT_FOUND` The storageID does not exist
    /// + `TEE_ERROR_ACCESS_CONFLICT`  Access conflict
    /// + `TEE_ERROR_OUT_OF_MEMORY`  Insufficient memory to complete the operation
    /// + `TEE_ERROR_STORAGE_NO_SPACE`  There is not enough space to create the object
    pub fn TEE_CreatePersistentObject(
        storage_id: ObjectStorageConstants,
        object_id: *const core::ffi::c_void,
        object_id_len: usize,
        flags: DataFlagConstants,
        attributes: *const __TeeObjectHandle,
        initial_data: *const core::ffi::c_void,
        initial_data_len: usize,
        object: &mut *mut __TeeObjectHandle,
    ) -> TeeResult;

    /// Open an existing permanent object, the returned handle can be used by the user to access
    /// the object's TeeAttribute and data stream
    ///
    /// # FunParameters
    ///
    /// + `storage_id` \[IN\] orresponding to a separate storage space for each application,
    /// the value is [ObjectStorageConstants]
    /// + `object_id` \[IN\]  object identifier, the name of the object to be opened
    /// + `object_id_len` \[IN\] The length of the object identifier by byte, no more than 128 bytes
    /// + `flags` \[IN\] Flags after object opened, the value can be one or more of [DataFlagConstants]
    /// + `object` \[OUT\] TEE_ObjectHandle returned after the function is successfully executed
    ///
    /// # Return
    ///
    /// + `TEE_SUCCESS`  Indicates that the function was executed successfully
    /// + `TEE_ERROR_ITEM_NOT_FOUND`  The storageID does not exist or cannot find object identifier
    /// + `TEE_ERROR_ACCESS_CONFLICT`  Access conflict
    /// + `TEE_ERROR_OUT_OF_MEMORY`  Insufficient memory to complete the operation
    pub fn TEE_OpenPersistentObject(
        storage_id: ObjectStorageConstants,
        object_id: *const core::ffi::c_void,
        object_id_len: usize,
        flags: DataFlagConstants,
        object: &mut *mut __TeeObjectHandle,
    ) -> TeeResult;

    /// Close the opened TEE_ObjectHandle and delete the object. The object must be a persistent object
    /// and must have been opened with `DataFlagConstants::ACCESS_WRITE_META` permission
    ///
    /// # FunParameters
    ///
    /// + `object` \[IN\]  TEE_ObjectHandle to be closed and deleted
    #[allow(dead_code)]
    pub fn TEE_CloseAndDeletePersistentObject(object: *const __TeeObjectHandle);

    /// Close the opened TEE_ObjectHandle and delete the object. The object must be a persistent object
    /// and must have been opened with `DataFlagConstants::ACCESS_WRITE_META` permission
    ///
    /// # FunParameters
    ///
    /// + `object` \[IN\]  TEE_ObjectHandle to be closed and deleted
    ///
    /// # Return
    ///
    /// + `TEE_SUCCESS`  Indicates that the function was executed successfully
    /// + `TEE_ERROR_STORAGE_NOT_AVAILABLE` Cannot access the storage area where the file is located
    pub fn TEE_CloseAndDeletePersistentObject1(object: *const __TeeObjectHandle) -> TeeResult;

    /// Change the object identifier,
    /// the TEE_ObjectHandle must be opened with `DataFlagConstants::ACCESS_WRITE_META` permission
    ///
    /// # FunParameters
    ///
    /// + `ojbect` \[IN/OUT\]  The object handle to be modified
    /// + `new_object_id` \[IN\] New object identifier
    /// + `new_object_id_len` \[IN\] New object identifier length
    ///
    /// # Return
    ///
    /// + `TEE_SUCCESS`  Indicates that the function was executed successfully
    pub fn TEE_RenamePersistentObject(
        object: *mut __TeeObjectHandle,
        new_object_id: *const core::ffi::c_void,
        new_object_id_len: usize,
    ) -> TeeResult;

    /// Allocate the handle of an uninitialized object enumerator
    ///
    /// # FunParameters
    /// + `obj_enumerator` \[OUT\]  Pointer to the handle of the newly created object enumerator
    ///
    /// # Return
    /// + `TEE_SUCCESS`  Indicates that the function was executed successfully
    /// + `TEE_ERROR_OUT_OF_MEMORY` No enough memory to allocate
    pub fn TEE_AllocatePersistentObjectEnumerator(
        obj_enumerator: &mut *mut __TeeObjectEnumHandle,
    ) -> TeeResult;

    /// Release a object enumerator handle that has allocated.
    /// The handle becomes invalid after the function is called, and all allocated are released.
    /// Use it in pair with [TEE_AllocatePersistentObjectEnumerator]
    ///
    /// # FunParameters
    ///
    /// + `obj_enumerator` \[IN\]  TeeObjectEnumHandle to be released
    pub fn TEE_FreePersistentObjectEnumerator(obj_enumerator: *mut __TeeObjectEnumHandle);

    /// Reset the temporary object enumerator to its initial state,
    /// that is, the state just after the allocate
    ///
    /// # FunParameters
    ///
    /// + `obj_enumerator` \[IN\]  TeeObjectEnumHandle to be reset
    pub fn TEE_ResetPersistentObjectEnumerator(obj_enumerator: *mut __TeeObjectEnumHandle);

    /// Start enumerating all objects in a given storage space,
    /// the information of the object can be obtained through the [TEE_GetNextPersistentObject] function
    ///
    /// # FunParameters
    ///
    /// + `obj_enumerator` \[IN\] TeeObjectEnumHandle of the allocated object enumerator
    /// + `storage_id` \[IN\] Correspond to a separate storage space for each application,
    /// the value is [ObjectStorageConstants], currently only supports `ObjectStorageConstants::PRIVATE`
    ///
    /// # Return
    ///
    /// + `TEE_SUCCESS`  Indicates that the function was executed successfully
    /// + `TEE_ITEM_NOT_FOUND storageID` is not TEE_STORAGE_PRIVATE
    /// or there is no object in the storage space
    pub fn TEE_StartPersistentObjectEnumerator(
        obj_enumerator: *mut __TeeObjectEnumHandle,
        storage_id: ObjectStorageConstants,
    ) -> TeeResult;

    /// Get the next object in the object enumerator, and return the object's TeeObjectInfo, objectID,
    /// objectIDLen information
    ///
    /// # FunParameters
    ///
    /// + `obj_enumerator` \[IN\]  TeeObjectEnumHandle of the initialized object enumerator
    /// + `object_info` \[OUT\]  Pointer to the structure used to store the obtained TeeObjectInfo
    /// + `object_id` \[OUT\] Pointer to a buffer, used to store the obtained objectID
    /// + `object_id_len` \[OUT\] Used to store the obtained objectIDLen
    ///
    /// # Return
    ///
    /// + `TEE_SUCCESS`  Indicates that the function was executed successfully
    /// + `TEE_ITEM_NOT_FOUND` The enumerator has no object or the enumerator has not been initialized
    pub fn TEE_GetNextPersistentObject(
        obj_enumerator: *mut __TeeObjectEnumHandle,
        object_info: *mut TeeObjectInfo,
        object_id: *mut core::ffi::c_void,
        object_id_len: *mut usize,
    ) -> TeeResult;

    /// Read size bytes of data from the object's data stream to the buffer,
    /// the TEE_ObjectHandle must have been opened with `DataFlagConstants::ACCESS_READ` permission
    ///
    /// # FunParameters
    ///
    /// + `objbect` \[IN\]  The TEE_ObjectHandle to be read
    /// + `buffer` \[OUT\]  Buffer for storing read data
    /// + `size` \[IN\]  Size of data to be read by byte
    /// + `count` \[OUT\]  Size of data actually read by byte
    ///
    /// # Return
    ///
    /// + `TEE_SUCCESS` Indicates that the function was executed successfully
    /// + `TEE_ERROR_OUT_OF_MEMORY` Insufficient memory to complete the operation
    pub fn TEE_ReadObjectData(
        object: *const __TeeObjectHandle,
        buffer: *mut core::ffi::c_void,
        size: usize,
        count: *mut u32,
    ) -> TeeResult;

    /// Write size bytes of data from the buffer to the data stream of the object.
    /// TEE_ObjectHandle must have been opened with `DataFlagConstants::ACCESS_WRITE` permission
    ///
    /// # FunParameters
    ///
    /// + `ojbect` \[IN\]  The TEE_ObjectHandle to be write
    /// + `buffer` \[IN\]  Store the data to be written
    /// + `size` \[IN\]  The length of the data to be written, the size does not exceed 4096 bytes
    ///
    /// # Return
    /// + `TEE_SUCCESS` Indicates that the function was executed successfully
    /// + `TEE_ERROR_OUT_OF_MEMORY`  Insufficient memory to complete the operation
    /// + `TEE_ERROR_STORAGE_NO_SPACE`  There is not enough space to perform the operation
    pub fn TEE_WriteObjectData(
        object: *mut __TeeObjectHandle,
        buffer: *const core::ffi::c_void,
        size: usize,
    ) -> TeeResult;

    /// This function changes the size of the data stream. If the size is smaller than the size of
    /// the current data stream, delete all excess bytes. If size is greater than the size of the
    /// current data stream, use '0' to expand
    /// TEE_ObjectHandle must be opened with `DataFlagConstants::ACCESS_WRITE` permission
    ///
    /// # FunParameters
    /// + `object` \[IN\]  TEE_ObjectHandle to be truncated
    /// + `size` \[IN\]  The new length of the data stream, the size does not exceed 4096 bytes
    ///
    /// # Return
    ///
    /// + `TEE_SUCCESS` Indicates that the function was executed successfully
    /// + `TEE_ERROR_STORAGE_NO_SPACE` There is not enough space to perform the operation
    pub fn TEE_TruncateObjectData(object: *mut __TeeObjectHandle, size: usize) -> TeeResult;

    /// Set the data stream position pointed to by TEE_ObjectHandle, and set the data stream position to:
    /// start position + offset
    /// The parameter whence controls the starting position of the offset,
    /// the value can choose in [TeeWhence], and the meaning is as follows:
    /// + `TEE_DATA_SEEK_SET`, the starting position of the data stream offset is the file header, which is 0
    /// + `TEE_DATA_SEEK_CUR`, the starting position of the data stream offset is the current position
    /// + `TEE_DATA_SEEK_END`, the starting position of the data stream offset is the end of the file
    /// When the parameter offset is a positive number, it is offset backward, and when it is negative, it is offset forward.
    ///
    /// # FunParameters
    ///
    /// + `object` \[IN\]  TEE_ObjectHandle to be set
    /// + `offset` \[IN\]  The size of the data stream position movement, the size does not exceed 4096 bytes
    /// + `whence` \[IN\]  The initial position of the data stream offset
    ///
    /// # Return
    ///
    /// + `TEE_SUCCESS` Indicates that the function was executed successfully
    /// + `TEE_ERROR_OVERFLOW`  The operation causes the value of the position indicator to exceed its
    /// system limit TEE_DATA_MAX_POSITION
    pub fn TEE_SeekObjectData(
        object: *const __TeeObjectHandle,
        offset: i32,
        whence: TeeWhence,
    ) -> TeeResult;

    /// Synchronize the opened TEE_ObjectHandle,
    /// and synchronize the corresponding security attribute files to the disk
    ///
    /// # FunParameters
    ///
    /// + `object` \[IN\]  TEE_ObjectHandle to be synchronized
    ///
    /// # Return
    ///
    /// + `TEE_SUCCESS`  Indicates that the function was executed successfully
    pub fn TEE_SyncPersistentObject(object: *mut __TeeObjectHandle) -> TeeResult;
}
