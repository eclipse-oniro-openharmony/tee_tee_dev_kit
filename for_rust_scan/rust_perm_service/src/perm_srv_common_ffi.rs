//!
//! Copyright (c) Huawei Technologies Co., Ltd. 2022-2023. All rights reserved.
//! Description: perm service ffi
//! Create: 2023-03-30
//!

use core::ffi::c_void;

use librust_service_ffi::{defines::SpawnUuid, tee_defines::TeeUuid, TeeResult};

use crate::{
    dlist::dlist_node,
    perm_srv_elf_verify::{
        ta_lib_img_unpack_ffi::{
            load_img_info, manifest_extension_t, ta_payload_layer_t, ta_property_t,
        },
        tee_comm_elf_verify_ffi::{ecc_pub_key_t, rsa_pub_key_t, sign_config_t},
        tee_elf_verify_ffi::{elf_hash_data, elf_verify_reply},
    },
    perm_srv_ta_config::ta_config_builder_ffi::config_info,
    permission_service_ffi::cert_param_t,
};

pub const GLOBAL_HANDLE: u32 = 0;
pub const SRE_OK: u32 = 0;
pub const REGISTER_ELF_REQ: u32 = 0x3E;

extern "C" {
    // perm_srv_ta_ctrl.h
    pub fn perm_srv_check_ta_deactivated(uuid: *const TeeUuid, version: u16) -> TeeResult;

    // handle_anti_rollback.h
    pub fn anti_version_rollback(reply: *const elf_verify_reply) -> TeeResult;
}

// tee_service_public.h
#[cfg(any(
    feature = "config_remote_attestation",
    feature = "config_remote_attestation_a32"
))]
pub const MAX_IMAGE_HASH_SIZE: usize = 64;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct reg_ta_info {
    pub taskid: u32,
    pub uuid: TeeUuid,
    pub userid: u32,
    pub ssa_enum_enable: bool,
    #[cfg(any(
        feature = "config_remote_attestation",
        feature = "config_remote_attestation_a32"
    ))]
    pub elf_hash: [u8; MAX_IMAGE_HASH_SIZE],
    #[cfg(any(
        feature = "config_remote_attestation",
        feature = "config_remote_attestation_a32"
    ))]
    pub hash_size: u32,
}

// msg_ops.h
pub type msg_pid_t = u32;
extern "C" {
    pub fn ipc_msg_snd(
        uw_msg_id: u32,
        uw_dst_pid: msg_pid_t,
        msgp: *const c_void,
        size: u16,
    ) -> u32;

    pub fn ipc_hunt_by_name(
        uc_core_id: u8,
        pthread_name: *const u8,
        puw_pid: *mut msg_pid_t,
    ) -> u32;
}

// perm_srv_common.h
extern "C" {
    pub fn perm_srv_map_from_task(
        taskid: u32,
        src_vaddr: u64,
        size: u32,
        dst_vaddr: *mut u64,
    ) -> i32;
    pub fn perm_srv_unmap_from_task(vaddr: u64, size: u32);
}

// tee_log.h
extern "C" {
    pub fn tee_print(log_level: u32, fmt_string: *const u8, ...);
}

extern "C" {
    pub fn printf(format: *const u8, ...) -> i32;
}

// C style print
// 1) str should end with \0
// 2) can not check format in compile time
// 3) use '%' instead of '{}' for format string
// 4) each TA has different log file (for example:/data/vendor/log/tee/LOG@UUID-0)
// example: tlogi!("xyz is %d\0", xyz);
#[cfg(not(test))]
#[macro_export]
macro_rules! tlogi {
    ($fmt:expr $(,$args:expr)*) => {
        unsafe {
            crate::perm_srv_common_ffi::tee_print(
                2,
                $fmt.as_ptr() as _
                $(,$args)*
            )
        }
    };
}

#[cfg(not(test))]
#[macro_export]
macro_rules! tloge {
    ($fmt:expr $(,$args:expr)*) => {
        unsafe {
            crate::perm_srv_common_ffi::tee_print(
                0,
                $fmt.as_ptr() as _
                $(,$args)*
            )
        }
    };
}

#[cfg(not(test))]
#[macro_export]
macro_rules! tlogd {
    ($fmt:expr $(,$args:expr)*) => {
        unsafe {
            crate::perm_srv_common_ffi::tee_print(
                3,
                $fmt.as_ptr() as _
                $(,$args)*
            )
        }
    };
}

#[cfg(test)]
#[macro_export]
macro_rules! tlogi {
    ($fmt:expr $(,$args:expr)*) => {
        unsafe {
            crate::perm_srv_common_ffi::printf(
                $fmt.as_ptr() as _
                $(,$args)*
            );
            crate::perm_srv_common_ffi::printf("\n\0".as_ptr() as _);
        }
    };
}

#[cfg(test)]
#[macro_export]
macro_rules! tloge {
    ($fmt:expr $(,$args:expr)*) => {
        unsafe {
            crate::perm_srv_common_ffi::printf(
                $fmt.as_ptr() as _
                $(,$args)*
            );
            crate::perm_srv_common_ffi::printf("\n\0".as_ptr() as _);
        }
    };
}

#[cfg(test)]
#[macro_export]
macro_rules! tlogd {
    ($fmt:expr $(,$args:expr)*) => {
        unsafe {
            crate::perm_srv_common_ffi::printf(
                $fmt.as_ptr() as _
                $(,$args)*
            );
            crate::perm_srv_common_ffi::printf("\n\0".as_ptr() as _);
        }
    };
}

pub const HMPID_OFFSET: u32 = 16;
pub const HMPID_MASK: u32 = (1u32 << HMPID_OFFSET) - 1;

// ipclib.h
#[macro_export]
macro_rules! pid_to_hmpid {
    ($pid:expr) => {
        ((($pid) & crate::perm_srv_common_ffi::HMPID_MASK) as u32)
    };
}

// ta_framework.h
pub const RPMB_TASK_NAME: &[u8] = b"task_rpmb\0";
pub const SSA_SERVICE_NAME: &[u8] = b"task_ssa\0";
pub const TUI_SERVICE_NAME: &[u8] = b"task_tui\0";

// tee_inner_uuid.h
pub const TEE_SERVICE_SE: TeeUuid = TeeUuid {
    time_low: 0x91f0cf6b,
    time_mid: 0xbd4b,
    time_hi_and_version: 0x456e,
    clock_seq_and_node: [0x86, 0x2d, 0x3f, 0xa6, 0x1a, 0xb1, 0xa4, 0xac],
};
pub const TEE_SERVICE_SEM: TeeUuid = TeeUuid {
    time_low: 0xaaa862d1,
    time_mid: 0x22fe,
    time_hi_and_version: 0x4609,
    clock_seq_and_node: [0xa4, 0xee, 0x86, 0x67, 0xf6, 0x53, 0x8f, 0x18],
};

// tee_uuid.h
pub const NODE_LEN: usize = 8;

// sre_access_control.h
pub const TIMER_GROUP_PERMISSION: u64 = 0x100000;
pub const GENERAL_GROUP_PERMISSION: u64 = 0x00000;

// asan1.h
pub const V_ASN1_SEQUENCE: u8 = 16;
pub const V_ASN1_INTEGER: i32 = 2;

// ac.h ac_dyanmic.h
extern "C" {
    pub fn ac_generate_dyn_uuid_data(uuid: *const TeeUuid) -> i32;
}

// tee_time_sys.h
extern "C" {
    pub fn set_ta_timer_permission(uuid: *const TeeUuid, permission: u64) -> i32;
}

// dyn_conf_dispatch_inf.h
pub const MAX_IMAGE_LEN: u32 = 0x800000;

#[repr(C)]
pub struct conf_node_t {
    pub head: dlist_node,
    pub tag: u32,
    pub tp: u32,
    pub size: u32,
    pub value: *const u8,
}

#[repr(C)]
pub struct conf_queue_t {
    pub queue: dlist_node,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct dyn_conf_t {
    pub dyn_conf_size: u32,
    pub dyn_conf_buffer: *mut u8,
}

// target_type.h
// enum target_type {
pub const TA_TARGET_TYPE: u32 = 0;
pub const DRV_TARGET_TYPE: u32 = 1;
pub const DYN_LIB_TARGET_TYPE: u32 = 2;
pub const SRV_TARGET_TYPE: u32 = 3;
pub const CLIENT_TARGET_TYPE: u32 = 4;
pub const LIVEPATCH_TARGET_TYPE: u32 = 5;
pub const MAX_TARGET_TYPE: u32 = 6;

// securec.h
extern "C" {
    pub fn snprintf_s(strDest: *mut u8, destMax: usize, count: usize, f: *const u8, ...) -> i32;
}

// permission_config.h
extern "C" {
    pub fn check_tui_permission(uuid: *const TeeUuid) -> bool;
    pub fn check_sem_permission(uuid: *const TeeUuid) -> bool;
    pub fn get_rpmb_permission(uuid: *const TeeUuid) -> u64;
    pub fn get_rpmb_threshold(uuid: *const TeeUuid) -> u32;
}

// tee_elf_verify.h
extern "C" {
    pub fn secure_elf_verify(req: *const c_void, rep: *mut c_void) -> TeeResult;
    pub fn tee_secure_img_parse_manifest(
        manifest_ext: *const u8,
        ext_size: *mut u32,
        control: bool,
        config_target_type: u32,
    ) -> TeeResult;
}

// perm_srv_ta_crl.h
pub const TLV_LLEN: usize = core::mem::size_of::<u32>();

// tee_hw_ext_api.h
extern "C" {
    pub fn tee_ext_get_device_unique_id(device_unique_id: *mut u8, length: &mut u32) -> TeeResult;
}

// dyn_conf_dispatch_inf.h
pub type handler_conf_to_obj = unsafe extern "C" fn(
    node: *mut *mut dlist_node,
    q: *const conf_queue_t,
    o: *mut c_void,
    s: u32,
) -> i32;
pub type handler_install_obj =
    unsafe extern "C" fn(o: *mut c_void, s: u32, q: *const conf_queue_t) -> i32;

extern "C" {
    pub fn handle_conf_node_to_obj(
        pos: *mut *mut dlist_node,
        handle: handler_conf_to_obj,
        obj: *mut c_void,
        obj_size: u32,
    ) -> i32;

    pub fn register_conf(
        dyn_conf: *const dyn_conf_t,
        handle: handler_install_obj,
        obj: *mut c_void,
        obj_size: u32,
    ) -> i32;
}

// ta_config_builder.h
extern "C" {
    pub fn check_device_id(config: *mut config_info, buff: *const u8, len: u32) -> TeeResult;
}

// libc
extern "C" {
    pub fn strtol(__restrict: *const c_void, __restrict: *const *const c_void, base: i32) -> i64;
    pub fn malloc(n: usize) -> *mut c_void;
    pub fn free(addr: *mut c_void);
}

// livepatch_load_api.h
extern "C" {
    pub fn is_livepatch_type(ty: i32) -> bool;
    pub fn parse_livepatch_manifest_item(ty: i32, value: *const u8, value_len: u32) -> TeeResult;
}

// tee_elf_verify_inner.h
extern "C" {
    pub fn get_ta_payload() -> *mut ta_payload_layer_t;
    pub fn get_img_info() -> *mut load_img_info;
    pub fn tee_secure_img_duplicate_buff(
        src: *const u8,
        src_size: u32,
        dst: *mut *mut u8,
    ) -> TeeResult;
    pub fn get_ta_property_ptr() -> *mut ta_property_t;
    pub fn tee_secure_img_manifest_extention_process(
        extension: *const u8,
        extension_size: u32,
        mani_ext: *mut manifest_extension_t,
        dyn_conf: *mut dyn_conf_t,
    ) -> TeeResult;
    pub fn copy_hash_data(hash_data: *mut elf_hash_data, hash_src: *mut u8, hash_src_size: u32);
    pub fn secure_img_copy_rsp_auth_info(rep: *mut elf_verify_reply) -> TeeResult;
}

// tee_elf_decrypt.h
extern "C" {
    pub fn tee_secure_img_decrypt_cipher_layer(
        cipher_layer: *const u8,
        cipher_size: u32,
        plaintext_layer: *mut u8,
        plaintext_size: *mut u32,
    ) -> TeeResult;
}

// tee_sms_signature.h
extern "C" {
    pub fn get_cms_signature_size(signature_buff: *const u8, signature_max_size: u32) -> u32;
}

// tee_crypto_hal.h
// enum CRYPTO_ENGINE
pub const DX_CRYPTO: u32 = 0;
pub const EPS_CRYPTO: u32 = 1;
pub const SOFT_CRYPTO: u32 = 2;
pub const SEC_CRYPTO: u32 = 3;
pub const CRYPTO_ENGINE_MAX: u32 = 1024;

// tee_v5_elf_verify.h
extern "C" {
    pub fn get_signature_verify_key_v5(
        key: *mut u64,
        config: *const sign_config_t,
        cert_param: *mut cert_param_t,
        is_dyn_apply: *mut bool,
    ) -> TeeResult;
}

// tee_v3_elf_verify.h
extern "C" {
    pub fn get_signature_verify_key_v3(
        key: *mut u64,
        config: *const sign_config_t,
        cert_param: *mut cert_param_t,
        is_dyn_apply: *mut bool,
    ) -> TeeResult;
}

// crypto_ec_wrapper.h
extern "C" {
    pub fn ecc_verify_digest(
        signature: *const u8,
        sig_len: u32,
        in_: *mut u8,
        in_len: u32,
        pub_: *mut ecc_pub_key_t,
    ) -> i32;
}

// check_ta_version.h
extern "C" {
    pub fn is_keywest_signature() -> bool;
}

// crypto_rsa_wrapper.h
extern "C" {
    pub fn rsa_verify_digest(
        signature: *mut u8,
        sig_size: u32,
        in_: *mut u8,
        in_len: u32,
        pub_: *const rsa_pub_key_t,
        salt_len: u32,
        hash_nid: i32,
        padding: i32,
    ) -> i32;
}

// ta_load_config.h
extern "C" {
    pub fn get_ta_signature_ctrl() -> bool;
}

// tee_perm_img.h
extern "C" {
    pub fn get_config_cert_param(
        cert_param: *mut cert_param_t,
        config: *mut sign_config_t,
    ) -> TeeResult;
}

// tee_load_key_ops.h
extern "C" {
    pub fn tee_secure_img_hash_ops(
        hash_context: *const u8,
        data_size: usize,
        hash: *mut u8,
        hash_size: usize,
    ) -> TeeResult;
}

// ta_cms_signature_verify.h
extern "C" {
    pub fn ta_cms_signature_verify(
        signature: *mut u8,
        signature_size: u32,
        hash: *mut u8,
        hash_size: u32,
    ) -> TeeResult;
}

// procmgr_ext.h
pub type pid_t = i32;
extern "C" {
    pub fn hm_getuuid(pid: pid_t, uuid: *mut SpawnUuid) -> i32;
}

// permission_config.h
extern "C" {
    pub fn check_is_blacklist_uuid(uuid: *const TeeUuid) -> bool;
}
