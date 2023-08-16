// Copyright (C) 2023 Huawei Technologies Co., Ltd.
// Licensed under the Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2.
// You may obtain a copy of Mulan PSL v2 at:
//     http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
use core::mem::MaybeUninit;

use crate::permission_service_ffi::*;
use crate::{
    perm_srv_common_ffi::{
        RPMB_TASK_NAME, SRE_OK, SSA_SERVICE_NAME, TEE_SERVICE_SE, TEE_SERVICE_SEM, TUI_SERVICE_NAME,
    },
    permission_service_ffi::perm_srv_req_msg_t,
    tlogd, tloge,
};
use librust_service_ffi::{tee_defines::TeeUuid, TeeResult};

use super::perm_srv_ta_config_ffi::ipc_hunt_by_name;
use super::ta_config_builder_ffi::{callee_ta_info, config_info};
use super::{perm_srv_get_config_by_taskid, perm_srv_get_config_by_uuid};

fn check_rpmb_task_id(sndr_taskid: u32) -> TeeResult {
    let mut handle: u32 = 0;

    let result = ipc_hunt_by_name(0, RPMB_TASK_NAME, &mut handle);
    if result == SRE_OK && sndr_taskid == handle {
        return TeeResult::TEE_SUCCESS;
    }

    return TeeResult::TEE_ERROR_ACCESS_DENIED;
}

fn check_ssa_task_id(sndr_taskid: u32) -> TeeResult {
    let mut handle = 0;

    let result = ipc_hunt_by_name(0, SSA_SERVICE_NAME, &mut handle);
    if result == SRE_OK && sndr_taskid == handle {
        return TeeResult::TEE_SUCCESS;
    }

    return TeeResult::TEE_ERROR_ACCESS_DENIED;
}

fn check_se_task_id(sndr_uuid: &TeeUuid) -> TeeResult {
    if *sndr_uuid == TEE_SERVICE_SE || *sndr_uuid == TEE_SERVICE_SEM {
        return TeeResult::TEE_SUCCESS;
    }

    return TeeResult::TEE_ERROR_ACCESS_DENIED;
}

fn check_tui_task_id(sndr_taskid: u32) -> TeeResult {
    let mut handle = 0;

    let result = ipc_hunt_by_name(0, TUI_SERVICE_NAME, &mut handle);
    if result == SRE_OK && sndr_taskid == handle {
        return TeeResult::TEE_SUCCESS;
    }

    return TeeResult::TEE_ERROR_ACCESS_DENIED;
}

fn check_sender_permission(
    msg: &perm_srv_req_msg_t,
    sndr_taskid: u32,
    sndr_uuid: &TeeUuid,
) -> TeeResult {
    let mut ret = TeeResult::TEE_ERROR_ACCESS_DENIED;

    let t = unsafe { msg.req_msg.query_perms.perm_type };
    match t {
        PERM_TYPE_RPMB_SIZE | PERM_TYPE_RPMB_CAPABILITY => {
            ret = check_rpmb_task_id(sndr_taskid);
        }
        PERM_TYPE_SFS_CAPABILITY | PERM_TYPE_MANAGE_INFO => {
            ret = check_ssa_task_id(sndr_taskid);
        }
        PERM_TYPE_SE_CAPABILITY => {
            ret = check_se_task_id(sndr_uuid);
        }
        PERM_TYPE_TUI_CAPABILITY => {
            ret = check_tui_task_id(sndr_taskid);
        }
        _ => {}
    }

    return ret;
}

fn perm_srv_query_perms_by_type(
    msg: &perm_srv_req_msg_t,
    sndr_taskid: u32,
    sndr_uuid: &TeeUuid,
    rsp: &mut perm_srv_reply_msg_t,
) -> TeeResult {
    let mut ret;
    let mut config: config_info = unsafe { MaybeUninit::zeroed().assume_init() };

    if unsafe { msg.req_msg.query_perms.checkby } == CHECK_BY_TASKID {
        let taskid = unsafe { msg.req_msg.query_perms.taskid };
        ret = perm_srv_get_config_by_taskid(taskid, Some(&mut config));
    } else if unsafe { msg.req_msg.query_perms.checkby } == CHECK_BY_UUID {
        let uuid = unsafe { msg.req_msg.query_perms.uuid };
        ret = perm_srv_get_config_by_uuid(&uuid, &mut config);
    } else {
        return TeeResult::TEE_ERROR_BAD_PARAMETERS;
    }

    if ret != TeeResult::TEE_SUCCESS
        || check_sender_permission(msg, sndr_taskid, sndr_uuid) != TeeResult::TEE_SUCCESS
    {
        ret = TeeResult::TEE_ERROR_ACCESS_DENIED;
        tloge!("sender %u has no permission to do query\0", sndr_taskid);
        rsp.reply.ret = ret;
        return ret;
    }

    let t = unsafe { msg.req_msg.query_perms.perm_type };
    match t {
        PERM_TYPE_RPMB_SIZE => rsp.reply.u.permsrsp.rpmb_size = config.control_info.rpmb_info.size,
        PERM_TYPE_RPMB_CAPABILITY => {
            rsp.reply.u.permsrsp.rpmb_capability = config.control_info.rpmb_info.permissions
        }
        PERM_TYPE_SFS_CAPABILITY => {
            rsp.reply.u.permsrsp.sfs_capability = config.control_info.sfs_info.permissions
        }
        PERM_TYPE_SE_CAPABILITY => {
            rsp.reply.u.permsrsp.se_capability = config.control_info.se_info.permissions
        }
        PERM_TYPE_TUI_CAPABILITY => {
            rsp.reply.u.permsrsp.tui_capability = config.control_info.tui_info.permissions
        }
        PERM_TYPE_MANAGE_INFO => rsp.reply.u.permsrsp.manager = config.control_info.ta_manager,
        _ => {}
    }
    rsp.reply.ret = ret;
    return ret;
}

#[no_mangle]
pub extern "C" fn perm_srv_query_perms(
    imsg: Option<&perm_srv_req_msg_t>,
    sndr_taskid: u32,
    isndr_uuid: Option<&TeeUuid>,
    irsp: Option<&mut perm_srv_reply_msg_t>,
) -> TeeResult {
    if let Some(rsp) = irsp {
        if let (Some(msg), Some(sndr_uuid)) = (imsg, isndr_uuid) {
            let ret = perm_srv_query_perms_by_type(msg, sndr_taskid, sndr_uuid, rsp);
            if ret != TeeResult::TEE_SUCCESS {
                tlogd!("query permissions error, 0x%x\0", ret.0);
            }

            return ret;
        } else {
            rsp.reply.ret = TeeResult::TEE_ERROR_BAD_PARAMETERS;
            return TeeResult::TEE_ERROR_BAD_PARAMETERS;
        }
    } else {
        return TeeResult::TEE_ERROR_BAD_PARAMETERS;
    }
}

fn is_right_cmd(callee_uuid: &TeeUuid, cmd: u32, callee_info: &callee_ta_info) -> bool {
    if callee_info.command_num != 0 {
        if *callee_uuid == callee_info.uuid {
            let sls = unsafe {
                core::slice::from_raw_parts(
                    callee_info.command_id as *const u32,
                    callee_info.command_num as usize,
                )
            };
            for c in sls {
                if cmd == *c {
                    return true;
                }
            }
        }
    }
    return false;
}

fn perm_srv_query_ta2ta_perm_by_uuid(
    caller_uuid: &TeeUuid,
    callee_uuid: &TeeUuid,
    cmd: u32,
) -> TeeResult {
    let mut config: config_info = unsafe { MaybeUninit::zeroed().assume_init() };

    if perm_srv_get_config_by_uuid(caller_uuid, &mut config) != TeeResult::TEE_SUCCESS {
        return TeeResult::TEE_ERROR_BAD_PARAMETERS;
    }

    let mut callee_info = config.control_info.callee_info;

    while callee_info != 0 {
        let info = unsafe { &*(callee_info as *const callee_ta_info) };
        if is_right_cmd(callee_uuid, cmd, info) {
            return TeeResult::TEE_SUCCESS;
        }
        callee_info = info.next;
    }

    return TeeResult::TEE_ERROR_GENERIC;
}

#[no_mangle]
pub extern "C" fn perm_srv_query_ta2ta_perm(
    imsg: Option<&perm_srv_req_msg_t>,
    _sndr_taskid: u32,
    isndr_uuid: Option<&TeeUuid>,
    irsp: Option<&mut perm_srv_reply_msg_t>,
) -> TeeResult {
    if let Some(rsp) = irsp {
        if let (Some(msg), Some(sndr_uuid)) = (imsg, isndr_uuid) {
            let uuid = unsafe { msg.req_msg.query_ta2ta_perm.uuid };

            let ret = perm_srv_query_ta2ta_perm_by_uuid(&uuid, sndr_uuid, unsafe {
                msg.req_msg.query_ta2ta_perm.cmd
            });
            if ret != TeeResult::TEE_SUCCESS {
                tloge!("query ta2ta fail\0");
            }
            rsp.reply.ret = ret;

            return ret;
        } else {
            rsp.reply.ret = TeeResult::TEE_ERROR_BAD_PARAMETERS;
            return TeeResult::TEE_ERROR_BAD_PARAMETERS;
        }
    } else {
        return TeeResult::TEE_ERROR_BAD_PARAMETERS;
    }
}
