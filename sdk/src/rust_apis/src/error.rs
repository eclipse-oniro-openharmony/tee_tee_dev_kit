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
use core::num::NonZeroU32;

macro_rules! back_to_enum {
    ($(#[$meta:meta])* $vis:vis enum $name:ident {
        $($(#[$vmeta:meta])* $vname:ident $(= $val:expr)?,)*
    }) => {
        $(#[$meta])*
        $vis enum $name {
            $($(#[$vmeta])* $vname $(= $val)?,)*
        }

        impl TryFrom<u32> for $name {
            type Error = u32;

            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    $(x if x == $name::$vname as u32 => Ok($name::$vname),)*
                    _ => Err(v),
                }
            }
        }
    }
}

back_to_enum! {

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum TeeError {
    /// command is invalid
    InvalidCmd = 0x00000001,
    /// service is not exist
    ServiceNotExist = 0x00000002,
    /// session is not exist
    SessionNotExist = 0x00000003,
    /// exceeds max session count
    SessionMaximum = 0x00000004,
    /// service already registered
    RegisterExistService = 0x00000005,
    /// internal error occurs
    TargetDeadFatal = 0x00000006,
    /// read data failed
    ReadData = 0x00000007,
    /// write data failed
    WriteData = 0x00000008,
    /// truncate data failed
    TruncateObject = 0x00000009,
    /// seek data failed
    SeekData = 0x0000000A,
    /// sync data failed
    SyncData = 0x0000000B,
    /// rename file failed
    RenameObject = 0x0000000C,
    /// error occurs when loading TA
    TrustedAppLoadError = 0x0000000D,
    /// TA type is inconsistent with the loading mode.
    OtrpLoadNotMatched = 0x80000100,
    /// the not open session's otrp service num exceeds
    OtrpLoadExceed = 0x80000101,
    /// uuid of load cmd is not inconsistent with the sec file
    OtrpAccessDenied = 0x80000102,
    /// otrp service is aged
    OtrpServiceAged = 0x80000103,
    /// I/O error occurs in storage operation
    StorageEio = 0x80001001,
    /// storage section is unavailable
    StorageEagain = 0x80001002,
    /// operation target is not directory
    StorageEnotdir = 0x80001003,
    /// cannot do this operation on directory
    StorageEisdir = 0x80001004,
    /// opened files exceed max count in system
    StorageEnfile = 0x80001005,
    /// opened files exceed max count for this process
    StorageEmfile = 0x80001006,
    /// stroage section is read only
    StorageErofs = 0x80001007,
    /// File path error
    StoragePathWrong = 0x8000100A,
    /// sevice msg queue overflow
    MsgQueueOverflow = 0x8000100B,
    /// file object has been damaged
    CorruptObject = 0xF0100001,
    /// storage section is unavailable
    StorageNotAvailable = 0xF0100003,
    /// cipher text is incorrect
    CiphertextInvalid = 0xF0100006,
    /// protocol error in socket connection
    TeeIsocketErrorProtocol = 0xF1007001,
    /// socket is closed by remote
    TeeIsocketErrorRemoteClosed = 0xF1007002,
    /// socket connection is timeout
    TeeIsocketErrorTimeout = 0xF1007003,
    /// no resource avaliable for socket connection
    TeeIsocketErrorOutOfResources = 0xF1007004,
    /// buffer is too large in socket connection
    TeeIsocketErrorLargeBuffer = 0xF1007005,
    /// warnning occurs in socket connection
    TeeIsocketWarningProtocol = 0xF1007006,
    /// generic error
    Generic = 0xFFFF0000,
    /// access is denied
    AccessDenied = 0xFFFF0001,
    /// operation has been canceled
    Cancel = 0xFFFF0002,
    /// conflict access error occurs
    AccessConflict = 0xFFFF0003,
    /// exceeds max data size
    ExcessData = 0xFFFF0004,
    /// incorrect data format
    BadFormat = 0xFFFF0005,
    /// incorrect parameters
    BadParameters = 0xFFFF0006,
    /// operation is not allowed in current state
    BadState = 0xFFFF0007,
    /// cannot find target item
    ItemNotFound = 0xFFFF0008,
    /// api is not implemented
    NotImplemented = 0xFFFF0009,
    /// api is not supported
    NotSupported = 0xFFFF000A,
    /// no data avaliable for this operation
    NoData = 0xFFFF000B,
    /// not memory avaliable for this operation
    OutOfMemory = 0xFFFF000C,
    /// system busy to handle this operation
    Busy = 0xFFFF000D,
    /// communication error with target
    Communication = 0xFFFF000E,
    /// security error occurs
    Security = 0xFFFF000F,
    /// buffer is too short for this operation
    ShortBuffer = 0xFFFF0010,
    /// operation is canceled
    ExternalCancel = 0xFFFF0011,
    /// service is in pending state(in asynchronous state)
    Pending = 0xFFFF2000,
    /// service is in pending state()
    Pending2 = 0xFFFF2001,
    /// reserved error definition
    Pending3 = 0xFFFF2002,
    /// operation is timeout
    Timeout = 0xFFFF3001,
    /// operation overflow
    Overflow = 0xFFFF300F,
    /// TA is crashed
    TargetDead = 0xFFFF3024,
    /// no enough space to store data
    StorageNoSpace = 0xFFFF3041,
    /// MAC operation failed
    MacInvalid = 0xFFFF3071,
    /// signature check failed
    SignatureInvalid = 0xFFFF3072,
    /// Interrupted by CFC. Broken control flow is detected.
    TeeClientIntr = 0xFFFF4000,
    /// time is not set
    TimeNotSet = 0xFFFF5000,
    /// time need to be reset
    TimeNeedsReset = 0xFFFF5001,
    /// system error
    Fail = 0xFFFF5002,
    /// base value of timer error codes
    Timer = 0xFFFF6000,
    /// failed to create timer
    TimerCreateFailed = 0xFFFF6001,
    /// failed to destroy timer
    TimerDestroyFailed = 0xFFFF6002,
    /// timer not found
    TimerNotFound = 0xFFFF6003,
    /// base value of RPMB error codes
    RpmbBase = 0xFFFF7000,
    /// generic error of RPMB operations
    RpmbGeneric = 0xFFFF7001,
    /// verify MAC failed in RPMB operations
    RpmbMacFail = 0xFFFF7002,
    /// invalid counter in RPMB operations
    RpmbCounterFail = 0xFFFF7003,
    /// addresss check failed in RPMB operations
    RpmbAddrFail = 0xFFFF7004,
    /// failed to write data to RPMB
    RpmbWriteFail = 0xFFFF7005,
    /// failed to read data in RPMB
    RpmbReadFail = 0xFFFF7006,
    /// key is not provisioned in RPMB
    RpmbKeyNotProgram = 0xFFFF7007,
    /// incorrect message type in RPMB response
    RpmbRespUnexpectMsgtype = 0xFFFF7100,
    /// incorrect message data block count in RPMB response
    RpmbRespUnexpectBlkcnt = 0xFFFF7101,
    /// incorrect message data block index in RPMB response
    RpmbRespUnexpectBlkidx = 0xFFFF7102,
    /// incorrect message data counter in RPMB response
    RpmbRespUnexpectWrcnt = 0xFFFF7103,
    /// incorrect message data nonce in RPMB response
    RpmbRespUnexpectNonce = 0xFFFF7104,
    /// incorrect message data MAC in RPMB response
    RpmbRespUnexpectMac = 0xFFFF7105,
    /// file not found in RPMB
    RpmbFileNotFound = 0xFFFF7106,
    /// not space left for RPMB operations
    RpmbNospc = 0xFFFF7107,
    /// exceeds max space of RPMB for this TA
    RpmbSpcConflict = 0xFFFF7108,
    /// RPMB service not ready
    RpmbNotAvailable = 0xFFFF7109,
    /// RPMB partition is damaged
    RpmbDamaged = 0xFFFF710A,
    /// TUI is being used
    TuiInUse = 0xFFFF7110,
    /// incorrect message switch channal in TUI response
    TuiSwitchChannal = 0xFFFF7111,
    /// incorrect message configurator driver in TUI response
    TuiCfgDriver = 0xFFFF7112,
    /// invalid TUI event
    TuiInvalidEvent = 0xFFFF7113,
    /// incorrect message polling events in TUI response
    TuiPollEvent = 0xFFFF7114,
    /// TUI is cancelled
    TuiCanceled = 0xFFFF7115,
    /// TUI is exited
    TuiExit = 0xFFFF7116,
    /// TUI unavailable
    TuiNotAvailable = 0xFFFF7117,
    /// sec flash is not available
    SecFlashNotAvailable = 0xFFFF7118,
    /// SE service has crashed or not enabled
    SesrvNotAvailable = 0xFFFF7119,
    /// BIO service is not available
    BiosrvNotAvailable = 0xFFFF711A,
    /// ROT service is not available
    RotsrvNotAvailable = 0xFFFF711B,
    /// ART service is not available
    ArtsrvNotAvailable = 0xFFFF711C,
    /// HSM service is not available
    HsmsrvNotAvailable = 0xFFFF711D,
    /// AntiRoot Response verify failed
    AntirootRspFail = 0xFFFF9110,
    /// AntiRoot ERROR during invokecmd
    AntirootInvokeError = 0xFFFF9111,
    /// audit failed
    AuditFail = 0xFFFF9112,
    /// unused
    Fail2 = 0xFFFF9113,
}
}

impl From<TeeError> for u32 {
    fn from(e: TeeError) -> u32 {
        // SAFETY: [TeeError] is `repr(u32)`, so it is guaranteed to fit
        e as u32
    }
}

impl From<TeeError> for NonZeroU32 {
    fn from(e: TeeError) -> NonZeroU32 {
        // SAFETY: [TeeError] values are guaranteed to be non-zero
        unsafe { Self::new_unchecked(e.into()) }
    }
}

impl From<TeeError> for TeeResult {
    fn from(e: TeeError) -> Self {
        Self(e as _)
    }
}

impl From<NonZeroU32> for TeeResult {
    fn from(val: NonZeroU32) -> Self {
        TeeResult(u32::from(val))
    }
}

impl TryFrom<FfiTeeError> for TeeError {
    type Error = FfiTeeError;
    fn try_from(e: FfiTeeError) -> Result<Self, Self::Error> {
        match TeeError::try_from(u32::from(e)) {
            Ok(o) => Ok(o),
            _ => Err(e),
        }
    }
}

/// TA errro code define
///
/// ```
/// # use error::FfiTeeError;
/// #[repr(u32)]
/// pub enum CustomError {
///     Example1 = 0xFFFFFF1,
///     Example2 = 0xFFFFFF2,
/// }
///
/// pub type CustomResult = Result<(), CustomError>;
///
/// impl Into<FfiTeeError> for CustomError {
///     fn into(self) -> FfiTeeError {
///         // SAFETY: there is no zero in the enum. CustomError is `repr(u32)`.
///         unsafe { TEE_ERROR::new_unchecked(self as u32) }
///     }
/// }
/// ```
pub type FfiTeeError = NonZeroU32;

/// A Rust Result type that can be seamlessly converted to and from [TeeResult]
///
/// This Result type should be used near the FFI boundary and will be converted by the rust_apis API
/// functions to a proper [TeeResult]. Further away from the FFI boundaries users are encouraged
/// to use the regular [Result] with their custom success and error types and simply implement a
/// conversion to the [FfiTeeError] where necessary.
///
/// The SDK provides [TeeError] as a Rust enum that can be converted to the FFI safe [FfiTeeError].
/// In the same fashion, Users are free to use their own error types, but should implement a
/// conversion to [FfiTeeError], so that their custom errors can be converted to [FfiTeeError].
pub type FfiResult = core::result::Result<(), FfiTeeError>;
