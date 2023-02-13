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

use crate::framework::defines::VsrootInfoT;

extern "C" {
    ///
    /// get phy addr by virt addr
    ///
    /// # Parameters
    /// addr: virt addr
    ///
    /// # Return
    /// phys addr if success
    /// NULL if fail
    pub fn drv_virt_to_phys(addr: usize) -> u64;

    ///
    /// get vsrootinfo for drv
    ///
    /// # Parameters
    /// info: point to vsroot struct
    ///
    /// # Return
    /// 0 if success
    /// !0 ifa fail
    pub fn drv_get_vsrootinfo(info: *mut VsrootInfoT) -> i32;
}
