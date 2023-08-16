// Copyright (C) 2023 Huawei Technologies Co., Ltd.
// Licensed under the Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2.
// You may obtain a copy of Mulan PSL v2 at:
//     http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
pub struct RwLock {}
 
pub type MovableRwLock = RwLock;
 
unsafe impl Send for RwLock {}
unsafe impl Sync for RwLock {}
 
impl RwLock {
    #[inline]
    pub const fn new() -> RwLock {
        panic!("Rwlock is unsupported.");
    }
 
    #[inline]
    pub unsafe fn read(&self) {
        panic!("Rwlock is unsupported.");
    }
 
    #[inline]
    pub unsafe fn try_read(&self) -> bool {
        panic!("Rwlock is unsupported.");
    }
 
    #[inline]
    pub unsafe fn write(&self) {
        panic!("Rwlock is unsupported.");
    }
 
    #[inline]
    pub unsafe fn try_write(&self) -> bool {
        panic!("Rwlock is unsupported.");
    }
 
    #[inline]
    pub unsafe fn read_unlock(&self) {
        panic!("Rwlock is unsupported.");
    }
 
    #[inline]
    pub unsafe fn write_unlock(&self) {
        panic!("Rwlock is unsupported.");
    }
}