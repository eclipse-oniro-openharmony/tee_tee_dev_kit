#![no_std]
use core::mem::size_of;

// use rust_drv_apis::crypto_driver_declare;
use rust_drv_apis::framework::DrvData;
use rust_drv_apis::tee_driver_declare;
use rust_drv_apis::tloge;
use rust_drv_apis::tlogi;

pub const MAX_BUF_LEN: usize = 1896;

#[allow(dead_code)]
struct ShareBufferArg {
    addr: u64,
    len: u32,
    share_token: u32,
}

pub const BUFFER_SIZE: usize = 0x1000;
pub const TOKEN_BUF_SIZE: usize = 0x1000;
pub const BUFFER1_LEN: usize = 21;
pub const BUFFER2_LEN: usize = 33;

extern "C" {
    pub fn malloc(size: usize) -> *mut u8;
    pub fn free(ptr: *mut u8);
}

#[allow(dead_code)]
struct BufferAarg {
    buffer1: [u8; BUFFER1_LEN],
    buffer1_len: u32,
    buffer2: [u8; BUFFER2_LEN],
    buffer2_len: u32,
    c1: u8,
    c2: u8,
    buffer_token: u32,
}

pub extern "C" fn init_test() -> i32 {
    tlogi!("driver init test end\n");
    return 0;
}

pub extern "C" fn ioctl_test(
    driver: Option<&mut DrvData>,
    _cmd: u32,
    _args: usize,
    _args_len: u32,
) -> i64 {
    if driver.is_none() {
        tloge!("ioctl invalid drv\n");
        return -1;
    }

    tlogi!("ioctl_test load!\n");

    return 0;
}

fn buf_init<'a>(args: u32) -> Option<&'a mut [u32]> {
    let buf = unsafe { malloc(TOKEN_BUF_SIZE * size_of::<u32>()) } as *mut u32;
    if buf.is_null() {
        tloge!("alloc buf failed\n");
        return None;
    }
    let s = unsafe { core::slice::from_raw_parts_mut(buf, TOKEN_BUF_SIZE) };

    let mut i = 0;
    while i < TOKEN_BUF_SIZE {
        s[i] = args;
        i += 1;
    }

    return Some(s);
}

pub extern "C" fn open_test(driver: Option<&mut DrvData>, args: usize, args_len: u32) -> i64 {
    if let Some(drv) = driver {
        if args == 0 && args_len == 0 {
            tloge!("input NULL param\n");
            return 0;
        }

        if args_len < size_of::<u32>() as u32 || args == 0 {
            tloge!("open invalid drv\n");
            return -1;
        }

        let open_succ = "hello rust driver open succ";
        tlogi!("{}", open_succ);

        let input = args as *const u32;
        let size = unsafe { *input };
        if size == u32::MAX {
            tloge!("open test input args is UINT32_MAX, just retrun -1\n");
            return -1;
        }

        let buf = buf_init(size);
        if let Some(buffer) = buf {
            drv.private_data = buffer.as_ptr() as _;
            tlogi!("driver open test begin: fd={} args=0x{:x}", drv.fd, size);
            return 0;
        }
        return -1;
    } else {
        tloge!("open invalid drv\n");
        return -1;
    }
}

pub extern "C" fn close_test(driver: Option<&mut DrvData>) -> i64 {
    if let Some(drv) = driver {
        tlogi!("driver close test begin: fd:{}", drv.fd);
        if !drv.private_data.is_null() {
            tloge!("free private data in close\n");
            unsafe { free(drv.private_data as _) };
        }

        return 0;
    } else {
        tloge!("close invalid drv\n");
        return -1;
    }
}

pub extern "C" fn suspend_test() -> i32 {
    tlogi!("suspend test begin\n");
    return 0;
}

pub extern "C" fn resume_test() -> i32 {
    tlogi!("resume test begin\n");
    return 0;
}

pub extern "C" fn suspend_s4_test() -> i32 {
    tlogi!("suspend_s4 test begin\n");
    return 0;
}

pub extern "C" fn resume_s4_test() -> i32 {
    tlogi!("resume_s4 test begin\n");
    return 0;
}

tee_driver_declare!(
    g_driver_drv_test_module,
    Some(init_test),
    Some(open_test),
    Some(ioctl_test),
    Some(close_test),
    Some(suspend_test),
    Some(resume_test),
    Some(suspend_s4_test),
    Some(resume_s4_test)
);

/*
crypto_driver_declare!(
    Some(init_test),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(suspend_test),
    Some(resume_test)
);
*/
