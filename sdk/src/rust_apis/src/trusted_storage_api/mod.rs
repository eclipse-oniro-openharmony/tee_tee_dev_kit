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

mod generic_object_ffi;
mod persistent_object_ffi;
mod transient_object_ffi;

use core::{
    marker::PhantomData,
    mem::{self, ManuallyDrop},
    ops::BitAnd,
};

pub use generic_object_ffi::*;

use crate::{
    crypto_api::{self, TeeOperationHandle},
    error::{FfiResult, FfiTeeError, TeeError},
    TeeResult,
};

use self::persistent_object_ffi::__TeeObjectEnumHandle;

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct TeeObjectInfo {
    pub object_type: TeeObjectType,
    pub object_size: u32,
    pub max_object_size: u32,
    pub object_usage: UsageConstants,
    pub data_size: u32,
    pub data_position: u32,
    pub handle_flags: HandleFlagConstants,
}

#[repr(transparent)]
#[derive(Clone, Copy, Default)]
pub struct TeeObjectType(u32);

impl TeeObjectType {
    pub const AES: Self = Self(0xA0000010);
    pub const DES: Self = Self(0xA0000011);
    pub const DES3: Self = Self(0xA0000013);
    pub const HMAC_MD5: Self = Self(0xA0000001);
    pub const HMAC_SHA1: Self = Self(0xA0000002);
    pub const HMAC_SHA224: Self = Self(0xA0000003);
    pub const HMAC_SHA256: Self = Self(0xA0000004);
    pub const HMAC_SHA384: Self = Self(0xA0000005);
    pub const HMAC_SHA512: Self = Self(0xA0000006);
    pub const RSA_PUBLIC_KEY: Self = Self(0xA0000030);
    pub const RSA_KEYPAIR: Self = Self(0xA1000030);
    pub const DSA_PUBLIC_KEY: Self = Self(0xA0000031);
    pub const DSA_KEYPAIR: Self = Self(0xA1000031);
    pub const DH_KEYPAIR: Self = Self(0xA1000032);
    pub const GENERIC_SECRET: Self = Self(0xA0000000);
    pub const DATA: Self = Self(0xA1000033);
    pub const DATA_GP1_1: Self = Self(0xA00000BF);
    pub const ECDSA_PUBLIC_KEY: Self = Self(0xA0000041);
    pub const ECDSA_KEYPAIR: Self = Self(0xA1000041);
    pub const ECDH_PUBLIC_KEY: Self = Self(0xA0000042);
    pub const ECDH_KEYPAIR: Self = Self(0xA1000042);
    pub const ED25519_PUBLIC_KEY: Self = Self(0xA0000043);
    pub const ED25519_KEYPAIR: Self = Self(0xA1000043);
    pub const X25519_PUBLIC_KEY: Self = Self(0xA0000044);
    pub const X25519_KEYPAIR: Self = Self(0xA1000044);
    pub const SM2_DSA_PUBLIC_KEY: Self = Self(0xA0000045);
    pub const SM2_DSA_KEYPAIR: Self = Self(0xA1000045);
    pub const SM2_KEP_PUBLIC_KEY: Self = Self(0xA0000046);
    pub const SM2_KEP_KEYPAIR: Self = Self(0xA1000046);
    pub const SM2_PKE_PUBLIC_KEY: Self = Self(0xA0000047);
    pub const SM2_PKE_KEYPAIR: Self = Self(0xA1000047);
    pub const HMAC_SM3: Self = Self(0xA0000007);
    pub const SM4: Self = Self(0xA0000014);
    pub const SIP_HASH: Self = Self(0xF0000002);
    pub const PBKDF2_HMAC: Self = Self(0xF0000004);

    pub const CORRUPTED_OBJECT: Self = Self(0xA00000BE);
}

/// Data stream positioning start position option, used in TEE_SeekObjectData function
#[repr(transparent)]
pub struct TeeWhence(u32);

impl TeeWhence {
    /// Position the starting position as the beginning of the data stream
    pub const TEE_DATA_SEEK_SET: u32 = 0;
    /// Position the starting position as the current data stream position
    pub const TEE_DATA_SEEK_CUR: u32 = 1;
    /// Position the starting position at the end of the data stream
    pub const TEE_DATA_SEEK_END: u32 = 2;
}

/// The keyUsage of TEE_ObjectHandle determines the usage of the object key
#[repr(transparent)]
#[derive(Copy, Clone, Default)]
pub struct UsageConstants(pub u32);

impl UsageConstants {
    /// The key of the object can be extracted
    pub const EXTRACTABLE: Self = Self(0x00000001);
    /// The key of the object can be used for encryption
    pub const ENCRYPT: Self = Self(0x00000002);
    /// The key of the object can be used for decryption
    pub const DECRYPT: Self = Self(0x00000004);
    /// The key of the object can be used for hash
    pub const MAC: Self = Self(0x00000008);
    /// The key of the object can be used for signing
    pub const SIGN: Self = Self(0x00000010);
    /// The key of the object can be used for verification
    pub const VERIFY: Self = Self(0x00000020);
    /// The key of the object can be used to derive
    pub const DERIVE: Self = Self(0x00000040);
    /// object initialization, all permissions are assigned by default
    pub const DEFAULT: Self = Self(0xFFFFFFFF);
}

/// The handleFlags of TEE_ObjectHandle indicate some information of the object,
/// whether it is a permanent object, whether it is initialized, etc.
#[repr(transparent)]
#[derive(Copy, Clone, Default)]
pub struct HandleFlagConstants(pub u32);

impl HandleFlagConstants {
    /// Persistent object
    pub const PERSISTENT: Self = Self(0x00010000);
    /// Object has been initialized
    pub const INITIALIZED: Self = Self(0x00020000);
    /// Unused
    pub const KEYSET: Self = Self(0x00040000);
    /// Unused
    pub const EXPECT_TWO_KEYS: Self = Self(0x00080000);
}

/// Storage ID, which defines the storage space of the corresponding application
#[repr(transparent)]
pub struct ObjectStorageConstants(u32);

impl ObjectStorageConstants {
    /// Separate private storage space for each application
    pub const PRIVATE: Self = Self(0x00000001);
    /// Separate personal storage space for application
    pub const PERSONAL: Self = Self(0x00000002);
    /// Add for secure flash storage
    pub const SEC_FLASH: Self = Self(0x80000000);
    /// Add for rpmb storage
    pub const RPMB: Self = Self(0x80000001);
    /// Add for storage ce
    pub const CE: Self = Self(0x80000002);
}

/// The handleFlags of TEE_ObjectHandle determines the access authority of
/// the TEE_ObjectHandle to the object data stream
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct DataFlagConstants(pub u32);

impl DataFlagConstants {
    /// Have read permission to the data stream, and can read
    pub const ACCESS_READ: Self = Self(0x00000001);
    /// Have write permission to the data stream, and can write and truncate
    pub const ACCESS_WRITE: Self = Self(0x00000002);
    /// Have WRITE_META permission for data stream, and can delete and rename operation
    pub const ACCESS_WRITE_META: Self = Self(0x00000004);
    /// Have shared read permissions on the data stream, you can open multiple
    /// TEE_ObjectHandles for concurrent reading
    pub const SHARE_READ: Self = Self(0x00000010);
    /// Have shared write permissions for the data stream, and multiple TEE_ObjectHandles
    /// can be opened for concurrent writing
    pub const SHARE_WRITE: Self = Self(0x00000020);
    /// Unused
    pub const CREATE: Self = Self(0x00000200);
    /// Protect an existing file with the same name. If the file with the same name does not exist,
    /// create a new data file; if the file with the same name exists, an error will be reported
    pub const OVERWRITE: Self = Self(0x00000400);
    /// If the bit27 is set to 1, it means deriving the 32-bytes TA root key at one time,
    /// if it is 0, it means  deriving two 16-bytes TA root keys and combined them together
    pub const DERIVE_32BYTES_KEY_ONCE: Self = Self(0x08000000);
    /// If bit28 is set to 1, it means AES256, if it is 0, it means AES128
    pub const AES256: Self = Self(0x10000000);
    /// If bit29 is set to 1, it means that the lower version will be opened first
    pub const OPEN_AESC: Self = Self(0x20000000);
}

/// Information of the object data part, the total length of the data part and the current
/// position of the data stream
#[derive(Default, Copy, Clone)]
pub struct ObjectDataInfo {
    // Data stream position
    pub pos: u32,
    /// Data stream length
    pub len: u32,
}

/// A Generic TEE Object
pub trait TeeObject {}
/// A Transient [TeeObject]
pub trait TransientObject: TeeObject {}

/// An initialized [TeeObject]
pub trait Initialized: TeeObject {}

pub struct InitializedTransientObject;
impl TransientObject for InitializedTransientObject {}
impl Initialized for InitializedTransientObject {}
impl TeeObject for InitializedTransientObject {}

pub struct UninitializedTransientObject;
impl TransientObject for UninitializedTransientObject {}
impl TeeObject for UninitializedTransientObject {}

/// A persistent [TeeObject]
pub struct PersistentObject;
impl Initialized for PersistentObject {}
impl TeeObject for PersistentObject {}

#[repr(transparent)]
pub struct TeeObjectHandle<T: TeeObject> {
    // An owned handle, which is guaranteed tobe a valid and unique pointer
    handle: *mut __TeeObjectHandle,
    _ty: core::marker::PhantomData<T>,
}

pub enum AllocateTransientObjectError {
    /// If allocating the object handle failed
    OutOfMemory,
    /// if the key size or object type is not supported.
    NotSupported,
}

impl From<AllocateTransientObjectError> for TeeError {
    fn from(e: AllocateTransientObjectError) -> Self {
        match e {
            AllocateTransientObjectError::NotSupported => TeeError::NotSupported,
            AllocateTransientObjectError::OutOfMemory => TeeError::OutOfMemory,
        }
    }
}

impl From<AllocateTransientObjectError> for FfiTeeError {
    fn from(e: AllocateTransientObjectError) -> Self {
        let tee_error: TeeError = e.into();
        tee_error.into()
    }
}

impl<T: TransientObject + TeeObject> TeeObjectHandle<T> {
    pub fn new(
        object_type: TeeObjectType,
        max_object_size: u32,
    ) -> Result<TeeObjectHandle<UninitializedTransientObject>, AllocateTransientObjectError> {
        let mut object_handle: *mut __TeeObjectHandle = core::ptr::null_mut();
        let res = unsafe {
            transient_object_ffi::TEE_AllocateTransientObject(
                object_type,
                max_object_size,
                &mut object_handle,
            )
        };
        match res {
            TeeResult::TEE_SUCCESS => {
                debug_assert!(!object_handle.is_null());
                let transient_object: TeeObjectHandle<UninitializedTransientObject> =
                    TeeObjectHandle {
                        // SAFETY: We trust the pointer set by `TEE_AllocateTransientObject` to be valid.
                        handle: object_handle,
                        _ty: core::marker::PhantomData,
                    };
                Ok(transient_object)
            }
            TeeResult::TEE_ERROR_OUT_OF_MEMORY => Err(AllocateTransientObjectError::OutOfMemory),
            TeeResult::TEE_ERROR_NOT_SUPPORTED => Err(AllocateTransientObjectError::NotSupported),
            _x => unreachable!("GPD specification"),
        }
    }

    pub fn reset(self) -> TeeObjectHandle<UninitializedTransientObject> {
        // SAFETY: The handle is owned, so resetting is ok
        unsafe {
            transient_object_ffi::TEE_ResetTransientObject(self.handle);
        }
        let new_handle = TeeObjectHandle {
            handle: self.handle,
            _ty: PhantomData,
        };
        mem::forget(self);
        new_handle
    }

    pub fn free(&mut self) {
        unsafe {
            transient_object_ffi::TEE_FreeTransientObject(self.handle);
        }
    }
}

// Note: We can call the same `TEE_CloseObject` on all types of functions, so lets just do that.
// Using the more specific functions for transient and persistent objects woule require
// specialization or (ab)using deref coeercions.
impl<T> Drop for TeeObjectHandle<T>
where
    T: TeeObject,
{
    fn drop(&mut self) {
        unsafe {
            generic_object_ffi::TEE_CloseObject(self.handle);
        }
    }
}

pub enum PersistentObjectError {
    /// The persistent object is corrupt.
    CorruptObject,
    /// The persistent Object is stored in an area which is currently inaccessible
    StorageNotAvailable,
}

// Function on a generic Object handle that may or may not be initialized.
impl<T: TeeObject> TeeObjectHandle<T> {
    #[cfg(any(feature = "api_level2", feature = "api_level3"))]
    pub fn object_info(&self) -> Result<TeeObjectInfo, PersistentObjectError> {
        let mut info = TeeObjectInfo::default();
        let res = unsafe { generic_object_ffi::TEE_GetObjectInfo1(self.handle as _, &mut info) };
        match res {
            TeeResult::TEE_SUCCESS => Ok(info),
            TeeResult::TEE_ERROR_CORRUPT_OBJECT => Err(PersistentObjectError::CorruptObject),
            TeeResult::TEE_ERROR_STORAGE_NOT_AVAILABLE => {
                Err(PersistentObjectError::StorageNotAvailable)
            }
            _ => unreachable!("GPD specification"),
        }

        // TEE_ERROR_CORRUPT_OBJECT: If the persistent object is corrupt. The object handle SHALL behave
        // based on the gpd.ta.doesNotCloseHandleOnCorruptObject property.
        // Note: Our implementation does not return this error and panics instead, so maybe we can
        // just ignore this
    }

    /// Restricts the object usage flags to contain at most the flags passed in `object_usage`.
    ///
    /// Bitflags not set in `object_usage` will be cleared, while set Bitflags will remain
    /// untouched.
    ///
    /// # Examples
    ///
    /// ```
    /// // The new_obj initially has all `UsageConstants set.
    /// let mut new_obj = TeeObjectHandle::<UninitializedTransientObject>::new(
    ///                     object_type,max_object_size);
    /// // Restrict new_obj to only `DECRYPT` and `ENCRYPT`.
    /// new_obj.restrict_usage(UsageConstants::DECRYPT | UsageConstants::ENCRYPT).unwrap();
    /// // Calling `restrict_usage()` again with all flags enabled has no effect since previously
    /// // cleared flags can't be restored.
    /// new_obj.restrict_usage(UsageConstants::all()).unwrap()
    /// ```
    #[cfg(any(feature = "api_level2", feature = "api_level3"))]
    pub fn restrict_usage(&mut self, object_usage: UsageConstants) -> crate::error::FfiResult {
        unsafe { generic_object_ffi::TEE_RestrictObjectUsage1(self.handle, object_usage.0) }.into()
    }
}

impl TeeObjectHandle<UninitializedTransientObject> {
    /// Mark the [TeeObjectHandle] as initialized
    ///
    /// An internal function to hide the details on how the struct is marked as
    /// initialized.
    ///
    /// # Safety
    ///
    /// The caller promises that the [TeeObjectHandle] has indeed been initialized
    unsafe fn into_initialized(self) -> TeeObjectHandle<InitializedTransientObject> {
        let new_handle = TeeObjectHandle {
            handle: self.handle,
            _ty: PhantomData,
        };
        mem::forget(self);
        new_handle
    }

    /// Populate an uninitialized transient object
    ///
    /// # Errors
    ///
    /// If an incorrect or inconsistent attribute value is detected. In this case the
    /// uninitialized transient object is returned.
    ///
    /// # Panics
    ///
    /// * If some mandatory attribute is missing
    /// * If attrs includes an attribute that is not defined for the object’s type.
    /// * If an attribute value is too big to fit within the maximum object size specified when the
    ///   object was created
    /// * If the implementation detects any other error associated with this function other than
    ///   bad parameters

    pub fn populate_transient_object(
        self,
        attrs: &[TeeAttribute],
    ) -> Result<TeeObjectHandle<InitializedTransientObject>, Self> {
        debug_assert!(attrs.len() <= u32::MAX as usize);
        let res = unsafe {
            transient_object_ffi::TEE_PopulateTransientObject(
                self.handle,
                attrs.as_ptr(),
                attrs.len() as u32,
            )
        };
        match res {
            TeeResult::TEE_SUCCESS => {
                // SAFETY: The initialization was successfull.
                unsafe { Ok(self.into_initialized()) }
            }
            TeeResult::TEE_ERROR_BAD_PARAMETERS => Err(self),
            // The TEE should panic for any other error reasons, so the following code really should
            // be unreachable.
            _ => unreachable!("Invalid error code"),
        }
    }

    pub fn derive_key(
        self,
        operation: &mut TeeOperationHandle,
        params: &[TeeAttribute],
    ) -> Result<TeeObjectHandle<InitializedTransientObject>, Self> {
        unsafe {
            crypto_api::TEE_DeriveKey(
                operation.handle,
                params.as_ptr() as _,
                params.len() as _,
                self.handle,
            );
        }
        Ok(unsafe { self.into_initialized() })
    }

    /// Generate a random key or key-pair
    pub fn generate_key(
        self,
        key_size: u32,
        params: &[TeeAttribute],
    ) -> Result<TeeObjectHandle<InitializedTransientObject>, Self> {
        debug_assert!(params.len() <= u32::MAX as usize);
        let res = unsafe {
            transient_object_ffi::TEE_GenerateKey(
                self.handle,
                key_size,
                params.as_ptr(),
                params.len() as u32,
            )
        };
        match res {
            TeeResult::TEE_SUCCESS => Ok(unsafe { self.into_initialized() }),
            TeeResult::TEE_ERROR_BAD_PARAMETERS => Err(self),
            _ => unreachable!("Invalid error code"),
        }
    }

    /// Initialize Self with the contents of `src_object`
    ///
    /// # Panics
    ///
    /// * If the type and size of srcObject and destObject are not compatible.
    /// * If the TEE implementation detects any other error associated with this function
    #[cfg(any(feature = "api_level2", feature = "api_level3"))]
    pub fn copy_attributes_from<SrcT: TeeObject + Initialized>(
        self,
        src_object: &TeeObjectHandle<SrcT>,
    ) -> Result<TeeObjectHandle<InitializedTransientObject>, (Self, PersistentObjectError)> {
        let res =
            // SAFETY: `self` is guaranteed to be an `UninitializedTransientObject`, and
            // `src_object` is guaranteed to be an initialized Object, so these two panic reasons
            // are statically prevented by the compiler
            unsafe { transient_object_ffi::TEE_CopyObjectAttributes1(self.handle, src_object.handle) };
        match res {
            // SAFETY: We trust TEE_CopyObjectAttributes1 to have properly initialized the handle.
            TeeResult::TEE_SUCCESS => Ok(unsafe { self.into_initialized() }),
            TeeResult::TEE_ERROR_CORRUPT_OBJECT => {
                Err((self, PersistentObjectError::CorruptObject))
            }
            TeeResult::TEE_ERROR_STORAGE_NOT_AVAILABLE => {
                Err((self, PersistentObjectError::StorageNotAvailable))
            }
            // Note: actually reachable on teeos............ Does not follow GPD specification
            // and does not panic for other errors.....
            _ => unreachable!("Invalid error code"),
        }
    }
}

pub enum TryFromObjectError {
    /// Error getting ObjectInfo from the source object
    StorageError(PersistentObjectError),
    Allocate(AllocateTransientObjectError),
}

#[cfg(any(feature = "api_level2", feature = "api_level3"))]
impl<T: TeeObject + Initialized> TryFrom<&TeeObjectHandle<T>>
    for TeeObjectHandle<InitializedTransientObject>
{
    type Error = TryFromObjectError;
    /// Try to create a new TeeObjectHandle based on an existing Object.
    fn try_from(src: &TeeObjectHandle<T>) -> Result<Self, Self::Error> {
        let obj_info = src
            .object_info()
            .map_err(TryFromObjectError::StorageError)?;
        let new_obj = TeeObjectHandle::<UninitializedTransientObject>::new(
            obj_info.object_type,
            obj_info.max_object_size,
        )
        .map_err(TryFromObjectError::Allocate)?;

        match new_obj.copy_attributes_from(src) {
            Ok(new_obj) => Ok(new_obj),
            Err((_new_obj, error)) => {
                Err(TryFromObjectError::StorageError(error))
                // implicitely drop `new_obj`
            }
        }
    }
}

impl<T: TeeObject + Initialized> TeeObjectHandle<T> {
    pub fn get_object_buffer_attribute(
        &self,
        attribute_id: TeeObjectAttribute,
        buffer: &mut [u8],
        size: &mut usize,
    ) -> crate::error::FfiResult {
        // However, calling
        // TEE_GetObjectBufferAttribute with a NULL buffer will trigger a TEE_ERROR_SHORT_BUFFER return
        // value (see section 3.4.4) and is guaranteed to return a size sufficient to hold the attribute.
        // -> Use alloc to allocate the buffer and correctly handle errors here.
        // Errors are well defined, other errors are a panic reason!
        unsafe {
            generic_object_ffi::TEE_GetObjectBufferAttribute(
                self.handle,
                attribute_id,
                buffer.as_mut_ptr(),
                size,
            )
        }
        .into()
    }

    pub fn get_object_value_attribute(
        &self,
        attribute_id: TeeObjectAttribute,
        a: &mut u32,
        b: &mut u32,
    ) -> crate::error::FfiResult {
        unsafe { generic_object_ffi::TEE_GetObjectValueAttribute(self.handle, attribute_id, a, b) }
            .into()
    }

    pub fn info_object_data(&self) -> Result<ObjectDataInfo, FfiTeeError> {
        let mut info = ObjectDataInfo::default();
        let res: FfiResult = unsafe {
            transient_object_ffi::TEE_InfoObjectData(self.handle, &mut info.pos, &mut info.len)
        }
        .into();
        res?;
        Ok(info)
    }
}

impl<'a, T: TeeObject + Initialized> TeeObjectHandle<T> {
    pub(crate) fn readonly_handle(&'a self) -> &'a __TeeObjectHandle {
        // SAFETY: We statically know that the Object handle has been initialized, so we can pass
        // out a reference as long as we borrow self.
        unsafe { &(*self.handle) }
    }
}

impl TeeObjectHandle<PersistentObject> {
    pub fn create<T: TeeObject + Initialized>(
        storage_id: ObjectStorageConstants,
        object_id: &core::ffi::CStr,
        flags: DataFlagConstants,
        attributes: Option<&TeeObjectHandle<T>>,
        initial_data: Option<&[u8]>,
    ) -> Result<TeeObjectHandle<PersistentObject>, FfiTeeError> {
        let mut object_handle: *mut __TeeObjectHandle = core::ptr::null_mut();
        let attr = match attributes {
            Some(x) => x.readonly_handle(),
            None => core::ptr::null(),
        };
        let mut init_data_len = 0;
        let init_data = match initial_data {
            Some(x) => {
                init_data_len = x.len();
                x.as_ptr()
            }
            None => core::ptr::null(),
        };
        let id = object_id.to_bytes();
        let res: FfiResult = unsafe {
            persistent_object_ffi::TEE_CreatePersistentObject(
                storage_id,
                id.as_ptr() as _,
                id.len(),
                flags,
                attr,
                init_data as _,
                init_data_len,
                &mut object_handle,
            )
        }
        .into();
        res?;
        let persistent_object: TeeObjectHandle<PersistentObject> = TeeObjectHandle {
            handle: object_handle,
            _ty: core::marker::PhantomData,
        };
        Ok(persistent_object)
    }
}

impl TeeObjectHandle<PersistentObject> {
    pub fn open(
        storage_id: ObjectStorageConstants,
        object_id: &core::ffi::CStr,
        flags: DataFlagConstants,
    ) -> Result<TeeObjectHandle<PersistentObject>, FfiTeeError> {
        let mut object_handle: *mut __TeeObjectHandle = core::ptr::null_mut();
        let id = object_id.to_bytes();
        let res: FfiResult = unsafe {
            persistent_object_ffi::TEE_OpenPersistentObject(
                storage_id,
                id.as_ptr() as _,
                id.len(),
                flags,
                &mut object_handle,
            )
        }
        .into();
        res?;
        let persistent_object: TeeObjectHandle<PersistentObject> = TeeObjectHandle {
            handle: object_handle,
            _ty: core::marker::PhantomData,
        };
        Ok(persistent_object)
    }

    pub fn delete(self) -> Result<(), (Self, FfiTeeError)> {
        let res: FfiResult =
            unsafe { persistent_object_ffi::TEE_CloseAndDeletePersistentObject1(self.handle) }
                .into();
        match res {
            Ok(()) => {
                // Safety: self handle has been released by `TEE_CloseAndDeletePersistentObject1`
                core::mem::forget(self);
                Ok(())
            }
            Err(e) => Err((self, e)),
        }
    }

    pub fn rename(&mut self, new_object_id: &core::ffi::CStr) -> FfiResult {
        let new_id = new_object_id.to_bytes();
        unsafe {
            persistent_object_ffi::TEE_RenamePersistentObject(
                self.handle,
                new_id.as_ptr() as _,
                new_id.len(),
            )
        }
        .into()
    }

    pub fn read(&self, buffer: &mut [u8]) -> Result<u32, FfiTeeError> {
        let mut count: u32 = 0;
        let res: FfiResult = unsafe {
            persistent_object_ffi::TEE_ReadObjectData(
                self.handle,
                buffer.as_mut_ptr() as _,
                buffer.len(),
                &mut count,
            )
        }
        .into();
        res?;
        Ok(count)
    }

    pub fn write(&mut self, buffer: &[u8]) -> FfiResult {
        unsafe {
            persistent_object_ffi::TEE_WriteObjectData(
                self.handle,
                buffer.as_ptr() as _,
                buffer.len(),
            )
        }
        .into()
    }

    pub fn truncate(&mut self, size: usize) -> FfiResult {
        unsafe { persistent_object_ffi::TEE_TruncateObjectData(self.handle, size) }.into()
    }

    pub fn seek(&self, offset: i32, whence: TeeWhence) -> FfiResult {
        unsafe { persistent_object_ffi::TEE_SeekObjectData(self.handle, offset, whence) }.into()
    }

    pub fn sync(&self) -> FfiResult {
        unsafe { persistent_object_ffi::TEE_SyncPersistentObject(self.handle) }.into()
    }
}

pub struct TeeObjectEnumHandle(*mut __TeeObjectEnumHandle);

impl TeeObjectEnumHandle {
    pub fn new() -> Result<Self, FfiTeeError> {
        let mut object_enumerator: *mut __TeeObjectEnumHandle = core::ptr::null_mut();
        let res: FfiResult = unsafe {
            persistent_object_ffi::TEE_AllocatePersistentObjectEnumerator(&mut object_enumerator)
        }
        .into();
        res?;
        Ok(TeeObjectEnumHandle(object_enumerator))
    }

    pub fn reset(&self) {
        unsafe {
            persistent_object_ffi::TEE_ResetPersistentObjectEnumerator(self.0);
        }
    }

    pub fn start(&self, storage_id: ObjectStorageConstants) -> FfiResult {
        unsafe { persistent_object_ffi::TEE_StartPersistentObjectEnumerator(self.0, storage_id) }
            .into()
    }

    /// # Return
    ///
    /// + `usize` next object id len
    pub fn next(
        &self,
        object_info: &mut TeeObjectInfo,
        object_id: &mut [u8],
    ) -> Result<usize, FfiTeeError> {
        let mut size: usize = object_id.len();
        let res: FfiResult = unsafe {
            persistent_object_ffi::TEE_GetNextPersistentObject(
                self.0,
                object_info,
                object_id.as_mut_ptr() as _,
                &mut size,
            )
        }
        .into();
        res?;
        Ok(size)
    }
}

impl Drop for TeeObjectEnumHandle {
    fn drop(&mut self) {
        unsafe { persistent_object_ffi::TEE_FreePersistentObjectEnumerator(self.0) }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct TeeObjectAttribute(u32);

impl BitAnd for TeeObjectAttribute {
    type Output = TeeObjectAttribute;

    fn bitand(self, Self(rhs): Self) -> Self::Output {
        let Self(lhs) = self;
        Self(lhs & rhs)
    }
}

impl TeeObjectAttribute {
    pub const SECRET_VALUE: Self = Self(0xC0000000);
    pub const RSA_MODULUS: Self = Self(0xD0000130);
    pub const RSA_PUBLIC_EXPONENT: Self = Self(0xD0000230);
    pub const RSA_PRIVATE_EXPONENT: Self = Self(0xC0000330);
    pub const RSA_PRIME1: Self = Self(0xC0000430);
    pub const RSA_PRIME2: Self = Self(0xC0000530);
    pub const RSA_EXPONENT1: Self = Self(0xC0000630);
    pub const RSA_EXPONENT2: Self = Self(0xC0000730);
    pub const RSA_COEFFICIENT: Self = Self(0xC0000830);
    pub const RSA_MGF1_HASH: Self = Self(0xF0000830);
    pub const DSA_PRIME: Self = Self(0xD0001031);
    pub const DSA_SUBPRIME: Self = Self(0xD0001131);
    pub const DSA_BASE: Self = Self(0xD0001231);
    pub const DSA_PUBLIC_VALUE: Self = Self(0xD0000131);
    pub const DSA_PRIVATE_VALUE: Self = Self(0xC0000231);
    pub const DH_PRIME: Self = Self(0xD0001032);
    pub const DH_SUBPRIME: Self = Self(0xD0001132);
    pub const DH_BASE: Self = Self(0xD0001232);
    pub const DH_X_BITS: Self = Self(0xF0001332);
    pub const DH_PUBLIC_VALUE: Self = Self(0xD0000132);
    pub const DH_PRIVATE_VALUE: Self = Self(0xC0000232);
    pub const RSA_OAEP_LABEL: Self = Self(0xD0000930);
    pub const RSA_PSS_SALT_LENGTH: Self = Self(0xF0000A30);
    pub const ECC_PUBLIC_VALUE_X: Self = Self(0xD0000141);
    pub const ECC_PUBLIC_VALUE_Y: Self = Self(0xD0000241);
    pub const ECC_PRIVATE_VALUE: Self = Self(0xC0000341);
    pub const ECC_CURVE: Self = Self(0xF0000441);
    pub const ED25519_CTX: Self = Self(0xD0000643);
    pub const ED25519_PUBLIC_VALUE: Self = Self(0xD0000743);
    pub const ED25519_PRIVATE_VALUE: Self = Self(0xC0000843);
    pub const ED25519_PH: Self = Self(0xF0000543);
    pub const X25519_PUBLIC_VALUE: Self = Self(0xD0000944);
    pub const X25519_PRIVATE_VALUE: Self = Self(0xC0000A44);
    pub const PBKDF2_HMAC_PASSWORD: Self = Self(0xD0000133);
    pub const PBKDF2_HMAC_SALT: Self = Self(0xD0000134);
    pub const PBKDF2_HMAC_DIGEST: Self = Self(0xF0000135);

    const FLAG_VALUE: Self = Self(0x20000000);
    const FLAG_PUBLIC: Self = Self(0x10000000);

    pub fn new(is_value: bool, is_public: bool) -> Self {
        let mut value = 0_u32;
        if is_value {
            value |= Self::FLAG_VALUE.0;
        }
        if is_public {
            value |= Self::FLAG_PUBLIC.0;
        }
        Self(value)
    }

    pub fn is_buffer(&self) -> bool {
        !self.is_value()
    }

    pub fn is_value(&self) -> bool {
        (*self & Self::FLAG_VALUE).0 != 0
    }

    pub fn is_protected(&self) -> bool {
        !self.is_public()
    }

    pub fn is_public(&self) -> bool {
        (*self & Self::FLAG_PUBLIC).0 != 0
    }
}

#[repr(C)]
#[derive(Default)]
pub struct TeeAttribute {
    pub attribute_id: TeeObjectAttribute,
    pub content: TeeAttributeContent,
}

#[repr(C)]
pub union TeeAttributeContent {
    pub ref_: ManuallyDrop<TeeAttributeContentRef>,
    pub value: TeeAttributeContentValue,
}

impl Default for TeeAttributeContent {
    fn default() -> Self {
        TeeAttributeContent {
            value: TeeAttributeContentValue { a: 0, b: 0 },
        }
    }
}
#[repr(C)]
pub struct TeeAttributeContentRef {
    /// `buffer` is read only.
    pub buffer: *const core::ffi::c_void,
    pub length: usize,
}
impl Default for TeeAttributeContentRef {
    fn default() -> Self {
        TeeAttributeContentRef {
            buffer: core::ptr::null(),
            length: 0,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct TeeAttributeContentValue {
    pub a: u32,
    pub b: u32,
}

impl TeeAttribute {
    pub fn init_ref_attribute(&mut self, attribute_id: TeeObjectAttribute, buffer: &[u8]) {
        unsafe {
            transient_object_ffi::TEE_InitRefAttribute(
                self,
                attribute_id,
                buffer.as_ptr(),
                buffer.len(),
            );
        }
    }
    pub fn init_value_attribute(&mut self, attribute_id: TeeObjectAttribute, a: u32, b: u32) {
        unsafe {
            transient_object_ffi::TEE_InitValueAttribute(self, attribute_id, a, b);
        }
    }
}
