// Copyright (C) 2023 Huawei Technologies Co., Ltd.
// Licensed under the Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2.
// You may obtain a copy of Mulan PSL v2 at:
//     http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
use std::{mem::MaybeUninit, ptr::null_mut};

use librust_service_ffi::{
    buffer::TeeMemory,
    core::TEE_Free,
    crypto_api::{TeeCryptoAlgorithmId, TeeOperationHandle, TeeOperationMode},
    tee_defines::TeeUuid,
    trusted_storage_api::{InitializedTransientObject, TeeAttribute, TeeObjectHandle},
    TeeResult,
};

use crate::{
    perm_srv_common_ffi::{
        dyn_conf_t, free, CLIENT_TARGET_TYPE, DRV_TARGET_TYPE, MAX_IMAGE_LEN, SOFT_CRYPTO,
        SRV_TARGET_TYPE, TA_TARGET_TYPE, TIMER_GROUP_PERMISSION,
    },
    perm_srv_ta_config::perm_srv_ta_config_ffi::{
        ac_generate_dyn_uuid_data, install_common_dyn_permission, install_drv_permission,
        install_drvcall_permission, register_conf, set_ta_timer_permission,
    },
    permission_service_ffi::{cert_param_t, MAX_PUB_KEY_SIZE},
    tlogd, tloge, tlogi,
};
use core::mem::size_of;

use super::{
    ta_lib_img_unpack_ffi::*,
    tee_comm_elf_verify_ffi::*,
    tee_elf_verify_ffi::{
        elf_hash_data, elf_verify_reply, get_img_info, get_ta_payload, leaf_cert,
    },
};

#[macro_export]
macro_rules! swap_16_endianness_big_to_small {
    ($x: expr) => {
        ((((($x as u16) & 0x00ff) << 8) | ((($x as u16) & 0xff00) >> 8)) as u16)
    };
}

// SAFETY: use global variables means so many unsafe, but all unsafe is SAFE because there is only one thread.
static mut g_is_encrypted_sec: bool = true;
static mut g_image_header: ta_image_hdr_sec_t = ta_image_hdr_sec_t {
    img_identity: teec_image_identity {
        magic_num1: 0,
        magic_num2: 0,
        version_num: 0,
    },
    context_len: 0,
    ta_key_version: 0,
};
static mut g_ta_cipher_layer: ta_cipher_layer_t = ta_cipher_layer_t {
    cipher_hdr: ta_cipher_hdr_t {
        key_size: 0,
        iv_size: 0,
        signature_alg: 0,
    },
    key: null_mut(),
    iv: null_mut(),
};
static mut g_cipher_layer_len: u32 = 0;

#[cfg(test)]
pub fn set_cipher_layer(alg: u32, keylen: usize, ivlen: usize) {
    use crate::perm_srv_common_ffi::malloc;

    unsafe {
        let a = malloc(keylen);
        g_ta_cipher_layer.key = a as _;
        let b = malloc(ivlen);
        g_ta_cipher_layer.iv = b as _;
        g_ta_cipher_layer.cipher_hdr.key_size = keylen as _;
        g_ta_cipher_layer.cipher_hdr.iv_size = ivlen as _;
        g_ta_cipher_layer.cipher_hdr.signature_alg = alg;
    }
}

pub(crate) fn overflow_check(a: u32, b: u32) -> bool {
    if a > u32::MAX - b {
        return true;
    }
    return false;
}

fn boundary_check(max_size: u32, input_size: u32) -> bool {
    if input_size > max_size {
        tloge!(
            "Failed to pass boundary check, max: 0x%x, size: 0x%x\0",
            max_size,
            input_size
        );
        return true;
    }
    return false;
}

#[no_mangle]
pub extern "C" fn get_img_header() -> *mut ta_image_hdr_sec_t {
    return unsafe { &mut g_image_header };
}

#[no_mangle]
pub extern "C" fn get_ta_cipher_layer_len() -> u32 {
    return unsafe { g_cipher_layer_len };
}

#[no_mangle]
pub extern "C" fn get_ta_cipher_layer() -> *mut ta_cipher_layer_t {
    return unsafe { &mut g_ta_cipher_layer };
}

pub(crate) fn handle_cipher_layer_len(cipher_layer_ver: u32) -> TeeResult {
    if cipher_layer_ver <= CIPHER_LAYER_KEY_V1 {
        unsafe { g_cipher_layer_len = CIPHER_LAYER_LEN_256 };
    } else if cipher_layer_ver == CIPHER_LAYER_KEY_V2 {
        unsafe { g_cipher_layer_len = CIPHER_LAYER_LEN_384 };
    } else if cipher_layer_ver == CIPHER_LAYER_KEY_V3 {
        unsafe { g_cipher_layer_len = CIPHER_LAYER_LEN_512 };
    } else {
        tloge!(
            "error cipher layer key version:cipher layer ver=%u\0",
            cipher_layer_ver
        );
        return TeeResult::TEE_ERROR_BAD_PARAMETERS;
    }
    return TeeResult::TEE_SUCCESS;
}

pub(crate) fn check_img_format_valid(config: &sign_config_t) -> bool {
    let ta_payload = unsafe { &*get_ta_payload() };
    if (config.key_style == PUB_KEY_RELEASE || config.key_style == PUB_KEY_GENERIC)
        && !unsafe { g_is_encrypted_sec }
        && config.sign_ta_alg == SIGN_SEC_ALG_DEFAULT
    {
        tloge!("Invalid ta key version 0, release key not support only sign for sec\0");
        return false;
    }

    if ta_payload.payload_hdr.format_version != CIPHER_LAYER_VERSION
        && ta_payload.payload_hdr.format_version != CERT_VERIFY_VERSION
    {
        tloge!(
            "Invalid format version: 0x%x\0",
            ta_payload.payload_hdr.format_version
        );
        return false;
    }

    return true;
}

#[no_mangle]
pub extern "C" fn tee_secure_img_header_check_v3v5() -> TeeResult {
    let g_image_hdr = unsafe { &mut g_image_header };
    let cipher_layer_ver;

    if overflow_check(
        g_image_hdr.context_len,
        size_of::<ta_image_hdr_sec_t>() as u32,
    ) {
        return TeeResult::TEE_ERROR_GENERIC;
    }
    if g_image_hdr.context_len + size_of::<ta_image_hdr_sec_t>() as u32 > MAX_IMAGE_LEN {
        tloge!("image hd err context len: 0x%x\0", g_image_hdr.context_len);
        tloge!(
            "image hd err ta hd len: 0x%x\0",
            size_of::<ta_image_hdr_sec_t>() as u32
        );
        return TeeResult::TEE_ERROR_GENERIC;
    }

    if g_image_hdr.ta_key_version == KEY_VER_NOT_ENCRYPT {
        unsafe {
            g_cipher_layer_len = size_of::<ta_cipher_hdr_t>() as u32;
        }
        unsafe {
            g_is_encrypted_sec = false;
        }
        return TeeResult::TEE_SUCCESS;
    } else if (g_image_hdr.ta_key_version & KEY_VER_MASK) != SEC_IMG_TA_KEY_VERSION {
        tloge!("Invalid ta key version: 0x%x\0", g_image_hdr.ta_key_version);
        return TeeResult::TEE_ERROR_GENERIC;
    }

    unsafe {
        g_is_encrypted_sec = true;
    }
    cipher_layer_ver = (g_image_hdr.ta_key_version >> KEY_VER_BITE) & KEY_VER_MASK;
    return handle_cipher_layer_len(cipher_layer_ver);
}

#[no_mangle]
pub extern "C" fn tee_secure_get_img_header_v3v5(share_buf: *const u8, buf_len: u32) -> TeeResult {
    if share_buf.is_null() {
        return TeeResult::TEE_ERROR_BAD_PARAMETERS;
    }
    if buf_len <= size_of::<ta_image_hdr_sec_t>() as u32 {
        tloge!("img buf len is 0x%x too small\0", buf_len);
        return TeeResult::TEE_ERROR_GENERIC;
    }
    let (sls, buf) = unsafe {
        let s = core::slice::from_raw_parts_mut(
            &mut g_image_header as *mut ta_image_hdr_sec_t as *mut u8,
            size_of::<ta_image_hdr_sec_t>(),
        );
        let b = core::slice::from_raw_parts(share_buf, size_of::<ta_image_hdr_sec_t>());
        (s, b)
    };
    sls.copy_from_slice(buf);

    return TeeResult::TEE_SUCCESS;
}

pub(crate) fn tee_secure_img_parse_cipher_layer(
    plaintext_hdr: &[u8],
    cipher_layer: &mut ta_cipher_layer_t,
) -> TeeResult {
    let mut off_set: usize = 0;

    if overflow_check(off_set as u32, size_of::<ta_cipher_hdr_t>() as u32) {
        return TeeResult::TEE_ERROR_GENERIC;
    }
    if boundary_check(
        plaintext_hdr.len() as u32,
        off_set as u32 + size_of::<ta_cipher_hdr_t>() as u32,
    ) {
        return TeeResult::TEE_ERROR_GENERIC;
    }

    let sls = unsafe {
        core::slice::from_raw_parts_mut(
            &mut (cipher_layer.cipher_hdr) as *mut ta_cipher_hdr_t as *mut u8,
            size_of::<ta_cipher_hdr_t>(),
        )
    };
    sls.copy_from_slice(&plaintext_hdr[off_set..(off_set + size_of::<ta_cipher_hdr_t>())]);

    off_set += size_of::<ta_cipher_hdr_t>();

    if cipher_layer.cipher_hdr.iv_size == 0
        && cipher_layer.cipher_hdr.key_size == 0
        && !unsafe { g_is_encrypted_sec }
    {
        tlogd!("not encrypt, no need duplicate iv & key buff\0");
        return TeeResult::TEE_SUCCESS;
    }

    if overflow_check(off_set as u32, cipher_layer.cipher_hdr.key_size) {
        return TeeResult::TEE_ERROR_GENERIC;
    }
    if boundary_check(
        plaintext_hdr.len() as u32,
        off_set as u32 + cipher_layer.cipher_hdr.key_size,
    ) {
        return TeeResult::TEE_ERROR_GENERIC;
    }
    let key = &plaintext_hdr[off_set..(off_set + cipher_layer.cipher_hdr.key_size as usize)];
    off_set += cipher_layer.cipher_hdr.key_size as usize;

    if overflow_check(off_set as u32, cipher_layer.cipher_hdr.iv_size) {
        return TeeResult::TEE_ERROR_GENERIC;
    }
    if boundary_check(
        plaintext_hdr.len() as u32,
        off_set as u32 + cipher_layer.cipher_hdr.iv_size,
    ) {
        return TeeResult::TEE_ERROR_GENERIC;
    }
    let iv = &plaintext_hdr[off_set..(off_set + cipher_layer.cipher_hdr.iv_size as usize)];

    let mut ret = tee_secure_img_duplicate_buff(iv, &mut (cipher_layer.iv));
    if ret != TeeResult::TEE_SUCCESS {
        tloge!("Failed to dump iv of TA image\0");
        return ret;
    }

    ret = tee_secure_img_duplicate_buff(key, &mut (cipher_layer.key));
    if ret != TeeResult::TEE_SUCCESS {
        tloge!("Failed to dump key of TA image\0");
        unsafe { TEE_Free(cipher_layer.iv as _) };
        cipher_layer.iv = null_mut();
        return ret;
    }
    return TeeResult::TEE_SUCCESS;
}

#[no_mangle]
pub extern "C" fn tee_secure_img_proc_cipher_layer(
    img_buf: *mut u8,
    img_size: u32,
    off_set: *mut u32,
    layer_size: *mut u32,
) -> TeeResult {
    if img_buf.is_null() || off_set.is_null() || layer_size.is_null() {
        return TeeResult::TEE_ERROR_BAD_PARAMETERS;
    }
    /* Locate the position of cipher layer */
    if unsafe { overflow_check(*off_set, g_cipher_layer_len) } {
        return TeeResult::TEE_ERROR_GENERIC;
    }
    if boundary_check(img_size, unsafe { *off_set + g_cipher_layer_len }) {
        return TeeResult::TEE_ERROR_GENERIC;
    }

    let cipher_layer = unsafe { (img_buf as u64) + (*off_set as u64) } as *mut u8;
    let mut ret;

    /* Decrypt cipher layer */
    let plaintext_layer = cipher_layer;
    if unsafe { g_is_encrypted_sec } {
        ret = unsafe {
            tee_secure_img_decrypt_cipher_layer(
                cipher_layer,
                g_cipher_layer_len,
                plaintext_layer,
                layer_size,
            )
        };
        if ret != TeeResult::TEE_SUCCESS {
            return ret;
        }
    } else {
        unsafe { *layer_size = g_cipher_layer_len };
    }

    let layer = unsafe { core::slice::from_raw_parts_mut(plaintext_layer, *layer_size as usize) };
    /* Parse cipher layer to get IV, AES key & signature algorithm */
    ret = tee_secure_img_parse_cipher_layer(&layer, unsafe { &mut g_ta_cipher_layer });
    if ret != TeeResult::TEE_SUCCESS {
        return ret;
    }

    unsafe { *off_set += g_cipher_layer_len };
    return TeeResult::TEE_SUCCESS;
}

#[no_mangle]
pub extern "C" fn tee_secure_img_get_signature_size(
    signature_alg: u32,
    signature_buff: *const u8,
    _signature_max_size: u32,
) -> u32 {
    let size;
    if signature_buff.is_null() {
        return SIGNATURE_SIZE_INVALID;
    }

    let alg = signature_alg & SIGN_ALG_MASK;
    size = match alg {
        SIGN_ALGO_RSA_2048 => RSA2048_SIGNATURE_SIZE,
        SIGN_ALGO_RSA_4096 => RSA4096_SIGNATURE_SIZE,
        SIGN_ALGO_ECC_256 => ECC256_SIGNATURE_SIZE,
        #[cfg(feature = "config_ta_cms_signature")]
        SIGN_ALGO_CMS => get_cms_signature_size(signature_buff, _signature_max_size),
        _ => {
            tloge!("Invalid signature algorithm: 0x%x\0", signature_alg);
            SIGNATURE_SIZE_INVALID
        }
    };

    return size;
}

pub const OP_SIZE: usize = 1024;
pub(crate) fn tee_sec_img_payload_decrypt_ops(
    key_obj: &mut TeeObjectHandleVar,
    src: &[u8],
    dst: &mut [u8],
    dst_len: &mut u32,
) -> TeeResult {
    let mut left_size: usize = src.len();
    let mut out_size: usize;
    let mut total_size: usize = 0;

    let mut crypto_ops = match TeeOperationHandle::allocate_operation(
        TeeCryptoAlgorithmId::TEE_ALG_AES_CBC_PKCS5,
        TeeOperationMode::TEE_MODE_DECRYPT,
        KEY_SIZE_MAX,
    ) {
        Ok(o) => o,
        Err(e) => {
            tloge!("Failed to allocate operation to decrypt TA image\0");
            return TeeResult(e.into());
        }
    };

    let ret = crypto_ops.set_crypto_flag(SOFT_CRYPTO);
    if let Err(r) = ret {
        return TeeResult(r.into());
    }

    let tmp = [key_obj as *mut TeeObjectHandleVar as u64, 0u64];
    let key_obj_handle = unsafe {
        &*(&tmp as *const u64 as u64 as *const TeeObjectHandle<InitializedTransientObject>)
    };

    match crypto_ops.set_operation_key(key_obj_handle) {
        Ok(_) => {}
        Err(e) => {
            tloge!("Set Operation Key fail\0");
            return TeeResult(e.into());
        }
    };
    let iv = unsafe {
        core::slice::from_raw_parts(
            g_ta_cipher_layer.iv,
            g_ta_cipher_layer.cipher_hdr.iv_size as usize,
        )
    };
    crypto_ops.cipher_init(Some(iv));

    let mut src_idx = 0usize;
    let mut dst_idx = 0usize;
    while left_size > OP_SIZE {
        let op_size = OP_SIZE;
        out_size = op_size;
        out_size = match crypto_ops.cipher_update(
            &src[src_idx..(op_size + src_idx)],
            &mut dst[dst_idx..(out_size + dst_idx)],
        ) {
            Ok(o) => o,
            Err(e) => {
                tloge!("TEE Cipher Update fail\0");
                return TeeResult(e.into());
            }
        };
        src_idx += op_size;
        left_size -= op_size;
        dst_idx += out_size;
        total_size += out_size;
    }

    /* update remain length for dst */
    if (*dst_len as usize) < total_size {
        return TeeResult::TEE_ERROR_OUT_OF_MEMORY;
    }
    out_size = *dst_len as usize - total_size;
    out_size = match crypto_ops.cipher_do_final(
        &src[src_idx..(src_idx + left_size)],
        &mut dst[dst_idx..(dst_idx + out_size)],
    ) {
        Ok(o) => o,
        Err(e) => {
            tloge!("Cipher Dofinal fail\0");
            return TeeResult(e.into());
        }
    };
    total_size += out_size;
    *dst_len = total_size as u32;
    return TeeResult::TEE_SUCCESS;
}

pub(crate) fn tee_secure_img_decrypt_payload(
    ciphertext_payload: &[u8],
    plaintext_payload: &mut [u8],
    plaintext_size: &mut u32,
) -> TeeResult {
    let ciphertext_len = ciphertext_payload.len();
    let check = ciphertext_len == 0
        || *plaintext_size == 0
        || *plaintext_size < ciphertext_payload.len() as u32;
    if check {
        return TeeResult::TEE_ERROR_BAD_PARAMETERS;
    }
    let mut key_obj: TeeObjectHandleVar = unsafe { MaybeUninit::zeroed().assume_init() };

    let attr = match TeeMemory::malloc(size_of::<TeeAttribute>(), 0) {
        Ok(o) => o,
        Err(e) => {
            tloge!("Failed to allocate key attribute\0");
            return e;
        }
    };
    unsafe {
        key_obj.attribute = attr.addr() as _;
        let attr_ = &mut *(key_obj.attribute);
        let attr_ref = &mut attr_.content.ref_;
        attr_ref.buffer = g_ta_cipher_layer.key as _;
        attr_ref.length = g_ta_cipher_layer.cipher_hdr.key_size as _;
    }

    let ret = tee_sec_img_payload_decrypt_ops(
        &mut key_obj,
        ciphertext_payload,
        plaintext_payload,
        plaintext_size,
    );
    attr.get_slice_mut().fill(0);
    if ret != TeeResult::TEE_SUCCESS {
        tloge!("Failed to decrypted TA image body\0");
        return ret;
    }
    return TeeResult::TEE_SUCCESS;
}

pub(crate) fn alloc_name_buffer_copy_mani_conf(manifest: &[u8]) -> TeeResult {
    let mut off_set: usize = 0;
    let img_info = unsafe { &mut *get_img_info() };
    let ta_property_ptr = get_ta_property_ptr();
    if ta_property_ptr.is_null() {
        return TeeResult::TEE_ERROR_BAD_PARAMETERS;
    }

    let uuid_sls = unsafe {
        core::slice::from_raw_parts_mut(
            &mut img_info.manifest.srv_uuid as *mut TeeUuid as *mut u8,
            size_of::<TeeUuid>(),
        )
    };
    uuid_sls.copy_from_slice(&manifest[off_set..(off_set + size_of::<TeeUuid>())]);
    off_set += size_of::<TeeUuid>();

    img_info.manifest.mani_info.service_name_len =
        (manifest.len() - size_of::<TeeUuid>() - size_of::<ta_property_t>()) as u32;
    let service_name =
        match TeeMemory::malloc(img_info.manifest.mani_info.service_name_len as usize + 1, 0) {
            Ok(o) => o,
            Err(e) => {
                tloge!("failed to allocate memory for service_name\0");
                return e;
            }
        };
    img_info.manifest.service_name = service_name.addr() as _;
    let service_name_sls =
        &mut service_name.get_slice_mut()[0..img_info.manifest.mani_info.service_name_len as usize];
    service_name_sls.copy_from_slice(
        &manifest[off_set..(off_set + img_info.manifest.mani_info.service_name_len as usize)],
    );
    off_set += img_info.manifest.mani_info.service_name_len as usize;

    let property_sls = unsafe {
        core::slice::from_raw_parts_mut(
            ta_property_ptr as *mut ta_property_t as *mut u8,
            size_of::<ta_property_t>(),
        )
    };
    property_sls.copy_from_slice(&manifest[off_set..(off_set + size_of::<ta_property_t>())]);
    if unsafe { &(*ta_property_ptr) }.single_instance == 0 {
        tloge!("only support single Instance as true\0");
        img_info.manifest.service_name = null_mut();
        return TeeResult::TEE_ERROR_BAD_FORMAT;
    }
    core::mem::forget(service_name);

    return TeeResult::TEE_SUCCESS;
}

pub const CRYPTO_MGR: &[u8] = b"crypto_mgr\0";
pub const TIMER_MGR: &[u8] = b"timer_mgr\0";

pub(crate) fn handle_drv_mani(drv_mani: &mut drv_mani_t) {
    if drv_mani.hardware_type == HARDWARE_ENGINE_CRYPTO {
        (&mut drv_mani.service_name[0..CRYPTO_MGR.len()]).copy_from_slice(CRYPTO_MGR);
        drv_mani.service_name_size = CRYPTO_MGR.len() as u32 - 1;
        drv_mani.srv_uuid = CRYPTOMGR;
    }

    if drv_mani.hardware_type == HARDWARE_TIMER_MGR {
        (&mut drv_mani.service_name[0..TIMER_MGR.len()]).copy_from_slice(TIMER_MGR);
        drv_mani.service_name_size = TIMER_MGR.len() as u32 - 1;
        drv_mani.srv_uuid = TEE_TIMERMGR_DRIVER;
    }
}

pub(crate) fn set_drv_manifest(drv_mani: &mut drv_mani_t) -> TeeResult {
    let img_info = unsafe { &mut *get_img_info() };
    let ta_property_p = get_ta_property_ptr();
    if ta_property_p.is_null()
        || img_info.manifest.mani_info.service_name_len > DRV_NAME_MAX_LEN as u32
    {
        return TeeResult::TEE_ERROR_BAD_PARAMETERS;
    }
    let ta_property_ptr = unsafe { &mut *ta_property_p };
    /* 1.set service name */
    if !img_info.manifest.service_name.is_null() {
        let service_sls = unsafe {
            core::slice::from_raw_parts(
                img_info.manifest.service_name,
                img_info.manifest.mani_info.service_name_len as _,
            )
        };
        (&mut drv_mani.service_name[0..img_info.manifest.mani_info.service_name_len as usize])
            .copy_from_slice(service_sls);
    }
    drv_mani.service_name_size = img_info.manifest.mani_info.service_name_len;

    /* 2.set uuid */
    drv_mani.srv_uuid = img_info.manifest.srv_uuid;

    /* 3.set keep alive */
    if ta_property_ptr.instance_keep_alive != 0 {
        drv_mani.keep_alive = true;
    } else {
        drv_mani.keep_alive = false;
    }

    /* 4.set size */
    drv_mani.data_size = ta_property_ptr.heap_size;
    drv_mani.stack_size = ta_property_ptr.stack_size;
    drv_mani.hardware_type = img_info.manifest.ext.hardware_type;

    handle_drv_mani(drv_mani);

    return TeeResult::TEE_SUCCESS;
}

pub const MAX_SERVICE_NAME_SIZE: usize = 64;
pub(crate) fn check_manifest_alloc_name(manifest: &[u8]) -> TeeResult {
    let check = (manifest.len() < (size_of::<TeeUuid>() + size_of::<ta_property_t>()))
        || (manifest.len()
            > (size_of::<TeeUuid>() + size_of::<ta_property_t>() + MAX_SERVICE_NAME_SIZE));
    if check {
        return TeeResult::TEE_ERROR_BAD_PARAMETERS;
    }

    let ret = alloc_name_buffer_copy_mani_conf(manifest);
    if ret != TeeResult::TEE_SUCCESS {
        return TeeResult::TEE_ERROR_BAD_PARAMETERS;
    }
    return TeeResult::TEE_SUCCESS;
}

pub(crate) fn handle_dyn_conf_buffer(dyn_conf: &mut dyn_conf_t, ext_size: &mut u32) -> i32 {
    let mut ret = TeeResult::TEE_ERROR_GENERIC;
    let img_info = unsafe { &mut *get_img_info() };

    if dyn_conf.dyn_conf_size >= *ext_size {
        tloge!(
            "dyn conf size is larger than ext size %u\0",
            dyn_conf.dyn_conf_size
        );
        return ret.0 as _;
    }

    *ext_size -= dyn_conf.dyn_conf_size;

    let mut drv_mani: drv_mani_t = unsafe { MaybeUninit::zeroed().assume_init() };
    if set_drv_manifest(&mut drv_mani) != TeeResult::TEE_SUCCESS {
        return ret.0 as _;
    }

    ret = TeeResult(register_conf(
        dyn_conf,
        install_common_dyn_permission,
        &mut drv_mani.srv_uuid as *mut TeeUuid as _,
        size_of::<TeeUuid>() as _,
    ) as u32);
    if ret != TeeResult::TEE_SUCCESS {
        tloge!("parse common dyn config failed\0");
        return ret.0 as _;
    }

    if img_info.manifest.ext.target_type == DRV_TARGET_TYPE as u16 {
        ret = TeeResult(register_conf(
            dyn_conf,
            install_drv_permission,
            &mut drv_mani as *mut drv_mani_t as _,
            size_of::<drv_mani_t>() as _,
        ) as u32);
    } else if img_info.manifest.ext.target_type == TA_TARGET_TYPE as u16
        || img_info.manifest.ext.target_type == SRV_TARGET_TYPE as u16
        || img_info.manifest.ext.target_type == CLIENT_TARGET_TYPE as u16
    {
        ret = TeeResult(register_conf(
            dyn_conf,
            install_drvcall_permission,
            &mut drv_mani.srv_uuid as *mut TeeUuid as _,
            size_of::<TeeUuid>() as _,
        ) as u32);
        #[cfg(not(any(feature = "config_timer_disable", feature = "config_off_drv_timer")))]
        {
            if ac_generate_dyn_uuid_data(&drv_mani.srv_uuid) == 0 {
                set_ta_timer_permission(&drv_mani.srv_uuid, TIMER_GROUP_PERMISSION);
            } else {
                tloge!("ac_generate_dyn_uuid_data failed\0");
                ret = TeeResult::TEE_ERROR_GENERIC;
            }
        }
    } else {
        ret = TeeResult::TEE_ERROR_GENERIC;
        tloge!("unknown target type\0");
    }

    return ret.0 as _;
}

/*
 * Process steps:
 * 1, Get the manifest UUID,
 * 2, Get the manifest stand config,
 * 3, Get the TA service name,
 * 4, Parse manifest extension config,
 */
#[no_mangle]
pub extern "C" fn tee_secure_img_parse_manifest(
    manifest_ext: *const u8,
    ext_size: *mut u32,
    control: bool,
    config_target_type: u32,
) -> TeeResult {
    let img_info = unsafe { &mut *get_img_info() };
    let check = (ext_size.is_null()) || (manifest_ext.is_null() && unsafe { *ext_size } > 0);
    if check {
        return TeeResult::TEE_ERROR_BAD_PARAMETERS;
    }

    if !control {
        tlogd!("check config and manifest target type\0");
        if config_target_type != img_info.manifest.ext.target_type as u32 {
            tloge!(
                "diff type con_type=%u mani_type=%u\0",
                config_target_type,
                img_info.manifest.ext.target_type as u32
            );
            return TeeResult::TEE_ERROR_BAD_PARAMETERS;
        }
    }
    let mut dyn_conf: dyn_conf_t = dyn_conf_t {
        dyn_conf_size: 0,
        dyn_conf_buffer: null_mut(),
    };
    let mut ret = tee_secure_img_manifest_extention_process(
        manifest_ext,
        unsafe { *ext_size },
        &mut img_info.manifest.ext,
        &mut dyn_conf,
    );

    if img_info.manifest.ext.api_level > API_LEVEL1_2 {
        tloge!(
            "invalid ta api level:%u\0",
            img_info.manifest.ext.api_level as u32
        );
        ret = TeeResult::TEE_ERROR_BAD_PARAMETERS;
    }
    loop {
        if ret != TeeResult::TEE_SUCCESS {
            tloge!("Manifest extension configuration is invalid\0");
            break;
        }

        if !dyn_conf.dyn_conf_buffer.is_null() {
            ret =
                TeeResult(handle_dyn_conf_buffer(&mut dyn_conf, unsafe { &mut *ext_size }) as u32);
            if ret != TeeResult::TEE_SUCCESS {
                tloge!("register dyn conf for dyn perm failed\0");
                break;
            }
            img_info.dyn_conf_registed = true;
        }
        return ret;
    }

    unsafe {
        if !dyn_conf.dyn_conf_buffer.is_null() {
            free(dyn_conf.dyn_conf_buffer as _);
        }
        TEE_Free(img_info.manifest.service_name as _);
        img_info.manifest.service_name = null_mut();
        if !img_info.manifest.ta_auth.caller_hash.is_null() {
            TEE_Free(img_info.manifest.ta_auth.caller_hash as _);
            img_info.manifest.ta_auth.caller_hash = null_mut();
        }
    }
    return ret;
}

/*
 * Process steps:
 * 1, Get the payload header,
 * 2, Get the manifest stand config,
 * 3, Get the manifest extension config,
 * 4, Get the TA ELF segment,
 * 5, Get the TA config segment,
 * 6, Parse manifest stand config & extension config,
 */
pub(crate) fn tee_secure_img_parse_payload(
    plaintext_payload: &mut [u8],
    payload: &mut ta_payload_layer_t,
) -> TeeResult {
    let mut off_set: usize = 0;
    let plaintext_size = plaintext_payload.len() as u32;

    if overflow_check(off_set as u32, size_of::<ta_payload_hdr_t>() as u32) {
        return TeeResult::TEE_ERROR_GENERIC;
    }
    if boundary_check(
        plaintext_size,
        (off_set + size_of::<ta_payload_hdr_t>()) as u32,
    ) {
        return TeeResult::TEE_ERROR_GENERIC;
    }
    let payload_sls = unsafe {
        core::slice::from_raw_parts_mut(
            &mut (payload.payload_hdr) as *mut ta_payload_hdr_t as *mut u8,
            size_of::<ta_payload_hdr_t>(),
        )
    };
    payload_sls
        .copy_from_slice(&plaintext_payload[off_set..(off_set + size_of::<ta_payload_hdr_t>())]);

    off_set += size_of::<ta_payload_hdr_t>();

    if overflow_check(off_set as u32, payload.payload_hdr.mani_info_size) {
        return TeeResult::TEE_ERROR_GENERIC;
    }
    if boundary_check(
        plaintext_size,
        off_set as u32 + payload.payload_hdr.mani_info_size,
    ) {
        return TeeResult::TEE_ERROR_GENERIC;
    }
    let mani_info =
        &plaintext_payload[off_set..(off_set + payload.payload_hdr.mani_info_size as usize)];
    off_set += payload.payload_hdr.mani_info_size as usize;

    if overflow_check(off_set as u32, payload.payload_hdr.mani_ext_size) {
        return TeeResult::TEE_ERROR_GENERIC;
    }
    if boundary_check(
        plaintext_size,
        off_set as u32 + payload.payload_hdr.mani_ext_size,
    ) {
        return TeeResult::TEE_ERROR_GENERIC;
    }
    let mani_ext = (plaintext_payload.as_ptr() as usize + off_set) as *const u8;
    off_set += payload.payload_hdr.mani_ext_size as usize;

    if overflow_check(off_set as u32, payload.payload_hdr.ta_elf_size) {
        return TeeResult::TEE_ERROR_GENERIC;
    }
    if boundary_check(
        plaintext_size,
        off_set as u32 + payload.payload_hdr.ta_elf_size,
    ) {
        return TeeResult::TEE_ERROR_GENERIC;
    }
    payload.ta_elf = (plaintext_payload.as_ptr() as usize + off_set) as _;
    off_set += payload.payload_hdr.ta_elf_size as usize;

    if overflow_check(off_set as u32, payload.payload_hdr.ta_conf_size) {
        return TeeResult::TEE_ERROR_GENERIC;
    }
    if boundary_check(
        plaintext_size,
        off_set as u32 + payload.payload_hdr.ta_conf_size,
    ) {
        return TeeResult::TEE_ERROR_GENERIC;
    }
    payload.ta_conf = (plaintext_payload.as_ptr() as usize + off_set) as _;

    let mut ret = check_manifest_alloc_name(mani_info);
    if ret != TeeResult::TEE_SUCCESS {
        return ret;
    }

    ret =
        tee_secure_img_parse_manifest(mani_ext, &mut (payload.payload_hdr.mani_ext_size), true, 0);
    if ret != TeeResult::TEE_SUCCESS {
        return ret;
    }

    return TeeResult::TEE_SUCCESS;
}

#[no_mangle]
pub extern "C" fn tee_secure_img_proc_payload(
    img_buf: *mut u8,
    img_size: u32,
    off_set: u32,
    layer_size: u32,
    plaintext_size: *mut u32,
) -> TeeResult {
    if img_buf.is_null() || plaintext_size.is_null() {
        return TeeResult::TEE_ERROR_BAD_PARAMETERS;
    }
    let ciphertext_size: u32 = unsafe { *plaintext_size };
    let ta_payload = unsafe { &mut *get_ta_payload() };

    /* Locate the position of image payload encrypted in AES algorithm */
    if overflow_check(off_set, ciphertext_size) {
        return TeeResult::TEE_ERROR_GENERIC;
    }
    if boundary_check(img_size, off_set + ciphertext_size) {
        return TeeResult::TEE_ERROR_GENERIC;
    }
    let ciphertext_payload = unsafe {
        core::slice::from_raw_parts_mut(
            (img_buf as usize + off_set as usize) as *mut u8,
            ciphertext_size as usize,
        )
    };

    /* Decrypt ciphertext payload */
    let mut plaintext_payload = unsafe {
        core::slice::from_raw_parts_mut(
            (img_buf as usize + off_set as usize) as *mut u8,
            ciphertext_size as usize,
        )
    };
    if unsafe { g_is_encrypted_sec } {
        let ret = tee_secure_img_decrypt_payload(ciphertext_payload, plaintext_payload, unsafe {
            &mut *plaintext_size
        });
        if ret != TeeResult::TEE_SUCCESS {
            return TeeResult::TEE_ERROR_BAD_FORMAT;
        }
    } else {
        unsafe { *plaintext_size = ciphertext_size };
    }

    /* Move identity layer, crypto layer & payload together to calculate the hash */
    if overflow_check(size_of::<ta_image_hdr_sec_t>() as u32, layer_size) {
        return TeeResult::TEE_ERROR_GENERIC;
    }
    if overflow_check(
        size_of::<ta_image_hdr_sec_t>() as u32 + layer_size,
        unsafe { *plaintext_size },
    ) {
        return TeeResult::TEE_ERROR_GENERIC;
    }
    if boundary_check(
        img_size,
        size_of::<ta_image_hdr_sec_t>() as u32 + layer_size + unsafe { *plaintext_size },
    ) {
        return TeeResult::TEE_ERROR_GENERIC;
    }
    let img_sls = unsafe { core::slice::from_raw_parts_mut(img_buf, img_size as usize) };
    img_sls.copy_within(
        (off_set as usize)..(off_set as usize + unsafe { *plaintext_size as usize }),
        size_of::<ta_image_hdr_sec_t>() + layer_size as usize,
    );

    plaintext_payload = unsafe {
        core::slice::from_raw_parts_mut(
            (img_buf as usize + layer_size as usize + size_of::<ta_image_hdr_sec_t>()) as *mut u8,
            *plaintext_size as usize,
        )
    };
    /* Parse plaintext payload */
    return tee_secure_img_parse_payload(plaintext_payload, ta_payload);
}

pub(crate) fn get_sign_config(config: &mut sign_config_t) {
    let sign_alg = unsafe { g_ta_cipher_layer.cipher_hdr.signature_alg };

    match sign_alg & SIGN_ALG_KEY_LEN_MASK {
        SIGN_ALGO_RSA_2048 => config.key_len = PUB_KEY_2048_BITS,
        SIGN_ALGO_RSA_4096 => config.key_len = PUB_KEY_4096_BITS,
        SIGN_ALGO_ECC_256 => config.key_len = PUB_KEY_256_BITS,
        #[cfg(feature = "config_ta_cms_signature")]
        SIGN_ALGO_CMS => config.key_len = PUB_KEY_2048_BITS,
        _ => {
            tloge!("sign alg is invalid!\0");
            return;
        }
    }
    config.hash_size = if (sign_alg & SIGN_ALG_HASH_MASK) != 0 {
        SHA512_LEN
    } else {
        SHA256_LEN
    };
    #[cfg(feature = "mbedtls_enable")]
    {
        config.hash_nid = if config.hash_size == SHA512_LEN {
            MBEDTLS_MD_SHA512
        } else {
            MBEDTLS_MD_SHA256
        };
        config.padding = if (sign_alg & SIGN_ALG_PADD_MASK) != 0 {
            MBEDTLS_RSA_PKCS_V21
        } else {
            MBEDTLS_RSA_PKCS_V15
        };
    }
    #[cfg(feature = "hitls_enable")]
    {
        config.hash_nid = if config.hash_size == SHA512_LEN {
            CRYPT_MD_SHA512
        } else {
            CRYPT_MD_SHA256
        };
        config.padding = if (sign_alg & SIGN_ALG_PADD_MASK) != 0 {
            CRYPT_CTRL_SET_RSA_EMSA_PSS
        } else {
            CRYPT_CTRL_SET_RSA_EMSA_PKCSV15
        };
    }
    #[cfg(any(feature = "openssl_enable", feature = "openssl3_enable"))]
    {
        config.hash_nid = if config.hash_size == SHA512_LEN {
            NID_sha512
        } else {
            NID_sha256
        };
        config.padding = if (sign_alg & SIGN_ALG_PADD_MASK) != 0 {
            RSA_PKCS1_PSS_PADDING
        } else {
            RSA_PKCS1_PADDING
        };
    }
    config.key_style = (sign_alg & SIGN_ALG_KEY_STYLE_MASK) >> SIGN_TA_KEY_TYPE_BITS;
    config.sign_ta_alg = (sign_alg >> SIGN_TA_ALG_BITS) & SIGN_ALG_TA_ALG_MASK;
}

pub(crate) fn get_signature_verify_key(
    _key: &mut u64,
    _config: &sign_config_t,
    _cert_param: &mut cert_param_t,
    _is_dyn_apply: &mut bool,
) -> TeeResult {
    let img = unsafe { &mut *get_img_info() };
    if img.img_version == CERT_VERIFY_VERSION {
        #[cfg(feature = "dyn_ta_support_v5")]
        return get_signature_verify_key_v5(_key, _config, _cert_param, _is_dyn_apply);
        #[cfg(not(feature = "dyn_ta_support_v5"))]
        return TeeResult::TEE_ERROR_NOT_SUPPORTED;
    } else {
        #[cfg(feature = "dyn_ta_support_v3")]
        return get_signature_verify_key_v3(_key, _config, _cert_param, _is_dyn_apply);
        #[cfg(not(feature = "dyn_ta_support_v3"))]
        return TeeResult::TEE_ERROR_NOT_SUPPORTED;
    }
}

fn ecc_signature_verify(signature: &mut [u8], hash: &mut [u8], key: &mut ecc_pub_key_t) -> i32 {
    let mut i: usize = 0;
    while i < signature.len() {
        if signature[i] != 0x00 {
            break;
        }
        i += 1;
    }
    return ecc_verify_digest(
        (signature.as_mut_ptr() as usize + i) as _,
        (signature.len() - i) as _,
        hash.as_mut_ptr() as _,
        hash.len() as _,
        key,
    );
}

pub const SIGNATURE_OK: i32 = 1;
pub(crate) fn do_ta_image_verify(
    signature: &mut [u8],
    hash: &mut [u8],
    config: &sign_config_t,
    key: &mut u64,
) -> TeeResult {
    if is_keywest_signature() || config.sign_ta_alg == SIGN_SEC_ALG_ECDSA {
        let hash = &mut hash[0..config.hash_size as usize];
        let result = ecc_signature_verify(signature, hash, unsafe {
            &mut *(*key as *mut ecc_pub_key_t)
        });
        if result != SIGNATURE_OK {
            tloge!("verify digest failed:%d\0", result);
            return TeeResult::TEE_ERROR_GENERIC;
        }
    } else {
        let result = rsa_verify_digest(
            signature,
            &mut hash[0..config.hash_size],
            *key as *const rsa_pub_key_t,
            config.hash_size as _,
            config.hash_nid,
            config.padding,
        );
        if result != 0 {
            tloge!("verify digest failed:%d\0", result);
            return TeeResult::TEE_ERROR_GENERIC;
        }
    }

    tlogd!("signature VerifyDigest success\0");
    return TeeResult::TEE_SUCCESS;
}

fn print_ta_sign_algorithm_info(config: &sign_config_t) {
    let ta_cipher_layer = unsafe { &mut *get_ta_cipher_layer() };

    tlogi!("sec config info:sign_alg=0x%x, key_len=%u, hash_size=%zu, hash_padding=0x%x, key_style=%s\0",
        ta_cipher_layer.cipher_hdr.signature_alg, config.key_len, config.hash_size,
        config.padding, if config.key_style == PUB_KEY_RELEASE { b"release\0".as_ptr()} else{ b"debug\0".as_ptr()});
}

fn free_mem_and_print_info(is_dyn_apply: bool, key: u64, config: &sign_config_t) {
    if is_dyn_apply {
        unsafe { TEE_Free(key as _) };
    }
    print_ta_sign_algorithm_info(config);
}

#[no_mangle]
pub extern "C" fn tee_secure_img_signature_verify(
    plaintext_payload: *const u8,
    plaintext_size: u32,
    signature: *mut u8,
    signature_size: u32,
    hash_data: *mut elf_hash_data,
) -> TeeResult {
    let mut ret: TeeResult;
    if plaintext_payload.is_null()
        || signature.is_null()
        || plaintext_size == 0
        || signature_size == 0
    {
        return TeeResult::TEE_ERROR_BAD_PARAMETERS;
    }
    let mut is_dyn_apply: bool = false;
    let mut hash = [0u8; HASH_LEN_MAX as usize];
    let mut config = sign_config_t {
        key_len: 0,
        hash_size: 0,
        hash_nid: 0,
        padding: 0,
        key_style: 0,
        sign_ta_alg: 0,
        is_oh: false,
    };
    let mut key: u64 = 0;
    let mut cert_param: cert_param_t = cert_param_t {
        ta_version: 0,
        cert_type: 0,
        public_key: [0u8; MAX_PUB_KEY_SIZE],
        cert_product_type: 0,
        sys_verify_ta: false,
    };

    get_sign_config(&mut config);

    if !check_img_format_valid(&config) {
        return TeeResult::TEE_ERROR_NOT_SUPPORTED;
    }

    loop {
        ret = tee_secure_img_hash_ops(
            plaintext_payload,
            plaintext_size as _,
            &mut hash[0..config.hash_size],
        );
        if ret != TeeResult::TEE_SUCCESS {
            break;
        }

        #[cfg(feature = "config_ta_cms_signature")]
        {
            #[cfg(not(feature = "config_ta_cms_rsa_signature"))]
            let mask_fit = true;
            #[cfg(feature = "config_ta_cms_rsa_signature")]
            let mask_fit = unsafe { g_ta_cipher_layer.cipher_hdr.signature_alg & SIGN_ALG_MASK }
                == SIGN_ALGO_CMS;

            if mask_fit {
                ret = ta_cms_signature_verify(
                    signature,
                    signature_size,
                    &mut hash[0..config.hash_size],
                );
                if ret != TeeResult::TEE_SUCCESS {
                    tloge!("verify digest failed:0x%x\0", ret);
                    return ret;
                }

                copy_hash_data(hash_data, &mut hash[0..config.hash_size]);
                return TeeResult::TEE_SUCCESS;
            }
        }

        ret = get_config_cert_param(&mut cert_param, &mut config);
        if ret != TeeResult::TEE_SUCCESS {
            break;
        }

        /* This is for 3rd party to developing TA with signature check off */
        if get_ta_signature_ctrl() {
            tloge!("DEBUG_VERSION: signature VerifyDigest is OFF\0");
            return TeeResult::TEE_SUCCESS;
        }

        ret = get_signature_verify_key(&mut key, &config, &mut cert_param, &mut is_dyn_apply);
        if ret != TeeResult::TEE_SUCCESS {
            break;
        }

        if key == 0 {
            return TeeResult(TEE_ERROR_IMG_VERIFY_FAIL);
        }

        let sigsls = unsafe { core::slice::from_raw_parts_mut(signature, signature_size as _) };
        ret = do_ta_image_verify(sigsls, &mut hash, &mut config, &mut key);
        if ret != TeeResult::TEE_SUCCESS {
            break;
        }

        /* copy hash data out of this func if hash_data buffer is not NULL */
        copy_hash_data(hash_data, &mut hash[0..config.hash_size]);
        ret = TeeResult::TEE_SUCCESS;
        break;
    }
    free_mem_and_print_info(is_dyn_apply, key, &config);
    return ret;
}

#[no_mangle]
pub extern "C" fn free_global_res() {
    unsafe {
        if !g_ta_cipher_layer.key.is_null() {
            let keysls = core::slice::from_raw_parts_mut(
                g_ta_cipher_layer.key,
                g_ta_cipher_layer.cipher_hdr.key_size as usize,
            );
            keysls.fill(0);
            TEE_Free(g_ta_cipher_layer.key as _);
            g_ta_cipher_layer.key = null_mut();
        }
        if !g_ta_cipher_layer.iv.is_null() {
            let ivsls = core::slice::from_raw_parts_mut(
                g_ta_cipher_layer.iv,
                g_ta_cipher_layer.cipher_hdr.iv_size as usize,
            );
            ivsls.fill(0);
            TEE_Free(g_ta_cipher_layer.iv as _);
            g_ta_cipher_layer.iv = null_mut();
        }

        let imgsls = core::slice::from_raw_parts_mut(
            &mut g_image_header as *mut ta_image_hdr_sec_t as *mut u8,
            size_of::<ta_image_hdr_sec_t>(),
        );
        imgsls.fill(0);
        let ciphersls = core::slice::from_raw_parts_mut(
            &mut g_ta_cipher_layer as *mut ta_cipher_layer_t as *mut u8,
            size_of::<ta_cipher_layer_t>(),
        );
        ciphersls.fill(0);
    }
}

pub const INVALID_OFFSET: i32 = -1;
#[no_mangle]
pub extern "C" fn secure_img_copy_rsp_v3v5(irep: Option<&mut elf_verify_reply>) -> TeeResult {
    if let Some(rep) = irep {
        let img_info = unsafe { &mut *get_img_info() };
        let ta_payload = unsafe { &mut *get_ta_payload() };

        rep.ta_property = img_info.manifest.mani_info.ta_property;
        rep.service_name_len = img_info.manifest.mani_info.service_name_len;
        if (img_info.manifest.mani_info.service_name_len as usize + 1)
            > SERVICE_NAME_MAX_IN_MANIFEST
        {
            tloge!("copy service name fail\0");
            return TeeResult::TEE_ERROR_GENERIC;
        }
        let servicesls = unsafe {
            core::slice::from_raw_parts_mut(
                img_info.manifest.service_name,
                img_info.manifest.mani_info.service_name_len as usize + 1,
            )
        };
        (&mut rep.service_name[0..(img_info.manifest.mani_info.service_name_len as usize + 1)])
            .copy_from_slice(servicesls);

        rep.payload_hdr = ta_payload.payload_hdr;
        rep.mani_ext = img_info.manifest.ext;
        rep.srv_uuid = img_info.manifest.srv_uuid;
        rep.dyn_conf_registed = img_info.dyn_conf_registed;

        if !ta_payload.ta_elf.is_null() {
            rep.off_ta_elf = (ta_payload.ta_elf as u64 - img_info.img_buf as u64) as _;
        } else {
            rep.off_ta_elf = INVALID_OFFSET;
        }

        if !img_info.manifest_buf.is_null() {
            rep.off_manifest_buf = (img_info.manifest_buf as u64 - img_info.img_buf as u64) as _;
        } else {
            rep.off_manifest_buf = INVALID_OFFSET;
        }

        if secure_img_copy_rsp_auth_info(rep) != TeeResult::TEE_SUCCESS {
            tloge!("copy auth info failed\0");
            return TeeResult::TEE_ERROR_GENERIC;
        }
        return TeeResult::TEE_SUCCESS;
    } else {
        return TeeResult::TEE_ERROR_BAD_PARAMETERS;
    }
}

#[no_mangle]
pub extern "C" fn get_sub_cert_from_certchain(
    cert: *const u8,
    cert_len: u32,
    ileaf_cert: Option<&mut leaf_cert>,
    isecondary_cert: Option<&mut leaf_cert>,
) -> TeeResult {
    let mut offset: u32 = 0;
    let tag_len: u32 = size_of::<u16>() as u32;
    let l_len: u32 = size_of::<u16>() as u32;
    let mut ret = TeeResult::TEE_ERROR_GENERIC;

    if cert.is_null() {
        return TeeResult::TEE_ERROR_BAD_PARAMETERS;
    }
    if let (Some(leaf_cert), Some(secondary_cert)) = (ileaf_cert, isecondary_cert) {
        /* second ca cert skip tag(uint16) */
        offset += tag_len;
        if boundary_check(cert_len, offset) {
            return TeeResult::TEE_ERROR_OUT_OF_MEMORY;
        }
        secondary_cert.cert_buf_len = swap_16_endianness_big_to_small!(unsafe {
            *((cert as u64 + offset as u64) as *const u16)
        }) as u32
            + tag_len
            + l_len;

        if boundary_check(cert_len, secondary_cert.cert_buf_len) {
            return TeeResult::TEE_ERROR_OUT_OF_MEMORY;
        }
        let cert_mem = match TeeMemory::malloc(secondary_cert.cert_buf_len as usize, 0) {
            Ok(o) => o,
            Err(e) => return e,
        };

        secondary_cert.cert_buf = cert_mem.addr() as _;
        let sls =
            unsafe { core::slice::from_raw_parts(cert, secondary_cert.cert_buf_len as usize) };
        cert_mem.get_slice_mut().copy_from_slice(sls);

        offset = secondary_cert.cert_buf_len;
        /* leaf cert skip tag(uint16) */
        offset += tag_len;
        if boundary_check(cert_len, offset) {
            secondary_cert.cert_buf = null_mut();
            secondary_cert.cert_buf_len = 0;
            return ret;
        }

        leaf_cert.cert_buf_len = swap_16_endianness_big_to_small!(unsafe {
            *((cert as u64 + offset as u64) as *const u16)
        }) as u32
            + tag_len
            + l_len;

        if boundary_check(
            cert_len,
            secondary_cert.cert_buf_len + leaf_cert.cert_buf_len,
        ) {
            secondary_cert.cert_buf = null_mut();
            secondary_cert.cert_buf_len = 0;
            return ret;
        }
        let leaf_mem = match TeeMemory::malloc(leaf_cert.cert_buf_len as usize, 0) {
            Ok(o) => o,
            Err(e) => {
                secondary_cert.cert_buf = null_mut();
                secondary_cert.cert_buf_len = 0;
                return e;
            }
        };
        leaf_cert.cert_buf = leaf_mem.addr() as _;
        let leaf_sls = unsafe {
            core::slice::from_raw_parts(
                (cert as u64 + secondary_cert.cert_buf_len as u64) as *const u8,
                leaf_cert.cert_buf_len as usize,
            )
        };
        leaf_mem.get_slice_mut().copy_from_slice(leaf_sls);

        ret = TeeResult::TEE_SUCCESS;
        core::mem::forget(cert_mem);
        core::mem::forget(leaf_mem);
        return ret;
    } else {
        return TeeResult::TEE_ERROR_BAD_PARAMETERS;
    }
}
