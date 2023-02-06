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
#![cfg_attr(feature = "no_std", no_std)]

#[cfg(feature = "no_std")]
mod alloc;
pub mod arith_api;
pub mod core;
pub mod crypto_api;
pub mod defines;
pub mod error;
pub mod ext_api;
pub mod msp;
#[cfg(feature = "no_std")]
mod panic;
pub mod parameters;
pub mod print;
pub mod property;
pub mod rpmb_fcntl;
pub mod sec_flash;
pub mod security;
pub mod tee_defines;
pub mod time;
pub mod trusted_storage_api;
pub mod uuid;

pub use tee_defines::TeeResult;
