//!
//! Copyright (c) Huawei Technologies Co., Ltd. 2022-2023. All rights reserved.
//! Description: perm service elf verify ffi
//! Create: 2023-03-30
//!

use librust_service_ffi::{tee_defines::TeeUuid, TeeResult};

use crate::perm_srv_elf_verify::ta_lib_img_unpack_ffi::{
    manifest_extension_t, ta_payload_hdr_t, ta_property_t, MAX_TAFS_NAME_LEN,
    SERVICE_NAME_MAX_IN_MANIFEST,
};

use super::ta_lib_img_unpack_ffi::{load_img_info, ta_payload_layer_t};

// tee_elf_verify.h
pub const SN_MAX_SIZE: usize = 64;
pub const MAX_IMAGE_HASH_SIZE: usize = 64;

#[repr(packed)]
#[derive(Clone, Copy)]
pub struct elf_verify_req {
    pub version: u32,
    pub img_size: u32,
    pub tmp_file: [u8; MAX_TAFS_NAME_LEN],
    pub otrp_ta: bool,
    pub auth_share_addr: u64,
    pub auth_share_size: u32,
    pub reply_share_addr: u64,
    pub reply_share_size: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct elf_verify_reply {
    pub service_name: [u8; SERVICE_NAME_MAX_IN_MANIFEST],
    pub service_name_len: u32,
    pub srv_uuid: TeeUuid,
    pub ta_property: ta_property_t,
    pub payload_hdr: ta_payload_hdr_t,
    pub off_manifest_buf: i32,
    pub off_ta_elf: i32,
    pub verify_result: TeeResult,
    pub mani_ext: manifest_extension_t,
    pub conf_registed: bool,
    pub dyn_conf_registed: bool,
    pub elf_verify_errno: i32,
    pub otrp_ta: bool,
    pub auth_share_addr: u64,
    pub auth_share_size: u32,
    pub auth_map_addr: u64,
    pub elf_hash: [u8; MAX_IMAGE_HASH_SIZE],
    pub hash_size: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct elf_hash_data {
    pub elf_hash: *mut u8,
    pub hash_size: u32,
}

#[repr(C)]
pub struct leaf_cert {
    pub cert_buf: *mut u8,
    pub cert_buf_len: u32,
}

pub fn secure_elf_verify(req: &elf_verify_req, rep: &mut elf_verify_reply) -> TeeResult {
    unsafe {
        crate::perm_srv_common_ffi::secure_elf_verify(
            req as *const elf_verify_req as _,
            rep as *mut elf_verify_reply as _,
        )
    }
}

pub fn tee_secure_img_parse_manifest(
    manifest_ext: &[u8],
    ext_size: &mut u32,
    control: bool,
    config_target_type: u32,
) -> TeeResult {
    unsafe {
        crate::perm_srv_common_ffi::tee_secure_img_parse_manifest(
            manifest_ext.as_ptr(),
            ext_size,
            control,
            config_target_type,
        )
    }
}

pub fn perm_srv_check_ta_deactivated(uuid: &TeeUuid, version: u16) -> TeeResult {
    unsafe { crate::perm_srv_common_ffi::perm_srv_check_ta_deactivated(uuid, version) }
}

pub fn anti_version_rollback(reply: *const elf_verify_reply) -> TeeResult {
    unsafe { crate::perm_srv_common_ffi::anti_version_rollback(reply) }
}

pub fn get_ta_payload() -> *mut ta_payload_layer_t {
    unsafe { crate::perm_srv_common_ffi::get_ta_payload() }
}

pub fn get_img_info() -> *mut load_img_info {
    unsafe { crate::perm_srv_common_ffi::get_img_info() }
}

pub fn check_is_blacklist_uuid(uuid: &TeeUuid) -> bool {
    unsafe { crate::perm_srv_common_ffi::check_is_blacklist_uuid(uuid) }
}
