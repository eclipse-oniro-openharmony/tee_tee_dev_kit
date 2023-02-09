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

use crate::framework::tee_defines::TeeUuid;
#[derive(PartialEq, Eq, Copy, Clone)]
#[repr(transparent)]
pub struct Uuid(TeeUuid);

impl Uuid {
    /// Check if the the `Uuid` is global task.
    pub fn is_global_task(&self) -> bool {
        // All non-zero uuids mark global task.
        *self == Uuid::default()
    }

    /// Return a default UUID with all zeros.
    pub const fn zero() -> Self {
        Uuid(TeeUuid {
            time_low: 0,
            time_mid: 0,
            time_hi_and_version: 0,
            clock_seq_and_node: [0; 8],
        })
    }
}

impl Default for Uuid {
    fn default() -> Self {
        Self::zero()
    }
}
