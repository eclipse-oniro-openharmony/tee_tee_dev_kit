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

use core::ffi::c_void;

pub const ENC_MODE: usize = 0x00;
pub const DEC_MODE: usize = 0x01;
pub const SIGN_MODE: usize = 0x02;
pub const VERIFY_MODE: usize = 0x03;
pub const DH_PKCS3_MODE: usize = 0x1;
pub const RSA_EXPONENT_LEN: usize = 4;
pub const RSA_MAX_KEY_SIZE: usize = 512;
pub const RSA_MAX_KEY_SIZE_CRT: usize = RSA_MAX_KEY_SIZE / 2;
pub const ECC_KEY_LEN: usize = 68;
pub const DRIVER_PADDING: usize = 0x00000001;
pub const DRIVER_CACHE: usize = 0x00000002;
pub const PBKDF2_SALT_SIZE_MIN: usize = 16;
pub const DERIVE_ROOT_KEY_ITER_NUM_MAX: usize = 10;

#[repr(transparent)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct CryptoEngine(u32);

impl CryptoEngine {
    pub const DX_CRYPTO_FLAG: Self = Self(0);
    pub const EPS_CRYPTO_FLAG: Self = Self(1);
    pub const SOFT_CRYPTO_FLAG: Self = Self(2);
    pub const SEC_CRYPTO_FLAG: Self = Self(3);
    pub const CRYPTO_ENGINE_MAX_FLAG: Self = Self(4);
}

impl From<CryptoEngine> for u32 {
    fn from(value: CryptoEngine) -> Self {
        value.0
    }
}

#[repr(transparent)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct CryptoErr(i32);

impl CryptoErr {
    pub const CRYPTO_NOT_SUPPORTED: Self = Self(-1);
    pub const CRYPTO_CIPHERTEXT_INVALID: Self = Self(-2);
    pub const CRYPTO_BAD_FORMAT: Self = Self(-3);
    pub const CRYPTO_BAD_PARAMETERS: Self = Self(-4);
    pub const CRYPTO_BAD_STATE: Self = Self(-5);
    pub const CRYPTO_SHORT_BUFFER: Self = Self(-6);
    pub const CRYPTO_OVERFLOW: Self = Self(-7);
    pub const CRYPTO_MAC_INVALID: Self = Self(-8);
    pub const CRYPTO_SIGNATURE_INVALID: Self = Self(-9);
    pub const CRYPTO_ERROR_SECURITY: Self = Self(-10);
    pub const CRYPTO_ERROR_OUT_OF_MEMORY: Self = Self(-11);
    pub const CRYPTO_SUCCESS: Self = Self(0);
}

impl From<CryptoErr> for i32 {
    fn from(value: CryptoErr) -> Self {
        value.0
    }
}

#[repr(transparent)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct CryptoAlgType(u32);

impl CryptoAlgType {
    pub const CRYPTO_TYPE_AES_ECB_NOPAD: Self = Self(0x10000010);
    pub const CRYPTO_TYPE_AES_CBC_NOPAD: Self = Self(0x10000110);
    pub const CRYPTO_TYPE_AES_ECB_PKCS5: Self = Self(0x10000020);
    pub const CRYPTO_TYPE_AES_CBC_PKCS5: Self = Self(0x10000220);
    pub const CRYPTO_TYPE_AES_CTR: Self = Self(0x10000210);
    pub const CRYPTO_TYPE_AES_CTS: Self = Self(0x10000310);
    pub const CRYPTO_TYPE_AES_XTS: Self = Self(0x10000410);
    pub const CRYPTO_TYPE_AES_OFB: Self = Self(0x10000510);
    pub const CRYPTO_TYPE_SM4_ECB: Self = Self(0x10000014);
    #[cfg(feature = "support_kms")]
    pub const CRYPTO_TYPE_SM4_ECB_PKCS7: Self = Self(0x10000024);
    pub const CRYPTO_TYPE_SM4_CBC: Self = Self(0x10000114);
    pub const CRYPTO_TYPE_SM4_CBC_PKCS7: Self = Self(0xF0000003);
    pub const CRYPTO_TYPE_SM4_CTR: Self = Self(0x10000214);
    pub const CRYPTO_TYPE_SM4_CFB128: Self = Self(0xF0000000);
    pub const CRYPTO_TYPE_SM4_GCM: Self = Self(0xF0000005);
    pub const CRYPTO_TYPE_SM4_XTS: Self = Self(0x10000414);
    pub const CRYPTO_TYPE_SM4_OFB: Self = Self(0x10000514);
    pub const CRYPTO_TYPE_DES_ECB_NOPAD: Self = Self(0x10000011);
    pub const CRYPTO_TYPE_DES_CBC_NOPAD: Self = Self(0x10000111);
    pub const CRYPTO_TYPE_DES3_ECB_NOPAD: Self = Self(0x10000013);
    pub const CRYPTO_TYPE_DES3_CBC_NOPAD: Self = Self(0x10000113);
    pub const CRYPTO_TYPE_HMAC_MD5: Self = Self(0x30000001);
    pub const CRYPTO_TYPE_HMAC_SHA1: Self = Self(0x30000002);
    pub const CRYPTO_TYPE_HMAC_SHA224: Self = Self(0x30000003);
    pub const CRYPTO_TYPE_HMAC_SHA256: Self = Self(0x30000004);
    pub const CRYPTO_TYPE_HMAC_SHA384: Self = Self(0x30000005);
    pub const CRYPTO_TYPE_HMAC_SHA512: Self = Self(0x30000006);
    pub const CRYPTO_TYPE_HMAC_SM3: Self = Self(0x30000007);
    pub const CRYPTO_TYPE_AES_CMAC: Self = Self(0x30000610);
    pub const CRYPTO_TYPE_AES_CBC_MAC_NOPAD: Self = Self(0x30000110);
    pub const CRYPTO_TYPE_AES_CBC_MAC_PKCS5: Self = Self(0x30000510);
    pub const CRYPTO_TYPE_AES_GMAC: Self = Self(0x30000810);
    pub const CRYPTO_TYPE_DES_CBC_MAC_NOPAD: Self = Self(0x30000111);
    pub const CRYPTO_TYPE_DES3_CBC_MAC_NOPAD: Self = Self(0x30000113);
    pub const CRYPTO_TYPE_AES_CCM: Self = Self(0x40000710);
    pub const CRYPTO_TYPE_AES_GCM: Self = Self(0x40000810);
    pub const CRYPTO_TYPE_DIGEST_MD5: Self = Self(0x50000001);
    pub const CRYPTO_TYPE_DIGEST_SHA1: Self = Self(0x50000002);
    pub const CRYPTO_TYPE_DIGEST_SHA224: Self = Self(0x50000003);
    pub const CRYPTO_TYPE_DIGEST_SHA256: Self = Self(0x50000004);
    pub const CRYPTO_TYPE_DIGEST_SHA384: Self = Self(0x50000005);
    pub const CRYPTO_TYPE_DIGEST_SHA512: Self = Self(0x50000006);
    pub const CRYPTO_TYPE_DIGEST_SM3: Self = Self(0x50000007);
    pub const CRYPTO_TYPE_RSAES_PKCS1_V1_5: Self = Self(0x60000130);
    pub const CRYPTO_TYPE_RSAES_PKCS1_OAEP_MGF1_SHA1: Self = Self(0x60210230);
    pub const CRYPTO_TYPE_RSAES_PKCS1_OAEP_MGF1_SHA224: Self = Self(0x60211230);
    pub const CRYPTO_TYPE_RSAES_PKCS1_OAEP_MGF1_SHA256: Self = Self(0x60212230);
    pub const CRYPTO_TYPE_RSAES_PKCS1_OAEP_MGF1_SHA384: Self = Self(0x60213230);
    pub const CRYPTO_TYPE_RSAES_PKCS1_OAEP_MGF1_SHA512: Self = Self(0x60214230);
    pub const CRYPTO_TYPE_RSA_NO_PAD: Self = Self(0x60000030);
    pub const CRYPTO_TYPE_SM2_KEP: Self = Self(0x60000045);
    pub const CRYPTO_TYPE_SM2_DSA_SM3: Self = Self(0x70006045);
    pub const CRYPTO_TYPE_RSASSA_PKCS1_V1_5_MD5: Self = Self(0x70001830);
    pub const CRYPTO_TYPE_RSASSA_PKCS1_V1_5_SHA1: Self = Self(0x70002830);
    pub const CRYPTO_TYPE_RSASSA_PKCS1_V1_5_SHA224: Self = Self(0x70003830);
    pub const CRYPTO_TYPE_RSASSA_PKCS1_V1_5_SHA256: Self = Self(0x70004830);
    pub const CRYPTO_TYPE_RSASSA_PKCS1_V1_5_SHA384: Self = Self(0x70005830);
    pub const CRYPTO_TYPE_RSASSA_PKCS1_V1_5_SHA512: Self = Self(0x70006830);
    pub const CRYPTO_TYPE_RSASSA_PKCS1_PSS_MGF1_MD5: Self = Self(0x70111930);
    pub const CRYPTO_TYPE_RSASSA_PKCS1_PSS_MGF1_SHA1: Self = Self(0x70212930);
    pub const CRYPTO_TYPE_RSASSA_PKCS1_PSS_MGF1_SHA224: Self = Self(0x70313930);
    pub const CRYPTO_TYPE_RSASSA_PKCS1_PSS_MGF1_SHA256: Self = Self(0x70414930);
    pub const CRYPTO_TYPE_RSASSA_PKCS1_PSS_MGF1_SHA384: Self = Self(0x70515930);
    pub const CRYPTO_TYPE_RSASSA_PKCS1_PSS_MGF1_SHA512: Self = Self(0x70616930);
    pub const CRYPTO_TYPE_ECDSA_SHA1: Self = Self(0x70001042);
    pub const CRYPTO_TYPE_ECDSA_SHA224: Self = Self(0x70002042);
    pub const CRYPTO_TYPE_ECDSA_SHA256: Self = Self(0x70003042);
    pub const CRYPTO_TYPE_ECDSA_SHA384: Self = Self(0x70004042);
    pub const CRYPTO_TYPE_ECDSA_SHA521: Self = Self(0x70005042);
    pub const CRYPTO_TYPE_ED25519: Self = Self(0x70005043);
    pub const CRYPTO_TYPE_DH_DERIVE_SECRET: Self = Self(0x80000032);
    pub const CRYPTO_TYPE_ECDH_DERIVE_SECRET: Self = Self(0x80000042);
    pub const CRYPTO_TYPE_X25519: Self = Self(0x80000044);
    pub const CRYPTO_TYPE_SM2_PKE: Self = Self(0x80000045);
    pub const CRYPTO_TYPE_GENERATE_RANDOM: Self = Self(0xf0000001);
    #[cfg(feature = "mbedtls_enable")]
    pub const CRYPTO_TYPE_SIP_HASH: Self = Self(0xF0000002);
}

impl From<CryptoAlgType> for u32 {
    fn from(value: CryptoAlgType) -> Self {
        value.0
    }
}

#[repr(transparent)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct CryptoCurveType(u32);

impl CryptoCurveType {
    pub const ECC_CURVE_NIST_P192: Self = Self(0x1);
    pub const ECC_CURVE_NIST_P224: Self = Self(0x2);
    pub const ECC_CURVE_NIST_P256: Self = Self(0x3);
    pub const ECC_CURVE_NIST_P384: Self = Self(0x4);
    pub const ECC_CURVE_NIST_P521: Self = Self(0x5);
    pub const ECC_CURVE_X25519: Self = Self(0x6);
    pub const ECC_CURVE_ED25519: Self = Self(0x7);
    pub const ECC_CURVE_SM2: Self = Self(0x8);
}

impl From<CryptoCurveType> for u32 {
    fn from(value: CryptoCurveType) -> Self {
        value.0
    }
}

#[repr(transparent)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct CryptoAttributeType(u32);

impl CryptoAttributeType {
    pub const CRYPTO_ATTR_RSA_OAEP_LABEL: Self = Self(0xD0000930);
    pub const CRYPTO_ATTR_RSA_MGF1_HASH: Self = Self(0xF0000830);
    pub const CRYPTO_ATTR_RSA_PSS_SALT_LENGTH: Self = Self(0xF0000A30);
    pub const CRYPTO_ATTR_ED25519_PH: Self = Self(0xF0000543);
    pub const CRYPTO_ATTR_ED25519_CTX: Self = Self(0xD0000643);
    pub const CRYPTO_ATTR_DH_PUBLIC_VALUE: Self = Self(0xD0000132);
    pub const CRYPTO_ATTR_ECC_PUBLIC_VALUE_X: Self = Self(0xD0000141);
    pub const CRYPTO_ATTR_ECC_PUBLIC_VALUE_Y: Self = Self(0xD0000241);
    pub const CRYPTO_ATTR_X25519_PUBLIC_VALUE: Self = Self(0xD0000944);
    pub const CRYPTO_ATTR_SM2_KEP_USER: Self = Self(0x30010005);
    pub const CRYPTO_ATTR_ECC_EPHEMERAL_PUBLIC_VALUE_X: Self = Self(0x30000006);
    pub const CRYPTO_ATTR_ECC_EPHEMERAL_PUBLIC_VALUE_Y: Self = Self(0x30000007);
    pub const CRYPTO_ATTR_SM2_ID_INITIATOR: Self = Self(0x30000008);
    pub const CRYPTO_ATTR_SM2_ID_RESPONDER: Self = Self(0x30000009);
    pub const CRYPTO_ATTR_SM2_KEP_CONFIRMATION_IN: Self = Self(0x3000000a);
    pub const CRYPTO_ATTR_SM2_KEP_CONFIRMATION_OUT: Self = Self(0x3000000b);
}

impl From<CryptoAttributeType> for u32 {
    fn from(value: CryptoAttributeType) -> Self {
        value.0
    }
}

#[repr(transparent)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct KeyTypeT(u32);

impl KeyTypeT {
    pub const CRYPTO_KEYTYPE_DEFAULT: Self = Self(0x0);
    pub const CRYPTO_KEYTYPE_USER: Self = Self(0x1);
    pub const CRYPTO_KEYTYPE_HUK: Self = Self(0x2);
    pub const CRYPTO_KEYTYPE_GID: Self = Self(0x3);
    pub const CRYPTO_KEYTYPE_RPMB: Self = Self(0x4);
}

impl From<KeyTypeT> for u32 {
    fn from(value: KeyTypeT) -> Self {
        value.0
    }
}

#[repr(transparent)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct CryptoKeyTypeId(u32);

impl CryptoKeyTypeId {
    pub const CRYPTO_KEY_TYPE_RSA_KEYPAIR: Self = Self(0xA1000030);
    pub const CRYPTO_KEY_TYPE_DH_KEYPAIR: Self = Self(0xA1000032);
    pub const CRYPTO_KEY_TYPE_ECDSA_KEYPAIR: Self = Self(0xA1000041);
    pub const CRYPTO_KEY_TYPE_ECDH_KEYPAIR: Self = Self(0xA1000042);
    pub const CRYPTO_KEY_TYPE_ED25519_KEYPAIR: Self = Self(0xA1000043);
    pub const CRYPTO_KEY_TYPE_X25519_KEYPAIR: Self = Self(0xA1000044);
    pub const CRYPTO_KEY_TYPE_SM2_DSA_KEYPAIR: Self = Self(0xA1000045);
    pub const CRYPTO_KEY_TYPE_SM2_PKE_KEYPAIR: Self = Self(0xA1000047);
}

impl From<CryptoKeyTypeId> for u32 {
    fn from(value: CryptoKeyTypeId) -> Self {
        value.0
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MemrefT {
    buffer: u64,
    size: u32,
}

#[repr(C)]
pub struct SymmeritKeyT {
    key_type: u32,
    key_buffer: u64,
    key_size: u32,
}

#[repr(C)]
pub struct AeInitData {
    nonce: u64,
    nonce_len: u32,
    tag_len: u32,
    aad_len: u32,
    payload_len: u32,
}

#[repr(C)]
pub struct RsaPubKeyT {
    e: [u8; RSA_EXPONENT_LEN],
    e_len: u32,
    n: [u8; RSA_MAX_KEY_SIZE],
    n_len: u32,
}

#[repr(C)]
pub struct RsaPrivKeyT {
    crt_mode: bool,
    e: [u8; RSA_EXPONENT_LEN],
    e_len: u32,
    n: [u8; RSA_MAX_KEY_SIZE],
    n_len: u32,
    d: [u8; RSA_MAX_KEY_SIZE],
    d_len: u32,
    p: [u8; RSA_MAX_KEY_SIZE_CRT],
    p_len: u32,
    q: [u8; RSA_MAX_KEY_SIZE_CRT],
    q_len: u32,
    dp: [u8; RSA_MAX_KEY_SIZE_CRT],
    dp_len: u32,
    dq: [u8; RSA_MAX_KEY_SIZE_CRT],
    dq_len: u32,
    qinv: [u8; RSA_MAX_KEY_SIZE_CRT],
    qinv_len: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ValueT {
    pub a: u32,
    pub b: u32,
}

#[repr(C)]
pub union MemrefValue {
    pub memref: MemrefT,
    pub value: ValueT,
}

#[repr(C)]
pub struct CryptoAttributeT {
    attribute_id: u32,
    content: MemrefValue,
}

#[repr(C)]
pub struct AsymmetricParamsT {
    param_count: u32,
    attribute: u64,
}

#[repr(C)]
pub struct EccPubKeyT {
    domain_id: u32,
    x: [u8; ECC_KEY_LEN],
    x_len: u32,
    y: [u8; ECC_KEY_LEN],
    y_len: u32,
}

#[repr(C)]
pub struct EccPrivKeyT {
    domain_id: u32,
    r: [u8; ECC_KEY_LEN],
    r_len: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct GenerateKeyT {
    pub q: u64,
    pub q_size: u32,
    pub l: u32,
    pub dh_mode: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DeriveKeyT {
    pub pub_key: u64,
    pub pub_key_size: u32,
    pub priv_key: u64,
    pub priv_key_size: u32,
}

#[repr(C)]
pub union DhParam {
    generate_key_t: GenerateKeyT,
    derive_key_t: DeriveKeyT,
}

#[repr(C)]
pub struct DhKeyT {
    prime: u64,
    prime_size: u32,
    generator: u64,
    generator_size: u32,
    dh_param: DhParam,
}

///
/// Description:     power on func for crypto hardware
/// Return:          CRYPTO_SUCCESS if success, othres means fail
///
pub type PowerOnFunc = extern "C" fn() -> i32;

///
/// Description:     power off func for crypto hardware
/// Return:          CRYPTO_SUCCESS if success, othres means fail
///
pub type PowerOffFunc = extern "C" fn() -> i32;

///
/// Description:     get algo context size
/// Input:           alg_type： algorithm type [CryptoAlgType]
/// Output:          NA
/// Return:          >0 if success, others means fail
///
pub type GetCtxSizeFunc = extern "C" fn(alg_type: u32) -> i32;

///
/// Description:    copy context
/// Input:          alg_typ： algorithm type [CryptoAlgType]
///                 src_ctx： source context pointer
/// Output:         dest_ctx： dest context pointer
/// Return:         CRYPTO_SUCCESS if success, othres means fail
///
pub type CtxCopyFunc = extern "C" fn(
    alg_type: u32,
    src_ctx: *const c_void,
    src_size: u32,
    dest_ctx: *mut c_void,
    dest_size: u32,
) -> i32;

///
/// get the ability of derive
///
pub type GetDriverAbilityFunc = extern "C" fn() -> i32;

///
/// Description:    init hash context
/// Input:          alg_type： algorithm type [CryptoAlgType]
/// Output:         ctx： handle for crypto
/// Return:         CRYPTO_SUCCESS if success, othres means fail
///
pub type HashInitFunc = extern "C" fn(ctx: *mut c_void, alg_type: u32) -> i32;

///
/// Description:    compute the hash of each data_in， and update context
/// Input:          ctx：             handle after hash_init
///                 data_in：         data in buffer
/// Output:         ctx：             handle after update
/// Return:         CRYPTO_SUCCESS if success, othres means fail
///
pub type HashUpdateFunc = extern "C" fn(ctx: *mut c_void, data_in: *const MemrefT) -> i32;

///
/// Description:    compute hash for input data, output the result
/// Input:          ctx：             handle should same as hash init and update
///                 data_in：         data in buffer
/// Output:         ctx：             handle after compute
///                 data_out：        data out buffer
/// Return:         CRYPTO_SUCCESS if success, othres means fail
///
pub type HashDoFinalFunc =
    extern "C" fn(ctx: *mut c_void, data_in: *const MemrefT, data_out: *mut MemrefT) -> i32;

///
/// Description:    interface for single hash compute
/// Input:          alg_type：      algorithm type [CryptoAlgType]
///                 data_in：       data in buffer
/// Output:         data_out：      data out buffer
/// Return:         CRYPTO_SUCCESS if success, othres means fail
pub type HashFunc =
    extern "C" fn(alg_type: u32, data_in: *const MemrefT, data_out: *mut MemrefT) -> i32;

///
/// Description:    init hmac context
/// Input:          alg_type：        algorithm type [CryptoAlgType]
///                 key：             key for hmac
/// Output:         ctx：             crypto handle
/// Return:         CRYPTO_SUCCESS if success, othres means fail
///
pub type HmacInitFunc =
    extern "C" fn(alg_type: u32, ctx: *mut c_void, key: *const SymmeritKeyT) -> i32;

///
/// Description:    compute each data in，update context
/// Input:          ctx：             handle after hmac init
///                 data_in：         data in buffer
/// Output:         ctx：             handle after update
/// Return:         CRYPTO_SUCCESS if success, othres means fail
///
pub type HmacUpdateFunc = extern "C" fn(ctx: *mut c_void, data_in: *const MemrefT) -> i32;

///
/// Description:    compute mac for input data，output the result
/// Input:          ctx：             handle should same as hmac init and update
///                 data_in：         data in buffer
/// Output:         ctx：             context after update
///                 data_out：        data out buffer
/// Return:         CRYPTO_SUCCESS if success, othres means fail
///
pub type HmacDoFinalFunc =
    extern "C" fn(ctx: *mut c_void, data_in: *const MemrefT, data_out: *mut MemrefT) -> i32;

///
/// Description:   interface for single hmac cmpute
/// Input:          alg_type：        algorithm type [CryptoAlgType]
///                 key：             HMAC key
///                 data_in：         data in buffer
/// Output:         data_out：        data out buffer
/// Return:         CRYPTO_SUCCESS if success, othres means fail
///
pub type HmacFunc = extern "C" fn(
    alg_type: u32,
    key: *const SymmeritKeyT,
    data_in: *const MemrefT,
    data_out: *mut MemrefT,
) -> i32;

///
/// Description:     init cipher context
/// Input:           alg_type：        algorithm type [CryptoAlgType]
///                  direction：       crypto type
///                  key：             cipher key
///                  iv：              vector for cipher
/// Output:          ctx：             context after init
/// Return:          CRYPTO_SUCCESS if success, othres means fail
///
pub type CipherInitFunc = extern "C" fn(
    alg_type: u32,
    ctx: *mut c_void,
    direction: u32,
    key: *const SymmeritKeyT,
    iv: *const MemrefT,
) -> i32;

///
/// Description:    compute cipher for each data, and update context
/// Input:          ctx：             handle after cipher init
///                 data_in：         data in buffer
/// Output:         ctx：             context after update
///                 data_out：        data out buffer
/// Return:         CRYPTO_SUCCESS if success, othres means fail
///
pub type CipherUpdateFunc =
    extern "C" fn(ctx: *mut c_void, data_in: *const MemrefT, data_out: *mut MemrefT) -> i32;

///
/// Description:    compute cipher data and output cihper result
/// Input:          ctx：             handle should same as cipher init and update
///                 data_in：         date in buffer
/// Output:         ctx：             handle after compute
///                 data_out：        data out buffer
/// Return:         CRYPTO_SUCCESS if success, othres means fail
///
pub type CipherDoFinalFunc =
    extern "C" fn(ctx: *mut c_void, data_in: *const MemrefT, data_out: *mut MemrefT) -> i32;

///
/// Description:    compute cipher input data, output compute result
/// Input:          alg_type：         algorithm type [CryptoAlgType]
///                 direction：        crypto type
///                 key：              cipher key
///                 iv：               vector for cipher
///                 data_in：          data in buffer
/// Output:         data_out：         data out buffer
/// Return:         CRYPTO_SUCCESS if success, othres means fail
///
pub type CipherFunc = extern "C" fn(
    alg_type: u32,
    direction: u32,
    key: *const SymmeritKeyT,
    iv: *const MemrefT,
    data_in: *const MemrefT,
    data_out: *mut MemrefT,
) -> i32;

///
/// Description:     init ae context
/// Input:           alg_type：               algorithm type [CryptoAlgType]
///                  direction：              crypto type
///                  key：                    aes key
///                  ae_init_param：          ae init params
/// Output:          ctx：                    crypto handle
/// Return:          NA
///
pub type AeInitFunc = extern "C" fn(
    alg_type: u32,
    ctx: *mut c_void,
    direction: u32,
    key: *const SymmeritKeyT,
    ae_init_param: *const AeInitData,
) -> i32;

///
/// Description:    compute aad data for ae, and update context
/// Input:          ctx：             handle after ae init
///                 aad_data：        AAD data buffer
/// Output:         NA
/// Return:         CRYPTO_SUCCESS if success, othres means fail
///
pub type AeUpdateAadFunc = extern "C" fn(ctx: *mut c_void, aad_data: *const MemrefT) -> i32;

///
/// Description:    compute each data for ae, and update context
/// Input:          ctx：             handle after ae init
///                 data_in：         data in buffer
/// Output:         ctx：             handle after update
///                 data_out：        data out buffer
/// Return:         CRYPTO_SUCCESS if success, othres means fail
pub type AeUpdateFunc =
    extern "C" fn(ctx: *mut c_void, data_in: *const MemrefT, data_out: *mut MemrefT) -> i32;

///
/// Description:    compute input data and output ecrypt result
/// Input:          ctx：             handle should same as ae init and update
///                 data_in：         data in buffer
/// Output:         ctx：             handle after update
///                 data_out：        data out buffer
///                 tag_out：         output tag
/// Return:         CRYPTO_SUCCESS if success, othres means fail
///
pub type AeEncFinalFunc = extern "C" fn(
    ctx: *mut c_void,
    data_in: *const MemrefT,
    data_out: *mut MemrefT,
    tag_out: *mut MemrefT,
) -> i32;

///
/// Description:    decrypt the data, output result
/// Input:          ctx：             should same as ae init and update
///                 data_in：         data in buffer
///                 tag_in：          tag input
/// Output:         ctx：             context after update
///                 data_out：        data out buffer
/// Return:         CRYPTO_SUCCESS if success, othres means fail
///
pub type AeDecFinalFunc = extern "C" fn(
    ctx: *mut c_void,
    data_in: *const MemrefT,
    tag_in: *const MemrefT,
    data_out: *mut MemrefT,
) -> i32;

///
/// Description:    generate key pair
/// Input:          key_size：      key size in bit
///                 e_value：       RSA public key e value，a small prime like：3， 7， 11，65537，at least 65537 for safety
///                 crt_mode：      generate CRT key pair?
/// Output:         key_pair：      key pair
/// Return:         CRYPTO_SUCCESS if success, othres means fail
///
pub type RsaGenerateKeypairFunc = extern "C" fn(
    key_size: u32,
    e_value: *const MemrefT,
    crt_mode: bool,
    key_pair: *mut RsaPrivKeyT,
) -> i32;

///
/// Description:    encrypt data by public key
/// Input:          alg_type：      algorithm type [CryptoAlgType]
///                 public_key：    public key pointer
///                 rsa_params：    extension params
///                 data_in：       data in buffer
/// Output:         data_out：      data out buffer
/// Return:         CRYPTO_SUCCESS if success, othres means fail
///
pub type RsaEncryptFunc = extern "C" fn(
    alg_type: u32,
    public_key: *const RsaPubKeyT,
    rsa_params: *const AsymmetricParamsT,
    data_in: *const MemrefT,
    data_out: *mut MemrefT,
) -> i32;

///
/// Description:    decrypt data by private key
/// Input:          alg_type：      algorithm type [CryptoAlgType]
///                 private_key：   private key pointer
///                 rsa_params：    extension params
///                 data_in：       data in buffer
/// Output:         data_out：      data out buffer
/// Return:         CRYPTO_SUCCESS if success, othres means fail
///
pub type RsaDecryptFunc = extern "C" fn(
    alg_type: u32,
    private_key: *const RsaPrivKeyT,
    rsa_params: *const AsymmetricParamsT,
    data_in: *const MemrefT,
    data_out: *mut MemrefT,
) -> i32;

///
/// Description:     sign data by private key
/// Input:           alg_type：      algorithm type [CryptoAlgType]
///                  private_key：   private key pointer
///                  rsa_params：    extension params
///                  digest：        digest data
/// Output:          signature：     signature data
/// Return:          CRYPTO_SUCCESS if success, othres means fail
///
pub type RsaSignDigestFunc = extern "C" fn(
    alg_type: u32,
    private_key: *const RsaPrivKeyT,
    rsa_params: *const AsymmetricParamsT,
    digest: *const MemrefT,
    signature: *mut MemrefT,
) -> i32;

///
/// Description:    verify signature by public key
/// Input:          alg_type：      algorithm type [CryptoAlgType]
///                 public_key：    public key pointer
///                 rsa_params：    extension params
///                 digest：        digest data
///                 signature：     signature data
/// Output:         NA
/// Return:         CRYPTO_SUCCESS if success, othres means fail
///
pub type RsaVerifyDigestFunc = extern "C" fn(
    alg_type: u32,
    public_key: *const RsaPubKeyT,
    rsa_params: *const AsymmetricParamsT,
    digest: *const MemrefT,
    signature: *const MemrefT,
) -> i32;

///
/// Description:     generate key pair by ecc curve
/// Input:            key_size：      key size in bit
///                   curve：         curve type
/// Output:           public_key：    public key pointer
///                   private_key：   private key pointer
/// Return:           CRYPTO_SUCCESS if success, othres means fail
///
pub type EccGeneteKeypairFunc = extern "C" fn(
    keysize: u32,
    curve: u32,
    public_key: *mut EccPrivKeyT,
    private_key: *mut EccPrivKeyT,
) -> i32;

///
/// Description:    encrypt data by ecc
/// Input:          alg_type：      algorithm type [CryptoAlgType]
///                 public_key：    public key pointer
///                 ec_params：     extension params
///                 data_in：       data in buffer
/// Output:         data_out：      data out buffer
/// Return:         CRYPTO_SUCCESS if success, othres means fail
///
pub type EccEncryptFunc = extern "C" fn(
    alg_type: u32,
    public_key: *const EccPubKeyT,
    ec_params: *const AsymmetricParamsT,
    data_in: *const MemrefT,
    data_out: *mut MemrefT,
) -> i32;

///
/// Description:    decrypt data by ecc
/// Input:          alg_type：      algorithm type [CryptoAlgType]
///                 private_key:    private key pointer
///                 ec_params：     extension params
///                 data_in：       data in buffer
/// Output:         data_out：      data out buffer
/// Return:         CRYPTO_SUCCESS if success, othres means fail
///
pub type EccDecryptFunc = extern "C" fn(
    alg_type: u32,
    private_key: *const EccPrivKeyT,
    ec_params: *const AsymmetricParamsT,
    data_in: *const MemrefT,
    data_out: *mut MemrefT,
) -> i32;

///
/// Description:    signature digest data by ecc
/// Input:          alg_type：      algorithm type [CryptoAlgType]
///                 private_key：   private key pointer
///                 ec_params：     extension params
///                 digest：        digest data
/// Output:         signature：     signature data
/// Return:         CRYPTO_SUCCESS if success, othres means fail
///
pub type EccSignDigestFunc = extern "C" fn(
    alg_type: u32,
    private_key: *const EccPrivKeyT,
    ec_params: *const AsymmetricParamsT,
    digest: *const MemrefT,
    signature: *mut MemrefT,
) -> i32;

///
/// Description:    verify signature by ecc public key
/// Input:          alg_type：      algorithm type [CryptoAlgType]
///                 public_key：    public key pointer
///                 ec_params：     extension params
///                 digest：        digest data
///                 signature：     signature data
/// Output:         NA
/// Return:         CRYPTO_SUCCESS if success, othres means fail
///
pub type EccVerifyDigestFunc = extern "C" fn(
    alg_type: u32,
    public_key: *const EccPubKeyT,
    ec_params: *const AsymmetricParamsT,
    digest: *const MemrefT,
    dignature: *const MemrefT,
) -> i32;

///
/// Description:   encrypt data input by private key
/// Input:         alg_type：      algorithm type [CryptoAlgType]
///                client_key：    client key
///                server_key：    server key
///                ec_params：     extension params
/// Output:        secret：        output key
/// Return:        CRYPTO_SUCCESS if success, othres means fail
///
pub type EcdhDeriveKeyFunc = extern "C" fn(
    alg_type: u32,
    client_key: *const EccPubKeyT,
    server_key: *const EccPrivKeyT,
    ec_params: *const AsymmetricParamsT,
    secret: *mut MemrefT,
) -> i32;

///
/// Description:    generate dh key
/// Input:          dh_generate_key_data： dh key derive param
/// Output:         pub_key：              public key pointer
///                 priv_key：             private key pointer
/// Return:         CRYPTO_SUCCESS if success, othres means fail
///
pub type DhGenerateKeyFunc = extern "C" fn(
    dh_generate_key_data: *const DhKeyT,
    pub_key: *mut MemrefT,
    priv_key: *mut MemrefT,
) -> i32;

///
/// Description:    verify signature data input
/// Input:          dh_derive_key_data：   dh key derive param
/// Output:         secret：               output key pointer
/// Return:         CRYPTO_SUCCESS if success, othres means fail
///
pub type DhDeriveKeyFunc =
    extern "C" fn(dh_derive_key_data: *const DhKeyT, secret: *mut MemrefT) -> i32;

///
/// Description:    generate random
/// Input:          size：           random buffer size
/// Output:         buffer：         random buffer
/// Return:         NA
///
pub type GenerateRandomFunc = extern "C" fn(buffer: *mut c_void, size: usize) -> i32;

///
/// Description:    get entropy source
/// Input:          size：           size of entropy
/// Output:         buffer：         entropy buffer
/// Return:         CRYPTO_SUCCESS if success, othres means fail
///
pub type GetEntropyFunc = extern "C" fn(buffer: *mut c_void, size: usize) -> i32;

///
/// Description:    
/// Input:          derive_type : derive type ,default HUK
///                 data_in：       data in buffer
/// Output:         data_out：      data out buffer
/// Return:         CRYPTO_SUCCESS if success, othres means fail
///
pub type DeriveRootKeyFunc =
    extern "C" fn(derive_type: u32, data_in: *const MemrefT, data_out: *mut MemrefT) -> i32;

///
/// Description:    
/// Input:            password： derive data
///                   salt: salt random value
///                   iterations： iteration times
///                   digest_type： digest algorithm
/// Output:           data_out： data out buffer
/// Return:           CRYPTO_SUCCESS if success, othres means fail
///
pub type Pbkdf2Func = extern "C" fn(
    password: *const MemrefT,
    salt: *const MemrefT,
    iterations: u32,
    digest_type: u32,
    data_out: *mut MemrefT,
) -> i32;

#[repr(C)]
pub struct CryptoOpsT {
    pub power_on: Option<PowerOnFunc>,
    pub power_off: Option<PowerOffFunc>,
    pub get_ctx_size: Option<GetCtxSizeFunc>,
    pub ctx_copy: Option<CtxCopyFunc>,
    pub get_driver_ability: Option<GetDriverAbilityFunc>,
    pub hash_init: Option<HashInitFunc>,
    pub hash_update: Option<HashUpdateFunc>,
    pub hash_dofinal: Option<HashDoFinalFunc>,
    pub hash: Option<HashFunc>,
    pub hmac_init: Option<HmacInitFunc>,
    pub hmac_update: Option<HmacUpdateFunc>,
    pub hmac_dofinal: Option<HmacDoFinalFunc>,
    pub hmac: Option<HmacFunc>,
    pub cipher_init: Option<CipherInitFunc>,
    pub cipher_update: Option<CipherUpdateFunc>,
    pub cipher_dofinal: Option<CipherDoFinalFunc>,
    pub cipher: Option<CipherFunc>,
    pub ae_init: Option<AeInitFunc>,
    pub ae_update_aad: Option<AeUpdateAadFunc>,
    pub ae_update: Option<AeUpdateFunc>,
    pub ae_enc_final: Option<AeEncFinalFunc>,
    pub ae_dec_final: Option<AeDecFinalFunc>,
    pub rsa_generate_keypair: Option<RsaGenerateKeypairFunc>,
    pub rsa_encrypt: Option<RsaEncryptFunc>,
    pub rsa_decrypt: Option<RsaDecryptFunc>,
    pub rsa_sign_digest: Option<RsaSignDigestFunc>,
    pub rsa_verify_digest: Option<RsaVerifyDigestFunc>,
    pub ecc_generate_keypair: Option<EccGeneteKeypairFunc>,
    pub ecc_encrypt: Option<EccEncryptFunc>,
    pub ecc_decrypt: Option<EccDecryptFunc>,
    pub ecc_sign_digest: Option<EccSignDigestFunc>,
    pub ecc_verify_digest: Option<EccVerifyDigestFunc>,
    pub ecdh_derive_key: Option<EcdhDeriveKeyFunc>,
    pub dh_generate_key: Option<DhGenerateKeyFunc>,
    pub dh_derive_key: Option<DhDeriveKeyFunc>,
    pub generate_random: Option<GenerateRandomFunc>,
    pub get_entropy: Option<GetEntropyFunc>,
    pub derive_root_key: Option<DeriveRootKeyFunc>,
    pub pbkdf2: Option<Pbkdf2Func>,
}

extern "C" {
    ///
    /// register crypto ops into crypto hal layer
    ///
    /// # Parameters
    /// engine: [CryptoEngine] , means use different hardware or software to do crypto actions
    /// ops: [CryptoOpsT] , the handle
    ///
    /// # Return
    /// 0 if success
    pub fn register_crypto_ops(engine: u32, ops: *const CryptoOpsT) -> i32;

    ///
    /// generate root key
    ///
    /// # Parameters
    /// key_type: [CryptoKeyTypeId]
    /// data_in: input data
    /// data_out: output data
    pub fn hw_derive_root_key(
        key_type: u32,
        data_in: *const MemrefT,
        data_out: *mut MemrefT,
    ) -> i32;

    ///
    /// generate random data
    ///
    /// # Parameters
    /// buffer: buffer to store random
    /// size: length of buffer
    ///
    /// # Return
    /// 0 if success
    pub fn hw_generate_random(buffer: *mut c_void, size: usize) -> i32;
}
