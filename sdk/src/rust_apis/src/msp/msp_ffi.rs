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
pub use crate::tee_defines::*;

#[allow(dead_code)]
extern "C" {
    /// GP Extend TEE API do factory recovery operation about msc device.
    ///
    /// # Params
    ///
    /// flags, indicate the operation type, now is not used,fixed as 0xffffffff.
    ///
    /// # Return
    ///
    /// Operation status: success(0) or other failure status.
    pub fn TEE_EXT_MspcRecovery(flags: u32) -> TeeResult;

    /// Power on mpsc by voteId. The Power off has to be paired and matched with power on.
    ///
    /// # Params
    ///
    /// voteId for different TA.
    ///
    /// # Return
    ///
    /// TEE_SUCCESS: successful, others: failed.
    pub fn TEE_EXT_MspcPowerOn(vote_id: u32) -> TeeResult;

    /// Power off mpsc by voteId. The Power off has to be paired and matched with power on.
    ///
    /// # Params
    ///
    /// voteId for different TA.
    ///
    /// # Return
    ///
    /// TEE_SUCCESS: successful, others: failed.
    pub fn TEE_EXT_MspcPowerOff(vote_id: u32) -> TeeResult;
}

#[repr(transparent)]
#[derive(Eq, PartialEq)]
pub struct FfiMspcStatus(u32);
impl FfiMspcStatus {
    /// See `MSPC_EXIST_MAGIC`
    pub const EXISTS: Self = Self(0x4C);
    /// See `MSPC_NOT_AVAILABLE_MAGIC`
    pub const NOT_AVAILABLE: Self = Self(0xB3);
}

#[allow(clippy::derivable_impls)]
impl Default for FfiMspcStatus {
    fn default() -> Self {
        FfiMspcStatus(0)
    }
}

extern "C" {
    /// TEE_EXT_MSPIsAvailable : Check whether msp core is exist.
    ///
    /// # Params
    ///
    /// status : The status of msp core.
    /// MSPC_EXIST_MAGIC :   msp core is exist.
    /// MSPC_NOT_AVAILABLE_MAGIC : msp core is absence.
    ///
    /// # Return
    ///
    /// SRE_OK: successful, others: failed.
    pub fn TEE_EXT_MSPIsAvailable(status: *mut FfiMspcStatus) -> TeeResult;
}
