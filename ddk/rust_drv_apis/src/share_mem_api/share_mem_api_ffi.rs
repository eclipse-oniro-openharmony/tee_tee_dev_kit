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
use core::ffi::c_void;

use crate::framework::tee_defines::TeeUuid;

extern "C" {
    ///
    /// get tlv sharedmem data
    /// when clear_flag is true, sharedmem buffer will be memset to zero
    /// after sharedmem acquired firstly
    ///
    /// # Parameters
    /// t_type: the type want to get
    /// type_size: length of type
    /// buffer: store data
    /// buffer_size: size of buffer
    /// clear_flag: if true, clear data after get
    ///
    /// # Return
    /// 0 if success
    /// !0 if fail
    ///
    pub fn get_tlv_sharedmem(
        t_type: *const u8,
        type_size: u32,
        buffer: *mut c_void,
        buffer_size: *mut u32,
        clear_flag: bool,
    ) -> i32;

    ///
    /// alloc sharemem by uuid
    ///
    /// # Parameters
    /// uuid: the UUID is identity of caller
    /// size: mem size to alloc
    ///
    /// # Return
    /// NULL if fail
    pub fn tee_alloc_sharemem_aux(uuid: *const TeeUuid, size: u32) -> *mut c_void;

    ///
    /// free sharemem
    ///
    /// # Parameters
    /// addr: mem addr
    /// size: mem size
    ///
    /// # Return
    /// 0 if succ
    pub fn tee_free_sharemem(addr: *mut c_void, size: u32) -> u32;
}
