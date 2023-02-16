use crate::framework::{ResumeFunc, SuspendFunc};

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
use super::crypto_driver_adaptor::*;
use core::ffi::c_void;

///
/// Description:       initial，can call ioremap and other init actions
/// Return:            CRYPTO_SUCCESS if success, others means fail
///
pub type InitFunc = extern "C" fn() -> i32;

///
/// Description:       initial，can call ioremap and other init actions
/// Input:             alg_type： algorithm type [CryptoAlgType]
/// Output:            is support?
/// Return:            CRYPTO_SUCCESS if success, others means fail
///
pub type IsAlgSupportFunc = extern "C" fn(alg_type: u32) -> bool;

///
/// Description:    get pbkdf2
/// Input:          buffer： in buffer
///                 size： buffer size
/// Output:         buffer： out buffer
/// Return:         CRYPTO_SUCCESS if success, others means fail
///
pub type GetOemkeyFunc = extern "C" fn(buffer: *mut c_void, size: usize) -> i32;

#[repr(C)]
pub struct CryptoDrvOpsT {
    pub init: Option<InitFunc>,
    pub is_alg_support: Option<IsAlgSupportFunc>,
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
    pub ecc_generate_keypair: Option<EccGenerateKeypairFunc>,
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
    pub get_oemkey: Option<GetOemkeyFunc>,
    pub suspend: Option<SuspendFunc>,
    pub resume: Option<ResumeFunc>,
}

#[macro_export]
macro_rules! crypto_driver_declare {
    ($func_init:expr, $func_is_alg_support:expr, $func_power_on:expr, $func_power_off:expr, $func_get_ctx_size:expr,
        $func_ctx_copy:expr, $func_get_driver_ability:expr,$func_hash_init:expr, $func_hash_update:expr,
        $func_hash_dofinal:expr, $func_hash:expr, $func_hmac_init:expr, $func_hmac_update:expr, $func_hmac_dofinal:expr,
        $func_hmac:expr, $func_cipher_init:expr,$func_cipher_update:expr, $func_cipher_dofinal:expr, $func_cipher:expr,
        $func_ae_init:expr, $func_ae_update_aad:expr, $func_ae_update:expr, $func_ae_enc_final:expr, $func_ae_dec_final:expr,
        $func_rsa_generate_keypair:expr, $func_rsa_encrypt:expr, $func_rsa_decrypt:expr, $func_rsa_sign_digest:expr,
        $func_rsa_verify_digest:expr, $func_ecc_generate_keypair:expr,$func_ecc_encrypt:expr, $func_ecc_decrypt:expr,
        $func_ecc_sign_digest:expr, $func_ecc_verify_digest:expr, $func_ecdh_derive_key:expr, $func_dh_generate_key:expr,
        $func_dh_derive_key:expr,$func_generate_random:expr, $func_get_entropy:expr, $func_derive_root_key:expr, $func_pbkdf2:expr,
        $func_get_oemkey:expr, $func_suspend:expr, $func_resume:expr) => {
        #[allow(dead_code)]
        #[no_mangle]
        pub static g_crypto_drv_ops: rust_drv_apis::crypto_framework::CryptoDrvOpsT =
            rust_drv_apis::crypto_framework::CryptoDrvOpsT {
                init: $func_init,
                is_alg_support: $func_is_alg_support,
                power_on: $func_power_on,
                power_off: $func_power_off,
                get_ctx_size: $func_get_ctx_size,
                ctx_copy: $func_ctx_copy,
                get_driver_ability: $func_get_driver_ability,
                hash_init: $func_hash_init,
                hash_update: $func_hash_update,
                hash_dofinal: $func_hash_dofinal,
                hash: $func_hash,
                hmac_init: $func_hmac_init,
                hmac_update: $func_hmac_update,
                hmac_dofinal: $func_hmac_dofinal,
                hmac: $func_hmac,
                cipher_init: $func_cipher_init,
                cipher_update: $func_cipher_update,
                cipher_dofinal: $func_cipher_dofinal,
                cipher: $func_cipher,
                ae_init: $func_ae_init,
                ae_update_aad: $func_ae_update_aad,
                ae_update: $func_ae_update,
                ae_enc_final: $func_ae_enc_final,
                ae_dec_final: $func_ae_dec_final,
                rsa_generate_keypair: $func_rsa_generate_keypair,
                rsa_encrypt: $func_rsa_encrypt,
                rsa_decrypt: $func_rsa_decrypt,
                rsa_sign_digest: $func_rsa_sign_digest,
                rsa_verify_digest: $func_rsa_verify_digest,
                ecc_generate_keypair: $func_ecc_generate_keypair,
                ecc_encrypt: $func_ecc_encrypt,
                ecc_decrypt: $func_ecc_decrypt,
                ecc_sign_digest: $func_ecc_sign_digest,
                ecc_verify_digest: $func_ecc_verify_digest,
                ecdh_derive_key: $func_ecdh_derive_key,
                dh_generate_key: $func_dh_generate_key,
                dh_derive_key: $func_dh_derive_key,
                generate_random: $func_generate_random,
                get_entropy: $func_get_entropy,
                derive_root_key: $func_derive_root_key,
                pbkdf2: $func_pbkdf2,
                get_oemkey: $func_get_oemkey,
                suspend: $func_suspend,
                resume: $func_resume,
            };
    };
}
