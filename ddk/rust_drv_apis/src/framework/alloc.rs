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
use core::alloc::{GlobalAlloc, Layout};
use core::ptr;

extern "C" {
    pub fn malloc(size: usize) -> *mut u8;
    pub fn free(ptr: *mut u8);
}

/// An allocator that simply uses the Systems `malloc`/`free`.
struct SystemAllocator;

/// # Safety
///
/// The code in the unsafe trait implementation upholds the following invariants:
/// * The code cannot panic, and thus can never cause an unwind.
/// * The implementation is minimal and the actual allocation is done by
/// * the System allocator, which we assume to be correct.
unsafe impl GlobalAlloc for SystemAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // SAFETY: We trust `malloc` to be implemented correctly.
        // * `alloc` requires the caller to ensure that layout.size() > 0.
        // * We do not require the allocated block of memory to be initialized.
        let p = unsafe { malloc(layout.size()) };
        if ((p as usize) % layout.align()) != 0 {
            ptr::null_mut()
        } else {
            p
        }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        // SAFETY: The Safety requirements of `dealloc` ensure that
        // ptr was allocated via this allocator (i.e. `malloc`) and
        // that the layout is the same as for the originally allocated
        // pointer. This upholds the Safety requirements of `free`.
        unsafe { free(ptr) };
    }
}

#[global_allocator]
static GLOBAL: SystemAllocator = SystemAllocator;
