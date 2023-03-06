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
use core::num::NonZeroU32;

use crate::{
    error::TeeError,
    tee_defines::{
        TeeParam, TeeParamType, TeeParamTypes, TeeParamTypesError, TeeParamValue, PARAM_COUNT,
    },
};

pub enum ParamError {
    TypeError(TeeParamTypesError),
    /// Not all types where None and the parameter array was a Null pointer.
    NullPtr,
    /// The size of `T` expected by the TA and the size provided by the caller do not match
    MemrefSizeMismatch,
}

impl From<TeeParamTypesError> for ParamError {
    fn from(e: TeeParamTypesError) -> Self {
        Self::TypeError(e)
    }
}

impl From<ParamError> for NonZeroU32 {
    fn from(_: ParamError) -> Self {
        TeeError::BadParameters.into()
    }
}

#[warn(dead_code)]
pub struct RustParameters<'a, T0, T1, T2, T3> {
    pub p0: RustParameter<'a, T0>,
    pub p1: RustParameter<'a, T1>,
    pub p2: RustParameter<'a, T2>,
    pub p3: RustParameter<'a, T3>,
}

impl<'a, T0, T1, T2, T3> RustParameters<'a, T0, T1, T2, T3> {
    /// Construct RustParameters from [TeeParamTypes] and [TeeParam]
    pub fn convert_from_tee_params(
        param_types: TeeParamTypes,
        tee_params: Option<&'a mut [TeeParam; PARAM_COUNT]>,
    ) -> Result<Self, ParamError> {
        // convert param_types to array of param_types
        let types: [TeeParamType; PARAM_COUNT] = param_types.try_convert_into_array()?;

        // If all parameter types are None, `tee_params` may be `None`.
        if types.iter().all(|t| *t == TeeParamType::NONE) {
            Ok(Self {
                p0: RustParameter::None,
                p1: RustParameter::None,
                p2: RustParameter::None,
                p3: RustParameter::None,
            })
        } else {
            let mut params = tee_params.ok_or(ParamError::NullPtr)?.chunks_mut(1);
            unsafe {
                Ok(Self {
                    // SAFETY: `params` should have exactly PARAM_COUNT (4) chunks of size 1, so we
                    // can safely unwrap. It should be possible to do this in a nicer way once
                    // `array_chunks_mut` stabilizes.
                    p0: RustParameter::<'a, T0>::convert_from_tee_param(
                        types[0],
                        &mut params.next().unwrap()[0],
                    )?,
                    p1: RustParameter::<'a, T1>::convert_from_tee_param(
                        types[1],
                        &mut params.next().unwrap()[0],
                    )?,
                    p2: RustParameter::<'a, T2>::convert_from_tee_param(
                        types[2],
                        &mut params.next().unwrap()[0],
                    )?,
                    p3: RustParameter::<'a, T3>::convert_from_tee_param(
                        types[3],
                        &mut params.next().unwrap()[0],
                    )?,
                })
            }
        }
    }
}

pub struct MemrefSlice<'a, T> {
    slice: &'a mut [T],
    // memref: &'a mut TeeParamMemref,
    // phantom: PhantomData<&'a mut T>,
    byte_len: &'a mut usize,
}

impl<'a, T> MemrefSlice<'a, T> {
    /// return `slice` mut in `MemrefSlice`
    pub fn get_slice_mut(&mut self) -> &mut [T] {
        let len = (*self.byte_len) / (core::mem::size_of::<T>());
        &mut self.slice[0..len]
    }

    /// return `slice` in `MemrefSlice`
    pub fn get_slice(&self) -> &[T] {
        let len = (*self.byte_len) / (core::mem::size_of::<T>());
        &self.slice[0..len]
    }

    /// modify length of the slice of T
    ///
    /// The `len` argument is the num of `elements`, not the num of bytes.
    /// `len` must be less than `slice.len`
    pub fn set_slice_len(&mut self, len: usize) -> Result<(), ParamError> {
        if len * core::mem::size_of::<T>() > *self.byte_len {
            Err(ParamError::MemrefSizeMismatch)
        } else {
            *self.byte_len = len * core::mem::size_of::<T>();
            Ok(())
        }
    }
}

impl<'a, T> MemrefSlice<'a, T>
where
    T: Copy,
{
    pub fn copy_from_slice(&mut self, buf: &[T]) -> Result<(), ParamError> {
        if buf.len() > self.slice.len() {
            Err(ParamError::MemrefSizeMismatch)
        } else {
            self.slice
                .get_mut(0..buf.len())
                .ok_or(ParamError::MemrefSizeMismatch)?
                .copy_from_slice(buf);
            *self.byte_len = buf.len() * core::mem::size_of::<T>();
            Ok(())
        }
    }
}

pub enum RustParameter<'a, T> {
    /// GPD: TEE_PARAM_TYPE_NONE
    None,
    /// GPD: TEE_PARAM_TYPE_VALUE_INPUT
    ValueInput(&'a TeeParamValue),
    /// GPD: TEE_PARAM_TYPE_VALUE_OUTPUT
    ValueOutput(&'a mut TeeParamValue),
    /// GPD: TEE_PARAM_TYPE_VALUE_INOUT
    ValueInout(&'a mut TeeParamValue),
    /// teeos add
    NullBuffer,
    /// GPD: TEE_PARAM_TYPE_MEMREF_INPUT
    MemrefInput(&'a [T]),
    /// GPD: TEE_PARAM_TYPE_MEMREF_OUTPUT
    MemrefOutput(MemrefSlice<'a, T>),
    /// TEE_PARAM_TYPE_MEMREF_INOUT
    MemrefInout(MemrefSlice<'a, T>),
    /// ION Memory
    IonInput(&'a [T]),
    IonSglistInput(&'a [T]),
    /// shared memory
    MemrefSharedInout(MemrefSlice<'a, T>),
    /// reserved memory
    ResmemInput(&'a [T]),
    ResmemOutput(MemrefSlice<'a, T>),
    ResmemInout(MemrefSlice<'a, T>),
}

impl<'a, T: 'a> RustParameter<'a, T> {
    // unsafe fn transmute_raw_buffer(memref: &TeeParamMemref) -> Result<&[T], ParamError> {
    unsafe fn transmute_raw_buffer(
        buffer: *const core::ffi::c_void,
        size: usize,
    ) -> Result<&'a [T], ParamError> {
        if core::mem::size_of::<T>() == 0 || size % core::mem::size_of::<T>() != 0 {
            Err(ParamError::MemrefSizeMismatch)
        } else {
            Ok(unsafe {
                core::slice::from_raw_parts(buffer.cast::<T>(), size / core::mem::size_of::<T>())
            })
        }
    }

    /// Reinterpret the raw buffer pointer as a mutable reference to type T
    ///
    /// # Safety
    ///
    /// * `memref.buffer` must be a valid **non-null** pointer to mutable memory of type `T`.
    unsafe fn transmute_raw_buffer_mut(
        buffer: *mut core::ffi::c_void,
        size: usize,
    ) -> Result<&'a mut [T], ParamError> {
        if core::mem::size_of::<T>() == 0 || size % core::mem::size_of::<T>() != 0 {
            Err(ParamError::MemrefSizeMismatch)
        } else {
            Ok(unsafe {
                core::slice::from_raw_parts_mut(
                    buffer.cast::<T>(),
                    size / core::mem::size_of::<T>(),
                )
            })
        }
    }

    ///
    ///
    /// # Safety
    ///
    /// * `T` must match what is on the C-side
    /// * `p_type` must specify the correct type of `TeeParam`
    pub unsafe fn convert_from_tee_param(
        p_type: TeeParamType,
        param: &'a mut TeeParam,
    ) -> Result<Self, ParamError> {
        match p_type {
            TeeParamType::NONE => Ok(Self::None),
            TeeParamType::ValueInput => unsafe { Ok(Self::ValueInput(&param.value)) },
            TeeParamType::ValueOutput => unsafe { Ok(Self::ValueOutput(&mut param.value)) },
            TeeParamType::ValueInout => unsafe { Ok(Self::ValueInout(&mut param.value)) },
            // `Value*` types must come before this line. Memref types must be placed _after_ this
            // line, since we must first check if the buffer is null.
            // Double check if it may or not be the TAs responsibility to allocate memroy
            //       for an output type in some of the cases. Looking at the GPD specification,
            //       I don't think this is a concern, but Huawei internal usage may be different!
            // SAFETY: We already checked for `None` and `Value*` types, so the variant must be
            //         a memory reference.
            _x if unsafe { param.memref.buffer.is_null() } => {
                // SAFETY: buffer and size are valid, promised by teeos
                unsafe {
                    debug_assert_eq!(param.memref.size, 0, "Size of Nullbuffer must 0");
                }
                Ok(Self::NullBuffer)
            }
            TeeParamType::MemrefInput => unsafe {
                let buffer = param.memref.buffer;
                let size = param.memref.size;
                Ok(Self::MemrefInput(Self::transmute_raw_buffer(buffer, size)?))
            },
            TeeParamType::MemrefOutput => unsafe {
                let buffer = param.memref.buffer;
                let size = param.memref.size;
                let memref = MemrefSlice {
                    slice: Self::transmute_raw_buffer_mut(buffer, size)?,
                    byte_len: &mut param.memref.size,
                };
                Ok(Self::MemrefOutput(memref))
            },
            TeeParamType::MemrefInout => unsafe {
                let buffer = param.memref.buffer;
                let size = param.memref.size;
                let memref = MemrefSlice {
                    slice: Self::transmute_raw_buffer_mut(buffer, size)?,
                    byte_len: &mut param.memref.size,
                };
                Ok(Self::MemrefInout(memref))
            },
            TeeParamType::IonInput => unsafe {
                let buffer = param.memref.buffer;
                let size = param.memref.size;
                Ok(Self::IonInput(Self::transmute_raw_buffer(buffer, size)?))
            },
            TeeParamType::IonSglistInput => unsafe {
                let buffer = param.memref.buffer;
                let size = param.memref.size;
                Ok(Self::IonSglistInput(Self::transmute_raw_buffer(
                    buffer, size,
                )?))
            },
            TeeParamType::MemrefSharedInout => unsafe {
                let buffer = param.memref.buffer;
                let size = param.memref.size;
                let memref = MemrefSlice {
                    slice: Self::transmute_raw_buffer_mut(buffer, size)?,
                    byte_len: &mut param.memref.size,
                };
                Ok(Self::MemrefSharedInout(memref))
            },
            TeeParamType::ResmemInput => unsafe {
                let buffer = param.memref.buffer;
                let size = param.memref.size;
                Ok(Self::ResmemInput(Self::transmute_raw_buffer(buffer, size)?))
            },
            TeeParamType::ResmemOutput => unsafe {
                let buffer = param.memref.buffer;
                let size = param.memref.size;
                let memref = MemrefSlice {
                    slice: Self::transmute_raw_buffer_mut(buffer, size)?,
                    byte_len: &mut param.memref.size,
                };
                Ok(Self::ResmemOutput(memref))
            },
            TeeParamType::ResmemInout => unsafe {
                let buffer = param.memref.buffer;
                let size = param.memref.size;
                let memref = MemrefSlice {
                    slice: Self::transmute_raw_buffer_mut(buffer, size)?,
                    byte_len: &mut param.memref.size,
                };
                Ok(Self::ResmemInout(memref))
            },
        }
    }
}
