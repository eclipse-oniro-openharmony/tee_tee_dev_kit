// Copyright (C) 2023 Huawei Technologies Co., Ltd.
// Licensed under the Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2.
// You may obtain a copy of Mulan PSL v2 at:
//     http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
use crate::tloge;

use super::huk_service_ffi::*;
use core::mem::size_of;
use librust_service_ffi::{
    self,
    buffer::{munmap, SecureBuffer, ShareMem, TeeMemory},
    tee_defines::TeeUuid,
    TeeResult,
};

pub const OUTER_ITER_MAX_NUM: u32 = 500;
pub const INNER_ITER_MAX_NUM: u32 = 10;

pub fn huk_task_takey_param_check(msg: &huk_srv_msg, uuid: Option<&TeeUuid>) -> TeeResult {
    if uuid.is_none() {
        tloge!("huk derive takey check uuid failed\0");
        return TeeResult::TEE_ERROR_BAD_PARAMETERS;
    }

    if (unsafe { msg.data.takey_msg.salt_buf } == 0)
        || (unsafe { msg.data.takey_msg.salt_size } == 0)
        || (unsafe { msg.data.takey_msg.salt_size } > CMAC_DERV_MAX_DATA_IN_SIZE)
    {
        tloge!("huk derive takey check salt messages failed\0");
        return TeeResult::TEE_ERROR_BAD_PARAMETERS;
    }

    if (unsafe { msg.data.takey_msg.key_buf } == 0)
        || (unsafe { msg.data.takey_msg.key_size } == 0)
        || (unsafe { msg.data.takey_msg.key_size } > CMAC_DERV_MAX_DATA_IN_SIZE)
    {
        tloge!("huk derive takey check key messages failed\0");
        return TeeResult::TEE_ERROR_BAD_PARAMETERS;
    }

    if unsafe { msg.data.takey_msg.outer_iter_num } > OUTER_ITER_MAX_NUM
        || unsafe { msg.data.takey_msg.inner_iter_num } > INNER_ITER_MAX_NUM
    {
        tloge!("huk derive takey check iter num failed\0");
        return TeeResult::TEE_ERROR_BAD_PARAMETERS;
    }
    return TeeResult::TEE_SUCCESS;
}

pub fn huksrv_map_from_task(
    in_task_id: u32,
    va_addr: u64,
    size: u32,
) -> Result<ShareMem, TeeResult> {
    #[cfg(feature = "config_tee_upgrade")]
    let size_max = AES_TEXT_LEN;
    #[cfg(not(feature = "config_tee_upgrade"))]
    let size_max = CMAC_DERV_MAX_DATA_IN_SIZE;

    if va_addr == 0 || size == 0 || size > size_max {
        tloge!("huk map check input param failed\0");
        return Err(TeeResult::TEE_ERROR_BAD_PARAMETERS);
    }

    ShareMem::map(in_task_id, va_addr, size as _)
}

#[no_mangle]
pub extern "C" fn huk_srv_map_from_task(
    in_task_id: u32,
    va_addr: u64,
    size: u32,
    virt_addr: *mut u64,
) -> i32 {
    if virt_addr.is_null() {
        return TeeResult::TEE_ERROR_BAD_PARAMETERS.0 as i32;
    }
    match huksrv_map_from_task(in_task_id, va_addr, size) {
        Ok(o) => {
            unsafe { *virt_addr = o.get_slice().as_ptr() as _ };
            core::mem::forget(o);
            return 0;
        }
        Err(e) => {
            return e.0 as i32;
        }
    }
}

#[no_mangle]
pub extern "C" fn huk_srv_task_unmap(virt_addr: u64, size: u32) {
    if virt_addr == 0 {
        return;
    }
    if unsafe { munmap(virt_addr as _, size as _) } != 0 {
        tloge!("huk srv unmap error\0");
    }
}

pub fn do_derive_takey(salt_tmp: &[u8], key_tmp: &mut [u8], inner_iter_num: u32) -> TeeResult {
    let salt = memref_t {
        buffer: salt_tmp.as_ptr() as _,
        size: salt_tmp.len() as u32,
    };
    let mut cmac = memref_t {
        buffer: key_tmp.as_mut_ptr() as _,
        size: key_tmp.len() as u32,
    };

    return TeeResult(
        crypto_derive_root_key(CRYPTO_KEYTYPE_HUK, &salt, &mut cmac, inner_iter_num) as u32,
    );
}

fn huk_derive_takey(
    salt_shared: ShareMem,
    key_shared: ShareMem,
    huk_access: &huk_access_table,
) -> TeeResult {
    if salt_shared.size() > CMAC_DERV_MAX_DATA_IN_SIZE as usize {
        return TeeResult::TEE_ERROR_BAD_PARAMETERS;
    }

    let mut salt_tmp_size: usize = salt_shared.size();

    let is_old_compatible: bool =
        is_huk_service_compatible() && check_huk_access_perm(huk_access.cmd_id, &(huk_access.uuid));
    if !is_old_compatible {
        salt_tmp_size += size_of::<TeeUuid>();
    }

    let mut _bind = match TeeMemory::malloc(salt_tmp_size, 0) {
        Ok(o) => o,
        Err(e) => {
            return e;
        }
    };
    let salt_tmp = SecureBuffer {
        buffer: _bind.get_slice_mut(),
    };
    let mut _bindk = match TeeMemory::malloc(key_shared.size(), 0) {
        Ok(o) => o,
        Err(e) => {
            return e;
        }
    };
    let mut key_tmp = SecureBuffer {
        buffer: _bindk.get_slice_mut(),
    };

    let salt_slice = &mut salt_tmp.buffer[0..salt_shared.size()];
    salt_slice.copy_from_slice(salt_shared.get_slice());
    if !is_old_compatible {
        let uuid_slice = &mut salt_tmp.buffer[salt_shared.size()..salt_tmp_size];
        let uuid: &[u8] = unsafe {
            core::slice::from_raw_parts(
                &(huk_access.uuid) as *const TeeUuid as *const u8,
                size_of::<TeeUuid>(),
            )
        };
        uuid_slice.copy_from_slice(uuid);
    }

    let ret = do_derive_takey(salt_tmp.buffer, &mut key_tmp.buffer, 1);
    if ret == TeeResult::TEE_SUCCESS {
        key_shared.get_slice_mut().copy_from_slice(key_tmp.buffer);
    } else {
        tloge!("huk cmac derive takey failed\0");
    }

    return ret;
}

#[no_mangle]
pub extern "C" fn huk_task_derive_takey(
    imsg: Option<&huk_srv_msg>,
    irsp: Option<&mut huk_srv_rsp>,
    sndr_pid: u32,
    uuid: Option<&TeeUuid>,
) -> TeeResult {
    if let (Some(rsp), Some(msg)) = (irsp, imsg) {
        let mut ret = huk_task_takey_param_check(msg, uuid);
        if ret != TeeResult::TEE_SUCCESS {
            rsp.data.ret = ret;
            return ret;
        }

        let salt_size: u32 = unsafe { msg.data.takey_msg.salt_size };
        let key_size: u32 = unsafe { msg.data.takey_msg.key_size };
        if let Ok(key) =
            huksrv_map_from_task(sndr_pid, unsafe { msg.data.takey_msg.key_buf }, key_size)
        {
            if let Ok(salt) =
                huksrv_map_from_task(sndr_pid, unsafe { msg.data.takey_msg.salt_buf }, salt_size)
            {
                let huk_access = huk_access_table {
                    cmd_id: unsafe { msg.header.send.msg_id } as _,
                    uuid: *uuid.unwrap(),
                };
                ret = huk_derive_takey(salt, key, &huk_access);
                rsp.data.ret = ret;
                return ret;
            } else {
                tloge!("huk service map salt buffer from 0x%x failed\0", sndr_pid);
                rsp.data.ret = TeeResult::TEE_ERROR_GENERIC;
                return rsp.data.ret;
            }
        } else {
            tloge!("huk service map takey buffer from 0x%x failed\0", sndr_pid);
            rsp.data.ret = TeeResult::TEE_ERROR_GENERIC;
            return rsp.data.ret;
        }
    } else {
        tloge!("huk derive takey check msg or rsp failed\0");
        return TeeResult::TEE_ERROR_BAD_PARAMETERS;
    }
}
