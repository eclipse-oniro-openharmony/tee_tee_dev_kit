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
pub use msp_ffi::FfiMspcStatus;

use crate::error::{FfiResult, FfiTeeError};

mod msp_ffi;

pub struct Msp;

pub enum MspStatus {
    Exist,
    NotAvailable,
    Unknown(FfiMspcStatus),
}

impl From<FfiMspcStatus> for MspStatus {
    fn from(status: FfiMspcStatus) -> Self {
        match status {
            FfiMspcStatus::EXISTS => Self::Exist,
            FfiMspcStatus::NOT_AVAILABLE => Self::NotAvailable,
            unknown => Self::Unknown(unknown),
        }
    }
}

impl Msp {
    pub fn recovery(flags: u32) -> crate::error::FfiResult {
        unsafe { msp_ffi::TEE_EXT_MspcRecovery(flags) }.into()
    }

    pub fn status() -> Result<MspStatus, FfiTeeError> {
        let mut status: FfiMspcStatus = FfiMspcStatus::default();
        let res: FfiResult = unsafe { msp_ffi::TEE_EXT_MSPIsAvailable(&mut status) }.into();
        res?;
        Ok(status.into())
    }
}
