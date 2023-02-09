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
use crate::framework::tee_defines::TeeResult;
use crate::framework::{print::tee_print, print::LogLevel};
use core::ffi::CStr;
use core::panic::PanicInfo;

extern "C" {
    #[doc = " Raises a Panic in the Trusted Application instance"]
    #[doc = ""]
    #[doc = " @param panicCode IN informative panic code defined by the TA"]
    #[doc = ""]
    #[doc = " @return void"]
    pub fn TEE_Panic(panicCode: TeeResult) -> !;
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        let file_name = location.file();
        // Avoid using alloc, by using stack.
        let mut file_name_buf: [u8; 128] = [0; 128];
        let file_max_len = file_name_buf.len() - 1;
        let len = file_name.len().min(file_max_len);
        // SAFETY: we use the minimum length of both slices, so the the access cannot fail.
        // The buffer was initialized to zero, and we did not overwrite the last 0, so the
        // string is guaranteed to be zero terminated. We assume that the file name does
        // not contain a zero.
        let file_name_cstr = unsafe {
            file_name_buf
                .get_unchecked_mut(0..len)
                .clone_from_slice(file_name.get_unchecked(0..len).as_bytes());
            CStr::from_bytes_with_nul_unchecked(&file_name_buf)
        };

        unsafe {
            tee_print(
                LogLevel::ERROR,
                "Panic at: File %s, line %u\0".as_ptr() as _,
                file_name_cstr.as_ptr(),
                location.line(),
            )
        };
    }
    unsafe { TEE_Panic(TeeResult(1)) }
}
