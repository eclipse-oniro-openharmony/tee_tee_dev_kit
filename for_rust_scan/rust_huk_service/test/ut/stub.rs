use crate::huk_service_ffi::memref_t;
use core::{ffi::c_void, ptr::null_mut};
use librust_service_ffi::tee_defines::TeeUuid;

extern "C" {
    pub fn printf(format: *const u8, ...) -> i32;
}

#[no_mangle]
pub extern "C" fn tee_print(_level: u32, _fmt: *const u8) {
    unsafe { printf(_fmt) };
    unsafe { printf("\n\0".as_ptr() as _) };
}

#[no_mangle]
pub extern "C" fn tee_map_sharemem(
    _src_task: u32,
    _vaddr: u64,
    _size: u64,
    _vaddr_out: *mut u64,
) -> i32 {
    if _vaddr == 0 || _size < 3 || _size == 65 {
        return -1;
    }
    unsafe { *_vaddr_out = _vaddr };
    0
}

#[no_mangle]
pub extern "C" fn munmap(_addr: *mut c_void, s: usize) -> i32 {
    if s == 0 {
        -1
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn is_huk_service_compatible_plat() -> bool {
    true
}

#[no_mangle]
pub extern "C" fn check_huk_access_permission(cmd_id: u32, _uuid: *const TeeUuid) -> bool {
    if cmd_id != 0 {
        true
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn tee_crypto_derive_root_key(
    derive_type: u32,
    data_in: &memref_t,
    _data_out: &mut memref_t,
    _iter_num: u32,
) -> i32 {
    if derive_type == 0
        || data_in.buffer == 0
        || data_in.size == 0
        || _iter_num == 0xfff
        || _iter_num == 0x7
    {
        return -1;
    }
    0
}

extern "C" {
    fn malloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

#[no_mangle]
pub extern "C" fn TEE_Malloc(size: usize, _hint: core::ffi::c_uint) -> *mut core::ffi::c_void {
    if size == 79 || size == 63 {
        return null_mut();
    }
    unsafe { malloc(size) as *mut c_void }
}

#[no_mangle]
pub extern "C" fn TEE_Free(buffer: *mut core::ffi::c_void) {
    unsafe { free(buffer as _) }
}
