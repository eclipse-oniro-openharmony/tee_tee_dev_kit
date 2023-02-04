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

pub use crate::tee_defines::*;

extern "C" {
    pub fn TEE_BigIntFMMSizeInU32(modulus_size_in_bits: isize) -> isize;

    pub fn TEE_BigIntFMMContextSizeInU32(modulus_size_in_bits: isize) -> isize;

    pub fn TEE_BigIntInit(big_int: *mut c_void, len: isize);

    pub fn TEE_BigIntInitFMMContext(context: *mut c_void, len: isize, modulus: *const c_void);

    #[cfg(any(feature = "api_level2", feature = "api_level3"))]
    pub fn TEE_BigIntInitFMMContext1(
        context: *mut c_void,
        len: isize,
        modulus: *const c_void,
    ) -> TeeResult;

    pub fn TEE_BigIntInitFMM(big_int_fmm: *mut c_void, len: isize);

    pub fn TEE_BigIntConvertFromOctetString(
        dest: *mut c_void,
        buffer: *const c_void,
        buffer_len: isize,
        sign: i32,
    ) -> TeeResult;

    pub fn TEE_BigIntConvertToOctetString(
        buffer: *mut c_void,
        buffer_len: *mut isize,
        big_int: *const c_void,
    ) -> TeeResult;

    pub fn TEE_BigIntConvertFromS32(dest: *mut c_void, short_val: i32);

    pub fn TEE_BigIntConvertToS32(dest: *mut i32, src: *const c_void) -> TeeResult;

    pub fn TEE_BigIntCmp(op1: *const c_void, op2: *const c_void) -> i32;

    pub fn TEE_BigIntCmpS32(op: *const c_void, short_val: i32) -> i32;

    pub fn TEE_BigIntShiftRight(dest: *mut c_void, op: *const c_void, bits: isize);

    pub fn TEE_BigIntGetBit(src: *const c_void, bit_index: u32) -> bool;

    pub fn TEE_BigIntGetBitCount(src: *const c_void) -> u32;

    #[cfg(feature = "api_level3")]
    pub fn TEE_BigIntSetBit(op: *mut c_void, bit_index: u32, value: bool) -> TeeResult;

    #[cfg(feature = "api_level3")]
    pub fn TEE_BigIntAssign(dest: *mut c_void, src: *const c_void) -> TeeResult;

    #[cfg(feature = "api_level3")]
    pub fn TEE_BigIntAbs(dest: *mut c_void, src: *const c_void) -> TeeResult;

    pub fn TEE_BigIntAdd(dest: *mut c_void, op1: *const c_void, op2: *const c_void);

    pub fn TEE_BigIntSub(dest: *mut c_void, op1: *const c_void, op2: *const c_void);

    pub fn TEE_BigIntNeg(dest: *mut c_void, op: *const c_void);

    pub fn TEE_BigIntMul(dest: *mut c_void, op1: *const c_void, op2: *const c_void);

    pub fn TEE_BigIntSquare(dest: *mut c_void, op: *const c_void);

    pub fn TEE_BigIntDiv(
        dest_q: *mut c_void,
        dest_r: *mut c_void,
        op1: *const c_void,
        op2: *const c_void,
    );

    pub fn TEE_BigIntMod(dest: *mut c_void, op: *const c_void, n: *const c_void);

    pub fn TEE_BigIntAddMod(
        dest: *mut c_void,
        op1: *const c_void,
        op2: *const c_void,
        n: *const c_void,
    );

    pub fn TEE_BigIntSubMod(
        dest: *mut c_void,
        op1: *const c_void,
        op2: *const c_void,
        n: *const c_void,
    );

    pub fn TEE_BigIntMulMod(
        dest: *mut c_void,
        op1: *const c_void,
        op2: *const c_void,
        n: *const c_void,
    );

    pub fn TEE_BigIntSquareMod(dest: *mut c_void, op: *const c_void, n: *const c_void);

    pub fn TEE_BigIntInvMod(dest: *mut c_void, op: *const c_void, n: *const c_void);

    pub fn TEE_BigIntRelativePrime(op1: *const c_void, op2: *const c_void) -> bool;

    pub fn TEE_BigIntComputeExtendedGcd(
        gcd: *mut c_void,
        u: *mut c_void,
        v: *mut c_void,
        op1: *const c_void,
        op2: *const c_void,
    );

    pub fn TEE_BigIntIsProbablePrime(op: *const c_void, confidence_level: u32) -> i32;

    pub fn TEE_BigIntConvertToFMM(
        dest: *mut c_void,
        src: *const c_void,
        n: *const c_void,
        context: *const c_void,
    );

    pub fn TEE_BigIntConvertFromFMM(
        dest: *mut c_void,
        src: *const c_void,
        n: *const c_void,
        context: *const c_void,
    );

    pub fn TEE_BigIntComputeFMM(
        dest: *mut c_void,
        op1: *const c_void,
        op2: *const c_void,
        n: *const c_void,
        context: *const c_void,
    );

    #[cfg(any(feature = "api_level2", feature = "api_level3"))]
    pub fn TEE_BigIntExpMod(
        dest: *mut c_void,
        op1: *const c_void,
        op2: *const c_void,
        n: *const c_void,
        context: *const c_void,
    ) -> TeeResult;

    pub fn EXT_TEE_BigIntExpMod(
        dest: *mut c_void,
        in_int: *const c_void,
        exp: *const c_void,
        n: *const c_void,
    ) -> bool;
}
