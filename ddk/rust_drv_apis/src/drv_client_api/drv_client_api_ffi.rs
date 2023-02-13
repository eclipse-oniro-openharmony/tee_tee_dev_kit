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

//!
//! this apis is for drv call drv
//!
use core::ffi::{c_char, c_void};

extern "C" {
    ///
    /// drv open drv
    ///
    /// # Parameters
    /// drv_name: the name of drv
    /// param&param_len: params for drv open
    ///
    /// # Return
    /// fd(>0) if success
    /// <0 if fail
    pub fn tee_drv_open(drv_name: *const c_char, param: *const c_void, param_len: u32) -> i64;

    ///
    /// drv ioctl
    ///
    /// # Parameters
    /// fd: fd
    /// cmd_id: the cmd to execute
    /// param&param_len: params for drv ioctl
    ///
    /// # Return
    /// -1 if fail
    /// othres if success
    pub fn tee_drv_ioctl(fd: i64, cmd_id: u32, param: *const c_void, param_len: u32) -> i64;

    ///
    /// close drv call
    ///
    /// # Parameters
    /// fd: fd
    ///
    /// # Return
    /// 0 if success
    /// !0 if fail
    pub fn tee_drv_close(fd: i64) -> i64;
}
