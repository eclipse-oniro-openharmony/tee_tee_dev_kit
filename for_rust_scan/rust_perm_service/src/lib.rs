//!
//! Copyright (c) Huawei Technologies Co., Ltd. 2022-2023. All rights reserved.
//! Description: rust perm service
//! Create: 2023-03-30
//!
#![feature(linked_list_cursors)]

#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub mod dlist;
#[allow(non_camel_case_types)]
pub mod perm_srv_common_ffi;
pub mod perm_srv_elf_verify;
#[allow(non_camel_case_types)]
pub mod perm_srv_ext_mf;
#[allow(non_camel_case_types)]
#[path = "perm_srv_ta_config/perm_srv_ta_config.rs"]
pub mod perm_srv_ta_config;
#[allow(non_camel_case_types)]
pub mod permission_service_ffi;
#[cfg(test)]
#[path = "../test/ut/mod.rs"]
pub mod ut;
