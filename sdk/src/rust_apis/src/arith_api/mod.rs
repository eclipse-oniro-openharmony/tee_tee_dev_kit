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
mod arith_api_ffi;

use core::mem::size_of;

use crate::{
    core::{TEE_Free, TEE_Malloc},
    error::{FfiResult, TeeError},
};

pub use arith_api_ffi::*;

#[repr(transparent)]
#[derive(Eq, PartialEq)]
pub struct ProbablePrimeResult(pub i32);
impl ProbablePrimeResult {
    pub const COMPOSITE_NUMBER: Self = Self(0);
    pub const PRIME: Self = Self(1);
    pub const NON_CONCLUSIVE: Self = Self(-1);
    pub const UNKNOWN: Self = Self(-2);
}

impl From<i32> for ProbablePrimeResult {
    fn from(f: i32) -> Self {
        match f {
            0 => Self::COMPOSITE_NUMBER,
            1 => Self::PRIME,
            -1 => Self::NON_CONCLUSIVE,
            _ => Self::UNKNOWN,
        }
    }
}

impl From<ProbablePrimeResult> for i32 {
    fn from(f: ProbablePrimeResult) -> Self {
        f.0
    }
}

#[repr(transparent)]
pub struct TeeBigInt {
    handle: *mut u8,
    _p: core::marker::PhantomData<*mut u8>,
}

#[repr(transparent)]
pub struct TeeBigIntFMM {
    handle: *mut u8,
    _p: core::marker::PhantomData<*mut u8>,
}

#[repr(transparent)]
pub struct TeeBigIntFMMContext {
    handle: *mut u8,
    _p: core::marker::PhantomData<*mut u8>,
}

pub const METADATA_SIZE_IN_U32: usize = 2;

#[macro_export]
macro_rules! bigint_size_in_u32 {
    ($n:expr) => {
        (((($n) + 31) / 32) + 2)
    };
}

#[macro_export]
macro_rules! fmm_size_in_u32 {
    ($n:expr) => {
        (((($n) + 31) / 32) + 2)
    };
}

#[macro_export]
macro_rules! fmmcontext_size_in_u32 {
    ($n:expr) => {
        (((($n) + 31) / 32) + 2)
    };
}

impl TeeBigInt {
    ///
    /// initializes big_int
    ///
    /// ### Params
    /// - N(IN) The size in uint32_t of the memory pointed to by big_int and metadata(8byte,2*u32)
    ///
    /// ### Return
    /// - big_int
    ///
    pub fn init(n: usize) -> Result<TeeBigInt, TeeError> {
        assert!(n > METADATA_SIZE_IN_U32);
        assert!(n <= isize::MAX as usize);
        let mut big_int = TeeBigInt {
            handle: core::ptr::null_mut(),
            _p: core::marker::PhantomData,
        };
        let buffer = unsafe { TEE_Malloc(n * size_of::<u32>(), 0) };
        if buffer.is_null() {
            return Err(TeeError::OutOfMemory);
        }
        big_int.handle = buffer as _;
        unsafe {
            arith_api_ffi::TEE_BigIntInit(
                buffer,
                n.try_into().expect("we sure that will not panic"),
            );
        }
        Ok(big_int)
    }

    ///
    /// returns the size of the array of uint32_t values
    ///
    /// ### Params
    /// - modulus_size_in_bits (IN)  Size of modulus in bits
    ///
    /// ### Return
    /// - Number of bytes needed to store a TeeBigIntFMM given a modulus of length modulus_size_in_bits
    ///
    pub fn fmmsize_in_u32(modulus_size_in_bits: isize) -> isize {
        unsafe { arith_api_ffi::TEE_BigIntFMMSizeInU32(modulus_size_in_bits) }
    }

    ///
    /// returns the size of the array of uint32_t values needed to represent a fast modular context
    ///
    /// ### Params
    /// - modulus_size_in_bits (IN)  Size of modulus in bits
    ///
    /// ### Return
    /// - Number of bytes needed to store a TeeBigIntFMMContext given a modulus of length modulus_size_in_bits
    ///
    pub fn fmmcontext_size_in_u32(modulus_size_in_bits: isize) -> isize {
        unsafe { arith_api_ffi::TEE_BigIntFMMContextSizeInU32(modulus_size_in_bits) }
    }

    ///
    /// converts a buffer_len byte octet string buffer into a TeeBigInt format.
    ///
    /// ### Params
    /// - dest (OUT) Pointer to a TeeBigInt to hold the result
    /// - buffer (IN) Pointer to the buffer containing the octet string representation of the integer
    /// - sign (IN) The sign of dest is set to the sign of sign
    ///
    /// ### Return
    /// - TEE_SUCCESS
    /// - TEE_ERROR_OVERFLOW: If memory allocation for the dest is too small
    ///
    pub fn convert_from_octet_string(dest: &mut TeeBigInt, buffer: &[u8], sign: i32) -> FfiResult {
        unsafe {
            arith_api_ffi::TEE_BigIntConvertFromOctetString(
                dest.handle as _,
                buffer.as_ptr() as _,
                buffer.len() as isize,
                sign,
            )
        }
        .into()
    }

    ///
    /// converts the absolute value of an integer in TeeBigInt format into an octet string
    ///
    /// ### Params
    /// - self (IN)  Pointer to the integer that will be converted to an octet string
    /// - buffer (OUT)  Output buffer where converted octet string representation of the integer is written
    /// - buffer_len (IN)  The length of *buffer in bytes
    ///
    /// ### Return
    /// - TEE_SUCCESS: support
    /// - TEE_ERROR_SHORT_BUFFER: If the output buffer is too small to contain the octet string
    ///
    pub fn convert_to_octet_string(&self, buffer: &mut [u8], buffer_len: &mut isize) -> FfiResult {
        unsafe {
            arith_api_ffi::TEE_BigIntConvertToOctetString(
                buffer.as_mut_ptr() as _,
                buffer_len,
                self.handle as _,
            )
        }
        .into()
    }

    ///
    /// sets *dest to the value short_val
    ///
    /// ### Params
    /// - dest (OUT) Pointer to a TeeBigInt to store the result
    /// - short_val (IN) Input value
    ///
    /// ### Return
    /// - void
    ///
    pub fn convert_from_s32(dest: &mut TeeBigInt, short_val: i32) {
        unsafe {
            arith_api_ffi::TEE_BigIntConvertFromS32(dest.handle as _, short_val);
        }
    }

    ///
    /// sets *dest to the value of src, including the sign of src.
    ///
    /// ### Params
    /// - self (IN) Pointer to the input value
    ///
    /// ### Return
    /// - dest (OUT) Pointer to an int32_t to store the result
    /// - TEE_ERROR_OVERFLOW: If src does not fit within an int32_t
    ///
    pub fn convert_to_s32(&self) -> Result<i32, TeeError> {
        let mut dest: i32 = 0;
        let res: FfiResult =
            unsafe { arith_api_ffi::TEE_BigIntConvertToS32(&mut dest, self.handle as _) }.into();
        match res {
            Err(e) => {
                let ec: TeeError = match e.try_into() {
                    Err(_e) => TeeError::Generic,
                    Ok(o) => o,
                };
                Err(ec)
            }
            Ok(_) => Ok(dest),
        }
    }

    ///
    /// checks whether op1>op2, op1==op2, or op1<op2
    ///
    /// ### Params
    /// - self (IN) Pointer to the first operand
    /// - op2 (IN) Pointer to the second operand
    ///
    /// ### Return
    /// - 0 if op1==op2
    /// - a positive number if op1>op2
    ///
    pub fn compare(&self, op2: &TeeBigInt) -> i32 {
        unsafe { arith_api_ffi::TEE_BigIntCmp(self.handle as _, op2.handle as _) }
    }

    ///
    /// checks whether op>short_val, op==short_val, or op<short_val
    ///
    /// ### Params
    /// - self (IN) Pointer to the first operand
    /// - short_val (IN) Pointer to the second operand
    ///
    /// ### Return
    /// - 0 if op1==short_val
    /// - a positive number if op1>short_val
    ///
    pub fn cmp_s32(&self, short_val: i32) -> i32 {
        unsafe { arith_api_ffi::TEE_BigIntCmpS32(self.handle as _, short_val) }
    }

    ///
    /// returns the bitIndexth bit of the natural binary representation of |src|
    ///
    /// ### Params
    /// - self (IN) Pointer to the integer
    /// - bit_index(IN) The offset of the bit to be read, starting at offset 0 for the least significant bit
    ///
    /// ### Return
    /// - true The Boolean value of the bitIndexth bit in |src| is '1'
    /// - false The Boolean value of the bitIndexth bit in |src| is '0'
    ///
    pub fn get_bit(&self, bit_index: u32) -> bool {
        unsafe { arith_api_ffi::TEE_BigIntGetBit(self.handle as _, bit_index) }
    }

    ///
    /// returns the number of bits in the natural binary representation of |src|; that is, the magnitude of src
    ///
    /// ### Params
    /// - self (IN) Pointer to the integer
    ///
    /// ### Return
    /// - 0 src equals zero
    /// - others The number of bits in the natural binary representation of |src|.
    ///
    pub fn get_bit_count(&self) -> u32 {
        unsafe { arith_api_ffi::TEE_BigIntGetBitCount(self.handle as _) }
    }

    ///
    /// sets the bitIndexth bit of the natural binary representation of |op| to 1 or 0
    ///
    /// ### Params
    /// - self (IN/OUT) Pointer to the integer
    /// - bit_index (IN) The offset of the bit to be set, starting at offset 0 for the least significant bit
    /// - value (IN) The bit value to set where true represents a '1' and false represents a '0'
    ///
    /// ### Return
    /// - TEE_SUCCESS: support
    /// - TEE_ERROR_OVERFLOW: If the bitIndexth bit is larger than allocated bit length of op
    ///
    #[cfg(feature = "api_level3")]
    pub fn set_bit(&mut self, bit_index: u32, value: bool) -> FfiResult {
        unsafe { arith_api_ffi::TEE_BigIntSetBit(self.handle as _, bit_index, value) }.into()
    }

    ///
    /// assigns the value of src to dest
    ///
    /// ### Params
    /// - self (IN) Pointer to the source operand
    /// - dest (OUT) Pointer to TeeBigInt to be assigned
    ///
    /// ### Return
    /// - TEE_SUCCESS: support
    /// - TEE_ERROR_OVERFLOW: In case the dest operand cannot hold the value of src
    ///
    #[cfg(feature = "api_level3")]
    pub fn assign(&self, dest: &mut TeeBigInt) -> FfiResult {
        unsafe { arith_api_ffi::TEE_BigIntAssign(dest.handle as _, self.handle as _) }.into()
    }

    ///
    /// computes |dest| = |op| >> bits
    ///
    /// ### Params
    /// - self (IN) Pointer to the operand to be shifted
    /// - dest (OUT) Pointer to TeeBigInt to hold the shifted result
    /// - bits (IN) Number of bits to shift
    ///
    /// ### Return
    /// - void
    ///
    pub fn shift_right(&self, dest: &mut TeeBigInt, bits: isize) {
        unsafe {
            arith_api_ffi::TEE_BigIntShiftRight(dest.handle as _, self.handle as _, bits);
        }
    }

    ///
    /// abs the value of |src| to dest
    ///
    /// ### Params
    /// - dest (OUT) the absolute of self
    /// - src (IN) Pointer to the source operand
    ///
    /// ### Return
    /// - TEE_SUCCESS
    /// - TEE_ERROR_OVERFLOW: In case the dest operand cannot hold the value of |src|
    ///
    #[cfg(feature = "api_level3")]
    pub fn abs(dest: &mut TeeBigInt, src: &TeeBigInt) -> FfiResult {
        unsafe { arith_api_ffi::TEE_BigIntAbs(dest.handle as _, src.handle as _) }.into()
    }

    ///
    /// computes dest = op1 + op2
    ///
    /// ### Params
    /// - dest (OUT) Pointer to TeeBigInt to store the result op1 + op2
    /// - src (IN) Pointer to the first operand
    /// - op2 (IN) Pointer to the second operand
    ///
    /// ### Return
    /// - void
    ///
    pub fn add(dest: &mut TeeBigInt, op1: &TeeBigInt, op2: &TeeBigInt) {
        unsafe { arith_api_ffi::TEE_BigIntAdd(dest.handle as _, op1.handle as _, op2.handle as _) }
    }

    ///
    /// computes dest = op1 - op2
    ///
    /// ### Params
    /// - dest (OUT) Pointer to TeeBigInt to store the result op1 - op2
    /// - op1 (IN) Pointer to the first operand
    /// - op2 (IN) Pointer to the second operand
    ///
    /// ### Return
    /// - void
    ///
    pub fn sub(dest: &mut TeeBigInt, op1: &TeeBigInt, op2: &TeeBigInt) {
        unsafe {
            arith_api_ffi::TEE_BigIntSub(dest.handle as _, op1.handle as _, op2.handle as _);
        }
    }

    ///
    /// negates an operand: dest = -op
    ///
    /// ### Params
    /// - dest (OUT) PPointer to TeeBigInt to store the result -op
    /// - op (IN) Pointer to the operand to be negated
    ///
    /// ### Return
    /// - void
    ///
    pub fn neg(dest: &mut TeeBigInt, op: &TeeBigInt) {
        unsafe {
            arith_api_ffi::TEE_BigIntNeg(dest.handle as _, op.handle as _);
        }
    }

    ///
    /// computes dest = op1 * op2
    ///
    /// ### Params
    /// - dest (OUT) Pointer to TeeBigInt to store the result op1 * op2
    /// - op1 (IN) Pointer to the first operand
    /// - op2 (IN) Pointer to the second operand
    ///
    /// ### Return
    /// - void
    ///
    pub fn mul(dest: &mut TeeBigInt, op1: &TeeBigInt, op2: &TeeBigInt) {
        unsafe {
            arith_api_ffi::TEE_BigIntMul(dest.handle as _, op1.handle as _, op2.handle as _);
        }
    }

    ///
    /// computes dest = op * op
    ///
    /// ### Params
    /// - dest (OUT) Pointer to TeeBigInt to store the result op * op
    /// - op (IN) Pointer to the operand to be squared
    ///
    /// ### Return
    /// - void
    ///
    pub fn square(dest: &mut TeeBigInt, op: &TeeBigInt) {
        unsafe {
            arith_api_ffi::TEE_BigIntSquare(dest.handle as _, op.handle as _);
        }
    }

    ///
    /// computes dest_r and dest_q such that op1 = dest_q * op2 + dest_r
    ///
    /// ### Params
    /// - dest_q (OUT) Pointer to a TeeBigInt to store the quotient
    /// - dest_r (IN) Pointer to a TeeBigInt to store the remainder
    /// - op1 (OUT) Pointer to the first operand, the dividend
    /// - op2 (IN) Pointer to the second operand, the divisor
    ///
    /// ### Return
    /// - void
    ///
    pub fn div(dest_q: &mut TeeBigInt, dest_r: &mut TeeBigInt, op1: &TeeBigInt, op2: &TeeBigInt) {
        unsafe {
            arith_api_ffi::TEE_BigIntDiv(
                dest_q.handle as _,
                dest_r.handle as _,
                op1.handle as _,
                op2.handle as _,
            );
        }
    }

    ///
    /// computes dest = op (mod n) such that 0 <= dest < n.
    ///
    /// ### Params
    /// - dest (OUT) Pointer to TeeBigInt to hold the result op (mod n)
    /// - op (IN) Pointer to the operand to be reduced mod n
    /// - n (IN) Pointer to the modulus. Modulus SHALL be larger than 1.
    ///
    /// ### Return
    /// - void
    ///
    pub fn mod_n(dest: &mut TeeBigInt, op: &TeeBigInt, n: &TeeBigInt) {
        unsafe {
            arith_api_ffi::TEE_BigIntMod(dest.handle as _, op.handle as _, n.handle as _);
        }
    }

    ///
    /// computes dest = (op1 + op2) (mod n).
    ///
    /// ### Params
    /// - dest (OUT) Pointer to TeeBigInt to hold the result (op1 + op2)(mod n)
    /// - op1 (IN) Pointer to the first operand
    /// - op2 (IN) Pointer to the second operand
    /// - n (IN) Pointer to the modulus. Modulus SHALL be larger than 1
    ///
    /// ### Return
    /// - void
    ///
    pub fn add_mod(dest: &mut TeeBigInt, op1: &TeeBigInt, op2: &TeeBigInt, n: &TeeBigInt) {
        unsafe {
            arith_api_ffi::TEE_BigIntAddMod(
                dest.handle as _,
                op1.handle as _,
                op2.handle as _,
                n.handle as _,
            );
        }
    }

    ///
    /// computes dest = (op1 - op2) (mod n).
    ///
    /// ### Params
    /// - dest (OUT) Pointer to TeeBigInt to hold the result (op1 - op2)(mod n)
    /// - op1 (IN) Pointer to the first operand
    /// - op2 (IN) Pointer to the second operand
    /// - n (IN) Pointer to the modulus. Modulus SHALL be larger than 1
    ///
    /// ### Return
    /// - void
    ///
    pub fn sub_mod(dest: &mut TeeBigInt, op1: &TeeBigInt, op2: &TeeBigInt, n: &TeeBigInt) {
        unsafe {
            arith_api_ffi::TEE_BigIntSubMod(
                dest.handle as _,
                op1.handle as _,
                op2.handle as _,
                n.handle as _,
            );
        }
    }

    ///
    /// computes dest = (op1 * op2) (mod n).
    ///
    /// ### Params
    /// - dest (OUT) Pointer to TeeBigInt to hold the result (op1 * op2)(mod n)
    /// - op1 (IN) Pointer to the first operand
    /// - op2 (IN) Pointer to the second operand
    /// - n (IN) Pointer to the modulus. Modulus SHALL be larger than 1
    ///
    /// ### Return
    /// - void
    ///
    pub fn mul_mod(dest: &mut TeeBigInt, op1: &TeeBigInt, op2: &TeeBigInt, n: &TeeBigInt) {
        unsafe {
            arith_api_ffi::TEE_BigIntMulMod(
                dest.handle as _,
                op1.handle as _,
                op2.handle as _,
                n.handle as _,
            );
        }
    }

    ///
    /// computes dest = (op * op) (mod n).
    ///
    /// ### Params
    /// - dest (OUT) Pointer to TeeBigInt to hold the result (op * op)(mod n)
    /// - op (IN) Pointer to the operand
    /// - n (IN) Pointer to the modulus. Modulus SHALL be larger than 1
    ///
    /// ### Return
    /// - void
    ///
    pub fn square_mod(dest: &mut TeeBigInt, op: &TeeBigInt, n: &TeeBigInt) {
        unsafe {
            arith_api_ffi::TEE_BigIntSquareMod(dest.handle as _, op.handle as _, n.handle as _);
        }
    }

    ///
    /// computes dest such that dest * op = 1 (mod n).
    ///
    /// ### Params
    /// - dest (OUT) Pointer to TEE_BigInt to hold the result (op^-1)(mod n)
    /// - op (IN) Pointer to the operand
    /// - n (IN) Pointer to the modulus. Modulus SHALL be larger than 1
    ///
    /// ### Return
    /// - void
    ///
    pub fn inv_mod(dest: &mut TeeBigInt, op: &TeeBigInt, n: &TeeBigInt) {
        unsafe {
            arith_api_ffi::TEE_BigIntInvMod(dest.handle as _, op.handle as _, n.handle as _);
        }
    }

    ///
    /// determines whether gcd(op1, op2) == 1.
    ///
    /// ### Params
    /// - op1 (IN) Pointer to the first operand
    /// - op2 (IN) Pointer to the second operand
    ///
    /// ### Return
    /// - true if gcd(op1, op2) == 1
    /// - false otherwise
    ///
    pub fn relative_prime(op1: &TeeBigInt, op2: &TeeBigInt) -> bool {
        unsafe { arith_api_ffi::TEE_BigIntRelativePrime(op1.handle as _, op2.handle as _) }
    }

    ///
    /// computes the greatest common divisor of the input parameters op1 and op2.
    ///
    /// ### Params
    /// - op1 (IN) Pointer to the first operand
    /// - op2 (IN) Pointer to the second operand
    ///
    /// ### Return
    /// - gcd (OUT) Pointer to TEE_BigInt to hold the greatest common divisor of op1 and op2
    /// - u (OUT) Pointer to TEE_BigInt to hold the first coefficient
    /// - v (OUT) Pointer to TEE_BigInt to hold the second coefficient
    ///
    pub fn compute_extended_gcd(
        gcd: &mut TeeBigInt,
        u: &mut TeeBigInt,
        v: &mut TeeBigInt,
        op1: &TeeBigInt,
        op2: &TeeBigInt,
    ) {
        unsafe {
            arith_api_ffi::TEE_BigIntComputeExtendedGcd(
                gcd.handle as _,
                u.handle as _,
                v.handle as _,
                op1.handle as _,
                op2.handle as _,
            );
        }
    }

    ///
    /// performs a probabilistic primality test on op
    ///
    /// ### Params
    /// - self (IN) Candidate number that is tested for primality
    /// - confidence_level (IN)  The desired confidence level for a non-conclusive test
    ///
    /// ### Return
    /// - 0 If op is a composite number
    /// - 1 If op is guaranteed to be prime
    /// - -1 If the test is non-conclusive but the probability that op is composite is less than 2^(-confidence_level)
    ///
    pub fn is_probable_prime(&self, confidence_level: u32) -> ProbablePrimeResult {
        unsafe { arith_api_ffi::TEE_BigIntIsProbablePrime(self.handle as _, confidence_level) }
            .into()
    }

    ///
    /// converts src into a representation suitable for doing fast modular multiplicatio
    ///
    /// ### Params
    /// - self (IN) Pointer to the TEE_BigInt to convert
    /// - dest (OUT) Pointer to an initialized TeeBigIntFMM memory area
    /// - n (IN) Pointer to the modulus
    /// - context (IN) Pointer to a context previously initialized using TEE_BigIntInitFMMContext1
    ///
    /// ### Return
    /// - void
    ///
    pub fn convert_to_fmm(
        &self,
        dest: &mut TeeBigIntFMM,
        n: &TeeBigInt,
        context: &TeeBigIntFMMContext,
    ) {
        unsafe {
            arith_api_ffi::TEE_BigIntConvertToFMM(
                dest.handle as _,
                self.handle as _,
                n.handle as _,
                context.handle as _,
            );
        }
    }

    ///
    /// converts src in the fast modular multiplication representation back to a TEE_BigInt representation
    ///
    /// ### Params
    /// - dest (OUT) Pointer to an initialized TeeBigIntFMM memory area to hold the converted result
    /// - src (IN) Pointer to a TeeBigIntFMM holding the value in the fast modular multiplication representation
    /// - n (IN) Pointer to the modulus
    /// - context (IN) Pointer to a context previously initialized using TEE_BigIntInitFMMContext1
    ///
    /// ### Return
    /// - void
    ///
    pub fn convert_from_fmm(
        dest: &mut TeeBigInt,
        src: &TeeBigIntFMM,
        n: &TeeBigInt,
        context: &TeeBigIntFMMContext,
    ) {
        unsafe {
            arith_api_ffi::TEE_BigIntConvertFromFMM(
                dest.handle as _,
                src.handle as _,
                n.handle as _,
                context.handle as _,
            );
        }
    }

    ///
    /// calculates dest = op1 * op2 in the fast modular multiplication representation
    ///
    /// ### Params
    /// - dest (OUT) Pointer to TeeBigIntFMM to hold the result op1 * op2
    /// - op1 (IN) Pointer to the first operand
    /// - op2 (IN) Pointer to the second operand
    /// - n (IN) Pointer to the modulus
    /// - context (IN) Pointer to a context previously initialized using TEE_BigIntInitFMMContext1
    ///
    /// ### Return
    /// - void
    ///
    pub fn compute_fmm(
        dest: &mut TeeBigIntFMM,
        op1: &TeeBigIntFMM,
        op2: &TeeBigIntFMM,
        n: &TeeBigInt,
        context: &TeeBigIntFMMContext,
    ) {
        unsafe {
            arith_api_ffi::TEE_BigIntComputeFMM(
                dest.handle as _,
                op1.handle as _,
                op2.handle as _,
                n.handle as _,
                context.handle as _,
            );
        }
    }

    ///
    /// computes dest = (op1 ^ op2) (mod n).
    ///
    /// ### Params
    /// - dest (OUT) Pointer to TEE_BigInt to hold the result (op1 ^ op2)(mod n)
    /// - op1 (IN) Pointer to the first operand
    /// - op2 (IN) Pointer to the second operand
    /// - n (IN) Pointer to the modulus
    /// - context (IN) Pointer to a context previously initialized using TEE_BigIntInitFMMContext1 or NULL
    ///
    /// ### Return
    /// - TEE_SUCCESS
    /// - TEE_ERROR_NOT_SUPPORTED: If the value of n is not supported
    ///
    #[cfg(any(feature = "api_level2", feature = "api_level3"))]
    pub fn exp_mod(
        dest: &mut TeeBigInt,
        op1: &TeeBigInt,
        op2: &TeeBigInt,
        n: &TeeBigInt,
        context: &TeeBigIntFMMContext,
    ) -> FfiResult {
        unsafe {
            arith_api_ffi::TEE_BigIntExpMod(
                dest.handle as _,
                op1.handle as _,
                op2.handle as _,
                n.handle as _,
                context.handle as _,
            )
        }
        .into()
    }

    ///
    /// check whether n exists to make dest = (op1 ^ op2) (mod n).
    ///
    /// ### Params
    /// - dest (OUT) Pointer to TEE_BigInt to hold the result (op1 ^ op2)(mod n)
    /// - in_int (IN) Pointer to the first operand
    /// - exp (IN) Pointer to the second operand
    /// - n (IN) Pointer to the modulus
    /// - context (IN) Pointer to a context previously initialized using TEE_BigIntInitFMMContext1 or NULL
    ///
    /// ### Return
    /// - true If the value of n is supported
    /// - false If the value of n is not supported
    ///
    pub fn ext_exp_mod(
        dest: &mut TeeBigInt,
        in_int: &TeeBigInt,
        exp: &TeeBigInt,
        n: &TeeBigInt,
    ) -> bool {
        unsafe {
            arith_api_ffi::EXT_TEE_BigIntExpMod(
                dest.handle as _,
                in_int.handle as _,
                exp.handle as _,
                n.handle as _,
            )
        }
    }
}

impl TeeBigIntFMM {
    ///
    /// initializes bigIntFMM and sets its represented value to zero.
    ///
    /// ### Params
    /// - Y(IN) The size in uint32_t of the memory pointed to by bigIntFMM, if len < 2, will panic.
    ///
    /// ### Return
    /// - big_int_fmm(OUT) the TeeBigIntFMM to be initialized
    ///
    pub fn init(y: usize) -> Result<TeeBigIntFMM, TeeError> {
        assert!(y >= METADATA_SIZE_IN_U32);
        assert!(y <= isize::MAX as usize);
        let mut big_int_fmm = TeeBigIntFMM {
            handle: core::ptr::null_mut(),
            _p: core::marker::PhantomData,
        };
        let buffer = unsafe { TEE_Malloc(y * size_of::<u32>(), 0) };
        if buffer.is_null() {
            return Err(TeeError::OutOfMemory);
        }
        big_int_fmm.handle = buffer as _;
        unsafe {
            arith_api_ffi::TEE_BigIntInitFMM(
                big_int_fmm.handle as _,
                y.try_into().expect("we sure that will not panic"),
            );
        }
        Ok(big_int_fmm)
    }
}

impl TeeBigIntFMMContext {
    ///
    /// calculates the necessary prerequisites for the fast modular multiplication and stores them in a context.
    ///
    /// ### Params
    /// - modulus(IN) The modulus
    /// - X(IN) The size in uint32_t of the memory pointed to by context, if len < 2, will panic.
    ///
    /// ### Return
    /// - context(OUT) the TeeBigIntFMMContext to be initialized
    ///
    pub fn init(x: usize, modulus: &TeeBigInt) -> Result<TeeBigIntFMMContext, TeeError> {
        assert!(x >= METADATA_SIZE_IN_U32);
        assert!(x <= isize::MAX as usize);
        let mut context = TeeBigIntFMMContext {
            handle: core::ptr::null_mut(),
            _p: core::marker::PhantomData,
        };
        let buffer = unsafe { TEE_Malloc(x * size_of::<u32>(), 0) };
        if buffer.is_null() {
            return Err(TeeError::OutOfMemory);
        }
        context.handle = buffer as _;
        unsafe {
            arith_api_ffi::TEE_BigIntInitFMMContext(
                context.handle as _,
                x.try_into().expect("we sure that will not panic"),
                modulus.handle as _,
            );
        }
        Ok(context)
    }

    ///
    /// calculates the necessary prerequisites for the fast modular multiplication and stores them in a context.
    ///
    /// ### Params
    /// - modulus(IN) The modulus
    /// - context(OUT) the TeeBigIntFMMContext to be initialized
    /// - X(IN) The size in uint32_t of the memory pointed to by context, if len < 2, will panic.
    ///
    /// ### Return
    /// - TEE_SUCCESS: success
    /// - other failed
    ///
    #[cfg(any(feature = "api_level2", feature = "api_level3"))]
    pub fn init1(x: usize, modulus: &TeeBigInt) -> Result<TeeBigIntFMMContext, TeeError> {
        let mut context = TeeBigIntFMMContext {
            handle: core::ptr::null_mut(),
            _p: core::marker::PhantomData,
        };
        let buffer = unsafe { TEE_Malloc(x * size_of::<u32>(), 0) };
        if buffer.is_null() {
            return Err(TeeError::OutOfMemory);
        }
        context.handle = buffer as _;

        let res: FfiResult = unsafe {
            arith_api_ffi::TEE_BigIntInitFMMContext1(
                context.handle as _,
                x.try_into().expect("we sure that will not panic"),
                modulus.handle as _,
            )
        }
        .into();
        match res {
            Err(e) => {
                let ec: TeeError = match e.try_into() {
                    Err(_e) => TeeError::Generic,
                    Ok(oe) => oe,
                };
                Err(ec)
            }
            Ok(_) => Ok(context),
        }
    }
}

impl Default for TeeBigIntFMMContext {
    fn default() -> Self {
        TeeBigIntFMMContext {
            handle: core::ptr::null_mut(),
            _p: core::marker::PhantomData,
        }
    }
}

impl Drop for TeeBigInt {
    fn drop(&mut self) {
        unsafe {
            TEE_Free(self.handle as _);
        }
    }
}

impl Drop for TeeBigIntFMM {
    fn drop(&mut self) {
        unsafe {
            TEE_Free(self.handle as _);
        }
    }
}

impl Drop for TeeBigIntFMMContext {
    fn drop(&mut self) {
        unsafe {
            TEE_Free(self.handle as _);
        }
    }
}
