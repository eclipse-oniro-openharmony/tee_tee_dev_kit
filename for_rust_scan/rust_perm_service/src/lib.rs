// Copyright (C) 2023 Huawei Technologies Co., Ltd.
// Licensed under the Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2.
// You may obtain a copy of Mulan PSL v2 at:
//     http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
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
