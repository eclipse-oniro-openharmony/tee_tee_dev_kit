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
mod crypto_api_ffi;

use crate::error::{FfiResult, FfiTeeError};
pub use crate::trusted_storage_api::TeeObjectHandle;
pub use crypto_api_ffi::*;

use crate::trusted_storage_api::TeeAttribute;

/// A guaranteed valid OperationHandle.
#[repr(transparent)]
pub struct TeeOperationHandle<'a> {
    pub(crate) handle: *mut __TeeOperationHandle,
    _p: core::marker::PhantomData<&'a *mut u8>,
}

impl Default for TeeOperationHandle<'_> {
    fn default() -> Self {
        Self {
            handle: core::ptr::null_mut(),
            _p: core::marker::PhantomData,
        }
    }
}

impl<'a> TeeOperationHandle<'a> {
    pub fn allocate_operation(
        algorithm: TeeCryptoAlgorithmId,
        mode: TeeOperationMode,
        max_key_size: u32,
    ) -> Result<TeeOperationHandle<'a>, FfiTeeError> {
        let mut operation = TeeOperationHandle::default();
        let res: FfiResult = unsafe {
            crypto_api_ffi::TEE_AllocateOperation(
                &mut operation.handle,
                algorithm,
                mode,
                max_key_size,
            )
        }
        .into();
        res?;
        assert!(!operation.handle.is_null());
        Ok(operation)
    }

    pub fn get_operation_info(&self) -> TeeOperationInfo {
        let mut operation_info = TeeOperationInfo::default();

        unsafe { crypto_api_ffi::TEE_GetOperationInfo(self.handle, &mut operation_info) }

        operation_info
    }

    pub fn reset_operation(&mut self) {
        unsafe {
            crypto_api_ffi::TEE_ResetOperation(self.handle);
        }
    }

    pub fn copy_from(&mut self, src_operation: &TeeOperationHandle) {
        unsafe {
            crypto_api_ffi::TEE_CopyOperation(self.handle, src_operation.handle);
        }
    }

    pub fn copy_to(&self, dest_operation: &mut TeeOperationHandle) {
        unsafe {
            crypto_api_ffi::TEE_CopyOperation(dest_operation.handle, self.handle);
        }
    }

    pub fn cipher_init(&mut self, iv: Option<&[u8]>) {
        let mut iv_len = 0;
        let iv_ptr = match iv {
            Some(x) => {
                iv_len = x.len();
                x.as_ptr()
            }
            None => core::ptr::null(),
        };
        unsafe {
            crypto_api_ffi::TEE_CipherInit(self.handle, iv_ptr as _, iv_len);
        }
    }

    pub fn cipher_update(
        &mut self,
        src_data: &[u8],
        dest_data: &mut [u8],
    ) -> Result<usize, FfiTeeError> {
        let mut dest_len = dest_data.len();
        let result: FfiResult = unsafe {
            crypto_api_ffi::TEE_CipherUpdate(
                self.handle,
                src_data.as_ptr() as _,
                src_data.len(),
                dest_data.as_mut_ptr() as _,
                &mut dest_len,
            )
        }
        .into();
        result?;
        Ok(dest_len)
    }

    pub fn cipher_do_final(
        &mut self,
        src_data: &[u8],
        dest_data: &mut [u8],
    ) -> Result<usize, FfiTeeError> {
        let mut dest_len = dest_data.len();
        let result: FfiResult = unsafe {
            crypto_api_ffi::TEE_CipherDoFinal(
                self.handle,
                src_data.as_ptr() as _,
                src_data.len(),
                dest_data.as_mut_ptr() as _,
                &mut dest_len,
            )
        }
        .into();
        result?;
        Ok(dest_len)
    }

    #[cfg(not(feature = "gp_support"))]
    #[cfg(any(feature = "api_level2", feature = "api_level3"))]
    pub fn digest_update(&mut self, chunk: &[u8]) {
        unsafe {
            crypto_api_ffi::TEE_DigestUpdate(self.handle, chunk.as_ptr() as _, chunk.len());
        }
    }

    #[cfg(not(feature = "gp_support"))]
    #[cfg(not(any(feature = "api_level2", feature = "api_level3")))]
    pub fn digest_update(&mut self, chunk: &[u8]) -> FfiResult {
        unsafe {
            crypto_api_ffi::TEE_DigestUpdate(self.handle, chunk.as_ptr() as _, chunk.len()).into()
        }
    }

    #[cfg(feature = "gp_support")]
    pub fn digest_update(&mut self, chunk: &[u8]) {
        unsafe {
            crypto_api_ffi::TEE_DigestUpdate(self.handle, chunk.as_ptr() as _, chunk.len());
        }
    }

    pub fn digest_do_final(
        &mut self,
        chunk: Option<&[u8]>,
        hash: &mut [u8],
    ) -> Result<usize, FfiTeeError> {
        let mut chunk_len = 0;
        let chunk_ptr = match chunk {
            Some(x) => {
                chunk_len = x.len();
                x.as_ptr()
            }
            None => core::ptr::null(),
        };
        let mut hash_len = hash.len();
        let res: FfiResult = unsafe {
            crypto_api_ffi::TEE_DigestDoFinal(
                self.handle,
                chunk_ptr as _,
                chunk_len,
                hash.as_mut_ptr() as _,
                &mut hash_len,
            )
        }
        .into();
        res?;
        Ok(hash_len)
    }

    pub fn mac_init(&mut self, iv: &[u8]) {
        unsafe {
            crypto_api_ffi::TEE_MACInit(self.handle, iv.as_ptr() as _, iv.len());
        }
    }

    pub fn mac_update(&mut self, chunk: &[u8]) {
        unsafe {
            crypto_api_ffi::TEE_MACUpdate(self.handle, chunk.as_ptr() as _, chunk.len());
        }
    }

    pub fn mac_compute_final(
        &mut self,
        message: &[u8],
        mac: &mut [u8],
        mac_len: &mut usize,
    ) -> FfiResult {
        unsafe {
            crypto_api_ffi::TEE_MACComputeFinal(
                self.handle,
                message.as_ptr() as _,
                message.len(),
                mac.as_mut_ptr() as _,
                mac_len,
            )
        }
        .into()
    }

    #[cfg(not(feature = "gp_support"))]
    #[cfg(any(feature = "api_level2", feature = "api_level3"))]
    pub fn mac_compare_final(&mut self, message: &[u8], mac: &[u8]) -> FfiResult {
        unsafe {
            crypto_api_ffi::TEE_MACCompareFinal(
                self.handle,
                message.as_ptr() as _,
                message.len(),
                mac.as_ptr() as _,
                mac.len(),
            )
        }
        .into()
    }

    #[cfg(not(feature = "gp_support"))]
    #[cfg(not(any(feature = "api_level2", feature = "api_level3")))]
    pub fn mac_compare_final(&mut self, message: &[u8], mac: &[u8], mac_len: &usize) -> FfiResult {
        unsafe {
            crypto_api_ffi::TEE_MACCompareFinal(
                self.handle,
                message.as_ptr() as _,
                message.len(),
                mac.as_ptr() as _,
                mac_len as _,
            )
        }
        .into()
    }

    #[cfg(feature = "gp_support")]
    pub fn mac_compare_final(&mut self, message: &[u8], mac: &[u8]) -> FfiResult {
        unsafe {
            crypto_api_ffi::TEE_MACCompareFinal(
                self.handle,
                message.as_ptr() as _,
                message.len(),
                mac.as_ptr() as _,
                mac.len(),
            )
        }
        .into()
    }

    pub fn ae_init(
        &mut self,
        nonce: &[u8],
        tag_len: u32,
        aad_len: usize,
        payload_len: usize,
    ) -> FfiResult {
        unsafe {
            crypto_api_ffi::TEE_AEInit(
                self.handle,
                nonce.as_ptr() as _,
                nonce.len(),
                tag_len,
                aad_len,
                payload_len,
            )
        }
        .into()
    }

    pub fn ae_update_aad(&mut self, aad_data: &[u8]) {
        unsafe {
            crypto_api_ffi::TEE_AEUpdateAAD(self.handle, aad_data.as_ptr() as _, aad_data.len());
        }
    }

    pub fn ae_update(
        &mut self,
        src_data: &[u8],
        dest_data: &mut [u8],
        dest_len: &mut usize,
    ) -> FfiResult {
        unsafe {
            crypto_api_ffi::TEE_AEUpdate(
                self.handle,
                src_data.as_ptr() as _,
                src_data.len(),
                dest_data.as_mut_ptr() as _,
                dest_len,
            )
        }
        .into()
    }

    pub fn ae_encrypt_final(
        &mut self,
        src_data: &[u8],
        dest_data: &mut [u8],
        dest_len: &mut usize,
        tag: &mut [u8],
        tag_len: &mut usize,
    ) -> FfiResult {
        unsafe {
            crypto_api_ffi::TEE_AEEncryptFinal(
                self.handle,
                src_data.as_ptr() as _,
                src_data.len(),
                dest_data.as_mut_ptr() as _,
                dest_len,
                tag.as_mut_ptr() as _,
                tag_len,
            )
        }
        .into()
    }

    pub fn ae_decrypt_final(
        &mut self,
        src_data: &[u8],
        dest_data: &mut [u8],
        dest_len: &mut usize,
        tag: &mut [u8],
    ) -> FfiResult {
        unsafe {
            crypto_api_ffi::TEE_AEDecryptFinal(
                self.handle,
                src_data.as_ptr() as _,
                src_data.len(),
                dest_data.as_mut_ptr() as _,
                dest_len,
                tag.as_mut_ptr() as _,
                tag.len(),
            )
        }
        .into()
    }

    pub fn asymmetric_encrypt(
        &mut self,
        params: &[TeeAttribute],
        src_data: &[u8],
        dest_data: &mut [u8],
        dest_len: &mut usize,
    ) -> FfiResult {
        unsafe {
            crypto_api_ffi::TEE_AsymmetricEncrypt(
                self.handle,
                params.as_ptr() as _,
                params.len() as u32,
                src_data.as_ptr() as _,
                src_data.len(),
                dest_data.as_mut_ptr() as _,
                dest_len,
            )
        }
        .into()
    }

    pub fn asymmetric_decrypt(
        &mut self,
        params: &[TeeAttribute],
        src_data: &[u8],
        dest_data: &mut [u8],
        dest_len: &mut usize,
    ) -> FfiResult {
        unsafe {
            crypto_api_ffi::TEE_AsymmetricDecrypt(
                self.handle,
                params.as_ptr() as _,
                params.len() as u32,
                src_data.as_ptr() as _,
                src_data.len(),
                dest_data.as_mut_ptr() as _,
                dest_len,
            )
        }
        .into()
    }

    pub fn asymmetric_sign_digest(
        &mut self,
        params: &[TeeAttribute],
        digest: &mut [u8],
        signature: &mut [u8],
        signature_len: &mut usize,
    ) -> FfiResult {
        unsafe {
            crypto_api_ffi::TEE_AsymmetricSignDigest(
                self.handle,
                params.as_ptr() as _,
                params.len() as u32,
                digest.as_mut_ptr() as _,
                digest.len(),
                signature.as_mut_ptr() as _,
                signature_len,
            )
        }
        .into()
    }

    pub fn asymmetric_verify_digest(
        &mut self,
        params: &[TeeAttribute],
        digest: &mut [u8],
        signature: &mut [u8],
    ) -> FfiResult {
        unsafe {
            crypto_api_ffi::TEE_AsymmetricVerifyDigest(
                self.handle,
                params.as_ptr() as _,
                params.len() as u32,
                digest.as_mut_ptr() as _,
                digest.len(),
                signature.as_mut_ptr() as _,
                signature.len(),
            )
        }
        .into()
    }

    #[cfg(any(feature = "api_level2", feature = "api_level3"))]
    pub fn get_operation_info_multiple(
        &mut self,
        operation_info_multiple: &mut TeeOperationInfoMultiple,
        operation_size: &usize,
    ) -> FfiResult {
        unsafe {
            crypto_api_ffi::TEE_GetOperationInfoMultiple(
                self.handle,
                operation_info_multiple,
                operation_size,
            )
        }
        .into()
    }
}

impl<'a> TeeOperationHandle<'a> {
    pub fn set_operation_key(
        &mut self,
        key: &TeeObjectHandle<crate::trusted_storage_api::InitializedTransientObject>,
    ) -> FfiResult {
        unsafe { crypto_api_ffi::TEE_SetOperationKey(self.handle, key.readonly_handle()) }.into()
    }

    pub fn set_operation_key2(
        &mut self,
        key1: &TeeObjectHandle<crate::trusted_storage_api::InitializedTransientObject>,
        key2: &TeeObjectHandle<crate::trusted_storage_api::InitializedTransientObject>,
    ) -> FfiResult {
        unsafe {
            crypto_api_ffi::TEE_SetOperationKey2(
                self.handle,
                key1.readonly_handle(),
                key2.readonly_handle(),
            )
        }
        .into()
    }
}

pub fn generate_random(random_buffer: &mut [u8]) {
    unsafe {
        crypto_api_ffi::TEE_GenerateRandom(random_buffer.as_mut_ptr() as _, random_buffer.len());
    }
}

#[cfg(any(feature = "api_level2", feature = "api_level3"))]
pub fn is_algorithm_supported(alg_id: u32, element: u32) -> FfiResult {
    unsafe { crypto_api_ffi::TEE_IsAlgorithmSupported(alg_id, element) }.into()
}

#[cfg(any(feature = "api_level2", feature = "api_level3"))]
pub fn is_hardware_support_algorithm(alg_type: u32) -> FfiResult {
    unsafe { crypto_api_ffi::TEE_IsHardWareSupportAlgorithm(alg_type) }.into()
}

impl Drop for TeeOperationHandle<'_> {
    fn drop(&mut self) {
        unsafe {
            crypto_api_ffi::TEE_FreeOperation(self.handle);
        }
    }
}
