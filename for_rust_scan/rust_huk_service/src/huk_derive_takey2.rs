// Copyright (C) 2023 Huawei Technologies Co., Ltd.
// Licensed under the Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2.
// You may obtain a copy of Mulan PSL v2 at:
//     http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
use core::{ffi::c_void, mem::size_of};

use crate::{
    huk_derive_takey::{do_derive_takey, huk_task_takey_param_check, huksrv_map_from_task},
    huk_service_ffi::{
        check_huk_access_perm, huk_access_table, huk_srv_msg, huk_srv_rsp,
        is_huk_service_compatible, memref_t,
    },
    tloge,
};
use librust_service_ffi::{
    buffer::{SecureBuffer, TeeMemory},
    tee_defines::TeeUuid,
    TeeResult,
};

pub const KEY_DERIVE_BLOCK_SIZE: usize = 16;
pub const ITER_DERIVE_KEY2_SIZE: usize = KEY_DERIVE_BLOCK_SIZE * 2;

fn do_derive_takey2(salt: &[u8], key: &mut [u8], inner_iter_num: u32) -> TeeResult {
    let tmp_size = salt.len() + 1; /* add additional 1 byte to store count */
    let mut _tmpvec = match TeeMemory::malloc(tmp_size, 0) {
        Ok(o) => o,
        Err(e) => {
            return e;
        }
    };
    let tmp_sec = _tmpvec.get_slice_mut();
    (&mut tmp_sec[0..salt.len()]).copy_from_slice(salt);

    let loop_time = key.len() / KEY_DERIVE_BLOCK_SIZE;

    for i in 0..loop_time {
        tmp_sec[salt.len()] = i as u8;
        let ret = do_derive_takey(
            tmp_sec,
            &mut key[KEY_DERIVE_BLOCK_SIZE * i..(KEY_DERIVE_BLOCK_SIZE * (i + 1))],
            inner_iter_num,
        );
        if ret != TeeResult::TEE_SUCCESS {
            tloge!("derive key for num:%u failed, ret=0x%x\0", i, ret.0);
            return ret;
        }
    }

    return TeeResult::TEE_SUCCESS;
}

fn do_derive_takey2_iter(
    huk_access: &huk_access_table,
    salt_shared: &mut memref_t,
    takey_shared: &mut memref_t,
    outer_iter_num: u32,
    inner_iter_num: u32,
) -> TeeResult {
    let mut ret = TeeResult::TEE_ERROR_GENERIC;
    let mut salt_tmp_size: usize = if salt_shared.size as usize > ITER_DERIVE_KEY2_SIZE {
        salt_shared.size as usize
    } else {
        ITER_DERIVE_KEY2_SIZE
    };

    let is_compatible =
        is_huk_service_compatible() && check_huk_access_perm(huk_access.cmd_id, &(huk_access.uuid));
    if !is_compatible {
        salt_tmp_size += size_of::<TeeUuid>();
    }

    let mut _salt_tmp = match TeeMemory::malloc(salt_tmp_size, 0) {
        Ok(o) => o,
        Err(e) => {
            return e;
        }
    };
    let salt_tmp = SecureBuffer {
        buffer: _salt_tmp.get_slice_mut(),
    };

    let mut _key_tmp = match TeeMemory::malloc(takey_shared.size as _, 0) {
        Ok(o) => o,
        Err(e) => {
            return e;
        }
    };
    let key_tmp = SecureBuffer {
        buffer: _key_tmp.get_slice_mut(),
    };

    let salt_slice = &mut salt_tmp.buffer[0..salt_shared.size as usize];
    let salt_shared_slice = unsafe {
        core::slice::from_raw_parts(salt_shared.buffer as *const u8, salt_shared.size as usize)
    };
    salt_slice.copy_from_slice(salt_shared_slice);

    if !is_compatible {
        let salt_uuid_slice = &mut salt_tmp.buffer
            [salt_shared.size as usize..(salt_shared.size as usize + size_of::<TeeUuid>())];
        let uuid_slice = unsafe {
            core::slice::from_raw_parts(
                &(huk_access.uuid) as *const TeeUuid as *const u8,
                size_of::<TeeUuid>(),
            )
        };
        salt_uuid_slice.copy_from_slice(uuid_slice);
    }

    for _i in 0..outer_iter_num {
        ret = do_derive_takey2(salt_tmp.buffer, key_tmp.buffer, inner_iter_num);
        if ret != TeeResult::TEE_SUCCESS {
            break;
        }
        (&mut salt_tmp.buffer[0..ITER_DERIVE_KEY2_SIZE])
            .copy_from_slice(&key_tmp.buffer[0..ITER_DERIVE_KEY2_SIZE]);
    }
    let key_shared_slice = unsafe {
        core::slice::from_raw_parts_mut(
            takey_shared.buffer as *mut c_void as *mut u8,
            takey_shared.size as usize,
        )
    };
    key_shared_slice.copy_from_slice(key_tmp.buffer);

    return ret;
}

#[no_mangle]
pub extern "C" fn huk_task_derive_takey2_iter(
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

        let key_size = unsafe { msg.data.takey_msg.key_size };
        let salt_size = unsafe { msg.data.takey_msg.salt_size };
        if key_size < ITER_DERIVE_KEY2_SIZE as _ {
            rsp.data.ret = TeeResult::TEE_ERROR_BAD_PARAMETERS;
            return TeeResult::TEE_ERROR_BAD_PARAMETERS;
        }

        if let Ok(takey_mem) =
            huksrv_map_from_task(sndr_pid, unsafe { msg.data.takey_msg.key_buf }, key_size)
        {
            if let Ok(salt_mem) =
                huksrv_map_from_task(sndr_pid, unsafe { msg.data.takey_msg.salt_buf }, salt_size)
            {
                let mut salt_shared = memref_t {
                    buffer: salt_mem.get_slice_mut().as_mut_ptr() as _,
                    size: salt_size,
                };
                let mut takey_shared = memref_t {
                    buffer: takey_mem.get_slice_mut().as_mut_ptr() as _,
                    size: key_size,
                };
                let huk_access = huk_access_table {
                    cmd_id: unsafe { msg.header.send.msg_id } as _,
                    uuid: *uuid.unwrap(),
                };
                ret = do_derive_takey2_iter(
                    &huk_access,
                    &mut salt_shared,
                    &mut takey_shared,
                    unsafe { msg.data.takey_msg.outer_iter_num },
                    unsafe { msg.data.takey_msg.inner_iter_num },
                );

                rsp.data.ret = ret;
                return ret;
            } else {
                tloge!("huk service map salt2 buffer from 0x%x failed\0", sndr_pid);
                rsp.data.ret = TeeResult::TEE_ERROR_GENERIC;
                return rsp.data.ret;
            }
        } else {
            tloge!("huk service map takey2 buffer from 0x%x failed\0", sndr_pid);
            rsp.data.ret = TeeResult::TEE_ERROR_GENERIC;
            return rsp.data.ret;
        }
    } else {
        tloge!("huk derive takey2 iter check rsp failed\0");
        return TeeResult::TEE_ERROR_BAD_PARAMETERS;
    }
}
