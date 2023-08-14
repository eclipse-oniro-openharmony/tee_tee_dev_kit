//! Copyright (c) Huawei Technologies Co., Ltd. 2022-2023. All rights reserved.
//! Description: perm service image elf verify ffi
//! Create: 2023-03-30
//!

use librust_service_ffi::{
    tee_defines::TeeUuid,
    trusted_storage_api::{TeeAttribute, TeeObjectInfo},
    TeeResult,
};

use crate::{perm_srv_common_ffi::dyn_conf_t, permission_service_ffi::cert_param_t};

use super::{
    ta_lib_img_unpack_ffi::{manifest_extension_t, ta_property_t},
    tee_elf_verify_ffi::{elf_hash_data, elf_verify_reply},
};

// tee_perm_img.h
#[repr(C)]
pub struct sign_config_t {
    pub key_len: u32,
    pub hash_size: usize,
    pub hash_nid: i32,
    pub padding: i32,
    pub key_style: u32,
    pub sign_ta_alg: u32,
    pub is_oh: bool,
}

// ta_verify_key.h

// enum verify_key_len
pub const PUB_KEY_2048_BITS: u32 = 2048;
pub const PUB_KEY_4096_BITS: u32 = 4096;
pub const PUB_KEY_256_BITS: u32 = 256;

// enum verify_key_style
pub const PUB_KEY_DEBUG: u32 = 0;
pub const PUB_KEY_RELEASE: u32 = 1;
pub const PUB_KEY_GENERIC: u32 = 2;

#[repr(C)]
pub struct ta_verify_key {
    pub key_len: u32,
    pub key_style: u32,
    pub key: *const u8,
}

// tee_v3_elf_verify.h tee_v5_elf_verify.h
pub const SIGN_SEC_ALG_ECDSA: u32 = 1;
pub const SIGN_SEC_ALG_RSA: u32 = 2;
pub const SIGN_SEC_ALG_DEFAULT: u32 = 0;
pub const HASH_LEN_MAX: u32 = 64;

// ta_framework.h
// enum TA_VERSION
pub const TA_SIGN_VERSION: u32 = 1; /* first version */
pub const TA_RSA2048_VERSION: u32 = 2; /* use rsa 2048, and use right crypt mode */
pub const CIPHER_LAYER_VERSION: u32 = 3;
pub const CERT_VERIFY_VERSION: u32 = 5;
pub const TA_THIRD_VERSION: u32 = 9;
pub const TA_SIGN_VERSION_MAX: u32 = 10;

pub const OBJECT_NAME_LEN_MAX: usize = 255;
#[repr(C)]
pub struct TeeObjectHandleVar {
    pub data_ptr: *mut u8,
    pub data_len: u32,
    pub data_name: [u8; OBJECT_NAME_LEN_MAX],
    pub object_info: *mut TeeObjectInfo,
    pub attribute: *mut TeeAttribute,
    pub attributes_en: u32,
    pub crt_mode: u32,
    pub infoattrfd: *mut u8,
    pub generate_flag: u32,
    pub storage_id: u32,
}

// tee_elf_verify_inner.h
pub fn tee_secure_img_duplicate_buff(src: &[u8], dst: *mut *mut u8) -> TeeResult {
    unsafe {
        crate::perm_srv_common_ffi::tee_secure_img_duplicate_buff(
            src.as_ptr() as _,
            src.len() as _,
            dst,
        )
    }
}

pub fn tee_secure_img_manifest_extention_process(
    extension: *const u8,
    extension_size: u32,
    mani_ext: &mut manifest_extension_t,
    dyn_conf: &mut dyn_conf_t,
) -> TeeResult {
    unsafe {
        crate::perm_srv_common_ffi::tee_secure_img_manifest_extention_process(
            extension,
            extension_size,
            mani_ext,
            dyn_conf,
        )
    }
}

pub fn tee_secure_img_decrypt_cipher_layer(
    cipher_layer: *const u8,
    cipher_size: u32,
    plaintext_layer: *mut u8,
    plaintext_size: *mut u32,
) -> TeeResult {
    unsafe {
        crate::perm_srv_common_ffi::tee_secure_img_decrypt_cipher_layer(
            cipher_layer,
            cipher_size,
            plaintext_layer,
            plaintext_size,
        )
    }
}

pub fn get_cms_signature_size(signature_buff: *const u8, signature_max_size: u32) -> u32 {
    unsafe {
        crate::perm_srv_common_ffi::get_cms_signature_size(signature_buff, signature_max_size)
    }
}

pub fn get_ta_property_ptr() -> *mut ta_property_t {
    unsafe { crate::perm_srv_common_ffi::get_ta_property_ptr() }
}

// tee_drv_internal.h
pub const DRV_NAME_MAX_LEN: usize = 32;

// dyn_conf_common.h
#[repr(C)]
pub struct drv_mani_t {
    pub srv_uuid: TeeUuid,
    pub service_name: [u8; DRV_NAME_MAX_LEN + 1],
    pub service_name_size: u32,
    pub keep_alive: bool,
    pub data_size: u32,
    pub stack_size: u32,
    pub hardware_type: u16,
}

// tee_inner_uuid.h
pub const CRYPTOMGR: TeeUuid = TeeUuid {
    time_low: 0x2427f879,
    time_mid: 0x4655,
    time_hi_and_version: 0x4367,
    clock_seq_and_node: [0x82, 0x31, 0xe5, 0x8e, 0x29, 0x45, 0xc9, 0xb8],
};
/* 24ba6cc9-9709-4473-bfe9-324cd9289c3d */
pub const TEE_TIMERMGR_DRIVER: TeeUuid = TeeUuid {
    time_low: 0x24ba6cc9,
    time_mid: 0x9709,
    time_hi_and_version: 0x4473,
    clock_seq_and_node: [0xbf, 0xe9, 0x32, 0x4c, 0xd9, 0x28, 0x9c, 0x3d],
};

// tee_defines.h
pub const API_LEVEL1_0: u16 = 1;
pub const API_LEVEL1_1_1: u16 = 2;
pub const API_LEVEL1_2: u16 = 3;

// md.h
// enum mbedtls_md_type_t {
pub const MBEDTLS_MD_NONE: i32 = 0; //< None.
pub const MBEDTLS_MD_MD5: i32 = 1; //< The MD5 message digest.
pub const MBEDTLS_MD_SHA1: i32 = 2; //< The SHA-1 message digest.
pub const MBEDTLS_MD_SHA224: i32 = 3; //< The SHA-224 message digest.
pub const MBEDTLS_MD_SHA256: i32 = 4; //< The SHA-256 message digest.
pub const MBEDTLS_MD_SHA384: i32 = 5; //< The SHA-384 message digest.
pub const MBEDTLS_MD_SHA512: i32 = 6; //< The SHA-512 message digest.
pub const MBEDTLS_MD_RIPEMD160: i32 = 7; // < The RIPEMD-160 message digest.

// rsa.h
pub const MBEDTLS_RSA_PKCS_V15: i32 = 0; //< Use PKCS#1 v1.5 encoding.
pub const MBEDTLS_RSA_PKCS_V21: i32 = 1; //< Use PKCS#1 v2.1 encoding.

pub const RSA_PKCS1_PADDING: i32 = 1;
pub const RSA_SSLV23_PADDING: i32 = 2;
pub const RSA_NO_PADDING: i32 = 3;
pub const RSA_PKCS1_OAEP_PADDING: i32 = 4;
pub const RSA_X931_PADDING: i32 = 5;
pub const RSA_PKCS1_PSS_PADDING: i32 = 6;

// obj_mac.h
pub const NID_sha512: i32 = 674;
pub const NID_sha256: i32 = 672;

// crypto_algid.h
// enum CRYPT_MD_AlgId {
pub const CRYPT_MD_MD4: i32 = 0;
pub const CRYPT_MD_MD5: i32 = 1;
pub const CRYPT_MD_SHA1: i32 = 2;
pub const CRYPT_MD_SHA224: i32 = 3;
pub const CRYPT_MD_SHA256: i32 = 4;
pub const CRYPT_MD_SHA384: i32 = 5;
pub const CRYPT_MD_SHA512: i32 = 6;
pub const CRYPT_MD_SHA3_224: i32 = 7;
pub const CRYPT_MD_SHA3_256: i32 = 8;
pub const CRYPT_MD_SHA3_384: i32 = 9;
pub const CRYPT_MD_SHA3_512: i32 = 10;
pub const CRYPT_MD_SHAKE128: i32 = 11;
pub const CRYPT_MD_SHAKE256: i32 = 12;
pub const CRYPT_MD_SM3: i32 = 13;
pub const CRYPT_MD_MAX: i32 = 14;

// crypto_types.h
// enum CRYPT_PkeyCtrl {
pub const CRYPT_CTRL_SET_ED25519_HASH_METHOD: i32 = 0;
pub const CRYPT_CTRL_SET_RSA_EMSA_PKCSV15: i32 = 1;
pub const CRYPT_CTRL_SET_RSA_EMSA_PSS: i32 = 2;
pub const CRYPT_CTRL_SET_RSA_SALT: i32 = 3;
pub const CRYPT_CTRL_SET_ECC_POINT_FORMAT: i32 = 4;
pub const CRYPT_CTRL_SET_ECC_USE_COFACTOR_MODE: i32 = 5;
pub const CRYPT_CTRL_SET_RSA_RSAES_OAEP: i32 = 6;
pub const CRYPT_CTRL_SET_RSA_OAEP_LABEL: i32 = 7;
pub const CRYPT_CTRL_SET_RSA_FLAG: i32 = 8;
pub const CRYPT_CTRL_CLR_RSA_FLAG: i32 = 9;
pub const CRYPT_CTRL_SET_RSA_RSAES_PKCSV15: i32 = 10;
pub const CRYPT_CTRL_SET_SM9_HASH_METHOD: i32 = 11;
pub const CRYPT_CTRL_SET_ED448_HASH_METHOD: i32 = 12;
pub const CRYPT_CTRL_SET_ED448_CONTEXT: i32 = 13;
pub const CRYPT_CTRL_SET_ED448_PREHASH: i32 = 14;
pub const CRYPT_CTRL_SET_SM2_USER_ID: i32 = 15;
pub const CRYPT_CTRL_SET_SM2_HASH_METHOD: i32 = 16;
pub const CRYPT_CTRL_SET_SM2_SERVER: i32 = 17;
pub const CRYPT_CTRL_GENE_SM2_R: i32 = 18;
pub const CRYPT_CTRL_SET_SM2_R: i32 = 19;
pub const CRYPT_CTRL_SM2_GET_SEND_CHECK: i32 = 20;
pub const CRYPT_CTRL_SM2_DO_CHECK: i32 = 21;

pub fn get_signature_verify_key_v5(
    key: &mut u64,
    config: &sign_config_t,
    cert_param: &mut cert_param_t,
    is_dyn_apply: &mut bool,
) -> TeeResult {
    unsafe {
        crate::perm_srv_common_ffi::get_signature_verify_key_v5(
            key,
            config,
            cert_param,
            is_dyn_apply,
        )
    }
}

pub fn get_signature_verify_key_v3(
    key: &mut u64,
    config: &sign_config_t,
    cert_param: &mut cert_param_t,
    is_dyn_apply: &mut bool,
) -> TeeResult {
    unsafe {
        crate::perm_srv_common_ffi::get_signature_verify_key_v3(
            key,
            config,
            cert_param,
            is_dyn_apply,
        )
    }
}

// crypto_ec_wrapper.h
pub const ECC_PUB_LEN: usize = 66;
#[repr(C)]
pub struct ecc_pub_key_t {
    pub domain_id: u32,
    pub x: [u8; ECC_PUB_LEN],
    pub x_len: u32,
    pub y: [u8; ECC_PUB_LEN],
    pub y_len: u32,
}

pub fn ecc_verify_digest(
    signature: *const u8,
    sig_len: u32,
    in_: *mut u8,
    in_len: u32,
    pub_: &mut ecc_pub_key_t,
) -> i32 {
    unsafe { crate::perm_srv_common_ffi::ecc_verify_digest(signature, sig_len, in_, in_len, pub_) }
}

pub fn is_keywest_signature() -> bool {
    unsafe { crate::perm_srv_common_ffi::is_keywest_signature() }
}

// crypto_rsa_wrapper.h
pub const RSA_PUB_LEN: usize = 1024;
pub const RSA_PRIV_LEN: usize = 512;

#[repr(C)]
pub struct rsa_pub_key_t {
    pub e: [u8; RSA_PUB_LEN],
    pub e_len: u32,
    pub n: [u8; RSA_PUB_LEN],
    pub n_len: u32,
}

pub fn rsa_verify_digest(
    signature: &mut [u8],
    in_: &mut [u8],
    pub_: *const rsa_pub_key_t,
    salt_len: u32,
    hash_nid: i32,
    padding: i32,
) -> i32 {
    unsafe {
        crate::perm_srv_common_ffi::rsa_verify_digest(
            signature.as_mut_ptr() as _,
            signature.len() as _,
            in_.as_mut_ptr() as _,
            in_.len() as _,
            pub_,
            salt_len,
            hash_nid,
            padding,
        )
    }
}

pub fn get_ta_signature_ctrl() -> bool {
    unsafe { crate::perm_srv_common_ffi::get_ta_signature_ctrl() }
}

pub fn get_config_cert_param(
    cert_param: &mut cert_param_t,
    config: &mut sign_config_t,
) -> TeeResult {
    unsafe { crate::perm_srv_common_ffi::get_config_cert_param(cert_param, config) }
}

pub fn copy_hash_data(hash_data: *mut elf_hash_data, hash: &mut [u8]) {
    unsafe {
        crate::perm_srv_common_ffi::copy_hash_data(
            hash_data,
            hash.as_mut_ptr() as _,
            hash.len() as _,
        )
    }
}

pub fn tee_secure_img_hash_ops(
    hash_context: *const u8,
    data_size: usize,
    hash: &mut [u8],
) -> TeeResult {
    unsafe {
        crate::perm_srv_common_ffi::tee_secure_img_hash_ops(
            hash_context,
            data_size,
            hash.as_mut_ptr(),
            hash.len(),
        )
    }
}

pub fn ta_cms_signature_verify(
    signature: *mut u8,
    signature_size: u32,
    hash: &mut [u8],
) -> TeeResult {
    unsafe {
        crate::perm_srv_common_ffi::ta_cms_signature_verify(
            signature,
            signature_size,
            hash.as_mut_ptr(),
            hash.len() as _,
        )
    }
}

pub fn secure_img_copy_rsp_auth_info(rep: &mut elf_verify_reply) -> TeeResult {
    unsafe { crate::perm_srv_common_ffi::secure_img_copy_rsp_auth_info(rep) }
}
