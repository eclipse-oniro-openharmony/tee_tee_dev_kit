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
#[cfg(feature = "no_std")]
mod alloc;
pub mod defines;
#[cfg(feature = "no_std")]
mod panic;
pub mod print;
pub mod tee_defines;
pub mod uuid;

use core::ffi::c_void;

pub use tee_defines::TeeResult;

use self::tee_defines::TeeUuid;

pub const DRV_NAME_MAX_LEN: u32 = 32;
pub const DRV_RESERVED_NUM: u32 = 8;

#[repr(C)]
pub struct DrvData {
    pub fd: i32,                   /* unique label which alloced by driver framework */
    pub taskid: u32,               /* caller taskid */
    pub private_data: *mut c_void, /* the private data associated with this fd */
    pub uuid: TeeUuid,             /* caller uuid */
}

///
/// for drv init func when dyn drv is load
///
pub type InitFunc = extern "C" fn() -> i32;

///
/// drv suspend handle func
///
pub type SuspendFunc = extern "C" fn() -> i32;

///
/// drv resume handle func
///
pub type ResumeFunc = extern "C" fn() -> i32;

///
/// drv ioctl handle func when drv is called by TA
/// or other drv
///
pub type IoctlFunc = extern "C" fn(
    /*drv*/ Option<&mut DrvData>,
    /*cmd*/ u32,
    /*args*/ usize,
    /*args_len*/ u32,
) -> i64;

///
/// drv open handle func when drv is opened by TA
/// or other drv
///
/// # Return
/// fd if success
/// <0 value
///
pub type OpenFunc = extern "C" fn(
    /*drv*/ Option<&mut DrvData>,
    /*args*/ usize,
    /*args_len*/ u32,
) -> i64;

///
/// drv close handle func when drv is closed by TA
/// or other drv
///
pub type CloseFunc = extern "C" fn(/*drv*/ Option<&mut DrvData>) -> i64;

#[repr(C)]
pub struct TeeDriverModule {
    pub init: Option<InitFunc>,
    pub ioctl: Option<IoctlFunc>,
    pub open: Option<OpenFunc>,
    pub close: Option<CloseFunc>,
    pub suspend: Option<SuspendFunc>,
    pub resume: Option<ResumeFunc>,
    pub suspend_s4: Option<SuspendFunc>,
    pub resume_s4: Option<ResumeFunc>,
    pub reserved: [u64; DRV_RESERVED_NUM as usize], /* has not used, just reserved */
}

///
/// different from C macro, rust macro_rules not support cancat ident,
/// so in rust tee_driver_declare, the name parameter should be the full-name: prefix with g_driver_,
/// for example: tee_driver_declare!(g_driver_drv_test_module,...); the name of driver is "drv_test_module".
///
#[macro_export]
macro_rules! tee_driver_declare {
    ($name:ident, $init:expr, $open:expr, $ioctl:expr, $close:expr, $suspend:expr, $resume:expr, $suspend_s4:expr, $resume_s4:expr) => {
        #[allow(dead_code)]
        #[no_mangle]
        pub static $name: rust_drv_apis::framework::TeeDriverModule =
            rust_drv_apis::framework::TeeDriverModule {
                init: $init,
                open: $open,
                ioctl: $ioctl,
                close: $close,
                suspend: $suspend,
                resume: $resume,
                suspend_s4: $suspend_s4,
                resume_s4: $resume_s4,
                reserved: [0u64; rust_drv_apis::framework::DRV_RESERVED_NUM as usize],
            };
    };
}
