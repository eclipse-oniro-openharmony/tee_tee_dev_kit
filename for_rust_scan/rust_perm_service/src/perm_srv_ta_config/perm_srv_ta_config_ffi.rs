//!
//! Copyright (c) Huawei Technologies Co., Ltd. 2022-2023. All rights reserved.
//! Description: rust perm srv ta config ffi
//! Create: 2023-03-30
//!

use core::ffi::c_void;

use librust_service_ffi::{defines::SpawnUuid, tee_defines::TeeUuid, TeeResult};

use crate::{
    dlist::dlist_node,
    perm_srv_common_ffi::{conf_queue_t, dyn_conf_t, handler_install_obj, msg_pid_t, pid_t},
    perm_srv_elf_verify::tee_elf_verify_ffi::SN_MAX_SIZE,
};

use super::ta_config_builder_ffi::config_info;

pub const PERMSRV_OK: i32 = 0;
pub const PERMSRV_ERROR: i32 = -1;

#[repr(C)]
pub struct task_config {
    pub taskid: u32,
    pub userid: u32,
    pub head: dlist_node,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct perm_config {
    pub tlv_buf: *const u8,
    pub tlv_len: u32,
    pub policy_version: u32,
    pub cn: [u8; SN_MAX_SIZE],
    pub cn_size: usize,
    pub cert_type: u32,
}

/* define TAG values for TLV Parser */
pub const TLV_TAG_CONFIG_INFO: i32 = 0x00;
pub const TLV_TAG_TA_BASIC_INFO: i32 = 0x01;
pub const TLV_TAG_TA_MANIFEST_INFO: i32 = 0x02;
pub const TLV_TAG_TA_CONTROL_INFO: i32 = 0x03;

pub const TLV_TAG_CALLEETA_INFO: i32 = 0x04;
pub const TLV_TAG_RPMB_INFO: i32 = 0x31;
pub const TLV_TAG_SFS_INFO: i32 = 0x32;
pub const TLV_TAG_SE_INFO: i32 = 0x33;
pub const TLV_TAG_TUI_INFO: i32 = 0x34;
pub const TLV_TAG_DEBUG_INFO: i32 = 0x35;
pub const TLV_TAG_CERT_INFO: i32 = 0x36;

pub const TLV_TAG_RPMB_PERMISSION: i32 = 0x71;
pub const TLV_TAG_SFS_PERMISSION: i32 = 0x72;
pub const TLV_TAG_CERT_PERMISSION: i32 = 0x73;
pub const TLV_TAG_CALLEETA_UUID: i32 = 0x41;

pub const TLV_TAG_UUID: i32 = 0x01 + 0xFF;
pub const TLV_TAG_SERVICE_NAME: i32 = 0x02 + 0xFF;
pub const TLV_TAG_SINGLE_INSTANCE: i32 = 0x11 + 0xFF;
pub const TLV_TAG_MULTI_SESSION: i32 = 0x12 + 0xFF;
pub const TLV_TAG_MULTI_COMMAND: i32 = 0x13 + 0xFF;
pub const TLV_TAG_HEAP_SIZE: i32 = 0x14 + 0xFF;
pub const TLV_TAG_STACK_SIZE: i32 = 0x15 + 0xFF;
pub const TLV_TAG_INSTANCE_KEEP_ALIVE: i32 = 0x16 + 0xFF;
pub const TLV_TAG_MEM_PAGE_ALIGN: i32 = 0x17 + 0xFF;
pub const TLV_TAG_TARGET_TYPE: i32 = 0x18 + 0xFF;
pub const TLV_TAG_SYS_VERIFY_TA: i32 = 0x19 + 0xFF;
pub const TLV_TAG_RPMB_SIZE: i32 = 0x21 + 0xFF;
pub const TLV_TAG_RPMB_INSE: i32 = 0x22 + 0xFF;
pub const TLV_TAG_RPMB_GENERAL: i32 = 0x23 + 0xFF;
pub const TLV_TAG_RPMB_SPECIFIC: i32 = 0x24 + 0xFF;
pub const TLV_TAG_SFS_PROVISION_KEY: i32 = 0x31 + 0xFF;
pub const TLV_TAG_SFS_INSE: i32 = 0x32 + 0xFF;
pub const TLV_TAG_SE_OPEN_SESSION: i32 = 0x41 + 0xFF;
pub const TLV_TAG_TUI_GENERAL: i32 = 0x61 + 0xFF;
pub const TLV_TAG_TA_MANAGER: i32 = 0x71 + 0xFF;
pub const TLV_TAG_CALLEETA_COMMAND_ID: i32 = 0x81 + 0xFF;
pub const TLV_TAG_DEBUG_STATUS: i32 = 0x51 + 0xFF;
pub const TLV_TAG_DEBUG_DEVICE_ID: i32 = 0x52 + 0xFF;

pub fn check_tui_permission(uuid: &TeeUuid) -> bool {
    unsafe { crate::perm_srv_common_ffi::check_tui_permission(uuid) }
}

pub fn check_sem_permission(uuid: &TeeUuid) -> bool {
    unsafe { crate::perm_srv_common_ffi::check_sem_permission(uuid) }
}

pub fn get_rpmb_permission(uuid: &TeeUuid) -> u64 {
    unsafe { crate::perm_srv_common_ffi::get_rpmb_permission(uuid) }
}

pub fn get_rpmb_threshold(uuid: &TeeUuid) -> u32 {
    unsafe { crate::perm_srv_common_ffi::get_rpmb_threshold(uuid) }
}

pub fn set_ta_timer_permission(uuid: &TeeUuid, permission: u64) -> i32 {
    unsafe { crate::perm_srv_common_ffi::set_ta_timer_permission(uuid, permission) }
}

pub fn ac_generate_dyn_uuid_data(uuid: &TeeUuid) -> i32 {
    unsafe { crate::perm_srv_common_ffi::ac_generate_dyn_uuid_data(uuid) }
}

pub fn ipc_msg_snd(uw_msg_id: u32, uw_dst_pid: msg_pid_t, msgp: *const c_void, size: u16) -> u32 {
    unsafe { crate::perm_srv_common_ffi::ipc_msg_snd(uw_msg_id, uw_dst_pid, msgp, size) }
}

pub fn ipc_hunt_by_name(uc_core_id: u8, pthread_name: &[u8], puw_pid: &mut msg_pid_t) -> u32 {
    unsafe {
        crate::perm_srv_common_ffi::ipc_hunt_by_name(
            uc_core_id,
            pthread_name.as_ptr(),
            puw_pid as _,
        )
    }
}

pub fn register_conf(
    dyn_conf: &dyn_conf_t,
    handle: handler_install_obj,
    obj: *mut c_void,
    obj_size: u32,
) -> i32 {
    unsafe { crate::perm_srv_common_ffi::register_conf(dyn_conf, handle, obj, obj_size) }
}

pub fn check_device_id(config: &mut config_info, buff: &[u8]) -> TeeResult {
    unsafe {
        crate::perm_srv_common_ffi::check_device_id(config, buff.as_ptr() as _, buff.len() as _)
    }
}

extern "C" {
    pub fn install_ta_config(
        obj: *mut core::ffi::c_void,
        obj_size: u32,
        conf_queue: *const conf_queue_t,
    ) -> i32;
}

extern "C" {
    pub fn install_common_dyn_permission(
        obj: *mut core::ffi::c_void,
        obj_size: u32,
        conf_queue: *const conf_queue_t,
    ) -> i32;
}

extern "C" {
    pub fn install_drv_permission(
        obj: *mut core::ffi::c_void,
        obj_size: u32,
        conf_queue: *const conf_queue_t,
    ) -> i32;
}

extern "C" {
    pub fn install_drvcall_permission(
        obj: *mut core::ffi::c_void,
        obj_size: u32,
        conf_queue: *const conf_queue_t,
    ) -> i32;
}

pub fn hm_getuuid(pid: pid_t, uuid: &mut SpawnUuid) -> i32 {
    unsafe { crate::perm_srv_common_ffi::hm_getuuid(pid, uuid) }
}
