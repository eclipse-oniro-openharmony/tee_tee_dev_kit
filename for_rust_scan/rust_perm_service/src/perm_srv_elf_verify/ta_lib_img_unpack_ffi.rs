//!
//! Copyright (c) Huawei Technologies Co., Ltd. 2022-2023. All rights reserved.
//! Description: perm service image structure ffi
//! Create: 2023-03-30
//!

// ta_lib_img_unpack.h

use librust_service_ffi::tee_defines::TeeUuid;

pub const MAX_TAFS_NAME_LEN: usize = 64;
pub const SERVICE_NAME_MAX_IN_MANIFEST: usize = 36;

pub const CA_CALLER_HASH_LEN: u32 = 32;

#[cfg(not(feature = "config_ta_auth_max_ca_caller_num32"))]
pub const TA_MAX_CALLER_NUM: u32 = 16;
#[cfg(feature = "config_ta_auth_max_ca_caller_num32")]
pub const TA_MAX_CALLER_NUM: u32 = 32;

pub const TA_AUTH_XML_HEADER_SIZE: u32 = 8;

pub const HEX_BASE: i32 = 16;
pub const DECIMAL_BASE: i32 = 10;
pub const KEY_VER_NOT_ENCRYPT: u32 = 0;
pub const SEC_IMG_TA_KEY_VERSION: u32 = 2;
pub const KEY_VER_BITE: u32 = 0x8;
pub const KEY_VER_MASK: u32 = 0xFF;

pub const RSA2048_SIGNATURE_SIZE: u32 = 256;
pub const RSA4096_SIGNATURE_SIZE: u32 = 512;
pub const ECC256_SIGNATURE_SIZE: u32 = 72;
pub const MAX_SIGNATURE_SIZE: u32 = 512;
pub const SIGNATURE_SIZE_INVALID: u32 = 0;

pub const SIGN_ALG_MASK: u32 = 0x0000FFFF;

pub const SIGN_ALGO_RSA_2048: u32 = 0x00002048;
pub const SIGN_ALGO_RSA_4096: u32 = 0x00004096;
pub const SIGN_ALGO_ECC_256: u32 = 0x00000256;
pub const SIGN_ALG_KEY_STYLE_MASK: u32 = 0xF0000000; /* 0: debug, 1: release, 2: generic */
pub const SIGN_ALG_PADD_MASK: u32 = 0x08000000; /* 0: pkcs1v5, 1: pss */
pub const SIGN_ALG_HASH_MASK: u32 = 0x04000000; /* 0: sha256, 1: sha512 */
pub const SIGN_ALG_KEY_LEN_MASK: u32 = 0x0000ffff; /* only support 2048/4096bits */
pub const SIGN_ALGO_CMS: u32 = 0x0000C000;

pub const IMAGE_BUF_EXTRA: u32 = 4096;
pub const SHA1_LEN: usize = 20;
pub const SHA256_LEN: usize = 32; /* now use sha256 hash alg */
pub const SHA512_LEN: usize = 64; /* now use sha256 hash alg */
pub const HASH_UPDATA_LEN: u32 = 1024; /* modify from 64 to 1024, reduce elf-load time */

pub const SIGN_TA_ALG_BITS: u32 = 20;
pub const SIGN_TA_KEY_TYPE_BITS: u32 = 28;
pub const SIGN_ALG_TA_ALG_MASK: u32 = 0xF;

pub const KEY_SIZE_MAX: u32 = 64;

// enum
pub const HARDWARE_ENGINE_CRYPTO: u16 = 1;
pub const HARDWARE_TIMER_MGR: u16 = 2;
pub const HARDWARE_ENGINE_MAX: u16 = 3;

pub const TEE_ERROR_IMG_DECRYPTO_FAIL: u32 = 0xFF01; //< Image decryption failed
pub const TEE_ERROR_IMG_VERIFY_FAIL: u32 = 0xFF02; //< Image verification failed
pub const TEE_ERROR_IMG_ELF_LOAD_FAIL: u32 = 0xFF03; //< Image loading failed
pub const TEE_ERROR_IMG_NEED_LOAD_FAIL: u32 = 0xFF04; //< Image loading judgement failed
pub const TEE_ERROR_IMG_PARSE_FAIL: u32 = 0xFF05; //< Image parse failed

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ta_property_t {
    pub single_instance: i32,
    pub multi_session: i32,
    pub multi_command: i32,
    pub heap_size: u32,
    pub stack_size: u32,
    pub instance_keep_alive: i32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct manifest_extension_t {
    pub distribution: u16,
    pub api_level: u16,
    pub sdk_version: u16,
    pub is_lib: bool,
    pub ssa_enum_enable: bool,
    pub otrp_flag: bool,
    pub mem_page_align: bool,
    pub sys_verify_ta: bool,
    pub target_type: u16,
    pub target_version: u16,
    pub hardware_type: u16,
    pub is_need_release_ta_res: bool,
    pub crash_callback: bool,
    pub is_need_create_msg: bool,
    pub is_need_release_msg: bool,
    #[cfg(any(
        feature = "config_apm_protection_a32",
        feature = "config_apm_protection"
    ))]
    pub apm_level: u16,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ta_payload_hdr_t {
    pub format_version: u32,
    pub mani_info_size: u32,
    pub mani_ext_size: u32,
    pub ta_elf_size: u32,
    pub ta_conf_size: u32,
}

#[repr(C)]
pub struct ta_payload_layer_t {
    pub payload_hdr: ta_payload_hdr_t,
    pub ta_elf: *mut u8,
    pub ta_conf: *mut u8,
    pub conf_registed: bool,
}

#[repr(C)]
pub struct manifest_info_t {
    pub ta_property: ta_property_t,
    pub elf_hash_len: u32,
    pub elf_cryptkey_len: u32,
    pub service_name_len: u32,
}

#[repr(C)]
pub struct ta_auth_t {
    pub auth_enable: bool,
    pub caller_num: u32,
    pub caller_hash: *mut u8,
}

#[repr(C)]
pub struct manifest_t {
    pub srv_uuid: TeeUuid,
    pub mani_info: manifest_info_t,
    pub hash_val: *mut u8,
    pub key_val: *mut u8,
    pub service_name: *mut u8,
    pub ext: manifest_extension_t,
    pub ta_auth: ta_auth_t,
}

#[repr(C)]
pub struct load_img_info {
    pub manifest: manifest_t,  /* save manifest info */
    pub manifest_buf: *mut u8, /* use malloc, save manifest extension */
    pub img_buf: *mut u8,      /* use rtosck mem, save image */
    pub img_offset: u32,
    pub img_size: u32,
    pub img_version: u32,
    pub dyn_conf_registed: bool, /* using for dyn perm */
}

#[repr(C)]
pub struct teec_image_identity {
    pub magic_num1: u32,
    pub magic_num2: u16,
    pub version_num: u16,
}

#[repr(C)]
pub struct ta_image_hdr_sec_t {
    pub img_identity: teec_image_identity,
    pub context_len: u32,
    pub ta_key_version: u32,
}

pub struct ta_cipher_hdr_t {
    pub key_size: u32,
    pub iv_size: u32,
    pub signature_alg: u32,
}

#[repr(C)]
pub struct ta_cipher_layer_t {
    pub cipher_hdr: ta_cipher_hdr_t,
    pub key: *mut u8,
    pub iv: *mut u8,
}

unsafe impl Sync for ta_cipher_layer_t {}

// enum cipher_layer_len_ver
pub const CIPHER_LAYER_LEN_256: u32 = 256;
pub const CIPHER_LAYER_LEN_384: u32 = 384;
pub const CIPHER_LAYER_LEN_512: u32 = 512;

// enum cipher_layer_key_ver
pub const CIPHER_LAYER_KEY_V1: u32 = 1; /* 2048 bits key, default is also 2048 bits */
pub const CIPHER_LAYER_KEY_V2: u32 = 2; /* 3072 bits key */
pub const CIPHER_LAYER_KEY_V3: u32 = 3; /* 4096 bits key */
