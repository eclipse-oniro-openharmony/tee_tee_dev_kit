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

extern "C" {
    ///
    /// read buffer from IO addr
    ///
    /// # Parameters
    /// dst: buffer to store data
    /// src: io addr
    /// len: read size
    pub fn read_from_io(dst: *mut c_void, src: /*volatile*/ *const c_void, len: usize);

    ///
    /// write buffer to IO addr
    ///
    /// # Parameters
    /// dst: io addr
    /// src: buffer to store data
    /// len: write size
    pub fn write_to_io(dst: /*volatile*/ *mut c_void, src: *const c_void, len: usize);

    ///
    /// map io addr
    ///
    /// # Parameters
    /// paddr: phy addr
    /// size: map size
    /// prot: map prot
    ///
    /// # Return
    /// virt addr if succ
    /// MAP_FAILED(-1) if fail
    pub fn ioremap(paddr: usize, size: usize, prot: i32) -> *mut c_void;

    ///
    /// unmap io addr
    ///
    /// # Parameters
    /// paddr: phy addr
    /// addr: virt addr
    ///
    /// # Return
    /// 0 if succ
    pub fn iounmap(paddr: usize, addr: *const c_void) -> i32;
}
