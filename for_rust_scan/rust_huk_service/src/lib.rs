// Copyright (C) 2023 Huawei Technologies Co., Ltd.
// Licensed under the Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2.
// You may obtain a copy of Mulan PSL v2 at:
//     http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
#![cfg_attr(not(test), no_std)]
pub mod huk_derive_takey;
pub mod huk_derive_takey2;
#[allow(non_camel_case_types)]
#[allow(dead_code)]
mod huk_service_ffi;
#[cfg(test)]
#[path = "../test/ut/mod.rs"]
mod ut;
