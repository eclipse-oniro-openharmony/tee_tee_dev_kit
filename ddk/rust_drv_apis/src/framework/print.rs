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
use core::fmt::{self, Write};

#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct LogLevel(u32);

impl LogLevel {
    pub const ERROR: Self = Self(0);
    pub const WARN: Self = Self(1);
    pub const INFO: Self = Self(2);
    pub const DEBUG: Self = Self(3);
    pub const VERBO: Self = Self(4);
    pub const ON: Self = Self(5);
}

extern "C" {
    pub fn tee_print(log_level: LogLevel, fmt_string: *const u8, ...);
}

pub const MAX_PRINT_LEN: usize = 255;

struct BufferedPrinter {
    len: usize,
    buf: [u8; MAX_PRINT_LEN],
}

impl BufferedPrinter {
    fn print_str(&self, line: u32, level: LogLevel) {
        let tag = match level {
            LogLevel::INFO => b"[info]\0".as_ptr(),
            LogLevel::ERROR => b"[error]\0".as_ptr(),
            LogLevel::VERBO => b"[verb]\0".as_ptr(),
            LogLevel::WARN => b"[warn]\0".as_ptr(),
            LogLevel::DEBUG => b"[debug]\0".as_ptr(),
            _ => b"[on]\0".as_ptr(),
        };
        unsafe {
            tee_print(
                level,
                "%s %u:%.*s\0".as_ptr() as _,
                tag as u64,
                line,
                self.len,
                self.buf.as_ptr() as u64,
            );
        }
    }
}

impl fmt::Write for BufferedPrinter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if s.len() > MAX_PRINT_LEN - self.len {
            return Ok(());
        } else {
            for c in s.as_bytes() {
                self.buf[self.len] = *c;
                self.len += 1;
            }
        }
        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(line: u32, level: LogLevel, args: fmt::Arguments) {
    let log_level = if cfg!(feature = "log_level_verbo") {
        LogLevel::VERBO
    } else if cfg!(feature = "log_level_debug") {
        LogLevel::DEBUG
    } else if cfg!(feature = "log_level_info") {
        LogLevel::INFO
    } else if cfg!(feature = "log_level_warn") {
        LogLevel::WARN
    } else if cfg!(feature = "log_level_error") {
        LogLevel::ERROR
    } else {
        LogLevel::INFO
    };
    if level.0 > log_level.0 {
        return;
    }
    let mut printer: BufferedPrinter = BufferedPrinter {
        len: 0,
        buf: [0u8; MAX_PRINT_LEN],
    };
    printer.write_fmt(args).unwrap();
    printer.print_str(line, level);
}

// Rust style print
// 1) str should NOT end with \0
// 2) Can check format in compile time
// 3) use '{}' instead of '%' for format string
// 4) each drv has different log file (for example:/data/vendor/log/tee/LOG@UUID-0)
// 5) if you use this macro to print (str, u8, u16, u32, i8, i16, i32, bool, u64, i64, usize, isize), you will get bigger drv so binary about 4k
// 6) if you use this macro to print (f32, f64), you will get bigger drv so binary about 22k
// example: tlogi!("xyz is {}", xyz);
#[macro_export]
macro_rules! tlogi {
    ($($arg:tt)*) => (rust_drv_apis::framework::print::_print(line!(), rust_drv_apis::framework::print::LogLevel::INFO, format_args!($($arg)*)));
}

#[macro_export]
macro_rules! tloge {
    ($($arg:tt)*) => (rust_drv_apis::framework::print::_print(line!(), rust_drv_apis::framework::print::LogLevel::ERROR, format_args!($($arg)*)));
}

#[macro_export]
macro_rules! tlogw {
    ($($arg:tt)*) => (rust_drv_apis::framework::print::_print(line!(), rust_drv_apis::framework::print::LogLevel::WARN, format_args!($($arg)*)));
}

#[macro_export]
macro_rules! tlogv {
    ($($arg:tt)*) => (rust_drv_apis::framework::print::_print(line!(), rust_drv_apis::framework::print::LogLevel::VERBO, format_args!($($arg)*)));
}

#[macro_export]
macro_rules! tlogd {
    ($($arg:tt)*) => (rust_drv_apis::framework::print::_print(line!(), rust_drv_apis::framework::print::LogLevel::DEBUG, format_args!($($arg)*)));
}
