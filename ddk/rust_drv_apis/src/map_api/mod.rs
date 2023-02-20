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
mod map_api_ffi;
pub use map_api_ffi::CacheModeType;
pub use map_api_ffi::PaddrT;

pub const MAP_FIXED_NOREPLACE: u32 = 0x100000;

pub const MAP_SHARED_VALIDATE: u32 = 0x03;

pub const MAP_FAILED: u64 = 0xffffffffffffffff;

pub const MAP_SHARED: u32 = 0x01;

pub const MAP_PRIVATE: u32 = 0x02;

pub const MAP_TYPE: u32 = 0x0f;

pub const MAP_FIXED: u32 = 0x10;

pub const MAP_ANON: u32 = 0x20;

pub const MAP_ANONYMOUS: u32 = MAP_ANON;

pub const MAP_NORESERVE: u32 = 0x4000;

pub const MAP_GROWSDOWN: u32 = 0x0100;

pub const MAP_DENYWRITE: u32 = 0x0800;

pub const MAP_EXECUTABLE: u32 = 0x1000;

pub const MAP_LOCKED: u32 = 0x2000;

pub const MAP_POPULATE: u32 = 0x8000;

pub const MAP_NONBLOCK: u32 = 0x10000;

pub const MAP_STACK: u32 = 0x20000;

pub const MAP_HUGETLB: u32 = 0x40000;

pub const MAP_SYNC: u32 = 0x80000;

pub const MAP_FILE: u32 = 0;

pub const MAP_HUGE_SHIFT: u32 = 26;

pub const MAP_HUGE_MASK: u32 = 0x3f;

pub const MAP_HUGE_16KB: u32 = 14 << 26;

pub const MAP_HUGE_64KB: u32 = 16 << 26;

pub const MAP_HUGE_512KB: u32 = 19 << 26;

pub const MAP_HUGE_1MB: u32 = 20 << 26;

pub const MAP_HUGE_2MB: u32 = 21 << 26;

pub const MAP_HUGE_8MB: u32 = 23 << 26;

pub const MAP_HUGE_16MB: u32 = 24 << 26;

pub const MAP_HUGE_32MB: u32 = 25 << 26;

pub const MAP_HUGE_256MB: u32 = 28 << 26;

pub const MAP_HUGE_512MB: u32 = 29 << 26;

pub const MAP_HUGE_1GB: u32 = 30 << 26;

pub const MAP_HUGE_2GB: u32 = 31 << 26;

pub const MAP_HUGE_16GB: u32 = 34 << 26;

pub const PROT_NONE: u32 = 0;

pub const PROT_READ: u32 = 1;

pub const PROT_WRITE: u32 = 2;

pub const PROT_EXEC: u32 = 4;

pub const PROT_GROWSDOWN: u32 = 0x01000000;

pub const PROT_GROWSUP: u32 = 0x02000000;

pub const MS_ASYNC: u32 = 1;

pub const MS_INVALIDATE: u32 = 2;

pub const MS_SYNC: u32 = 4;

pub const MCL_CURRENT: u32 = 1;

pub const MCL_FUTURE: u32 = 2;

pub const MCL_ONFAULT: u32 = 4;

pub const POSIX_MADV_NORMAL: u32 = 0;

pub const POSIX_MADV_RANDOM: u32 = 1;

pub const POSIX_MADV_SEQUENTIAL: u32 = 2;

pub const POSIX_MADV_WILLNEED: u32 = 3;

pub const POSIX_MADV_DONTNEED: u32 = 4;

#[repr(C)]
pub struct MapApi {}

impl MapApi {
    pub fn tee_map_secure(
        paddr: PaddrT,
        size: u64,
        buffer: &mut [u8],
        cache_mode: CacheModeType,
    ) -> i32 {
        unsafe { map_api_ffi::tee_map_secure(paddr, size, buffer.as_mut_ptr() as _, cache_mode) }
    }

    pub fn tee_map_nonsecure(
        paddr: PaddrT,
        size: u64,
        buffer: &mut [u8],
        cache_mode: CacheModeType,
    ) -> i32 {
        unsafe { map_api_ffi::tee_map_nonsecure(paddr, size, buffer.as_mut_ptr() as _, cache_mode) }
    }
}
