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

use crate::error::TeeError;
use core::mem::ManuallyDrop;

use usize;

pub const API_LEVEL1_0: u32 = 1;
pub const API_LEVEL1_1_1: u32 = 2;
pub const API_LEVEL1_2: u32 = 3;
pub const TEE_PARAMS_NUM: u32 = 4;
pub const TEE_ORIGIN_TEE: u32 = 3;
pub const TEE_ORIGIN_TRUSTED_APP: u32 = 4;
pub const TEE_TIMEOUT_INFINITE: u32 = 4294967295;
pub const PARAM_COUNT: usize = 4;

#[repr(C)]
pub struct TeeParamMemref {
    pub buffer: *mut core::ffi::c_void,
    pub size: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TeeParamValue {
    pub a: u32,
    pub b: u32,
}

#[repr(C)]
pub union TeeParam {
    pub memref: ManuallyDrop<TeeParamMemref>,
    pub value: TeeParamValue,
}

impl Default for TeeParam {
    fn default() -> Self {
        Self::new_value(0, 0)
    }
}

impl TeeParam {
    pub fn new_value(a: u32, b: u32) -> TeeParam {
        TeeParam {
            value: TeeParamValue { a, b },
        }
    }
}

/// Error information when converting a [TeeParamTypes] into [TeeParamTypeArray]
pub enum TeeParamTypesError {
    /// An Unknown TeeParameter type.
    ///
    /// The first tuple member represents the index of the first unknown parameter type, e.g. a `0`
    /// means that the first parameter type could not be parsed. Valid range: 0 - 3
    /// The second tuple member contains the integer of the raw parameter type which could not be
    /// mapped to a known parameter type. Valid range: 0x0 - 0xF.
    UnknownParamType((u8, u8)),
    /// The actual parameters did not match the expected parameters.
    ///
    /// # Tuple members
    ///
    /// 1. The index of the first parameter that did not match.
    /// 2. The expected [TeeParamType].
    /// 2. The actual [TeeParamType] that was received.
    ParameterMismatch(u8, TeeParamType, TeeParamType),
    /// An Error in the implementation of parsing [TeeParamTypes] occured.
    InternalError,
}

impl From<TeeParamTypesError> for TeeError {
    fn from(_: TeeParamTypesError) -> Self {
        TeeError::BadParameters
    }
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct TeeParamTypes(pub u32);

impl TeeParamTypes {
    fn get_param_type(&self, index: usize) -> Result<TeeParamType, TeeParamTypesError> {
        if index >= PARAM_COUNT {
            Err(TeeParamTypesError::InternalError)
        } else {
            // SAFETY: the cast is guaranteed to be lossless due to masking with `0xF`.
            let raw: u8 = ((self.0 >> (index * 4)) & 0xF) as u8;
            TeeParamType::try_from(raw)
                // SAFETY: We checked `index` to be in the range of 0-3, so the cast is guaranteed
                //         to be lossless.
                .map_err(|_| TeeParamTypesError::UnknownParamType((index as u8, raw)))
        }
    }

    /// Convert the raw [TeeParamTypeArray] into an array of parameter types.
    pub fn try_convert_into_array(self) -> Result<TeeParamTypeArray, TeeParamTypesError> {
        Ok([
            self.get_param_type(0)?,
            self.get_param_type(1)?,
            self.get_param_type(2)?,
            self.get_param_type(3)?,
        ])
    }

    /// Verify that the expected parameters match the actual parameters.
    ///
    /// # Example
    ///
    /// ```
    /// let raw = TeeParamTypes(1u32);
    /// let expected = [TeeParamType::ValueInput, TeeParamType::None, TeeParamType::None, TeeParamType::None];
    /// assert!(raw.check_param_type(expected).is_ok())
    /// ```
    pub fn check_param_type(
        self,
        expected_params: &TeeParamTypeArray,
    ) -> Result<(), TeeParamTypesError> {
        self.try_convert_into_array()?
            .iter()
            .enumerate()
            .try_for_each(|(index, actual_param)| {
                let eq = *actual_param == expected_params[index];
                eq.then_some(()).ok_or_else(|| {
                    TeeParamTypesError::ParameterMismatch(
                        // SAFETY: Index is at most 3, so guaranteed to fit into a `u8`.
                        index as u8,
                        expected_params[index],
                        *actual_param,
                    )
                })
            })
    }
}

impl From<TeeParamTypeArray> for TeeParamTypes {
    fn from(v: TeeParamTypeArray) -> Self {
        let mut raw = 0_u32;
        for (index, val) in v.iter().enumerate() {
            // use num_enum here instead of as to be future proof
            raw |= (*val as u32) << (index * 4)
        }
        Self(raw)
    }
}

pub type TeeParamTypeArray = [TeeParamType; PARAM_COUNT];

/// Describes the type of a [TeeParam]
///
/// # Safety
/// Same as C
#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TeeParamType {
    /// GPD: TEE_PARAM_TYPE_NONE
    NONE = 0,
    /// GPD: TEE_PARAM_TYPE_VALUE_INPUT
    ValueInput = 1,
    /// GPD: TEE_PARAM_TYPE_VALUE_OUTPUT
    ValueOutput = 2,
    /// GPD: TEE_PARAM_TYPE_VALUE_INOUT
    ValueInout = 3,
    /// GPD: TEE_PARAM_TYPE_MEMREF_INPUT
    MemrefInput = 5,
    /// GPD: TEE_PARAM_TYPE_MEMREF_OUTPUT
    MemrefOutput = 6,
    /// TEE_PARAM_TYPE_MEMREF_INOUT
    MemrefInout = 7,
    IonInput = 8,
    IonSglistInput = 9,
    MemrefSharedInout = 10,
    ResmemInput = 12,
    ResmemOutput = 13,
    ResmemInout = 14,
}

impl TryFrom<u8> for TeeParamType {
    type Error = TeeParamTypesError;

    fn try_from(x: u8) -> Result<Self, Self::Error> {
        match x {
            0 => Ok(Self::NONE),
            1 => Ok(Self::ValueInput),
            2 => Ok(Self::ValueOutput),
            3 => Ok(Self::ValueInout),
            5 => Ok(Self::MemrefInput),
            6 => Ok(Self::MemrefOutput),
            7 => Ok(Self::MemrefInout),
            8 => Ok(Self::IonInput),
            9 => Ok(Self::IonSglistInput),
            10 => Ok(Self::MemrefSharedInout),
            12 => Ok(Self::ResmemInput),
            13 => Ok(Self::ResmemOutput),
            14 => Ok(Self::ResmemInout),
            _ => Err(Self::Error::UnknownParamType((0, x))),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct TeeUuid {
    pub time_low: u32,
    pub time_mid: u16,
    pub time_hi_and_version: u16,
    pub clock_seq_and_node: [u8; 8usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TeeIdentity {
    pub login: u32,
    pub uuid: TeeUuid,
}

/// The Result type of the GPD API
///
#[repr(transparent)]
#[must_use]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TeeResult(pub u32);
pub type TeecResult = TeeResult;
pub type TeeTASessionHandle = u32;

impl TeeResult {
    /// success
    pub const TEE_SUCCESS: Self = Self(0x00000000);
    /// command is invalid
    pub const TEE_ERROR_INVALID_CMD: Self = Self(0x00000001);
    /// service is not exist
    pub const TEE_ERROR_SERVICE_NOT_EXIST: Self = Self(0x00000002);
    /// session is not exist
    pub const TEE_ERROR_SESSION_NOT_EXIST: Self = Self(0x00000003);
    /// exceeds max session count
    pub const TEE_ERROR_SESSION_MAXIMUM: Self = Self(0x00000004);
    /// service already registered
    pub const TEE_ERROR_REGISTER_EXIST_SERVICE: Self = Self(0x00000005);
    /// internal error occurs
    pub const TEE_ERROR_TARGET_DEAD_FATAL: Self = Self(0x00000006);
    /// read data failed
    pub const TEE_ERROR_READ_DATA: Self = Self(0x00000007);
    /// write data failed
    pub const TEE_ERROR_WRITE_DATA: Self = Self(0x00000008);
    /// truncate data failed
    pub const TEE_ERROR_TRUNCATE_OBJECT: Self = Self(0x00000009);
    /// seek data failed
    pub const TEE_ERROR_SEEK_DATA: Self = Self(0x0000000A);
    /// sync data failed
    pub const TEE_ERROR_SYNC_DATA: Self = Self(0x0000000B);
    /// rename file failed
    pub const TEE_ERROR_RENAME_OBJECT: Self = Self(0x0000000C);
    /// error occurs when loading TA
    pub const TEE_ERROR_TRUSTED_APP_LOAD_ERROR: Self = Self(0x0000000D);
    /// TA type is inconsistent with the loading mode.
    pub const TEE_ERROR_OTRP_LOAD_NOT_MATCHED: Self = Self(0x80000100);
    /// the not open session's otrp service num exceeds
    pub const TEE_ERROR_OTRP_LOAD_EXCEED: Self = Self(0x80000101);
    /// uuid of load cmd is not inconsistent with the sec file
    pub const TEE_ERROR_OTRP_ACCESS_DENIED: Self = Self(0x80000102);
    /// otrp service is aged
    pub const TEE_ERROR_OTRP_SERVICE_AGED: Self = Self(0x80000103);
    /// I/O error occurs in storage operation
    pub const TEE_ERROR_STORAGE_EIO: Self = Self(0x80001001);
    /// storage section is unavailable
    pub const TEE_ERROR_STORAGE_EAGAIN: Self = Self(0x80001002);
    /// operation target is not directory
    pub const TEE_ERROR_STORAGE_ENOTDIR: Self = Self(0x80001003);
    /// cannot do this operation on directory
    pub const TEE_ERROR_STORAGE_EISDIR: Self = Self(0x80001004);
    /// opened files exceed max count in system
    pub const TEE_ERROR_STORAGE_ENFILE: Self = Self(0x80001005);
    /// opened files exceed max count for this process
    pub const TEE_ERROR_STORAGE_EMFILE: Self = Self(0x80001006);
    /// stroage section is read only
    pub const TEE_ERROR_STORAGE_EROFS: Self = Self(0x80001007);
    /// File path error
    pub const TEE_ERROR_STORAGE_PATH_WRONG: Self = Self(0x8000100A);
    /// sevice msg queue overflow
    pub const TEE_ERROR_MSG_QUEUE_OVERFLOW: Self = Self(0x8000100B);
    /// file object has been damaged
    pub const TEE_ERROR_CORRUPT_OBJECT: Self = Self(0xF0100001);
    /// storage section is unavailable
    pub const TEE_ERROR_STORAGE_NOT_AVAILABLE: Self = Self(0xF0100003);
    /// cipher text is incorrect
    pub const TEE_ERROR_CIPHERTEXT_INVALID: Self = Self(0xF0100006);
    /// protocol error in socket connection
    pub const TEE_ISOCKET_ERROR_PROTOCOL: Self = Self(0xF1007001);
    /// socket is closed by remote
    pub const TEE_ISOCKET_ERROR_REMOTE_CLOSED: Self = Self(0xF1007002);
    /// socket connection is timeout
    pub const TEE_ISOCKET_ERROR_TIMEOUT: Self = Self(0xF1007003);
    /// no resource avaliable for socket connection
    pub const TEE_ISOCKET_ERROR_OUT_OF_RESOURCES: Self = Self(0xF1007004);
    /// buffer is too large in socket connection
    pub const TEE_ISOCKET_ERROR_LARGE_BUFFER: Self = Self(0xF1007005);
    /// warnning occurs in socket connection
    pub const TEE_ISOCKET_WARNING_PROTOCOL: Self = Self(0xF1007006);
    /// generic error
    pub const TEE_ERROR_GENERIC: Self = Self(0xFFFF0000);
    /// access is denied
    pub const TEE_ERROR_ACCESS_DENIED: Self = Self(0xFFFF0001);
    /// operation has been canceled
    pub const TEE_ERROR_CANCEL: Self = Self(0xFFFF0002);
    /// conflict access error occurs
    pub const TEE_ERROR_ACCESS_CONFLICT: Self = Self(0xFFFF0003);
    /// exceeds max data size
    pub const TEE_ERROR_EXCESS_DATA: Self = Self(0xFFFF0004);
    /// incorrect data format
    pub const TEE_ERROR_BAD_FORMAT: Self = Self(0xFFFF0005);
    /// incorrect parameters
    pub const TEE_ERROR_BAD_PARAMETERS: Self = Self(0xFFFF0006);
    /// operation is not allowed in current state
    pub const TEE_ERROR_BAD_STATE: Self = Self(0xFFFF0007);
    /// cannot find target item
    pub const TEE_ERROR_ITEM_NOT_FOUND: Self = Self(0xFFFF0008);
    /// api is not implemented
    pub const TEE_ERROR_NOT_IMPLEMENTED: Self = Self(0xFFFF0009);
    /// api is not supported
    pub const TEE_ERROR_NOT_SUPPORTED: Self = Self(0xFFFF000A);
    /// no data avaliable for this operation
    pub const TEE_ERROR_NO_DATA: Self = Self(0xFFFF000B);
    /// not memory avaliable for this operation
    pub const TEE_ERROR_OUT_OF_MEMORY: Self = Self(0xFFFF000C);
    /// system busy to handle this operation
    pub const TEE_ERROR_BUSY: Self = Self(0xFFFF000D);
    /// communication error with target
    pub const TEE_ERROR_COMMUNICATION: Self = Self(0xFFFF000E);
    /// security error occurs
    pub const TEE_ERROR_SECURITY: Self = Self(0xFFFF000F);
    /// buffer is too short for this operation
    pub const TEE_ERROR_SHORT_BUFFER: Self = Self(0xFFFF0010);
    /// operation is canceled
    pub const TEE_ERROR_EXTERNAL_CANCEL: Self = Self(0xFFFF0011);
    /// service is in pending state(in asynchronous state)
    pub const TEE_PENDING: Self = Self(0xFFFF2000);
    /// service is in pending state()
    pub const TEE_PENDING2: Self = Self(0xFFFF2001);
    /// reserved error definition
    pub const TEE_PENDING3: Self = Self(0xFFFF2002);
    /// operation is timeout
    pub const TEE_ERROR_TIMEOUT: Self = Self(0xFFFF3001);
    /// operation overflow
    pub const TEE_ERROR_OVERFLOW: Self = Self(0xFFFF300F);
    /// TA is crashed
    pub const TEE_ERROR_TARGET_DEAD: Self = Self(0xFFFF3024);
    /// no enough space to store data
    pub const TEE_ERROR_STORAGE_NO_SPACE: Self = Self(0xFFFF3041);
    /// MAC operation failed
    pub const TEE_ERROR_MAC_INVALID: Self = Self(0xFFFF3071);
    /// signature check failed
    pub const TEE_ERROR_SIGNATURE_INVALID: Self = Self(0xFFFF3072);
    /// Interrupted by CFC. Broken control flow is detected.
    pub const TEE_CLIENT_INTR: Self = Self(0xFFFF4000);
    /// time is not set
    pub const TEE_ERROR_TIME_NOT_SET: Self = Self(0xFFFF5000);
    /// time need to be reset
    pub const TEE_ERROR_TIME_NEEDS_RESET: Self = Self(0xFFFF5001);
    /// system error
    pub const TEE_FAIL: Self = Self(0xFFFF5002);
    /// base value of timer error codes
    pub const TEE_ERROR_TIMER: Self = Self(0xFFFF6000);
    /// failed to create timer
    pub const TEE_ERROR_TIMER_CREATE_FAILED: Self = Self(0xFFFF6001);
    /// failed to destory timer
    pub const TEE_ERROR_TIMER_DESTORY_FAILED: Self = Self(0xFFFF6002);
    /// timer not found
    pub const TEE_ERROR_TIMER_NOT_FOUND: Self = Self(0xFFFF6003);
    /// base value of RPMB error codes
    pub const TEE_ERROR_RPMB_BASE: Self = Self(0xFFFF7000);
    /// generic error of RPMB operations
    pub const TEE_ERROR_RPMB_GENERIC: Self = Self(0xFFFF7001);
    /// verify MAC failed in RPMB operations
    pub const TEE_ERROR_RPMB_MAC_FAIL: Self = Self(0xFFFF7002);
    /// invalid counter in RPMB operations
    pub const TEE_ERROR_RPMB_COUNTER_FAIL: Self = Self(0xFFFF7003);
    /// addresss check failed in RPMB operations
    pub const TEE_ERROR_RPMB_ADDR_FAIL: Self = Self(0xFFFF7004);
    /// failed to write data to RPMB
    pub const TEE_ERROR_RPMB_WRITE_FAIL: Self = Self(0xFFFF7005);
    /// failed to read data in RPMB
    pub const TEE_ERROR_RPMB_READ_FAIL: Self = Self(0xFFFF7006);
    /// key is not provisioned in RPMB
    pub const TEE_ERROR_RPMB_KEY_NOT_PROGRAM: Self = Self(0xFFFF7007);
    /// incorrect message type in RPMB response
    pub const TEE_ERROR_RPMB_RESP_UNEXPECT_MSGTYPE: Self = Self(0xFFFF7100);
    /// incorrect message data block count in RPMB response
    pub const TEE_ERROR_RPMB_RESP_UNEXPECT_BLKCNT: Self = Self(0xFFFF7101);
    /// incorrect message data block index in RPMB response
    pub const TEE_ERROR_RPMB_RESP_UNEXPECT_BLKIDX: Self = Self(0xFFFF7102);
    /// incorrect message data counter in RPMB response
    pub const TEE_ERROR_RPMB_RESP_UNEXPECT_WRCNT: Self = Self(0xFFFF7103);
    /// incorrect message data nonce in RPMB response
    pub const TEE_ERROR_RPMB_RESP_UNEXPECT_NONCE: Self = Self(0xFFFF7104);
    /// incorrect message data MAC in RPMB response
    pub const TEE_ERROR_RPMB_RESP_UNEXPECT_MAC: Self = Self(0xFFFF7105);
    /// file not found in RPMB
    pub const TEE_ERROR_RPMB_FILE_NOT_FOUND: Self = Self(0xFFFF7106);
    /// not space left for RPMB operations
    pub const TEE_ERROR_RPMB_NOSPC: Self = Self(0xFFFF7107);
    /// exceeds max space of RPMB for this TA
    pub const TEE_ERROR_RPMB_SPC_CONFLICT: Self = Self(0xFFFF7108);
    /// RPMB service not ready
    pub const TEE_ERROR_RPMB_NOT_AVAILABLE: Self = Self(0xFFFF7109);
    /// RPMB partition is damaged
    pub const TEE_ERROR_RPMB_DAMAGED: Self = Self(0xFFFF710A);
    /// TUI is being used
    pub const TEE_ERROR_TUI_IN_USE: Self = Self(0xFFFF7110);
    /// incorrect message switch channal in TUI response
    pub const TEE_ERROR_TUI_SWITCH_CHANNAL: Self = Self(0xFFFF7111);
    /// incorrect message configurator driver in TUI response
    pub const TEE_ERROR_TUI_CFG_DRIVER: Self = Self(0xFFFF7112);
    /// invalid TUI event
    pub const TEE_ERROR_TUI_INVALID_EVENT: Self = Self(0xFFFF7113);
    /// incorrect message polling events in TUI response
    pub const TEE_ERROR_TUI_POLL_EVENT: Self = Self(0xFFFF7114);
    /// TUI is cancelled
    pub const TEE_ERROR_TUI_CANCELED: Self = Self(0xFFFF7115);
    /// TUI is exited
    pub const TEE_ERROR_TUI_EXIT: Self = Self(0xFFFF7116);
    /// TUI unavailable
    pub const TEE_ERROR_TUI_NOT_AVAILABLE: Self = Self(0xFFFF7117);
    /// sec flash is not available
    pub const TEE_ERROR_SEC_FLASH_NOT_AVAILABLE: Self = Self(0xFFFF7118);
    /// SE service has crashed or not enabled
    pub const TEE_ERROR_SESRV_NOT_AVAILABLE: Self = Self(0xFFFF7119);
    /// BIO service is not available
    pub const TEE_ERROR_BIOSRV_NOT_AVAILABLE: Self = Self(0xFFFF711A);
    /// ROT service is not available
    pub const TEE_ERROR_ROTSRV_NOT_AVAILABLE: Self = Self(0xFFFF711B);
    /// ART service is not available
    pub const TEE_ERROR_ARTSRV_NOT_AVAILABLE: Self = Self(0xFFFF711C);
    /// HSM service is not available
    pub const TEE_ERROR_HSMSRV_NOT_AVAILABLE: Self = Self(0xFFFF711D);
    /// AntiRoot Response verify failed
    pub const TEE_ERROR_ANTIROOT_RSP_FAIL: Self = Self(0xFFFF9110);
    /// AntiRoot ERROR during invokecmd
    pub const TEE_ERROR_ANTIROOT_INVOKE_ERROR: Self = Self(0xFFFF9111);
    /// audit failed
    pub const TEE_ERROR_AUDIT_FAIL: Self = Self(0xFFFF9112);
    /// unused
    pub const TEE_FAIL2: Self = Self(0xFFFF9113);
}

impl From<crate::error::FfiResult> for TeeResult {
    fn from(res: crate::error::FfiResult) -> Self {
        match res {
            Ok(_) => Self::TEE_SUCCESS,
            Err(code) => Self(code.into()),
        }
    }
}

impl From<TeeResult> for crate::error::FfiResult {
    fn from(res: TeeResult) -> Self {
        match res {
            TeeResult::TEE_SUCCESS => Ok(()),
            // SAFETY: x is not zero
            x => unsafe { Err(crate::error::FfiTeeError::new_unchecked(x.0)) },
        }
    }
}
