//!
//! Copyright (c) Huawei Technologies Co., Ltd. 2022-2023. All rights reserved.
//! Description: rust ta config build ffi
//! Create: 2023-03-30
//!

use librust_service_ffi::tee_defines::TeeUuid;

use crate::dlist::dlist_node;

/* CN format in TA's certificate: "uuid string" + "_" + "service name" */
pub const POLICY_OLD_VERSION: u8 = 0;
pub const POLICY_VERSION_ONE: u8 = 1;
pub const UUID_STR_LEN: usize = 36;
pub const MAX_CALLEE_COMMAND_COUNT: usize = 100;
pub const MAX_CALLEE_TA_COUNT: u32 = 100;
pub const TA_CERT_MAX_CN_INFO_LEN: usize = 64;
pub const TA_CERT_CN_UNDERLINE_SIZE: usize = 1;
pub const RPMB_GENERAL_PERMISSION: u64 = 0x01;
pub const RPMB_RESET_PERMISSION: u64 = 0x04;
pub const SE_OPEN_SESSION_PERMISSION: u64 = 0x01;
pub const TUI_PERMISSION: u64 = 0x01;
pub const CERT_GENERAL_PERMISSION: u64 = 0x01;
pub const MAX_SERVICE_NAME_LEN: usize = 40;
pub const TLV_DEVICE_ID_LEN: usize = 64;
pub const DEVICE_ID_LEN: usize = 32;
pub const LEN_OFFSET_VALUE: u8 = 4;

// enum ta_config_tags
pub const CONFIGINFO: i32 = 0;
pub const CONFIGINFO_TA_BASIC_INFO: i32 = 1;
pub const CONFIGINFO_TA_BASIC_INFO_SERVICE_NAME: i32 = 2;
pub const CONFIGINFO_TA_BASIC_INFO_SERVICE_NAME_SERVICE_NAME: i32 = 3;
pub const CONFIGINFO_TA_BASIC_INFO_UUID: i32 = 4;
pub const CONFIGINFO_TA_BASIC_INFO_UUID_UUID: i32 = 5;
pub const CONFIGINFO_TA_MANIFEST_INFO: i32 = 6;
pub const CONFIGINFO_TA_MANIFEST_INFO_INSTANCE_KEEP_ALIVE: i32 = 7;
pub const CONFIGINFO_TA_MANIFEST_INFO_INSTANCE_KEEP_ALIVE_INSTANCE_KEEP_ALIVE: i32 = 8;
pub const CONFIGINFO_TA_MANIFEST_INFO_STACK_SIZE: i32 = 9;
pub const CONFIGINFO_TA_MANIFEST_INFO_STACK_SIZE_STACK_SIZE: i32 = 10;
pub const CONFIGINFO_TA_MANIFEST_INFO_HEAP_SIZE: i32 = 11;
pub const CONFIGINFO_TA_MANIFEST_INFO_HEAP_SIZE_HEAP_SIZE: i32 = 12;
pub const CONFIGINFO_TA_MANIFEST_INFO_TARGET_TYPE: i32 = 13;
pub const CONFIGINFO_TA_MANIFEST_INFO_TARGET_TYPE_TARGET_TYPE: i32 = 14;
pub const CONFIGINFO_TA_MANIFEST_INFO_MULTI_COMMAND: i32 = 15;
pub const CONFIGINFO_TA_MANIFEST_INFO_MULTI_COMMAND_MULTI_COMMAND: i32 = 16;
pub const CONFIGINFO_TA_MANIFEST_INFO_MULTI_SESSION: i32 = 17;
pub const CONFIGINFO_TA_MANIFEST_INFO_MULTI_SESSION_MULTI_SESSION: i32 = 18;
pub const CONFIGINFO_TA_MANIFEST_INFO_SINGLE_INSTANCE: i32 = 19;
pub const CONFIGINFO_TA_MANIFEST_INFO_SINGLE_INSTANCE_SINGLE_INSTANCE: i32 = 20;
pub const CONFIGINFO_TA_CONTROL_INFO: i32 = 21;
pub const CONFIGINFO_TA_CONTROL_INFO_RPMB_INFO: i32 = 22;
pub const CONFIGINFO_TA_CONTROL_INFO_RPMB_INFO_RPMB_SIZE: i32 = 23;
pub const CONFIGINFO_TA_CONTROL_INFO_RPMB_INFO_RPMB_SIZE_RPMB_SIZE: i32 = 24;
pub const CONFIGINFO_TA_CONTROL_INFO_RPMB_INFO_RPMB_PERMISSION: i32 = 25;
pub const CONFIGINFO_TA_CONTROL_INFO_RPMB_INFO_RPMB_PERMISSION_RPMB_GENERAL: i32 = 26;
pub const CONFIGINFO_TA_CONTROL_INFO_RPMB_INFO_RPMB_PERMISSION_RPMB_GENERAL_RPMB_GENERAL: i32 = 27;
pub const CONFIGINFO_TA_CONTROL_INFO_SE_INFO: i32 = 28;
pub const CONFIGINFO_TA_CONTROL_INFO_SE_INFO_SE_OPEN_SESSION: i32 = 29;
pub const CONFIGINFO_TA_CONTROL_INFO_SE_INFO_SE_OPEN_SESSION_SE_OPEN_SESSION: i32 = 30;
pub const CONFIGINFO_TA_CONTROL_INFO_TUI_INFO: i32 = 31;
pub const CONFIGINFO_TA_CONTROL_INFO_TUI_INFO_TUI_GENERAL: i32 = 32;
pub const CONFIGINFO_TA_CONTROL_INFO_TUI_INFO_TUI_GENERAL_TUI_GENERAL: i32 = 33;
pub const CONFIGINFO_TA_CONTROL_INFO_DEBUG_INFO: i32 = 34;
pub const CONFIGINFO_TA_CONTROL_INFO_DEBUG_INFO_DEBUG_STATUS: i32 = 35;
pub const CONFIGINFO_TA_CONTROL_INFO_DEBUG_INFO_DEBUG_STATUS_DEBUG_STATUS: i32 = 36;
pub const CONFIGINFO_TA_CONTROL_INFO_DEBUG_INFO_DEBUG_DEVICE_ID: i32 = 37;
pub const CONFIGINFO_TA_CONTROL_INFO_DEBUG_INFO_DEBUG_DEVICE_ID_DEBUG_DEVICE_ID: i32 = 38;
pub const CONFIGINFO_TA_MANIFEST_INFO_MEM_PAGE_ALIGN: i32 = 39;
pub const CONFIGINFO_TA_MANIFEST_INFO_MEM_PAGE_ALIGN_MEM_PAGE_ALIGN: i32 = 40;
pub const CONFIGINFO_TA_MANIFEST_INFO_SYS_VERIFY_TA: i32 = 41;
pub const CONFIGINFO_TA_MANIFEST_INFO_SYS_VERIFY_TA_SYS_VERIFY_TA: i32 = 42;
pub const CONFIGINFO_TA_CONTROL_INFO_TA_MANAGER: i32 = 43;
pub const CONFIGINFO_TA_CONTROL_INFO_TA_MANAGER_TA_MANAGER: i32 = 44;
pub const CONFIGINFO_TA_CONTROL_INFO_CERT_INFO: i32 = 45;
pub const CONFIGINFO_TA_CONTROL_INFO_CERT_INFO_CERT_PERMISSION: i32 = 46;
pub const CONFIGINFO_TA_CONTROL_INFO_CERT_INFO_CERT_PERMISSION_CERT_PERMISSION: i32 = 47;
pub const CONFIGINFO_UNUSED: i32 = 48;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ta_manifest_info {
    pub single_instance: bool,
    pub multi_session: bool,
    pub multi_command: bool,
    pub instance_keep_alive: bool,
    pub heap_size: u32,
    pub stack_size: u32,
    pub mem_page_align: bool,
    pub target_type: u32,
    pub sys_verify_ta: bool,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ta_rpmb_info {
    pub size: u32,
    pub permissions: u64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ta_sfs_info {
    pub permissions: u64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ta_se_info {
    pub permissions: u64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ta_tui_info {
    pub permissions: u64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ta_cert_perm_info {
    pub permissions: u64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ta_debug_info {
    pub status: bool,
    pub valid_device: bool,
}

#[repr(C)]
pub struct callee_ta_info {
    pub next: u64, // pointer
    pub uuid: TeeUuid,
    pub command_num: u32,
    pub command_id: u64, // pointer
}

#[repr(C)]
pub struct ta_control_info {
    pub rpmb_info: ta_rpmb_info,
    pub sfs_info: ta_sfs_info,
    pub se_info: ta_se_info,
    pub tui_info: ta_tui_info,
    pub cert_info: ta_cert_perm_info,
    pub ta_manager: u32,
    pub callee_info: u64, // pointer
    pub debug_info: ta_debug_info,
}

#[repr(C)]
pub struct config_info {
    pub head: dlist_node,
    pub uuid: TeeUuid,
    pub service_name: [u8; MAX_SERVICE_NAME_LEN],
    pub service_name_len: u32,
    pub version: u32,
    pub manifest_info: ta_manifest_info,
    pub control_info: ta_control_info,
}
