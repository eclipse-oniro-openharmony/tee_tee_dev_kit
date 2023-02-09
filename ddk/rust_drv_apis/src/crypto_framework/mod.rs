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
mod crypto_driver_adaptor;
mod crypto_driver_adaptor_ops;
pub use crypto_driver_adaptor::*;
pub use crypto_driver_adaptor_ops::*;

#[repr(C)]
pub struct CryptoFramework {}

impl CryptoFramework {
    pub fn register_crypto_ops(engine: u32, ops: &CryptoOpsT) -> i32 {
        unsafe { crypto_driver_adaptor::register_crypto_ops(engine, ops) }
    }

    pub fn hw_derive_root_key(key_type: u32, data_in: &MemrefT, data_out: &mut MemrefT) -> i32 {
        unsafe { crypto_driver_adaptor::hw_derive_root_key(key_type, data_in, data_out) }
    }

    pub fn hw_generate_random(buffer: &mut [u8]) -> i32 {
        unsafe {
            crypto_driver_adaptor::hw_generate_random(buffer.as_mut_ptr() as _, buffer.len() as _)
        }
    }
}
