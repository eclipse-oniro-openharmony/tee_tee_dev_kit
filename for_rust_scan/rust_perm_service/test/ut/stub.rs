// Copyright (C) 2023 Huawei Technologies Co., Ltd.
// Licensed under the Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2.
// You may obtain a copy of Mulan PSL v2 at:
//     http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
use core::{ffi::c_void, ptr::null_mut};
use std::{mem::MaybeUninit, ptr::null};

use librust_service_ffi::{
    crypto_api::{TeeCryptoAlgorithmId, TeeOperationMode, __TeeOperationHandle},
    defines::SpawnUuid,
    tee_defines::TeeUuid,
    trusted_storage_api::__TeeObjectHandle,
    TeeResult,
};

use crate::{
    perm_srv_common_ffi::{
        conf_queue_t, dyn_conf_t, free, handler_install_obj, malloc, msg_pid_t, pid_t,
    },
    perm_srv_elf_verify::{
        ta_lib_img_unpack_ffi::{
            load_img_info, manifest_extension_t, manifest_info_t, manifest_t, ta_auth_t,
            ta_payload_hdr_t, ta_payload_layer_t, ta_property_t,
        },
        tee_comm_elf_verify_ffi::{
            ecc_pub_key_t, rsa_pub_key_t, sign_config_t, CERT_VERIFY_VERSION, CIPHER_LAYER_VERSION,
        },
        tee_elf_verify_ffi::{elf_hash_data, elf_verify_reply, elf_verify_req},
    },
    perm_srv_ta_config::ta_config_builder_ffi::config_info,
    permission_service_ffi::cert_param_t,
};

#[no_mangle]
pub extern "C" fn snprintf_s(_buffer: *mut u8, size: u64, _fmt: *const u8) -> i32 {
    if size < 30 || size > 40 {
        0
    } else {
        36
    }
}

#[no_mangle]
pub extern "C" fn check_tui_permission(_uuid: *const TeeUuid) -> bool {
    true
}

#[no_mangle]
pub extern "C" fn check_sem_permission(_uuid: *const TeeUuid) -> bool {
    true
}

#[no_mangle]
pub extern "C" fn get_rpmb_permission(_uuid: *const TeeUuid) -> u64 {
    1
}

#[no_mangle]
pub extern "C" fn get_rpmb_threshold(_uuid: *const TeeUuid) -> u64 {
    1
}

#[no_mangle]
pub extern "C" fn hm_getuuid(_pid: pid_t, _uuid: *mut SpawnUuid) -> i32 {
    let uuid = &mut unsafe { *_uuid };
    uuid.uuid.time_low = 1;
    0
}

#[no_mangle]
pub extern "C" fn set_ta_timer_permission(_uuid: *const TeeUuid, _permission: u64) -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn ipc_hunt_by_name(
    _uc_core_id: u8,
    _pthread_name: *const u8,
    _puw_pid: *mut msg_pid_t,
) -> u32 {
    0
}

#[no_mangle]
pub extern "C" fn tee_map_sharemem(
    _src_task: u32,
    _vaddr: u64,
    _size: u64,
    _vaddr_out: *mut u64,
) -> i32 {
    if _vaddr == 0 || _size < 3 || _size == 65 {
        return -1;
    }
    unsafe { *_vaddr_out = _vaddr };
    0
}

#[no_mangle]
pub extern "C" fn munmap(_addr: *mut c_void, s: usize) -> i32 {
    if s == 0 {
        -1
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn TEE_Malloc(size: usize, _hint: core::ffi::c_uint) -> *mut core::ffi::c_void {
    if size == 79 || size == 63 || size == 4 {
        return null_mut();
    }
    unsafe { malloc(size) as *mut c_void }
}

#[no_mangle]
pub extern "C" fn TEE_Free(buffer: *mut core::ffi::c_void) {
    unsafe { free(buffer as _) }
}

static mut PAYLOAD: ta_payload_layer_t = ta_payload_layer_t {
    payload_hdr: ta_payload_hdr_t {
        format_version: CIPHER_LAYER_VERSION,
        mani_ext_size: 1024,
        ta_elf_size: 4096,
        mani_info_size: 198,
        ta_conf_size: 198,
    },
    ta_elf: null_mut(),
    ta_conf: null_mut(),
    conf_registed: false,
};

pub fn set_payload_format() {
    unsafe {
        PAYLOAD.payload_hdr.format_version = 0xff;
    }
}

pub const NAME: &[u8] = b"xxx";

static mut IMG_INFO: load_img_info = load_img_info {
    manifest: manifest_t {
        ta_auth: ta_auth_t {
            caller_num: 0,
            caller_hash: null_mut(),
            auth_enable: false,
        },
        ext: manifest_extension_t {
            distribution: 0,
            api_level: 0,
            sdk_version: 0,
            is_lib: false,
            ssa_enum_enable: false,
            otrp_flag: false,
            mem_page_align: false,
            sys_verify_ta: false,
            target_type: 0,
            target_version: 0,
            hardware_type: 0,
            is_need_release_ta_res: false,
            crash_callback: false,
            is_need_create_msg: false,
            is_need_release_msg: false,
        },
        service_name: NAME.as_ptr() as _,
        hash_val: null_mut(),
        key_val: null_mut(),
        srv_uuid: TeeUuid {
            time_low: 0,
            time_mid: 0,
            time_hi_and_version: 0,
            clock_seq_and_node: [0u8; 8],
        },
        mani_info: manifest_info_t {
            elf_cryptkey_len: 0,
            service_name_len: 3,
            elf_hash_len: 0,
            ta_property: ta_property_t {
                single_instance: 0,
                multi_session: 0,
                multi_command: 0,
                heap_size: 0,
                stack_size: 0,
                instance_keep_alive: 0,
            },
        },
    },
    manifest_buf: null_mut(),
    img_buf: null_mut(),
    img_offset: 0,
    img_size: 0,
    img_version: 0,
    dyn_conf_registed: false,
};

#[no_mangle]
pub extern "C" fn get_ta_payload() -> *mut ta_payload_layer_t {
    unsafe { &mut PAYLOAD as *mut ta_payload_layer_t as _ }
}

#[no_mangle]
pub extern "C" fn get_img_info() -> *mut load_img_info {
    unsafe { &mut IMG_INFO as *mut load_img_info as _ }
}

pub fn set_img_info_v5() {
    unsafe {
        IMG_INFO.img_version = CERT_VERIFY_VERSION;
    }
}

#[no_mangle]
pub extern "C" fn tee_secure_img_manifest_extention_process(
    _extension: *const u8,
    _extension_size: u32,
    _mani_ext: *mut manifest_extension_t,
    _dyn_conf: *mut dyn_conf_t,
) -> TeeResult {
    if _extension_size == 1023 {
        return TeeResult::TEE_ERROR_BAD_PARAMETERS;
    }
    if _extension_size == 1025 {
        let dync = unsafe { &mut *_dyn_conf };
        let dyncf = unsafe { malloc(10) };
        dync.dyn_conf_buffer = dyncf as _;
        dync.dyn_conf_size = 10;
    }
    TeeResult::TEE_SUCCESS
}

static mut PROP: ta_property_t = ta_property_t {
    single_instance: 0,
    multi_session: 0,
    multi_command: 0,
    heap_size: 0,
    stack_size: 0,
    instance_keep_alive: 0,
};

#[no_mangle]
pub extern "C" fn get_ta_property_ptr() -> *mut ta_property_t {
    unsafe { &mut PROP as _ }
}

#[no_mangle]
pub extern "C" fn register_conf(
    _dyn_conf: *const dyn_conf_t,
    _handle: handler_install_obj,
    _obj: *mut c_void,
    _obj_size: u32,
) -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn check_device_id(_config: *mut config_info, _buff: *const u8) -> TeeResult {
    TeeResult::TEE_SUCCESS
}

#[no_mangle]
pub extern "C" fn install_ta_config(
    _obj: *mut core::ffi::c_void,
    _obj_size: u32,
    _conf_queue: *const conf_queue_t,
) -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn install_common_dyn_permission(
    _obj: *mut core::ffi::c_void,
    _obj_size: u32,
    _conf_queue: *const conf_queue_t,
) -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn install_drv_permission(
    _obj: *mut core::ffi::c_void,
    _obj_size: u32,
    _conf_queue: *const conf_queue_t,
) -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn install_drvcall_permission(
    _obj: *mut core::ffi::c_void,
    _obj_size: u32,
    _conf_queue: *const conf_queue_t,
) -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn ac_generate_dyn_uuid_data(_uuid: *const TeeUuid) -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn ipc_msg_snd(
    _uw_msg_id: u32,
    _uw_dst_pid: msg_pid_t,
    _msgp: *const c_void,
    _size: u16,
) -> u32 {
    if _uw_dst_pid == 2 {
        return 1;
    }
    0
}

#[test]
pub fn stub_test() {
    let mut out = 0u64;
    let ret = crate::ut::stub::tee_map_sharemem(0, 0, 48, &mut out as _);
    assert_eq!(ret, -1);
    let ret = crate::ut::stub::tee_map_sharemem(0, 1, 48, &mut out as _);
    assert_eq!(ret, 0);
    let ret = crate::ut::stub::munmap(null_mut() as _, 10);
    assert_eq!(ret, 0);
    let ret = crate::ut::stub::munmap(null_mut() as _, 0);
    assert_eq!(ret, -1);
    let ret = tee_secure_img_manifest_extention_process(null(), 0, null_mut(), null_mut());
    assert_eq!(ret.0, 0);
    let ret = get_ta_property_ptr();
    assert_ne!(ret, null_mut());
    let ret = install_ta_config(null_mut(), 0, null() as _);
    assert_eq!(ret, 0);
    let ret = install_common_dyn_permission(null_mut(), 0, null() as _);
    assert_eq!(ret, 0);
    let ret = install_drv_permission(null_mut(), 0, null() as _);
    assert_eq!(ret, 0);
    let ret = install_drvcall_permission(null_mut(), 0, null() as _);
    assert_eq!(ret, 0);
}

static mut MUSIZE: i32 = 0;

#[no_mangle]
pub extern "C" fn secure_elf_verify(_req: *const c_void, _rsp: *mut c_void) -> TeeResult {
    let req = unsafe { &*(_req as *const elf_verify_req) };
    if req.img_size > 0 {
        unsafe {
            MUSIZE = req.auth_share_size as _;
            println!("size {}", MUSIZE);
        }
        return TeeResult::TEE_SUCCESS;
    }
    TeeResult::TEE_ERROR_BAD_PARAMETERS
}

#[no_mangle]
pub extern "C" fn perm_srv_check_ta_deactivated(_uuid: *const TeeUuid, _version: u16) -> TeeResult {
    if _version == 2 {
        return TeeResult::TEE_ERROR_GENERIC;
    }
    TeeResult::TEE_SUCCESS
}

#[no_mangle]
pub extern "C" fn anti_version_rollback(_reply: *const elf_verify_reply) -> TeeResult {
    TeeResult::TEE_SUCCESS
}

#[test]
pub fn test_for_anti_version_rollback() {
    let reply = unsafe { MaybeUninit::<elf_verify_reply>::zeroed().assume_init() };
    let ret = crate::perm_srv_elf_verify::tee_elf_verify_ffi::anti_version_rollback(
        &reply as *const elf_verify_reply,
    );
    assert_eq!(ret.0, 0);
}

#[no_mangle]
pub extern "C" fn perm_srv_map_from_task(
    _taskid: u32,
    _src_vaddr: u64,
    _size: u32,
    _dst_vaddr: *mut u64,
) -> i32 {
    if _src_vaddr == 0 {
        return -1;
    }
    unsafe {
        *_dst_vaddr = _src_vaddr;
    }
    0
}

#[no_mangle]
pub extern "C" fn perm_srv_unmap_from_task(_vaddr: u64, _size: u32) {}

#[no_mangle]
pub extern "C" fn check_is_blacklist_uuid(_uuid: &TeeUuid) -> bool {
    if unsafe { MUSIZE } == 32 {
        return true;
    }
    false
}

#[no_mangle]
pub extern "C" fn secure_img_copy_rsp_auth_info(rep: *mut elf_verify_reply) -> TeeResult {
    let repp = unsafe { &*(rep) };
    if repp.otrp_ta {
        return TeeResult::TEE_ERROR_GENERIC;
    }
    TeeResult::TEE_SUCCESS
}

#[no_mangle]
pub extern "C" fn ecc_verify_digest(
    _signature: *const u8,
    _sig_len: u32,
    _in_: *mut u8,
    _in_len: u32,
    _pub_: &mut ecc_pub_key_t,
) -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn is_keywest_signature() -> bool {
    false
}

#[no_mangle]
pub extern "C" fn rsa_verify_digest(
    _signature: *mut u8,
    _in_: *mut u8,
    _pub_: *const rsa_pub_key_t,
    _salt_len: u32,
    _hash_nid: i32,
    _padding: i32,
) -> i32 {
    if _signature as u64 == 1 {
        return -1;
    }
    0
}

#[no_mangle]
pub extern "C" fn get_ta_signature_ctrl() -> bool {
    false
}

#[no_mangle]
pub extern "C" fn get_config_cert_param(
    _cert_param: *mut cert_param_t,
    _config: *mut sign_config_t,
) -> TeeResult {
    TeeResult::TEE_SUCCESS
}

#[no_mangle]
pub extern "C" fn copy_hash_data(_hash_data: *mut elf_hash_data, _hash: *mut u8) {}

#[no_mangle]
pub extern "C" fn tee_secure_img_hash_ops(
    _hash_context: *const u8,
    _data_size: usize,
    _hash: *mut u8,
) -> TeeResult {
    TeeResult::TEE_SUCCESS
}

pub const KEY: &[u8] = b"xxx";
#[no_mangle]
pub extern "C" fn get_signature_verify_key_v3(
    _key: *mut u64,
    _config: *const sign_config_t,
    _cert_param: *mut cert_param_t,
    _is_dyn_apply: *mut bool,
) -> TeeResult {
    unsafe { *_key = KEY.as_ptr() as _ }
    TeeResult::TEE_SUCCESS
}

#[no_mangle]
pub extern "C" fn get_signature_verify_key_v5(
    _key: *mut u64,
    _config: *const sign_config_t,
    _cert_param: *mut cert_param_t,
    _is_dyn_apply: *mut bool,
) -> TeeResult {
    unsafe { *_key = KEY.as_ptr() as _ }
    TeeResult::TEE_SUCCESS
}

#[no_mangle]
pub extern "C" fn tee_secure_img_duplicate_buff(
    _src: *const u8,
    _src_size: u32,
    _dst: *mut *mut u8,
) -> TeeResult {
    TeeResult::TEE_SUCCESS
}

#[no_mangle]
pub extern "C" fn tee_secure_img_decrypt_cipher_layer(
    _cipher_layer: *const u8,
    _cipher_size: u32,
    _plaintext_layer: *mut u8,
    _plaintext_size: *mut u32,
) -> TeeResult {
    TeeResult::TEE_SUCCESS
}

#[no_mangle]
pub extern "C" fn TEE_FreeOperation(_operation_handle: *mut __TeeOperationHandle) {}

#[no_mangle]
pub extern "C" fn TEE_AllocateOperation(
    _operation: &mut *mut __TeeOperationHandle,
    _algorithm: TeeCryptoAlgorithmId,
    _mode: TeeOperationMode,
    _max_key_size: u32,
) -> TeeResult {
    *_operation = 1 as _;
    TeeResult::TEE_SUCCESS
}

#[no_mangle]
pub extern "C" fn TEE_CipherInit(
    _operation: *mut __TeeOperationHandle,
    _iv: *const c_void,
    _iv_len: usize,
) {
}

#[no_mangle]
pub extern "C" fn TEE_CipherUpdate(
    _operation: *mut __TeeOperationHandle,
    _src_data: *const c_void,
    _src_len: usize,
    _dest_data: *mut c_void,
    _dest_len: *mut usize,
) -> TeeResult {
    TeeResult::TEE_SUCCESS
}

#[no_mangle]
pub extern "C" fn TEE_CipherDoFinal(
    _operation: *mut __TeeOperationHandle,
    _src_data: *const c_void,
    _src_len: usize,
    _dest_data: *mut c_void,
    _dest_len: *mut usize,
) -> TeeResult {
    TeeResult::TEE_SUCCESS
}

#[no_mangle]
pub extern "C" fn TEE_SetCryptoFlag(
    _operation: *mut __TeeOperationHandle,
    _crypto: u32,
) -> TeeResult {
    TeeResult::TEE_SUCCESS
}
#[no_mangle]
pub extern "C" fn TEE_SetOperationKey(
    _operation: *mut __TeeOperationHandle,
    _key: *const __TeeObjectHandle,
) -> TeeResult {
    TeeResult::TEE_SUCCESS
}
