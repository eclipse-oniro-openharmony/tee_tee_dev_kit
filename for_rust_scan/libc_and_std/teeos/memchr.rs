// Copyright (C) 2023 Huawei Technologies Co., Ltd.
// Licensed under the Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2.
// You may obtain a copy of Mulan PSL v2 at:
//     http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
use crate::sys::libc;
 
pub fn memchr(needle: u8, haystack: &[u8]) -> Option<usize> {
    let p = unsafe {
        libc::memchr(
            haystack.as_ptr() as *const libc::c_void,
            needle as libc::c_int,
            haystack.len(),
        )
    };
    if p.is_null() {
        None
    } else {
        Some(p.addr() - haystack.as_ptr().addr())
    }
}
 
pub fn memrchr(needle: u8, haystack: &[u8]) -> Option<usize> {
    #[cfg(target_os = "linux")]
    fn memrchr_specific(needle: u8, haystack: &[u8]) -> Option<usize> {
        // GNU's memrchr() will - unlike memchr() - error if haystack is empty.
        if haystack.is_empty() {
            return None;
        }
        let p = unsafe {
            libc::memrchr(
                haystack.as_ptr() as *const libc::c_void,
                needle as libc::c_int,
                haystack.len(),
            )
        };
        // FIXME: this should *likely* use `offset_from`, but more
        // investigation is needed (including running tests in miri).
        if p.is_null() {
            None
        } else {
            Some(p.addr() - haystack.as_ptr().addr())
        }
    }
 
    #[cfg(not(target_os = "linux"))]
    fn memrchr_specific(needle: u8, haystack: &[u8]) -> Option<usize> {
        core::slice::memchr::memrchr(needle, haystack)
    }
 
    memrchr_specific(needle, haystack)
}