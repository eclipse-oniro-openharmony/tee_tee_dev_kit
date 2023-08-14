//!
//! Copyright (c) Huawei Technologies Co., Ltd. 2022-2023. All rights reserved.
//! Description: rust huk service
//! Create: 2023-03-30
//!

#![cfg_attr(not(test), no_std)]
pub mod huk_derive_takey;
pub mod huk_derive_takey2;
#[allow(non_camel_case_types)]
#[allow(dead_code)]
mod huk_service_ffi;
#[cfg(test)]
#[path = "../test/ut/mod.rs"]
mod ut;
