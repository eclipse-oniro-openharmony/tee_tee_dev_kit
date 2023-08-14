//!
//! Copyright (c) Huawei Technologies Co., Ltd. 2022-2023. All rights reserved.
//! Description: perm service ffi
//! Create: 2023-03-30
//!

use librust_service_ffi::{tee_defines::TeeUuid, TeeResult};

use crate::{
    perm_srv_common_ffi::reg_ta_info, perm_srv_elf_verify::tee_elf_verify_ffi::elf_verify_req,
};

// permission_service.h

pub const CHECK_BY_UUID: u32 = 0;
pub const CHECK_BY_TASKID: u32 = 1;

// enum PERM_TYPE
pub const PERM_TYPE_RPMB_SIZE: u32 = 0x01;
pub const PERM_TYPE_RPMB_CAPABILITY: u32 = 0x02;
pub const PERM_TYPE_SFS_CAPABILITY: u32 = 0x03;
pub const PERM_TYPE_SE_CAPABILITY: u32 = 0x04;
pub const PERM_TYPE_TUI_CAPABILITY: u32 = 0x05;
pub const PERM_TYPE_MANAGE_INFO: u32 = 0x06;
pub const PERM_TYPE_CERT_CAPABILITY: u32 = 0x07;

// ta_cert_t
pub const TA_DEBUG_CERT: u32 = 0;
pub const TA_RELEASE_CERT: u32 = 1;
pub const TA_CERT_MAX: u32 = 2;

#[repr(packed)]
#[derive(Copy, Clone)]
pub struct Send {
    pub msg_class: u8,
    pub msg_flags: u8,
    pub msg_id: u16,
    pub msg_size: u32,
}

#[repr(packed)]
#[derive(Copy, Clone)]
pub struct Reply {
    pub ret_val: i64,
    pub msg_size: u32,
    pub reserve: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hm_msg_header {
    pub send: Send,
    pub reply: Reply,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct perm_srv_set_config_t {
    pub config_file: u64, /* pointer */
    pub len: u32,
    pub cert_param: u64, /* cert_param_t pointer */
    pub uuid: TeeUuid,
    pub service_name: u64, /* pointer */
    pub service_name_len: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct perm_srv_query_tarun_t {
    pub uuid: TeeUuid,
    pub mani_val: u64, /* pointer */
    pub len: u32,
    pub distribution: u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct perm_srv_query_perms_t {
    pub uuid: TeeUuid,
    pub taskid: u32,
    pub checkby: u32,
    pub perm_type: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct perm_srv_query_ta2ta_perm_t {
    pub uuid: TeeUuid,
    pub cmd: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct perm_srv_set_crl_cert_t {
    pub crl_cert_buff: u64, /* pointer */
    pub crl_cert_size: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct perm_srv_set_ta_ctrl_list_t {
    pub ctrl_list_buff: u64, /* pointer */
    pub ctrl_list_size: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct perm_srv_set_ta_cert_t {
    pub ta_cert_buff: u64, /* pointer */
    pub ta_cert_size: u32,
    pub pub_key_buff: u64, /* pointer */
    pub pub_key_size: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct perm_srv_ta_unload_t {
    pub uuid: TeeUuid,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct perm_srv_crl_update_t {
    pub buffer: u64, /* pointer */
    pub size: u32,
    pub sharemem_index: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct perm_srv_ca_hashfile_verify_t {
    pub buffer: u64, /* pointer */
    pub size: u32,
    pub sharemem_index: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct req_crt_t {
    pub dst: u64, /* pointer */
    pub len: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union perm_srv_msgbody_t {
    pub ta_config: perm_srv_set_config_t,
    pub ta_run: perm_srv_query_tarun_t,
    pub query_perms: perm_srv_query_perms_t,
    pub query_ta2ta_perm: perm_srv_query_ta2ta_perm_t,
    pub reg_ta: reg_ta_info,
    pub ta_unload: perm_srv_ta_unload_t,
    pub crl_cert: perm_srv_set_crl_cert_t,
    pub ctrl_list: perm_srv_set_ta_ctrl_list_t,
    pub verify_req: elf_verify_req,
    pub crl_update_req: perm_srv_crl_update_t,
    pub ta_cert: perm_srv_set_ta_cert_t,
    pub ca_hashfile_verify: perm_srv_ca_hashfile_verify_t,
    pub crt: req_crt_t,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct perm_srv_sharememrsp_t {
    pub sharemem_index: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct rsp_crt_t {
    pub len: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union perm_srv_permsrsp_t {
    pub rpmb_size: u32,
    pub rpmb_capability: u64,
    pub sfs_capability: u64,
    pub se_capability: u64,
    pub tui_capability: u64,
    pub manager: u32,
    pub crt: rsp_crt_t,
    pub cert_capability: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union rspbody_t_union {
    pub sharememrsp: perm_srv_sharememrsp_t,
    pub permsrsp: perm_srv_permsrsp_t,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct perm_srv_rspbody_t {
    pub ret: TeeResult,
    pub u: rspbody_t_union,
}

#[repr(packed)]
#[derive(Clone, Copy)]
/* struct for req msg and reply msg */
pub struct perm_srv_req_msg_t {
    pub header: hm_msg_header,
    pub req_msg: perm_srv_msgbody_t,
}

#[repr(packed)]
#[derive(Clone, Copy)]
pub struct perm_srv_reply_msg_t {
    pub header: hm_msg_header,
    pub reply: perm_srv_rspbody_t,
}

pub type ta_cert_t = i32;

pub const MAX_PUB_KEY_SIZE: usize = 2056;
#[repr(C)]
pub struct cert_param_t {
    pub ta_version: u32,
    pub cert_type: ta_cert_t,
    pub public_key: [u8; MAX_PUB_KEY_SIZE],
    pub cert_product_type: u8,
    pub sys_verify_ta: bool,
}
