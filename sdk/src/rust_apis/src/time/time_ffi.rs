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
use crate::tee_defines::TeeResult;
use crate::time::TeeTime;

extern "C" {
    #[doc = " Get current TEE system time"]
    #[doc = ""]
    #[doc = " @param time OUT current system time"]
    #[doc = " @return void"]
    pub fn TEE_GetSystemTime(time: *mut TeeTime);
}

extern "C" {
    #[doc = " Waits for the specified number of milliseconds"]
    #[doc = ""]
    #[doc = " @param timeout IN specified number of milliseconds"]
    #[doc = ""]
    #[doc = " @return  TEE_SUCCESS success"]
    #[doc = " @return  TEE_ERROR_CANCEL the wait has been cancelled"]
    #[doc = " @return  TEE_ERROR_OUT_OF_MEMORY not enough memory is available to complete the operation"]
    pub fn TEE_Wait(timeout: u32) -> TeeResult;
}

extern "C" {
    #[doc = " Retrieves the persistent time of the Trusted Application"]
    #[doc = ""]
    #[doc = " @param time IN the persistent time of the Trusted Application"]
    #[doc = ""]
    #[doc = " @return  TEE_SUCCESS success"]
    #[doc = " @return  TEE_ERROR_TIME_NOT_SET the persistent time has not been set"]
    #[doc = " @return  TEE_ERROR_TIME_NEEDS_RESET the persistent time has been set but may have been"]
    #[doc = " corrupted and MUST no longer be trusted"]
    #[doc = " @return  TEE_ERROR_OVERFLOW the number of seconds in the TA Persistent Time overflows the range of a uint32_t"]
    #[doc = " @return  TEE_ERROR_OUT_OF_MEMORY not enough memory is available to complete the operation"]
    pub fn TEE_GetTAPersistentTime(time: *mut TeeTime) -> TeeResult;
}

extern "C" {
    #[doc = " Set the persistent time of the current Trusted Application"]
    #[doc = ""]
    #[doc = " @param time IN the persistent time of the Trusted Application"]
    #[doc = ""]
    #[doc = " @return  TEE_SUCCESS success"]
    #[doc = " @return  TEE_ERROR_OUT_OF_MEMORY not enough memory is available to complete the operation"]
    #[doc = " @return  TEE_ERROR_STORAGE_NO_SPACE insufficient storage space is available to complete the operation"]
    pub fn TEE_SetTAPersistentTime(time: *mut TeeTime) -> TeeResult;
}

extern "C" {
    #[doc = " Get current REE system time"]
    #[doc = ""]
    #[doc = " @param time OUT current REE system time"]
    #[doc = " @return void"]
    pub fn TEE_GetREETime(time: *mut TeeTime);
}
