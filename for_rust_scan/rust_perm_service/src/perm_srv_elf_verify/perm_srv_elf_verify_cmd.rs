// Copyright (C) 2023 Huawei Technologies Co., Ltd.
// Licensed under the Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2.
// You may obtain a copy of Mulan PSL v2 at:
//     http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
use core::mem::{size_of, MaybeUninit};

use librust_service_ffi::{tee_defines::TeeUuid, TeeResult};

use crate::{
    perm_srv_common_ffi::{
        perm_srv_map_from_task, perm_srv_unmap_from_task, GLOBAL_HANDLE, REGISTER_ELF_REQ, SRE_OK,
    },
    perm_srv_elf_verify::tee_elf_verify_ffi::{elf_verify_reply, elf_verify_req},
    perm_srv_elf_verify::{
        ta_lib_img_unpack_ffi::ta_property_t, tee_elf_verify_ffi::check_is_blacklist_uuid,
        tee_elf_verify_ffi::perm_srv_check_ta_deactivated,
    },
    perm_srv_ta_config::{
        perm_srv_get_config_by_uuid, perm_srv_ta_config_ffi::ipc_msg_snd,
        ta_config_builder_ffi::config_info,
    },
    permission_service_ffi::{perm_srv_reply_msg_t, perm_srv_req_msg_t},
    tloge,
};

use super::tee_elf_verify_ffi::{anti_version_rollback, secure_elf_verify};

#[repr(C)]
pub struct PermShareMem {
    addr: u64,
    s: usize,
}

impl PermShareMem {
    pub fn map(src_task: u32, vaddr: u64, size: usize) -> Result<PermShareMem, TeeResult> {
        let mut outaddr: u64 = 0;
        let ret = unsafe { perm_srv_map_from_task(src_task, vaddr, size as _, &mut outaddr) };
        if ret == 0 {
            return Ok(PermShareMem {
                addr: outaddr,
                s: size,
            });
        }
        Err(TeeResult::TEE_ERROR_GENERIC)
    }

    // pub fn size(&self) -> usize {
    //     return self.s;
    // }

    pub fn get_slice(&self) -> &[u8] {
        let sls = unsafe { core::slice::from_raw_parts(self.addr as *const u8, self.s) };

        sls
    }

    // pub fn get_slice_mut(&self) -> &mut [u8] {
    //     let sls = unsafe { core::slice::from_raw_parts_mut(self.addr as *mut u8, self.s) };

    //     sls
    // }
}

impl Drop for PermShareMem {
    fn drop(&mut self) {
        unsafe {
            perm_srv_unmap_from_task(self.addr as _, self.s as _);
        }
    }
}

#[inline(always)]
fn i32_to_bool(val: i32) -> bool {
    val != 0
}

pub(crate) fn perm_srv_ta_run_authorization_check(
    uuid: &TeeUuid,
    manifest: &ta_property_t,
    target_version: u16,
    mem_page_align: bool,
) -> TeeResult {
    let mut config: config_info = unsafe { MaybeUninit::zeroed().assume_init() };
    #[cfg(feature = "config_dyn_import_cert")]
    let mut is_valid_device = true;
    #[cfg(not(feature = "config_dyn_import_cert"))]
    let is_valid_device;

    let ret = perm_srv_check_ta_deactivated(uuid, target_version);
    if ret != TeeResult::TEE_SUCCESS {
        tloge!("The TA version %u is not allowed\0", target_version as u32);
        return TeeResult::TEE_ERROR_GENERIC;
    }

    if perm_srv_get_config_by_uuid(uuid, &mut config) != TeeResult::TEE_SUCCESS {
        tloge!("Failed to get config by uuid\n");
        return TeeResult::TEE_ERROR_GENERIC;
    }

    /* Skipping device validity check for Third-Party TAs (Non-System TAs) on the Cloud */
    #[cfg(feature = "config_dyn_import_cert")]
    if config.manifest_info.sys_verify_ta {
        is_valid_device = config.control_info.debug_info.valid_device;
    }

    #[cfg(not(feature = "config_dyn_import_cert"))]
    {
        is_valid_device = config.control_info.debug_info.valid_device;
    }

    let is_invalid = (manifest.heap_size <= config.manifest_info.heap_size)
        && (manifest.stack_size <= config.manifest_info.stack_size)
        && i32_to_bool(manifest.instance_keep_alive) == config.manifest_info.instance_keep_alive
        && i32_to_bool(manifest.multi_command) == config.manifest_info.multi_command
        && i32_to_bool(manifest.multi_session) == config.manifest_info.multi_session
        && i32_to_bool(manifest.single_instance) == config.manifest_info.single_instance
        && is_valid_device
        && mem_page_align == config.manifest_info.mem_page_align;
    if is_invalid {
        return TeeResult::TEE_SUCCESS;
    } else {
        tloge!(
            "heap size 0x%x : 0x%x\0",
            manifest.heap_size,
            config.manifest_info.heap_size
        );
        tloge!(
            "stack size 0x%x : 0x%x\0",
            manifest.stack_size,
            config.manifest_info.stack_size
        );
        tloge!(
            "keep alive 0x%x : 0x%x\0",
            manifest.instance_keep_alive,
            config.manifest_info.instance_keep_alive as i32
        );
        tloge!(
            "multi command 0x%x : 0x%x\0",
            manifest.multi_command,
            config.manifest_info.multi_command as i32
        );
        tloge!(
            "multi session 0x%x : 0x%x\0",
            manifest.multi_session,
            config.manifest_info.multi_session as i32
        );
        tloge!(
            "single instance 0x%x : 0x%x\0",
            manifest.single_instance,
            config.manifest_info.single_instance as i32
        );
        tloge!("is valid device 0x%x\0", is_valid_device as i32);
        tloge!(
            "mem page align 0x%x : 0x%x\0",
            mem_page_align as i32,
            config.manifest_info.mem_page_align as i32
        );
    }

    tloge!("ta run authorization check manifest compare error\0");

    return TeeResult::TEE_ERROR_GENERIC;
}

fn check_perm_srv_elf_verify(msg: &perm_srv_req_msg_t, sndr_taskid: u32) -> TeeResult {
    if sndr_taskid != GLOBAL_HANDLE {
        tloge!("taload permission denied\0");
        return TeeResult::TEE_ERROR_ACCESS_DENIED;
    }

    if unsafe { msg.header.send.msg_size } as usize != size_of::<elf_verify_req>() {
        tloge!(
            "elf verify req msg size %u invalid %u\0",
            msg.header.send.msg_size,
            size_of::<elf_verify_req>() as u32
        );
        return TeeResult::TEE_ERROR_BAD_PARAMETERS;
    }

    return TeeResult::TEE_SUCCESS;
}

fn handle_elf_verify(req: &elf_verify_req, reply: &mut elf_verify_reply) -> TeeResult {
    let mut ret = secure_elf_verify(req, reply);
    if ret != TeeResult::TEE_SUCCESS {
        tloge!("secure elf verify failed, ret=0x%x\0", ret.0);
        return ret;
    }
    if check_is_blacklist_uuid(&(reply.srv_uuid)) {
        tloge!(
            "failed to load ta. this uuid %x is in blacklist\0",
            reply.srv_uuid.time_low
        );
        return TeeResult::TEE_ERROR_ACCESS_DENIED;
    }
    if reply.payload_hdr.ta_conf_size > 0 {
        ret = perm_srv_ta_run_authorization_check(
            &(reply.srv_uuid),
            &(reply.ta_property),
            reply.mani_ext.target_version,
            reply.mani_ext.mem_page_align,
        );
    }
    if ret == TeeResult::TEE_SUCCESS {
        ret = anti_version_rollback(reply);
    }
    return ret;
}

fn raw_copy<T: Sized>(a: *mut T, b: *const T) {
    let ax = unsafe { core::slice::from_raw_parts_mut(a as *mut u8, size_of::<T>()) };
    let bx = unsafe { core::slice::from_raw_parts(b as *const u8, size_of::<T>()) };
    ax.copy_from_slice(bx);
}

#[no_mangle]
pub extern "C" fn perm_srv_elf_verify(
    imsg: Option<&perm_srv_req_msg_t>,
    sndr_taskid: u32,
    _sndr_uuid: Option<&TeeUuid>,
    _rsp: Option<&mut perm_srv_reply_msg_t>,
) -> TeeResult {
    if let Some(msg) = imsg {
        let mut req: elf_verify_req = unsafe { MaybeUninit::zeroed().assume_init() };
        raw_copy(&mut req, unsafe { &msg.req_msg.verify_req });
        let mut ret = TeeResult::TEE_ERROR_BAD_PARAMETERS;
        let mut reply: elf_verify_reply = unsafe { MaybeUninit::zeroed().assume_init() };

        loop {
            if check_perm_srv_elf_verify(msg, sndr_taskid) != TeeResult::TEE_SUCCESS {
                tloge!("bad paramters\0");
                break;
            }

            reply.auth_share_addr = req.auth_share_addr;
            reply.auth_share_size = req.auth_share_size;
            match PermShareMem::map(sndr_taskid, req.auth_share_addr, req.auth_share_size as _) {
                Ok(o) => {
                    reply.auth_map_addr = o.get_slice().as_ptr() as _;
                }
                Err(_e) => {
                    tloge!("auth map from 0x%x failed\0", sndr_taskid);
                    ret = TeeResult::TEE_ERROR_GENERIC;
                    break;
                }
            };

            let reply_shared = match PermShareMem::map(
                sndr_taskid,
                req.reply_share_addr,
                req.reply_share_size as _,
            ) {
                Ok(o) => o,
                Err(_e) => {
                    tloge!("reply map from 0x%x failed\0", sndr_taskid);
                    ret = TeeResult::TEE_ERROR_GENERIC;
                    break;
                }
            };

            ret = handle_elf_verify(&req, &mut reply);
            let shared_reply = reply_shared.get_slice().as_ptr() as u64;
            raw_copy(shared_reply as *mut elf_verify_reply, &reply);

            break;
        }

        reply.verify_result = ret;
        let result = ipc_msg_snd(
            REGISTER_ELF_REQ,
            sndr_taskid,
            &reply.verify_result as *const TeeResult as _,
            size_of::<TeeResult>() as _,
        );
        if result != SRE_OK {
            tloge!("send reg elf req msg to failed, ret=0x%x\0", result);
            return TeeResult::TEE_ERROR_COMMUNICATION;
        }
        return ret;
    } else {
        tloge!("msg is null\0");
        return TeeResult::TEE_ERROR_BAD_PARAMETERS;
    }
}
