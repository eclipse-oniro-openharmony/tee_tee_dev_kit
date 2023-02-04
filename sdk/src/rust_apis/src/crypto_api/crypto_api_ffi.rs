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
use usize;

pub use crate::tee_defines::*;
use crate::trusted_storage_api::{TeeAttribute, __TeeObjectHandle};

pub const TEE_MAX_KEY_SIZE_IN_BITS: u32 = 8192;
pub const SW_RSA_KEYLEN: u32 = 1024;
pub const TEE_DH_MAX_SIZE_OF_OTHER_INFO: usize = 64; /* bytes */

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct TeeOperationConstants(u32);

impl TeeOperationConstants {
    pub const TEE_OPERATION_INVALID: Self = Self(0x0);
    pub const TEE_OPERATION_CIPHER: Self = Self(0x1);
    pub const TEE_OPERATION_MAC: Self = Self(3);
    pub const TEE_OPERATION_AE: Self = Self(4);
    pub const TEE_OPERATION_DIGEST: Self = Self(5);
    pub const TEE_OPERATION_ASYMMETRIC_CIPHER: Self = Self(6);
    pub const TEE_OPERATION_ASYMMETRIC_SIGNATURE: Self = Self(7);
    pub const TEE_OPERATION_KEY_DERIVATION: Self = Self(8);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct TeeCryptoAlgorithmId(u32);
impl TeeCryptoAlgorithmId {
    pub const TEE_ALG_INVALID: Self = Self(0x0);
    pub const TEE_ALG_AES_ECB_NOPAD: Self = Self(0x10000010);
    pub const TEE_ALG_AES_CBC_NOPAD: Self = Self(0x10000110);
    pub const TEE_ALG_AES_CTR: Self = Self(0x10000210);
    pub const TEE_ALG_AES_CTS: Self = Self(0x10000310);
    pub const TEE_ALG_AES_XTS: Self = Self(0x10000410);
    pub const TEE_ALG_AES_CBC_MAC_NOPAD: Self = Self(0x30000110);
    pub const TEE_ALG_AES_CBC_MAC_PKCS5: Self = Self(0x30000510);
    pub const TEE_ALG_AES_CMAC: Self = Self(0x30000610);
    pub const TEE_ALG_AES_GMAC: Self = Self(0x30000810);
    pub const TEE_ALG_AES_CCM: Self = Self(0x40000710);
    pub const TEE_ALG_AES_GCM: Self = Self(0x40000810);
    pub const TEE_ALG_DES_ECB_NOPAD: Self = Self(0x10000011);
    pub const TEE_ALG_DES_CBC_NOPAD: Self = Self(0x10000111);
    pub const TEE_ALG_DES_CBC_MAC_NOPAD: Self = Self(0x30000111);
    pub const TEE_ALG_DES_CBC_MAC_PKCS5: Self = Self(0x30000511);
    pub const TEE_ALG_DES3_ECB_NOPAD: Self = Self(0x10000013);
    pub const TEE_ALG_DES3_CBC_NOPAD: Self = Self(0x10000113);
    pub const TEE_ALG_DES3_CBC_MAC_NOPAD: Self = Self(0x30000113);
    pub const TEE_ALG_DES3_CBC_MAC_PKCS5: Self = Self(0x30000513);
    pub const TEE_ALG_RSASSA_PKCS1_V1_5_MD5: Self = Self(0x70001830);
    pub const TEE_ALG_RSASSA_PKCS1_V1_5_SHA1: Self = Self(0x70002830);
    pub const TEE_ALG_RSASSA_PKCS1_V1_5_SHA224: Self = Self(0x70003830);
    pub const TEE_ALG_RSASSA_PKCS1_V1_5_SHA256: Self = Self(0x70004830);
    pub const TEE_ALG_RSASSA_PKCS1_V1_5_SHA384: Self = Self(0x70005830);
    pub const TEE_ALG_RSASSA_PKCS1_V1_5_SHA512: Self = Self(0x70006830);
    pub const TEE_ALG_RSASSA_PKCS1_PSS_MGF1_MD5: Self = Self(0x70111930);
    pub const TEE_ALG_RSASSA_PKCS1_PSS_MGF1_SHA1: Self = Self(0x70212930);
    pub const TEE_ALG_RSASSA_PKCS1_PSS_MGF1_SHA224: Self = Self(0x70313930);
    pub const TEE_ALG_RSASSA_PKCS1_PSS_MGF1_SHA256: Self = Self(0x70414930);
    pub const TEE_ALG_RSASSA_PKCS1_PSS_MGF1_SHA384: Self = Self(0x70515930);
    pub const TEE_ALG_RSASSA_PKCS1_PSS_MGF1_SHA512: Self = Self(0x70616930);
    pub const TEE_ALG_RSAES_PKCS1_V1_5: Self = Self(0x60000130);
    pub const TEE_ALG_RSAES_PKCS1_OAEP_MGF1_SHA1: Self = Self(0x60210230);
    #[cfg(feature = "gp_support")]
    pub const TEE_ALG_RSAES_PKCS1_OAEP_MGF1_SHA224: Self = Self(0x60310230);
    #[cfg(feature = "gp_support")]
    pub const TEE_ALG_RSAES_PKCS1_OAEP_MGF1_SHA256: Self = Self(0x60410230);
    #[cfg(feature = "gp_support")]
    pub const TEE_ALG_RSAES_PKCS1_OAEP_MGF1_SHA384: Self = Self(0x60510230);
    #[cfg(feature = "gp_support")]
    pub const TEE_ALG_RSAES_PKCS1_OAEP_MGF1_SHA512: Self = Self(0x60610230);
    #[cfg(not(feature = "gp_support"))]
    pub const TEE_ALG_RSAES_PKCS1_OAEP_MGF1_SHA224: Self = Self(0x60211230);
    #[cfg(not(feature = "gp_support"))]
    pub const TEE_ALG_RSAES_PKCS1_OAEP_MGF1_SHA256: Self = Self(0x60212230);
    #[cfg(not(feature = "gp_support"))]
    pub const TEE_ALG_RSAES_PKCS1_OAEP_MGF1_SHA384: Self = Self(0x60213230);
    #[cfg(not(feature = "gp_support"))]
    pub const TEE_ALG_RSAES_PKCS1_OAEP_MGF1_SHA512: Self = Self(0x60214230);
    pub const TEE_ALG_RSA_NOPAD: Self = Self(0x60000030);
    pub const TEE_ALG_DSA_SHA1: Self = Self(0x70002131);
    pub const TEE_ALG_DSA_SHA224: Self = Self(0x70003131);
    pub const TEE_ALG_DSA_SHA256: Self = Self(0x70004131);
    pub const TEE_ALG_DH_DERIVE_SHARED_SECRET: Self = Self(0x80000032);
    pub const TEE_ALG_MD5: Self = Self(0x50000001);
    pub const TEE_ALG_SHA1: Self = Self(0x50000002);
    pub const TEE_ALG_SHA224: Self = Self(0x50000003);
    pub const TEE_ALG_SHA256: Self = Self(0x50000004);
    pub const TEE_ALG_SHA384: Self = Self(0x50000005);
    pub const TEE_ALG_SHA512: Self = Self(0x50000006);
    pub const TEE_ALG_HMAC_MD5: Self = Self(0x30000001);
    pub const TEE_ALG_HMAC_SHA1: Self = Self(0x30000002);
    pub const TEE_ALG_HMAC_SHA224: Self = Self(0x30000003);
    pub const TEE_ALG_HMAC_SHA256: Self = Self(0x30000004);
    pub const TEE_ALG_HMAC_SHA384: Self = Self(0x30000005);
    pub const TEE_ALG_HMAC_SHA512: Self = Self(0x30000006);
    pub const TEE_ALG_HMAC_SM3: Self = Self(0x30000007);
    pub const TEE_ALG_AES_ECB_PKCS5: Self = Self(0x10000020);
    pub const TEE_ALG_AES_CBC_PKCS5: Self = Self(0x10000220);
    pub const TEE_ALG_ECDSA_SHA1: Self = Self(0x70001042);
    pub const TEE_ALG_ECDSA_SHA224: Self = Self(0x70002042);
    pub const TEE_ALG_ECDSA_SHA256: Self = Self(0x70003042);
    pub const TEE_ALG_ECDSA_SHA384: Self = Self(0x70004042);
    pub const TEE_ALG_ECDSA_SHA512: Self = Self(0x70005042);
    pub const TEE_ALG_ED25519: Self = Self(0x70005043);
    pub const TEE_ALG_ECDH_DERIVE_SHARED_SECRET: Self = Self(0x80000042);
    pub const TEE_ALG_X25519: Self = Self(0x80000044);
    pub const TEE_ALG_ECC: Self = Self(0x80000001);
    pub const TEE_ALG_ECDSA_P192: Self = Self(0x70001042);
    pub const TEE_ALG_ECDSA_P224: Self = Self(0x70002042);
    pub const TEE_ALG_ECDSA_P256: Self = Self(0x70003042);
    pub const TEE_ALG_ECDSA_P384: Self = Self(0x70004042);
    pub const TEE_ALG_ECDSA_P521: Self = Self(0x70005042);
    pub const TEE_ALG_ECDH_P192: Self = Self(0x80001042);
    pub const TEE_ALG_ECDH_P224: Self = Self(0x80002042);
    pub const TEE_ALG_ECDH_P256: Self = Self(0x80003042);
    pub const TEE_ALG_ECDH_P384: Self = Self(0x80004042);
    pub const TEE_ALG_ECDH_P521: Self = Self(0x80005042);
    pub const TEE_ALG_SIP_HASH: Self = Self(0xF0000002);
    pub const TEE_ALG_SM2_DSA_SM3: Self = Self(0x70006045);
    pub const TEE_ALG_SM2_PKE: Self = Self(0x80000045);
    pub const TEE_ALG_SM3: Self = Self(0x50000007);
    pub const TEE_ALG_SM4_ECB_NOPAD: Self = Self(0x10000014);
    pub const TEE_ALG_SM4_CBC_NOPAD: Self = Self(0x10000114);
    pub const TEE_ALG_SM4_CBC_PKCS7: Self = Self(0xF0000003);
    pub const TEE_ALG_SM4_CTR: Self = Self(0x10000214);
    pub const TEE_ALG_SM4_CFB128: Self = Self(0xF0000000);
    pub const TEE_ALG_SM4_XTS: Self = Self(0x10000414);
    pub const TEE_ALG_SM4_OFB: Self = Self(0x10000514);
    pub const TEE_ALG_AES_OFB: Self = Self(0x10000510);
    pub const TEE_ALG_SM4_GCM: Self = Self(0xF0000005);
}

impl From<TeeCryptoAlgorithmId> for u32 {
    fn from(e: TeeCryptoAlgorithmId) -> u32 {
        e.0
    }
}

pub const TEE_OPTIONAL_ELEMENT_NONE: u32 = 0x00000000;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct TeeEccCurve(pub u32);
impl TeeEccCurve {
    pub const TEE_ECC_CURVE_NIST_P192: Self = Self(0x00000001);
    pub const TEE_ECC_CURVE_NIST_P224: Self = Self(0x00000002);
    pub const TEE_ECC_CURVE_NIST_P256: Self = Self(0x00000003);
    pub const TEE_ECC_CURVE_NIST_P384: Self = Self(0x00000004);
    pub const TEE_ECC_CURVE_NIST_P521: Self = Self(0x00000005);
    pub const TEE_ECC_CURVE_SM2: Self = Self(0x00000300); /* CURVE_SM2 256 bits */
    pub const TEE_ECC_CURVE_25519: Self = Self(0x00000200); /* CURVE_25519 256 bits */
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct TeeDhHashMode(pub u32);
impl TeeDhHashMode {
    pub const TEE_DH_HASH_SHA1_MODE: Self = Self(0);
    pub const TEE_DH_HASH_SHA224_MODE: Self = Self(1);
    pub const TEE_DH_HASH_SHA256_MODE: Self = Self(2);
    pub const TEE_DH_HASH_SHA384_MODE: Self = Self(3);
    pub const TEE_DH_HASH_SHA512_MODE: Self = Self(4);
    pub const TEE_DH_HASH_NUM_OF_MODES: Self = Self(5);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct TeeDhOpModeT(pub u32);
impl TeeDhOpModeT {
    pub const TEE_DH_PKCS3_MODE: Self = Self(0); /* PKCS3 */
    pub const TEE_DH_ANSI_X942_MODE: Self = Self(1); /* X942 */
    pub const TEE_DH_NUM_OF_MODES: Self = Self(2); /* num of modes */
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct TeeDhDeriveFuncMode(pub u32);
impl TeeDhDeriveFuncMode {
    pub const TEE_DH_ASN1_DERIVE_MODE: Self = Self(0); /* ASN1_DerivMode */
    pub const TEE_DH_CONCAT_DERIVE_MODE: Self = Self(1); /* ConcatDerivMode */
    pub const TEE_DH_X963_DERIVE_MODE: Self = Self(1); /* X963_DerivMode */
    pub const TEE_DH_OMADRM_DERIVE_MODE: Self = Self(2); /* OMADRM_DerivMode */
    pub const TEE_DH_ISO18033_KDF1_DERIVE_MODE: Self = Self(3); /* ISO18033_KDF1_DerivMode */
    pub const TEE_DH_ISO18033_KDF2_DERIVE_MODE: Self = Self(4); /* ISO18033_KDF2_DerivMode */
    pub const TEE_DH_DERIVE_FUNC_NUM_OF_MODES: Self = Self(5); /* num of modes */
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct TeeDkObjectAttribute(pub u32);
impl TeeDkObjectAttribute {
    pub const TEE_DK_SECRECT: Self = Self(0); /* A pointer to shared secret value */
    pub const TEE_DK_OTHER: Self = Self(1); /* A pointer to structure containing other data */
    pub const TEE_DK_HASH_MODE: Self = Self(2); /* The enumerator ID of the HASH function to be used */
    pub const TEE_DK_DERIVATION_MODE: Self = Self(3); /* The enumerator ID of the derivation function mode */
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct TeeOperationMode(u32);
impl TeeOperationMode {
    pub const TEE_MODE_ENCRYPT: Self = Self(0);
    pub const TEE_MODE_DECRYPT: Self = Self(1);
    pub const TEE_MODE_SIGN: Self = Self(2);
    pub const TEE_MODE_VERIFY: Self = Self(3);
    pub const TEE_MODE_MAC: Self = Self(4);
    pub const TEE_MODE_DIGEST: Self = Self(5);
    pub const TEE_MODE_DERIVE: Self = Self(6);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct TeeOperationState(pub u32);
impl TeeOperationState {
    pub const TEE_OPERATION_STATE_INITIAL: Self = Self(0x00000000);
    pub const TEE_OPERATION_STATE_ACTIVE: Self = Self(0x00000001);
}

impl From<TeeOperationMode> for u32 {
    fn from(e: TeeOperationMode) -> u32 {
        // SAFETY: [TeeOperationMode] is `repr(u32)`, so it is guaranteed to fit
        e.0
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TeeDhOtherInfo {
    pub algorithm_id: [u8; TEE_DH_MAX_SIZE_OF_OTHER_INFO], /* object ID(OID) */
    pub size_of_algorithm_id: u32,                         /* length of AlgorithmID */
    pub party_u_info: [u8; TEE_DH_MAX_SIZE_OF_OTHER_INFO], /* public info of sender */
    pub size_of_party_u_info: u32,                         /* length of PartyUInfo */
    pub party_v_info: [u8; TEE_DH_MAX_SIZE_OF_OTHER_INFO], /* public info of receiver */
    pub size_of_party_v_info: u32,                         /* length of PartyVInfo */
    pub supp_priv_info: [u8; TEE_DH_MAX_SIZE_OF_OTHER_INFO], /* shared private info */
    pub size_of_supp_priv_info: u32,                       /* length of SuppPrivInfo */
    pub supp_pub_info: [u8; TEE_DH_MAX_SIZE_OF_OTHER_INFO], /* shared public info */
    pub size_of_supp_pub_info: u32,                        /* length of SuppPubInfo */
}

#[repr(C)]
pub struct TeeOperationInfo<'a> {
    pub algorithm: TeeCryptoAlgorithmId,
    pub operation_class: TeeOperationConstants,
    pub mode: TeeOperationMode,
    pub digest_length: u32,
    pub max_key_size: u32,
    pub key_size: u32,
    pub required_key_usage: u32,
    pub handle_state: u32,
    key_value: *mut c_void,
    _p: core::marker::PhantomData<&'a *mut u8>,
}

impl Default for TeeOperationInfo<'_> {
    fn default() -> Self {
        Self {
            algorithm: TeeCryptoAlgorithmId::TEE_ALG_INVALID,
            operation_class: TeeOperationConstants::TEE_OPERATION_INVALID,
            mode: TeeOperationMode::TEE_MODE_ENCRYPT,
            digest_length: 0,
            max_key_size: 0,
            key_size: 0,
            required_key_usage: 0,
            handle_state: 0,
            key_value: core::ptr::null_mut::<c_void>(),
            _p: core::marker::PhantomData,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TeeOperationInfoKey {
    pub key_size: u32,
    pub required_key_usage: u32,
}

#[repr(C)]
pub struct TeeOperationInfoMultiple {
    pub algorithm: TeeCryptoAlgorithmId,
    pub operation_class: TeeOperationConstants,
    pub mode: TeeOperationMode,
    pub digest_length: u32,
    pub max_key_size: u32,
    pub handle_state: u32,
    pub operation_state: u32,
    pub number_of_keys: u32,
    pub key_information: *mut TeeOperationInfoKey,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CryptoUint2Uint {
    pub src: u32,
    pub dest: u32,
}

#[repr(C)]
pub struct OperationSrcDest {
    pub src_data: *mut c_void,
    pub src_len: usize,
    pub dest_data: *mut c_void,
    pub dest_len: *mut usize,
}

#[repr(C)]
pub struct OperationAeInit {
    pub nonce: *mut c_void,
    pub nonce_len: usize,
    pub tag_len: u32,
    pub aad_len: usize,
    pub payload_len: usize,
}

#[repr(C)]
pub struct __TeeOperationHandle {
    _unused: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

extern "C" {
    ///
    /// alloc operation handle
    ///
    /// ### Params
    /// - operation (IN/OUT)
    /// - algorithm (IN)  #TEE_CRYPTO_ALGORITHM_ID
    /// - mode (IN)  #TEE_OperationMode
    /// - maxKeySize (IN)  The max key size
    ///
    /// ### Return
    /// - TEE_SUCCESS succss
    /// - TEE_ERROR_OUT_OF_MEMORY #usize malloc failed
    /// - TEE_ERROR_NOT_SUPPORTE #TEE_CRYPTO_ALGORITHM_ID not support
    /// - TEE_ERROR_GENERIC other failed
    ///
    pub fn TEE_AllocateOperation(
        operation: &mut *mut __TeeOperationHandle,
        algorithm: TeeCryptoAlgorithmId,
        mode: TeeOperationMode,
        max_key_size: u32,
    ) -> TeeResult;

    ///
    /// free Operation handle
    ///
    /// ### Params
    /// - operation (IN/OUT) operation handle
    ///
    /// ### Return
    /// - void
    ///
    pub fn TEE_FreeOperation(operation_handle: *mut __TeeOperationHandle);

    ///
    /// get Operation Info
    ///
    /// ### Params
    /// - operation (IN)
    /// - operationInfo (IN/OUT)  #TEE_OperationInfo
    ///
    /// ### Return
    /// - void
    ///
    pub fn TEE_GetOperationInfo(
        operation: *const __TeeOperationHandle,
        operation_info: *mut TeeOperationInfo,
    );

    ///
    /// reset operation handle
    ///
    /// ### Params
    /// - operation (IN/OUT)
    ///
    /// ### Return
    /// - void
    ///
    pub fn TEE_ResetOperation(operation: *mut __TeeOperationHandle);

    ///
    /// set operation key
    ///
    /// ### Params
    /// - operation (IN/OUT)
    /// - key (IN)
    ///
    /// ### Return
    /// - TEE_SUCCESS succss
    /// - TEE_ERROR_BAD_PARAMETERS the params is invalid
    /// - TEE_ERROR_OUT_OF_MEMORY  malloc key buffer failed
    ///
    pub fn TEE_SetOperationKey(
        operation: *mut __TeeOperationHandle,
        key: *const __TeeObjectHandle,
    ) -> TeeResult;

    ///
    /// set operation key1 and key2
    ///
    /// ### Params
    /// - operation (IN/OUT)
    /// - key1 (IN)
    /// - key2 (IN)
    ///
    /// ### Return
    /// - TEE_SUCCESS succss
    /// - TEE_ERROR_BAD_PARAMETERS the params is invalid
    ///
    pub fn TEE_SetOperationKey2(
        operation: *mut __TeeOperationHandle,
        key1: *const __TeeObjectHandle,
        key2: *const __TeeObjectHandle,
    ) -> TeeResult;

    ///
    /// copy src operation to dest operation
    ///
    /// ### Params
    /// - dstOperation (IN/OUT)
    /// - srcOperation (IN)
    ///
    /// ### Return
    /// - void
    ///
    pub fn TEE_CopyOperation(
        dest_operation: *mut __TeeOperationHandle,
        src_operation: *const __TeeOperationHandle,
    );

    ///
    /// init cipher context
    ///
    /// ### Params
    /// - operation (IN/OUT)
    /// - IV (IN)  the iv buffer, set NULL if not use
    /// - IVLen (IN)  the length of iv buffer
    ///
    /// ### Return
    /// - void
    ///
    pub fn TEE_CipherInit(operation: *mut __TeeOperationHandle, iv: *const c_void, iv_len: usize);

    ///
    /// do cipher update
    ///
    /// ### Params
    /// - operation (IN/OUT)
    /// - srcData (IN)  the src data
    /// - srcLen (IN)  the length of src data
    /// - destData (OUT) the dest data
    /// - destLen (OUT) the length of dest data
    ///
    /// ### Return
    /// - TEE_SUCCESS succss
    /// - TEE_ERROR_BAD_PARAMETERS the params is invalid
    /// - TEE_ERROR_GENERIC other error
    ///
    pub fn TEE_CipherUpdate(
        operation: *mut __TeeOperationHandle,
        src_data: *const c_void,
        src_len: usize,
        dest_data: *mut c_void,
        dest_len: *mut usize,
    ) -> TeeResult;

    ///
    /// do cipher finish
    ///
    /// ### Params
    /// - operation (IN/OUT)
    /// - srcData (IN)  the src data
    /// - srcLen (IN)  the length of src data
    /// - destData (OUT) the dest data
    /// - destLen (OUT) the length of dest data
    ///
    /// ### Return
    /// - TEE_SUCCESS succss
    /// - TEE_ERROR_BAD_PARAMETERS the params is invalid
    /// - TEE_ERROR_GENERIC other error
    ///
    pub fn TEE_CipherDoFinal(
        operation: *mut __TeeOperationHandle,
        src_data: *const c_void,
        src_len: usize,
        dest_data: *mut c_void,
        dest_len: *mut usize,
    ) -> TeeResult;

    ///
    /// do digest update
    ///
    /// ### Params
    /// - operation (IN/OUT)
    /// - chunk (IN) the chunk buffer
    /// - chunkSize (IN) the length of chunk buffer
    ///
    /// ### Return
    /// - TEE_SUCCESS succss
    /// - TEE_ERROR_GENERIC other error
    ///
    #[cfg(not(feature = "gp_support"))]
    #[cfg(any(feature = "api_level2", feature = "api_level3"))]
    pub fn TEE_DigestUpdate(
        operation: *mut __TeeOperationHandle,
        chunk: *const c_void,
        chunk_size: usize,
    );

    #[cfg(not(feature = "gp_support"))]
    #[cfg(not(any(feature = "api_level2", feature = "api_level3")))]
    pub fn TEE_DigestUpdate(
        operation: *mut __TeeOperationHandle,
        chunk: *const c_void,
        chunk_size: usize,
    ) -> TeeResult;

    #[cfg(feature = "gp_support")]
    pub fn TEE_DigestUpdate(
        operation: *mut __TeeOperationHandle,
        chunk: *const c_void,
        chunk_size: usize,
    );
    ///
    /// do digest finish
    ///
    /// ### Params
    /// - operation (IN/OUT)
    /// - chunk (IN) the chunk buffer
    /// - chunkSize (IN) the length of chunk buffer
    /// - hash (OUT)  the hash buffer
    /// - hashLen (OUT)  the length of hash buffer
    ///
    /// ### Return
    /// - TEE_SUCCESS success
    /// - TEE_ERROR_GENERIC other error
    ///
    pub fn TEE_DigestDoFinal(
        operation: *mut __TeeOperationHandle,
        chunk: *const c_void,
        chunk_len: usize,
        hash: *mut c_void,
        hash_len: *mut usize,
    ) -> TeeResult;

    ///
    /// do mac init
    ///
    /// ### Params
    /// - operation (IN/OUT)
    /// - IV (IN)  the iv buffer, set NULL if not use
    /// - IVLen (IN)  the length of iv buffer
    ///
    /// ### Return
    /// - void
    ///
    pub fn TEE_MACInit(operation: *mut __TeeOperationHandle, iv: *const c_void, iv_len: usize);

    ///
    /// do mac update
    ///
    /// ### Params
    /// - operation (IN/OUT)
    /// - chunk (IN) the chunk buffer
    /// - chunkSize (IN) the length of chunk buffer
    ///
    /// ### Return
    /// - void
    ///
    pub fn TEE_MACUpdate(
        operation: *mut __TeeOperationHandle,
        chunk: *const c_void,
        chunk_size: usize,
    );

    ///
    /// do mac finish
    ///
    /// ### Params
    /// - operation (IN/OUT)
    /// - message (IN) the message buffer
    /// - messageLen (IN) the length of message buffer
    /// - mac (OUT)  the mac buffer
    /// - macLen (OUT)  the length of mac buffer
    ///
    /// ### Return
    /// - TEE_SUCCESS success
    /// - TEE_ERROR_GENERIC other error
    ///
    pub fn TEE_MACComputeFinal(
        operation: *mut __TeeOperationHandle,
        message: *const c_void,
        message_len: usize,
        mac: *mut c_void,
        mac_len: *mut usize,
    ) -> TeeResult;

    ///
    /// do mac finish and compare
    ///
    /// ### Params
    /// - operation (IN/OUT)
    /// - message (IN) the message buffer
    /// - messageLen (IN) the length of message buffer
    /// - mac (IN)  the mac buffer
    /// - macLen (IN)  the length of mac buffer
    ///
    /// ### Return
    /// - TEE_SUCCESS success
    /// - TEE_ERROR_GENERIC other error
    /// - TEE_ERROR_MAC_INVALID compare failed
    ///
    #[cfg(not(feature = "gp_support"))]
    #[cfg(any(feature = "api_level2", feature = "api_level3"))]
    pub fn TEE_MACCompareFinal(
        operation: *mut __TeeOperationHandle,
        message: *const c_void,
        message_len: usize,
        mac: *const c_void,
        mac_len: usize,
    ) -> TeeResult;

    #[cfg(not(feature = "gp_support"))]
    #[cfg(not(any(feature = "api_level2", feature = "api_level3")))]
    pub fn TEE_MACCompareFinal(
        operation: *mut __TeeOperationHandle,
        message: *const c_void,
        message_len: usize,
        mac: *const c_void,
        mac_len: *const usize,
    ) -> TeeResult;

    #[cfg(feature = "gp_support")]
    pub fn TEE_MACCompareFinal(
        operation: *mut __TeeOperationHandle,
        message: *const c_void,
        message_len: usize,
        mac: *const c_void,
        mac_len: usize,
    ) -> TeeResult;

    ///
    /// do derive key
    ///
    /// ### Params
    /// - operation (IN/OUT)
    /// - params (IN) #TeeAttribute
    /// - paramCount (IN) the count of param
    /// - derivedKey (OUT)
    ///
    pub fn TEE_DeriveKey(
        operation: *mut __TeeOperationHandle,
        params: *const TeeAttribute,
        param_count: u32,
        derived_key: *const __TeeObjectHandle,
    );

    ///
    /// generate random data
    ///
    /// ### Params
    /// - randomBuffer (IN/OUT)  the random buffer
    /// - randomBufferLen (IN)  the length of random buffer
    pub fn TEE_GenerateRandom(random_buffer: *mut c_void, random_buffer_len: usize);

    /// do ae init
    ///
    /// ### Params
    /// - operation (IN/OUT)
    /// - nonce (IN) the nounce buffer
    /// - nonceLen (IN) the length of nounce
    /// - tagLen (IN) the length of tag
    /// - AADLen (IN) the length of aad
    /// - payloadLen (IN) the length of payload
    ///
    /// ### Return
    /// - TEE_SUCCESS success
    /// - TEE_ERROR_GENERIC other error
    ///
    pub fn TEE_AEInit(
        operation: *mut __TeeOperationHandle,
        nonce: *const c_void,
        nonce_len: usize,
        tag_len: u32,
        AAD_len: usize,
        payload_len: usize,
    ) -> TeeResult;

    /// update ae aad
    ///
    /// ### Params
    /// - operation (IN/OUT)
    /// - AADdata (IN) the aad buffer
    /// - AADdataLen (IN) the length of aad buffer
    pub fn TEE_AEUpdateAAD(
        operation: *mut __TeeOperationHandle,
        AAD_data: *const c_void,
        AAD_data_len: usize,
    );

    ///
    /// do ae update
    ///
    /// ### Params
    /// - operation (IN/OUT)
    /// - srcData (IN)  the src data
    /// - srcLen (IN)  the length of src data
    /// - destData (OUT) the dest data
    /// - destLen (OUT) the length of dest data
    ///
    /// ### Return
    /// - TEE_SUCCESS succss
    /// - TEE_ERROR_GENERIC other error
    ///
    pub fn TEE_AEUpdate(
        operation: *mut __TeeOperationHandle,
        src_data: *const c_void,
        src_len: usize,
        dest_data: *mut c_void,
        dest_len: *mut usize,
    ) -> TeeResult;

    ///
    /// do ae encrypt
    ///
    /// ### Params
    /// - operation (IN/OUT)
    /// - srcData (IN)  the src data
    /// - srcLen (IN)  the length of src data
    /// - destData (OUT) the dest data
    /// - destLen (OUT) the length of dest data
    /// - tag (OUT) the tag buffer
    /// - tagLen (OUT) the length of tag buffer
    ///
    /// ### Return
    /// - TEE_SUCCESS succss
    /// - TEE_ERROR_GENERIC other error
    ///
    pub fn TEE_AEEncryptFinal(
        operation: *mut __TeeOperationHandle,
        src_data: *const c_void,
        src_len: usize,
        dest_data: *mut c_void,
        dest_len: *mut usize,
        tag: *mut c_void,
        tag_len: *mut usize,
    ) -> TeeResult;

    ///
    /// do ae decrypt
    ///
    /// ### Params
    /// - operation (IN/OUT)
    /// - srcData (IN)  the src data
    /// - srcLen (IN)  the length of src data
    /// - destData (OUT) the dest data
    /// - destLen (OUT) the length of dest data
    /// - tag (IN) the tag buffer
    /// - tagLen (IN) the length of tag buffer
    ///
    /// ### Return
    /// - TEE_SUCCESS succss
    /// - TEE_ERROR_MAC_INVALID the tag is invalid
    ///
    pub fn TEE_AEDecryptFinal(
        operation: *mut __TeeOperationHandle,
        src_data: *const c_void,
        src_len: usize,
        dest_data: *mut c_void,
        dest_len: *mut usize,
        tag: *mut c_void,
        tag_len: usize,
    ) -> TeeResult;

    ///
    /// do asymmetric encrypt
    ///
    /// ### Params
    /// - operation (IN/OUT)
    /// - params (IN)  #TeeAttribute
    /// - paramCount (IN) the count of params
    /// - srcData (IN)  the src data
    /// - srcLen (IN)  the length of src data
    /// - destData (OUT) the dest data
    /// - destLen (OUT) the length of dest data
    ///
    /// ### Return
    /// - TEE_SUCCESS succss
    /// - TEE_ERROR_BAD_PARAMETERS the params is invalid
    /// - TEE_ERROR_GENERIC other error
    ///
    pub fn TEE_AsymmetricEncrypt(
        operation: *mut __TeeOperationHandle,
        params: *const TeeAttribute,
        param_count: u32,
        src_data: *const c_void,
        src_len: usize,
        dest_data: *mut c_void,
        dest_len: *mut usize,
    ) -> TeeResult;

    ///
    /// do asymmetric decrypt
    ///
    /// ### Params
    /// - operation (IN/OUT)
    /// - params (IN)  #TeeAttribute
    /// - paramCount (IN) the count of params
    /// - srcData (IN)  the src data
    /// - srcLen (IN)  the length of src data
    /// - destData (OUT) the dest data
    /// - destLen (OUT) the length of dest data
    ///
    /// ### Return
    /// - TEE_SUCCESS succss
    /// - TEE_ERROR_BAD_PARAMETERS the params is invalid
    /// - TEE_ERROR_GENERIC other error
    ///
    pub fn TEE_AsymmetricDecrypt(
        operation: *mut __TeeOperationHandle,
        params: *const TeeAttribute,
        param_count: u32,
        src_data: *const c_void,
        src_len: usize,
        dest_data: *mut c_void,
        dest_len: *mut usize,
    ) -> TeeResult;

    ///
    /// do asymmetric sign
    ///
    /// ### Params
    /// - operation (IN/OUT)
    /// - params (IN)  #TeeAttribute
    /// - paramCount (IN) the count of params
    /// - digest (IN)  the digest data
    /// - digestLen (IN)  the length of digest data
    /// - signature (OUT) the signature data
    /// - signatureLen (OUT) the length of signature data
    ///
    /// ### Return
    /// - TEE_SUCCESS succss
    /// - TEE_ERROR_BAD_PARAMETERS the params is invalid
    /// - TEE_ERROR_GENERIC other error
    ///
    pub fn TEE_AsymmetricSignDigest(
        operation: *mut __TeeOperationHandle,
        params: *const TeeAttribute,
        param_count: u32,
        digest: *const c_void,
        digest_len: usize,
        signature: *mut c_void,
        signature_len: *mut usize,
    ) -> TeeResult;

    ///
    /// do asymmetric verify
    ///
    /// ### Params
    /// - operation (IN/OUT)
    /// - params (IN)  #TeeAttribute
    /// - paramCount (IN) the count of params
    /// - digest (IN)  the digest data
    /// - digestLen (IN)  the length of digest data
    /// - signature (OUT) the signature data
    /// - signatureLen (OUT) the length of signature data
    ///
    /// ### Return
    /// - TEE_SUCCESS succss
    /// - TEE_ERROR_BAD_PARAMETERS the params is invalid
    /// - TEE_ERROR_GENERIC other error
    ///
    pub fn TEE_AsymmetricVerifyDigest(
        operation: *mut __TeeOperationHandle,
        params: *const TeeAttribute,
        param_count: u32,
        digest: *const c_void,
        digest_len: usize,
        signature: *mut c_void,
        signature_len: usize,
    ) -> TeeResult;

    ///
    /// Get Operation Info multiple
    ///
    /// ### Params
    /// - operation (IN)
    /// - operationInfoMultiple (IN/OUT) #TEE_OperationInfoMultiple
    /// - operationSize (IN/OUT) the size of operation handle
    ///
    /// ### Return
    /// - TEE_SUCCESS succss
    /// - TEE_ERROR_BAD_PARAMETERS the params is invalid
    /// - TEE_ERROR_SHORT_BUFFER the buffer is not large enough
    ///
    #[cfg(any(feature = "api_level2", feature = "api_level3"))]
    pub fn TEE_GetOperationInfoMultiple(
        operation: *mut __TeeOperationHandle,
        operation_info_multiple: *mut TeeOperationInfoMultiple,
        operation_size: *const usize,
    ) -> TeeResult;

    ///
    /// check whether the algorithm is sopported
    ///
    /// ### Params
    /// - algId (IN)  the algorithm
    /// - element (IN) the element
    ///
    /// ### Return
    /// - TEE_SUCCESS support
    /// - TEE_ERROR_NOT_SUPPORTED not support
    ///
    #[cfg(any(feature = "api_level2", feature = "api_level3"))]
    pub fn TEE_IsAlgorithmSupported(alg_id: u32, element: u32) -> TeeResult;

    #[cfg(any(feature = "api_level2", feature = "api_level3"))]
    pub fn TEE_IsHardWareSupportAlgorithm(alg_type: u32) -> TeeResult;
}
