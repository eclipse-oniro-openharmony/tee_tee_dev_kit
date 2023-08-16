// Copyright (C) 2023 Huawei Technologies Co., Ltd.
// Licensed under the Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2.
// You may obtain a copy of Mulan PSL v2 at:
//     http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
use librust_service_ffi::{tee_defines::TeeUuid, TeeResult};

pub const EOK: i32 = 0;

// huk_service_msg.h
pub const CMAC_DERV_MAX_DATA_IN_SIZE: u32 = 0x400;

// crypto_driver_adaptor.h
pub const CRYPTO_KEYTYPE_HUK: u32 = 0x2;

// hm_msg_type.h
#[repr(packed)]
#[derive(Copy, Clone)]
pub struct Send {
    pub msg_class: u8,
    pub msg_flags: u8,
    pub msg_id: u16,
    pub msg_size: u32,
}

#[repr(packed)]
#[derive(Copy, Clone)]
pub struct Reply {
    pub ret_val: i64,
    pub msg_size: u32,
    pub reserve: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hm_msg_header {
    pub send: Send,
    pub reply: Reply,
}

// huk_service_msg.h
pub const AES_TEXT_LEN: u32 = 3269;

#[derive(Clone, Copy)]
#[repr(C)]
pub enum huk_commands_id {
    CMD_HUK_DERIVE_TAKEY = 0x100,
    CMD_HUK_GET_DEVICEID = 0x101,
    CMD_HUK_PROVISION_KEY = 0x102,
    CMD_HUK_DERIVE_PLAT_ROOT_KEY = 0x103,
    CMD_HUK_DERIVE_TAKEY2 = 0x104,
    #[cfg(feature = "config_tee_upgrade")]
    CMD_HUK_DECRYPT_TEEUPGRADEKEY = 0x108,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct derive_key_msg {
    pub salt_buf: u64,
    pub salt_size: u32,
    pub key_buf: u64,
    pub key_size: u32,
    pub outer_iter_num: u32,
    pub inner_iter_num: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct get_info_msg {
    pub buf: u64,
    pub size: u32,
}

pub const SIZE_MAX_EXINFO: usize = 64;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct derive_plat_key_msg {
    pub keytype: u32,
    pub keysize: u32,
    pub exinfo: [u8; SIZE_MAX_EXINFO],
    pub exinfo_size: u32,
    pub csc_type: u32,
    pub csc_uuid: TeeUuid,
    pub attri_size: u32,
    pub attri_buff: u64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union huk_srv_msg_data {
    pub takey_msg: derive_key_msg,
    pub deviceid_msg: get_info_msg,
    pub provisionkey_msg: get_info_msg,
    #[cfg(feature = "config_tee_upgrade")]
    pub teeupgradekey_msg: get_info_msg,
    pub plat_key_msg: derive_plat_key_msg,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct get_info_rsp {
    pub size: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union huk_srv_rsp_data_union {
    pub provisionkey_rsp: get_info_rsp,
    #[cfg(feature = "config_tee_upgrade")]
    pub teeupgradekey_rsp: get_info_rsp,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct huk_srv_rsp_data {
    pub ret: TeeResult,
    pub u: huk_srv_rsp_data_union,
}

#[repr(packed)]
#[derive(Clone, Copy)]
pub struct huk_srv_msg {
    pub header: hm_msg_header,
    pub data: huk_srv_msg_data,
}

#[repr(packed)]
#[derive(Clone, Copy)]
pub struct huk_srv_rsp {
    pub header: hm_msg_header,
    pub data: huk_srv_rsp_data,
}

// huk_service_config.h
#[repr(C)]
#[derive(Copy, Clone)]
pub struct huk_access_table {
    pub cmd_id: u32,
    pub uuid: TeeUuid,
}

// crypto_driver_adaptor.h
#[repr(C)]
#[derive(Copy, Clone)]
pub struct memref_t {
    pub buffer: u64,
    pub size: u32,
}

// crypto_hal_derive_key.h
extern "C" {
    pub fn tee_crypto_derive_root_key(
        derive_type: u32,
        data_in: *const memref_t,
        data_out: *mut memref_t,
        iter_num: u32,
    ) -> i32;
}

pub fn crypto_derive_root_key(
    derive_type: u32,
    data_in: &memref_t,
    data_out: &mut memref_t,
    iter_num: u32,
) -> i32 {
    unsafe { tee_crypto_derive_root_key(derive_type, data_in, data_out, iter_num) }
}

// tee_log.h
extern "C" {
    pub fn tee_print(log_level: u32, fmt_string: *const u8, ...);
}

// C style print
// 1) str should end with \0
// 2) can not check format in compile time
// 3) use '%' instead of '{}' for format string
// 4) each TA has different log file (for example:/data/vendor/log/tee/LOG@UUID-0)
// example: tlogi!("xyz is %d\0", xyz);
#[macro_export]
macro_rules! tlogi {
    ($fmt:expr $(,$args:expr)*) => {
        unsafe {
            crate::huk_service_ffi::tee_print(
                2,
                $fmt.as_ptr() as _
                $(,$args)*
            )
        }
    };
}

#[macro_export]
macro_rules! tloge {
    ($fmt:expr $(,$args:expr)*) => {
        unsafe {
            crate::huk_service_ffi::tee_print(
                0,
                $fmt.as_ptr() as _
                $(,$args)*
            )
        }
    };
}

// huk_service_config.h

extern "C" {
    pub fn is_huk_service_compatible_plat() -> bool;
    pub fn check_huk_access_permission(cmd_id: u32, uuid: *const TeeUuid) -> bool;
}

pub fn is_huk_service_compatible() -> bool {
    unsafe { is_huk_service_compatible_plat() }
}

pub fn check_huk_access_perm(cmd_id: u32, uuid: &TeeUuid) -> bool {
    unsafe { check_huk_access_permission(cmd_id, uuid) }
}
