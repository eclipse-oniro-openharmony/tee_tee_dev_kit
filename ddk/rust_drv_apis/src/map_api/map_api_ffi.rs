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

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CacheModeType(u32);

impl CacheModeType {
    pub const CACHE: Self = Self(1);
    pub const NON_CACHE: Self = Self(0);
    pub const CACHE_MODE_DEVICE: Self = Self(2);
}

pub type PaddrT = u64;

extern "C" {
    ///
    /// map secure addr
    ///
    /// # Parameters
    /// paddr: phy addr to map
    /// size: map size
    /// vaddr: virt addr
    /// cache_mode: cache mode for map_secure
    ///
    /// # Return
    /// 0 if success
    /// !0 if fail
    pub fn tee_map_secure(
        paddr: PaddrT,
        size: u64,
        vaddr: *mut usize,
        cache_mode: CacheModeType,
    ) -> i32;

    ///
    /// map non secure addr
    ///
    /// # Parameters
    /// paddr: phy addr to map
    /// size: map size
    /// vaddr: virt addr
    /// cache_mode: cache mode for map_nonsecure
    ///
    /// # Return
    /// 0 if success
    /// !0 if fail
    pub fn tee_map_nonsecure(
        paddr: PaddrT,
        size: u64,
        vaddr: *mut usize,
        cache_mode: CacheModeType,
    ) -> i32;
}
