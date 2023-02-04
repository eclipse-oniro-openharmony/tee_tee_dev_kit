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
use core::ffi::c_void;

use crate::error::{FfiResult, FfiTeeError};

use super::tee_defines::{TeeParam, TeeParamTypes, TeeResult, TeeTASessionHandle, TeeUuid};

pub struct TA2TASession(TeeTASessionHandle);
pub const TEE_MEMORY_ACCESS_READ: u32 = 1;
pub const TEE_MEMORY_ACCESS_WRITE: u32 = 2;

impl TA2TASession {
    pub fn open_ta_session(
        destination: &TeeUuid,
        cancellation_request_time_out: u32,
        param_types: TeeParamTypes,
        params: &mut [TeeParam; 4],
        return_origin: &mut u32,
    ) -> Result<Self, FfiTeeError> {
        let mut handle = TeeTASessionHandle::default();
        let res: FfiResult = unsafe {
            TEE_OpenTASession(
                destination as *const TeeUuid,
                cancellation_request_time_out,
                param_types,
                params.as_mut_ptr(),
                &mut handle,
                return_origin,
            )
        }
        .into();
        res?;
        Ok(TA2TASession(handle))
    }
    pub fn invoke_command(
        &self,
        cancellation_request_time_out: u32,
        command_id: u32,
        param_types: TeeParamTypes,
        params: &mut [TeeParam; 4],
        return_origin: &mut u32,
    ) -> FfiResult {
        unsafe {
            TEE_InvokeTACommand(
                self.0,
                cancellation_request_time_out,
                command_id,
                param_types,
                params.as_mut_ptr(),
                return_origin,
            )
        }
        .into()
    }
}

impl Drop for TA2TASession {
    fn drop(&mut self) {
        unsafe { TEE_CloseTASession(self.0) };
    }
}

extern "C" {
    #[doc = " Raises a Panic in the Trusted Application instance"]
    #[doc = ""]
    #[doc = " @param panicCode IN informative panic code defined by the TA"]
    #[doc = ""]
    #[doc = " @return void"]
    pub fn TEE_Panic(panicCode: TeeResult) -> !;
}

extern "C" {
    #[doc = " opens a new session with a Trusted Application"]
    #[doc = ""]
    #[doc = " @param destination IN  A pointer to a TeeUuid structure containing the UUID of the destination Trusted"]
    #[doc = " Application"]
    #[doc = " @param cancellationRequestTimeout IN Timeout in milliseconds or the special value"]
    #[doc = " @param paramTypes  IN  The types of all parameters passed in the operation"]
    #[doc = " @param params      IN  The parameters passed in the operation"]
    #[doc = " @param session     OUT A pointer to a variable that will receive the client session handle"]
    #[doc = " @param returnOriginOUT A pointer to a variable which will contain the return origin"]
    #[doc = ""]
    #[doc = " @return TEE_SUCCESS open session successfully"]
    #[doc = " @return TEE_ERROR_ITEM_NOT_FOUND failed to find target TA in TEE"]
    #[doc = " @return TEE_ERROR_ACCESS_DENIED access to the destination Trusted Application is denied"]
    pub fn TEE_OpenTASession(
        destination: *const TeeUuid,
        cancellationRequestTimeout: u32,
        paramTypes: TeeParamTypes,
        params: *mut TeeParam,
        session: *mut TeeTASessionHandle,
        returnOrigin: *mut u32,
    ) -> TeeResult;
}

extern "C" {
    #[doc = " closes a client session opened by TEE_OpenTASession"]
    #[doc = ""]
    #[doc = " @param session IN session handle opened by TEE_OpenTASession"]
    #[doc = ""]
    #[doc = " @return void"]
    pub fn TEE_CloseTASession(session: TeeTASessionHandle);
}

extern "C" {
    #[doc = " invokes a command within a session opened between the client Trusted Application instance"]
    #[doc = " and a destination Trusted Application instance"]
    #[doc = ""]
    #[doc = " @param session      IN An opened session handle"]
    #[doc = " @param cancellationRequestTimeout IN Timeout in milliseconds or the special value"]
    #[doc = " @param commandID    IN The identifier of the Command to invoke"]
    #[doc = " @param paramTypes   IN The types of all parameters passed in the operation"]
    #[doc = " @param params       IN The parameters passed in the operation"]
    #[doc = " @param returnOrigin IN A pointer to a variable which will contain the return origin"]
    #[doc = ""]
    #[doc = " @return TEE_SUCCESS invoke operation successfully"]
    #[doc = " @return TEE_ERROR_ACCESS_DENIED invoke command to target TA is denied"]
    pub fn TEE_InvokeTACommand(
        session: TeeTASessionHandle,
        cancellationRequestTimeout: u32,
        commandID: u32,
        paramTypes: TeeParamTypes,
        params: *mut TeeParam,
        returnOrigin: *mut u32,
    ) -> TeeResult;
}

extern "C" {
    #[doc = " not supported"]
    pub fn TEE_GetCancellationFlag() -> bool;
}
extern "C" {
    #[doc = " not supported"]
    pub fn TEE_UnmaskCancellation() -> bool;
}
extern "C" {
    #[doc = " not supported"]
    pub fn TEE_MaskCancellation() -> bool;
}

extern "C" {
    /// alloc memrory of size bytes with hint value
    /// The pointer returned will be compatible to any C basic data type
    ///
    /// # FunParameters
    ///
    /// + `size` size of memory that will be allocated
    /// + `hint` a flag, 0 mean that the memory returned will filled with "\0"
    ///
    /// # Return
    ///
    /// + return a pointer to the new allocated memory
    /// + return `NULL` means failed when allocated
    pub fn TEE_Malloc(size: usize, hint: core::ffi::c_uint) -> *mut core::ffi::c_void;

    /// release the memory which allocated by [TEE_Malloc]
    ///
    /// # Safety
    ///
    /// if buffer equals to `NULL`, `TEE_Free` will do nothing
    /// Caller should make sure that the buffer is created by [TEE_Malloc] or [TEE_Realloc]
    /// and should NOT free one memory twice, operation result is unpredictable
    ///
    /// # FunParameters
    ///
    /// + `buffer` pointer to memory
    pub fn TEE_Free(buffer: *mut core::ffi::c_void);

    /// realloc memory of new size bytes
    ///
    /// # Safety
    /// If new size larger than old size, the contents of old memory will not changed,
    /// the remained memory are random bytes
    /// There will be a new allocate action when modify the size of memory
    /// If allocated failed , old memory will be returned and this function will return NULL
    /// If buffer equals to `NULL`, this function is same to malloc
    ///
    /// # FunParameters
    ///
    /// + `buffer` pointer to memory
    /// + `new_size` new size
    ///
    /// # Return
    ///
    /// + return pointer to new memory, should NOT be NULL
    /// + return `NULL` means failed
    pub fn TEE_Realloc(buffer: *mut core::ffi::c_void, new_size: usize) -> *mut core::ffi::c_void;

    /// fill the first size bytes of buffer with x
    ///
    /// # FunParameters
    ///
    /// + `buffer` the pointer of buffer
    /// + `x` fill value
    /// + `size` number of bytes
    pub fn TEE_MemFill(buffer: *mut core::ffi::c_void, x: core::ffi::c_char, size: usize);

    /// copy size bytes from src to dest
    ///
    /// # FunParameters
    ///
    /// + `dest` dest buffer pointer
    /// + `src` src buffer pointer
    /// + `size` number of bytes
    pub fn TEE_MemMove(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, size: usize);

    pub fn TEE_MemCompare(buffer1: *const c_void, buffer2: *const c_void, size: usize) -> i32;

    pub fn TEE_SetInstanceData(instance_data: *mut c_void);

    pub fn TEE_GetInstanceData() -> *mut c_void;

    pub fn TEE_CheckMemoryAccessRights(
        access_flags: u32,
        buffer: *const c_void,
        size: usize,
    ) -> TeeResult;
}
