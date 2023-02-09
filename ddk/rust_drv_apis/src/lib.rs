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

pub mod addr_api;
pub mod crypto_framework;
pub mod dma_api;
pub mod drv_api;
pub mod framework;
pub mod hwi_api;
pub mod io_api;
pub mod map_api;
pub mod mem_copy_api;
pub mod share_mem_api;

pub use framework::tee_defines::TeeResult;
