// Copyright (C) 2023 Huawei Technologies Co., Ltd.
// Licensed under the Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2.
// You may obtain a copy of Mulan PSL v2 at:
//     http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
//! Implementation of `std::os` functionality for teeos
 
use core::marker::PhantomData;
 
use crate::error::Error as StdError;
use crate::ffi::{OsStr, OsString};
use crate::fmt;
use crate::io;
use crate::path;
use crate::path::PathBuf;
use crate::sys::libc;
 
use super::unsupported;
 
pub fn errno() -> i32 {
    unsafe { (*libc::__errno_location()) as i32 }
}
 
// Hardcoded to return 4096, since `sysconf` is only implemented as a stub.
pub fn page_size() -> usize {
    // unsafe { libc::sysconf(libc::_SC_PAGESIZE) as usize };
    4096
}
 
// Everything below are stubs and copied from unsupported.rs
 
pub fn error_string(_errno: i32) -> String {
    "error string unimplemented".to_string()
}
 
pub fn getcwd() -> io::Result<PathBuf> {
    unsupported()
}
 
pub fn chdir(_: &path::Path) -> io::Result<()> {
    unsupported()
}
 
pub struct SplitPaths<'a>(!, PhantomData<&'a ()>);
 
pub fn split_paths(_unparsed: &OsStr) -> SplitPaths<'_> {
    panic!("unsupported")
}
 
impl<'a> Iterator for SplitPaths<'a> {
    type Item = PathBuf;
    fn next(&mut self) -> Option<PathBuf> {
        self.0
    }
}
 
#[derive(Debug)]
pub struct JoinPathsError;
 
pub fn join_paths<I, T>(_paths: I) -> Result<OsString, JoinPathsError>
where
    I: Iterator<Item = T>,
    T: AsRef<OsStr>,
{
    Err(JoinPathsError)
}
 
impl fmt::Display for JoinPathsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "not supported on this platform yet".fmt(f)
    }
}
 
impl StdError for JoinPathsError {
    #[allow(deprecated)]
    fn description(&self) -> &str {
        "not supported on this platform yet"
    }
}
 
pub fn current_exe() -> io::Result<PathBuf> {
    unsupported()
}
 
pub struct Env(!);
 
impl Iterator for Env {
    type Item = (OsString, OsString);
    fn next(&mut self) -> Option<(OsString, OsString)> {
        self.0
    }
}
 
pub fn env() -> Env {
    panic!("not supported on this platform")
}
 
pub fn getenv(_: &OsStr) -> Option<OsString> {
    None
}
 
pub fn setenv(_: &OsStr, _: &OsStr) -> io::Result<()> {
    Err(io::Error::new(io::ErrorKind::Unsupported, "cannot set env vars on this platform"))
}
 
pub fn unsetenv(_: &OsStr) -> io::Result<()> {
    Err(io::Error::new(io::ErrorKind::Unsupported, "cannot unset env vars on this platform"))
}
 
pub fn temp_dir() -> PathBuf {
    panic!("no filesystem on this platform")
}
 
pub fn home_dir() -> Option<PathBuf> {
    None
}
 
pub fn exit(_code: i32) -> ! {
    panic!("TA should not call `exit`")
}
 
pub fn getpid() -> u32 {
    panic!("no pids on this platform")
}