// Copyright (C) 2023 Huawei Technologies Co., Ltd.
// Licensed under the Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2.
// You may obtain a copy of Mulan PSL v2 at:
//     http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
use core::mem::size_of;
use std::{collections::LinkedList, sync::Mutex};

use librust_service_ffi::{
    buffer::TeeMemory, core::TEE_Free, defines::SpawnUuid, tee_defines::TeeUuid, TeeResult,
};

use crate::{
    perm_srv_common_ffi::*, perm_srv_elf_verify::tee_elf_verify_ffi::tee_secure_img_parse_manifest,
    perm_srv_ta_config::perm_srv_ta_config_ffi::TLV_TAG_CALLEETA_UUID,
    permission_service_ffi::TA_RELEASE_CERT, pid_to_hmpid, tlogd, tloge,
};

use self::{
    perm_srv_ta_config_ffi::{
        ac_generate_dyn_uuid_data, check_device_id, check_sem_permission, check_tui_permission,
        get_rpmb_permission, get_rpmb_threshold, hm_getuuid, register_conf,
        set_ta_timer_permission, *,
    },
    ta_config_builder_ffi::*,
};
pub mod perm_srv_ta_config_cmd;
#[allow(non_camel_case_types)]
pub mod perm_srv_ta_config_ffi;
pub mod ta_config_builder_ffi;

pub const UINT8_TYPE_BITS_LEN: u32 = 8;
pub const ASN1_TLV_TAG_OFFSET: u8 = 1; /* 1 byte for tag */
pub const ASN1_INTEGER: u8 = 2;
pub const ASN1_SEQUENCE: u8 = 16;
pub const TLV_LEN_OFFSET: u8 = 2;
pub const TLV_VALUE_OFFSET: u8 = 3; /* 1 byte for tag, 2 bytes for len */
pub const TLV_MAX_LEN: u32 = 0xffffu32 + TLV_VALUE_OFFSET as u32; /* 3 is offset value */
pub const POS_ARRAY_SIZE: usize = 5;
pub const H_L_ERROR_NUM_VAL: i32 = -1;

pub const BYTE_COUNT_IN_UUID: usize = 16;
pub const UUID_FORMAT_STRLEN: u32 = 37;

pub const POLICY_VER_XML2TLV_PARSE_INDEX: u32 = 1; /* tool type for parse xml */
pub const XML2TLV_PY_VALUE: u32 = 1 << POLICY_VER_XML2TLV_PARSE_INDEX; /* python parse xml */
pub const XML2TLV_JAR_VALUE: u32 = 0 << POLICY_VER_XML2TLV_PARSE_INDEX; /* jar parse xml */
pub const XML2TLV_PARSE_BIT_MAP: u32 = 1 << POLICY_VER_XML2TLV_PARSE_INDEX;

static G_CONFIG_LIST: Mutex<LinkedList<&mut config_info>> =
    Mutex::new(LinkedList::<&mut config_info>::new());

pub const BITS_OF_BYTE: u32 = 8;
pub const DYN_CONFING_TAG: &[u8] = b"gpd.ta.dynConf\0";

pub(crate) fn byte_to_integer(bytes: &[u8], val_size: usize) -> u32 {
    let mut res: u32 = 0;
    if bytes.len() < val_size {
        return 0;
    }

    for i in 0..val_size {
        res = (res << BITS_OF_BYTE) + bytes[i] as u32;
    }
    return res;
}

pub const ALP_TO_DIGIT_GAP: u8 = 10;
pub(crate) fn asc2hex(a: u8) -> i8 {
    let is_digital = a >= b'0' && a <= b'9';
    let is_lower_letters = a >= b'a' && a <= b'f';
    let is_bigger_letters = a >= b'A' && a <= b'F';

    if is_digital {
        return (a - b'0') as i8;
    } else if is_lower_letters {
        return (a - b'a' + ALP_TO_DIGIT_GAP) as i8;
    } else if is_bigger_letters {
        return (a - b'A' + ALP_TO_DIGIT_GAP) as i8;
    }

    return PERMSRV_ERROR as i8;
}

pub const CHAR_COUNT_PER_BYTE: usize = 2;
pub const HALF_BYTE_SIZE: u8 = 4;
pub(crate) fn get_byte_value_from_buff(buff: &[u8], res: &mut u8) -> i32 {
    let check = buff.len() < CHAR_COUNT_PER_BYTE;
    if check {
        tloge!("invalid parameter\0");
        return PERMSRV_ERROR;
    }

    let h_val = asc2hex(buff[0]);
    let l_val = asc2hex(buff[1]);
    if (h_val as i32 == H_L_ERROR_NUM_VAL) || (l_val as i32 == H_L_ERROR_NUM_VAL) {
        return PERMSRV_ERROR;
    }

    *res = (((h_val as u8) << HALF_BYTE_SIZE) | l_val as u8) as u8;
    return 0;
}

pub(crate) fn perm_srv_convert_str_to_uuid(buff: &[u8], uuid: &mut TeeUuid) -> i32 {
    let len = buff.len();
    let mut p = 0;
    let mut add_pos: u8 = 0;
    let mut tmp_val: u8 = 0;
    /* These numbers are marked '-' */
    let add_pos_array: [u8; POS_ARRAY_SIZE] = [8, 13, 18, 23, 36];
    let mut parsed_buffer = [0u8; BYTE_COUNT_IN_UUID];

    for i in 0..BYTE_COUNT_IN_UUID {
        if get_byte_value_from_buff(&buff[p..len], &mut tmp_val) != 0 {
            return PERMSRV_ERROR;
        }
        parsed_buffer[i] = tmp_val;
        p += CHAR_COUNT_PER_BYTE;

        if add_pos as usize >= POS_ARRAY_SIZE {
            break;
        }

        if p as u8 == add_pos_array[add_pos as usize] {
            if p >= len || buff[p] != b'-' {
                break;
            }

            p += 1;
            add_pos += 1;
        }
    }
    /* not touch the end of buff */
    if p != len {
        return PERMSRV_ERROR;
    }

    add_pos = 0;
    uuid.time_low = byte_to_integer(
        &parsed_buffer[0..(BYTE_COUNT_IN_UUID - add_pos as usize)],
        size_of::<u32>(),
    );
    add_pos += size_of::<u32>() as u8;
    uuid.time_mid = byte_to_integer(
        &parsed_buffer[add_pos as usize..BYTE_COUNT_IN_UUID],
        size_of::<u16>(),
    ) as u16;
    add_pos += size_of::<u16>() as u8;
    uuid.time_hi_and_version = byte_to_integer(
        &parsed_buffer[add_pos as usize..BYTE_COUNT_IN_UUID],
        size_of::<u16>(),
    ) as u16;
    add_pos += size_of::<u16>() as u8;
    for i in 0..NODE_LEN {
        uuid.clock_seq_and_node[i] = parsed_buffer[i + add_pos as usize];
    }
    return 0;
}

pub(crate) fn parser_callee_ta_uuid(buff: &[u8], uuid: &mut TeeUuid) -> i32 {
    let len = buff.len();
    if len <= size_of::<u16>() + UUID_STR_LEN {
        tloge!("invalid calleeTA uuid info\0");
        return PERMSRV_ERROR;
    }

    let value_len: u16 = byte_to_integer(buff, size_of::<u16>()) as u16;
    if value_len as usize != UUID_STR_LEN {
        tloge!("invalid calleeTA uuid info\0");
        return PERMSRV_ERROR;
    }

    /* can make sure buffer is bigger enough */
    let ret = perm_srv_convert_str_to_uuid(
        &buff[size_of::<u16>()..(value_len as usize + size_of::<u16>())],
        uuid,
    );
    return ret;
}

/*
 *           inner tlv format
 * +++++++++++++++++++++++++++++++++++++++++++++++++++++++
 * +----tag----+----len----+-----------value-------------+
 * +  1 byte   +  1 byte   + 1 or 2 byte(depend on len)  +
 * +++++++++++++++++++++++++++++++++++++++++++++++++++++++
 */
fn parse_inner_tag(tlv: &[u8], total_len: &mut u8) -> i32 {
    let tlv_len = tlv.len();
    if tlv_len < TLV_VALUE_OFFSET as usize {
        return PERMSRV_ERROR;
    }

    if tlv[0] != ASN1_INTEGER {
        return PERMSRV_ERROR;
    }

    let len: u8 = tlv[ASN1_TLV_TAG_OFFSET as usize] as u8;
    if len == ASN1_TLV_TAG_OFFSET {
        *total_len = TLV_VALUE_OFFSET;
        return tlv[TLV_LEN_OFFSET as usize] as i32;
    } else if len == TLV_LEN_OFFSET && tlv_len > TLV_VALUE_OFFSET as usize {
        *total_len = TLV_LLEN as u8;
        return tlv[TLV_VALUE_OFFSET as usize] as i32
            + ((tlv[TLV_LEN_OFFSET as usize] as i32) << UINT8_TYPE_BITS_LEN);
    } else {
        /* tag value is too big >= 65536 */
        return PERMSRV_ERROR;
    }
}

/*
 * ++++++++++++++++++++++++++++++++++++++++++++++++++++++
 * +--ASN1 INTEGER--+----len----+--[inner tlv][value]---+
 * +-----1 byte-----+---2 byte---+------(4+4)-----------+
 * ++++++++++++++++++++++++++++++++++++++++++++++++++++++
 */
pub const COMMAND_ID_INNER_TLV_LEN: u8 = 4;
pub const COMMAND_ID_INFO_LEN: u8 =
    TLV_VALUE_OFFSET + COMMAND_ID_INNER_TLV_LEN + size_of::<u32>() as u8;

pub(crate) fn parser_callee_ta_command_id(buff: &[u8], callee: &mut callee_ta_info) -> i32 {
    let mut offset: u32 = 0;
    let mut inner_tlv_len: u8 = 0;
    let mut i = 0;
    let len = buff.len();

    let mut is_invalid = ((len / COMMAND_ID_INFO_LEN as usize) > MAX_CALLEE_COMMAND_COUNT)
        || ((len % COMMAND_ID_INFO_LEN as usize) != 0);
    if is_invalid {
        tloge!("invalid command 0x%x\0", len);
        return PERMSRV_ERROR;
    }
    let command_num = len / COMMAND_ID_INFO_LEN as usize;

    let command_id = match TeeMemory::malloc(command_num * size_of::<u32>(), 0) {
        Ok(o) => o,
        Err(_) => {
            tloge!("malloc command_id failed\0");
            return PERMSRV_ERROR;
        }
    };

    while (offset as usize) < len {
        let tag = parse_inner_tag(
            &buff[(offset + TLV_VALUE_OFFSET as u32) as usize..len],
            &mut inner_tlv_len,
        );
        is_invalid =
            tag != TLV_TAG_CALLEETA_COMMAND_ID || inner_tlv_len != COMMAND_ID_INNER_TLV_LEN;
        if is_invalid {
            return PERMSRV_ERROR;
        }

        let index: u8 = TLV_VALUE_OFFSET + inner_tlv_len;
        if (offset as usize + index as usize + size_of::<u32>()) > len {
            tloge!("invalid callee command ID len\0");
            return PERMSRV_ERROR;
        }
        let sls =
            unsafe { core::slice::from_raw_parts_mut(command_id.addr() as *mut u32, command_num) };
        sls[i] = byte_to_integer(
            &buff[(offset as usize + index as usize)..len],
            size_of::<u32>(),
        );

        offset += COMMAND_ID_INFO_LEN as u32;
        i += 1;
        if i >= command_num * size_of::<u32>() {
            break;
        }
    }

    callee.command_num = command_num as _;
    callee.command_id = command_id.addr();
    core::mem::forget(command_id);
    return PERMSRV_OK;
}

pub(crate) fn is_duplicate_callee(config: &config_info, callee_info: &callee_ta_info) -> bool {
    let mut info = config.control_info.callee_info;
    while info != 0 {
        let callinfo = unsafe { &*(info as *const callee_ta_info) };
        if callee_info.uuid == callinfo.uuid {
            return true;
        }
        info = callinfo.next;
    }
    return false;
}

pub(crate) fn parser_fill_callee(buff: &[u8], value_len: u32, config: &mut config_info) -> i32 {
    let len = buff.len();
    let mut inner_tlv_len: u8 = 0;

    let mut index: u32 = TLV_VALUE_OFFSET as u32;
    if len < index as usize {
        return PERMSRV_ERROR;
    }

    let mut command_len: u32 = len as u32 - index;
    let tag = parse_inner_tag(
        &buff[index as usize..(index as usize + command_len as usize)],
        &mut inner_tlv_len,
    );
    let mut is_invalid = tag != TLV_TAG_CALLEETA_UUID
        || value_len <= (inner_tlv_len as u32 + size_of::<u16>() as u32 + UUID_STR_LEN as u32);
    if is_invalid {
        tloge!("invalid tag value for calleeTA\0");
        return PERMSRV_ERROR;
    }

    index += inner_tlv_len as u32;

    let callee_info_mem = match TeeMemory::malloc(size_of::<callee_ta_info>(), 0) {
        Ok(o) => o,
        Err(_) => {
            return PERMSRV_ERROR;
        }
    };
    let callee_info = unsafe { &mut *(callee_info_mem.addr() as *mut callee_ta_info) };

    command_len = value_len - inner_tlv_len as u32;
    let mut ret = parser_callee_ta_uuid(
        &buff[index as usize..(index as usize + command_len as usize)],
        &mut callee_info.uuid,
    );
    is_invalid = (ret != PERMSRV_OK) || (is_duplicate_callee(config, callee_info));
    if is_invalid {
        tloge!("parser callee ta uuid failed\0");
        return PERMSRV_ERROR;
    }

    index += (size_of::<u16>() + UUID_STR_LEN) as u32;

    command_len =
        value_len - (inner_tlv_len as u32 + size_of::<u16>() as u32 + UUID_STR_LEN as u32);
    ret = parser_callee_ta_command_id(
        &buff[index as usize..(index as usize + command_len as usize)],
        callee_info,
    );
    if ret != PERMSRV_OK {
        tloge!("parser callee command id failed\0");
        return PERMSRV_ERROR;
    }

    callee_info.next = config.control_info.callee_info;
    config.control_info.callee_info = callee_info_mem.addr();
    core::mem::forget(callee_info_mem);

    return PERMSRV_OK;
}

pub const CALLEE_UUID_INNER_TLV_LEN: u8 = 3;
/*
 * ++++++++++++++++++++++++++++++++  calleeTA info  ++++++++++++++++++++++++++++++++++
 * +--ASN1 tag--+----len----+--[inner tlv]  [[value_len][value]]  [child elements]---+
 * +---1 byte---+--2 byte---+--------------------depend on len-----------------------+
 * +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
 *
 * +++++++++++ [[value_len][value]]  [child elements]++++++++++++
 * +--value_len--+----value----+-------[child elements]---------+
 * +---2 byte---+----x bytes----+----------x bytes--------------+
 * ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
 */
pub(crate) fn parser_callee_info(buff: &[u8], config: &mut config_info) -> i32 {
    let len = buff.len();
    let mut offset: usize = 0;
    let mut i: u32 = 0;

    while offset < len {
        let is_invalid = (i > MAX_CALLEE_TA_COUNT)
            || ((len - offset) < (TLV_VALUE_OFFSET + CALLEE_UUID_INNER_TLV_LEN) as usize);
        if is_invalid {
            tloge!("invalid len for calleeTA info\0");
            return PERMSRV_ERROR;
        }

        let tag_type = buff[offset];
        if tag_type != ASN1_SEQUENCE {
            return PERMSRV_ERROR;
        }

        let value_len = byte_to_integer(
            &buff[(offset + ASN1_TLV_TAG_OFFSET as usize)..len],
            size_of::<u16>(),
        );
        if value_len > (len as u32 - TLV_VALUE_OFFSET as u32) {
            tloge!("invalid value len for single calleeTA info\0");
            return PERMSRV_ERROR;
        }

        let ret = parser_fill_callee(&buff[offset..len], value_len, config);
        if ret != PERMSRV_OK {
            return PERMSRV_ERROR;
        }

        i += 1;
        if (u32::MAX - offset as u32) <= (TLV_VALUE_OFFSET as u32 + value_len) {
            tloge!("invalid tlv data\0");
            return PERMSRV_ERROR;
        }

        offset += TLV_VALUE_OFFSET as usize + value_len as usize;
    }

    return PERMSRV_OK;
}

/*
 * ++++++++++++++++++++++++++++++++++++++++++++++++++
 * +--ASN1 tag--+----len----+--[inner tlv][value]---+
 * +---1 byte---+--2 byte---+------depend on len----+
 * ++++++++++++++++++++++++++++++++++++++++++++++++++
 */
fn config_tlv_check_node(
    buff: &[u8],
    offset: &mut usize,
    tag: &mut i32,
    value_len: &mut u16,
) -> i32 {
    let len = buff.len();
    let mut inner_tlv_len: u8 = 0;

    if len < TLV_VALUE_OFFSET as usize {
        tloge!("invalid buff len %d\0", len as i32);
        return PERMSRV_ERROR;
    }

    *tag = parse_inner_tag(&buff[TLV_VALUE_OFFSET as usize..len], &mut inner_tlv_len);
    if *tag < 0 {
        tloge!("invalid tag value\0");
        return PERMSRV_ERROR;
    }

    *offset = (TLV_VALUE_OFFSET + inner_tlv_len) as usize;

    let tlv_value_len: u16 =
        byte_to_integer(&buff[ASN1_TLV_TAG_OFFSET as usize..len], size_of::<u16>()) as u16;
    if tlv_value_len <= inner_tlv_len as u16 {
        tloge!("tlv value len 0x%x\0", tlv_value_len as u32);
        return PERMSRV_ERROR;
    }

    *value_len = tlv_value_len - inner_tlv_len as u16;
    if (*value_len) as usize >= len {
        tloge!("invalid value len %d\0", *value_len as i32);
        return PERMSRV_ERROR;
    }

    if len - (*value_len as usize) < (*offset) as usize {
        tloge!("invalid value len\0");
        return PERMSRV_ERROR;
    }

    return PERMSRV_OK;
}

pub(crate) fn get_tag_uuid(buff: &[u8], config: &mut config_info) -> i32 {
    let len = buff.len();

    if len != UUID_STR_LEN {
        tloge!("config tlv parser invalid uuid\0");
        return PERMSRV_ERROR;
    }

    /* make sure buffer is big enough */
    let ret = perm_srv_convert_str_to_uuid(buff, &mut (config.uuid));
    if ret != PERMSRV_OK {
        tloge!("invalid uuid\0");
        return PERMSRV_ERROR;
    }

    return PERMSRV_OK;
}

pub(crate) fn config_tlv_parser_basic_info(buff: &[u8], config: &mut config_info) -> i32 {
    let len = buff.len();
    let mut tag: i32 = 0;
    let mut value_len: u16 = 0;
    let mut offset: usize = 0;
    let mut child_offset: usize = 0;

    while offset < len {
        let mut ret = config_tlv_check_node(
            &buff[offset..len],
            &mut child_offset,
            &mut tag,
            &mut value_len,
        );
        if ret != PERMSRV_OK {
            tloge!("invalid tlv data\0");
            return PERMSRV_ERROR;
        }

        match tag {
            TLV_TAG_UUID => {
                ret = get_tag_uuid(
                    &buff[(offset + child_offset)..(offset + child_offset + value_len as usize)],
                    config,
                );
            }
            TLV_TAG_SERVICE_NAME => {
                if (value_len as usize) > MAX_SERVICE_NAME_LEN || value_len == 0 {
                    return PERMSRV_ERROR;
                }
                let sls = &mut (config.service_name)[0..value_len as usize];
                sls.copy_from_slice(
                    &buff[(offset + child_offset)..(offset + child_offset + value_len as usize)],
                );
                config.service_name_len = value_len as u32;
            }
            _ => {}
        }
        let is_invalid = ((u32::max as usize - offset) <= (child_offset + value_len as usize))
            || (ret != PERMSRV_OK);
        if is_invalid {
            tloge!("invalid tlv data\0");
            config.service_name_len = 0;
            return PERMSRV_ERROR;
        }

        offset += child_offset + value_len as usize;
    }
    return PERMSRV_OK;
}

pub(crate) fn config_tlv_value_to_manifest_info(buff: &[u8], tag: i32, config: &mut config_info) {
    match tag {
        TLV_TAG_SINGLE_INSTANCE => config.manifest_info.single_instance = buff[0] != 0,
        TLV_TAG_MULTI_SESSION => config.manifest_info.multi_session = buff[0] != 0,
        TLV_TAG_HEAP_SIZE => {
            config.manifest_info.heap_size = byte_to_integer(buff, size_of::<u32>())
        }
        TLV_TAG_STACK_SIZE => {
            config.manifest_info.stack_size = byte_to_integer(buff, size_of::<u32>())
        }
        TLV_TAG_INSTANCE_KEEP_ALIVE => config.manifest_info.instance_keep_alive = buff[0] != 0,
        TLV_TAG_MEM_PAGE_ALIGN => config.manifest_info.mem_page_align = buff[0] != 0,
        TLV_TAG_TARGET_TYPE => {
            config.manifest_info.target_type = byte_to_integer(buff, size_of::<u32>())
        }
        #[cfg(feature = "config_dyn_import_cert")]
        TLV_TAG_SYS_VERIFY_TA => config.manifest_info.sys_verify_ta = buff[0] != 0,
        _ => {}
    }
}

pub(crate) fn config_tlv_parser_manifest_info(buff: &[u8], config: &mut config_info) -> i32 {
    let len = buff.len();
    let mut tag: i32 = 0;
    let mut offset: usize = 0;
    let mut value_len: u16 = 0;
    let mut child_offset: usize = 0;

    while offset < len {
        let ret = config_tlv_check_node(
            &buff[offset..len],
            &mut child_offset,
            &mut tag,
            &mut value_len,
        );
        if ret != PERMSRV_OK {
            tloge!("invalid tlv data\0");
            return PERMSRV_ERROR;
        }
        let is_invalid = (tag == TLV_TAG_HEAP_SIZE || tag == TLV_TAG_STACK_SIZE)
            && (value_len as usize != size_of::<u32>());
        if is_invalid {
            return PERMSRV_ERROR;
        }

        config_tlv_value_to_manifest_info(
            &buff[(offset + child_offset)..(offset + child_offset + value_len as usize)],
            tag,
            config,
        );

        if (u32::max as usize - offset) <= (child_offset + value_len as usize) {
            tloge!("invalid tlv data\0");
            return PERMSRV_ERROR;
        }

        offset += child_offset + value_len as usize;
    }

    return PERMSRV_OK;
}

pub(crate) fn tlv_tag_rpmb_general(buff: &[u8], config: &mut config_info) {
    if buff[0] != 0 {
        config.control_info.rpmb_info.permissions |= RPMB_GENERAL_PERMISSION;
    }
}

pub(crate) fn tlv_tag_rpmb_specific(buff: &[u8], config: &mut config_info) {
    if buff[0] != 0 {
        config.control_info.rpmb_info.permissions |= RPMB_RESET_PERMISSION;
    }
}

pub(crate) fn config_tlv_parser_rpmb_permission(buff: &[u8], config: &mut config_info) -> i32 {
    let len = buff.len();
    let mut child_offset: usize = 0;
    let mut tag: i32 = 0;
    let mut offset_value: usize = 0;
    let mut value_len: u16 = 0;

    while offset_value < len {
        let ret = config_tlv_check_node(
            &buff[offset_value..len],
            &mut child_offset,
            &mut tag,
            &mut value_len,
        );
        if ret != PERMSRV_OK {
            tloge!("invalid tlv data\0");
            return PERMSRV_ERROR;
        }

        match tag {
            TLV_TAG_RPMB_GENERAL => tlv_tag_rpmb_general(
                &buff[(offset_value + child_offset)
                    ..(offset_value + child_offset + value_len as usize)],
                config,
            ),
            TLV_TAG_RPMB_SPECIFIC => tlv_tag_rpmb_specific(
                &buff[(offset_value + child_offset)
                    ..(offset_value + child_offset + value_len as usize)],
                config,
            ),
            _ => {}
        }

        if (u32::max as usize - offset_value) <= (child_offset + value_len as usize) {
            tloge!("invalid tlv data\0");
            return PERMSRV_ERROR;
        }

        offset_value += child_offset + value_len as usize;
    }

    return PERMSRV_OK;
}

pub(crate) fn config_tlv_parser_rpmb_info(buff: &[u8], config: &mut config_info) -> i32 {
    let len = buff.len();
    let mut child_offset: usize = 0;
    let mut tag: i32 = 0;
    let mut offset: usize = 0;
    let mut value_len: u16 = 0;

    while offset < len {
        let mut ret = config_tlv_check_node(
            &buff[offset..len],
            &mut child_offset,
            &mut tag,
            &mut value_len,
        );
        let mut is_invalid = ret != PERMSRV_OK
            || (tag == TLV_TAG_RPMB_SIZE && value_len as usize != size_of::<u32>());
        if is_invalid {
            tloge!("invalid tlv data\0");
            return PERMSRV_ERROR;
        }

        match tag {
            TLV_TAG_RPMB_PERMISSION => {
                ret = config_tlv_parser_rpmb_permission(
                    &buff[(offset + child_offset)..(offset + child_offset + value_len as usize)],
                    config,
                )
            }
            TLV_TAG_RPMB_SIZE => {
                /* Already judged the value_len */
                config.control_info.rpmb_info.size =
                    byte_to_integer(&buff[(offset + child_offset)..len], size_of::<u32>());
                ret = PERMSRV_OK;
            }
            _ => ret = PERMSRV_OK,
        }

        is_invalid = (u32::MAX as usize - offset) <= (child_offset + value_len as usize)
            || (ret != PERMSRV_OK);
        if is_invalid {
            tloge!("invalid tlv data\0");
            return PERMSRV_ERROR;
        }

        offset += child_offset + value_len as usize;
    }

    return PERMSRV_OK;
}

pub(crate) fn config_tlv_parser_info(buff: &[u8], config: &mut config_info) -> i32 {
    let len = buff.len();
    let mut child_offset: usize = 0;
    let mut tag: i32 = 0;
    let mut offset: usize = 0;
    let mut value_len: u16 = 0;

    while offset < len {
        let ret = config_tlv_check_node(
            &buff[offset..len],
            &mut child_offset,
            &mut tag,
            &mut value_len,
        );
        if ret != PERMSRV_OK {
            tloge!("invalid tlv data\0");
            return PERMSRV_ERROR;
        }

        match tag {
            TLV_TAG_SE_OPEN_SESSION => {
                if buff[offset + child_offset] != 0 {
                    config.control_info.se_info.permissions |= SE_OPEN_SESSION_PERMISSION;
                }
            }
            TLV_TAG_TUI_GENERAL => {
                if buff[offset + child_offset] != 0 {
                    config.control_info.tui_info.permissions |= TUI_PERMISSION;
                }
            }
            TLV_TAG_DEBUG_DEVICE_ID => {
                let _ = check_device_id(
                    config,
                    &buff[(offset + child_offset)..(offset + child_offset + value_len as usize)],
                );
            }
            #[cfg(feature = "config_dyn_import_cert")]
            TLV_TAG_CERT_PERMISSION => {
                if buff[offset + child_offset] != 0 {
                    config.control_info.cert_info.permissions |= CERT_GENERAL_PERMISSION;
                }
            }
            _ => {}
        }

        if (u32::MAX as usize - offset) <= (child_offset + value_len as usize) {
            tloge!("invalid tlv data\0");
            return PERMSRV_ERROR;
        }

        offset += child_offset + value_len as usize;
    }

    return PERMSRV_OK;
}

pub const TA_MANAGER_TRUSTONIC: u32 = 1;
pub const G_TA_MANAGER_TRUSTONIC: &[u8] = b"Trustonic\0";
pub(crate) fn parser_ta_manager(buff: &[u8], config: &mut config_info) {
    let len = buff.len();
    if len == (G_TA_MANAGER_TRUSTONIC.len() - 1) && (&G_TA_MANAGER_TRUSTONIC[0..len] == buff) {
        config.control_info.ta_manager = TA_MANAGER_TRUSTONIC;
    }
}

pub(crate) fn config_tlv_parser_control_info(buff: &[u8], config: &mut config_info) -> i32 {
    let len = buff.len();
    let mut child_offset: usize = 0;
    let mut tag: i32 = 0;
    let mut offset: usize = 0;
    let mut value_len: u16 = 0;

    while offset < len {
        let mut ret = config_tlv_check_node(
            &buff[offset..len],
            &mut child_offset,
            &mut tag,
            &mut value_len,
        );
        if ret != PERMSRV_OK {
            tloge!("invalid tlv data\0");
            return PERMSRV_ERROR;
        }

        match tag {
            TLV_TAG_RPMB_INFO => {
                ret = config_tlv_parser_rpmb_info(
                    &buff[(offset + child_offset)..(offset + child_offset + value_len as usize)],
                    config,
                )
            }
            #[cfg(feature = "config_dyn_import_cert")]
            TLV_TAG_CERT_INFO => {
                ret = config_tlv_parser_info(
                    &buff[(offset + child_offset)..(offset + child_offset + value_len as usize)],
                    config,
                )
            }
            TLV_TAG_SE_INFO | TLV_TAG_TUI_INFO | TLV_TAG_DEBUG_INFO => {
                ret = config_tlv_parser_info(
                    &buff[(offset + child_offset)..(offset + child_offset + value_len as usize)],
                    config,
                )
            }
            TLV_TAG_CALLEETA_INFO => {
                ret = parser_callee_info(
                    &buff[(offset + child_offset)..(offset + child_offset + value_len as usize)],
                    config,
                )
            }
            TLV_TAG_TA_MANAGER => parser_ta_manager(
                &buff[(offset + child_offset)..(offset + child_offset + value_len as usize)],
                config,
            ),
            _ => {}
        }
        let is_invalid = ret != PERMSRV_OK
            || ((u32::MAX as usize - offset) <= (child_offset + value_len as usize));
        if is_invalid {
            tloge!("parser control info failed\0");
            return PERMSRV_ERROR;
        }

        offset += child_offset + value_len as usize;
    }

    return PERMSRV_OK;
}

fn config_tlv_parser_child_sequences(buff: &[u8], config: &mut config_info) -> i32 {
    let len = buff.len();
    let mut child_offset: usize = 0;
    let mut tag: i32 = 0;
    let mut offset: usize = 0;
    let mut value_len: u16 = 0;

    while offset < len {
        let mut ret = config_tlv_check_node(
            &buff[offset..len],
            &mut child_offset,
            &mut tag,
            &mut value_len,
        );
        if ret != PERMSRV_OK {
            tloge!("invalid tlv data\0");
            return PERMSRV_ERROR;
        }

        match tag {
            TLV_TAG_TA_BASIC_INFO => {
                ret = config_tlv_parser_basic_info(
                    &buff[(offset + child_offset)..(offset + child_offset + value_len as usize)],
                    config,
                )
            }
            TLV_TAG_TA_MANIFEST_INFO => {
                ret = config_tlv_parser_manifest_info(
                    &buff[(offset + child_offset)..(offset + child_offset + value_len as usize)],
                    config,
                )
            }
            TLV_TAG_TA_CONTROL_INFO => {
                ret = config_tlv_parser_control_info(
                    &buff[(offset + child_offset)..(offset + child_offset + value_len as usize)],
                    config,
                )
            }
            _ => {}
        }
        let is_invalid = ret != PERMSRV_OK
            || ((u32::MAX as usize - offset) <= (child_offset + value_len as usize));
        if is_invalid {
            tloge!("parser control info failed\0");
            return PERMSRV_ERROR;
        }

        offset += child_offset + value_len as usize;
    }

    return PERMSRV_OK;
}

fn parser_jar_tlv_to_config(buff: &[u8], config: &mut config_info) -> i32 {
    let mut tag: i32 = 0;
    let mut offset: usize = 0;
    let mut value_len: u16 = 0;

    if buff[0] != ASN1_SEQUENCE {
        tloge!("invalid tag value\0");
        return PERMSRV_ERROR;
    }

    let mut ret = config_tlv_check_node(buff, &mut offset, &mut tag, &mut value_len);
    if ret != PERMSRV_OK {
        tloge!("invalid tlv data\0");
        return PERMSRV_ERROR;
    }

    ret = config_tlv_parser_child_sequences(&buff[offset..(offset + value_len as usize)], config);
    return ret;
}

fn add_timer_perm_by_config(config: &config_info) -> TeeResult {
    let uuid = config.uuid;

    if ac_generate_dyn_uuid_data(&uuid) != 0 {
        tloge!("ac_generate_dyn_uuid_data failed\0");
        return TeeResult::TEE_ERROR_GENERIC;
    }

    let permission = TIMER_GROUP_PERMISSION;
    if (config.control_info.se_info.permissions & SE_OPEN_SESSION_PERMISSION) != 0 {
        if set_ta_timer_permission(&uuid, permission) != 0 {
            tloge!("fail to add timer group permission\0");
            return TeeResult::TEE_ERROR_GENERIC;
        }
    }

    return TeeResult::TEE_SUCCESS;
}

fn check_cn_validation(cn: &[u8], info: &config_info) -> TeeResult {
    let cn_size = cn.len();
    let mut buff = [0u8; TA_CERT_MAX_CN_INFO_LEN];

    let param_invalid = cn_size > TA_CERT_MAX_CN_INFO_LEN
        || (info.service_name_len as usize
            > (TA_CERT_MAX_CN_INFO_LEN - (UUID_STR_LEN + TA_CERT_CN_UNDERLINE_SIZE)));

    if param_invalid {
        return TeeResult::TEE_ERROR_BAD_PARAMETERS;
    }

    if cn_size != (info.service_name_len as usize + UUID_STR_LEN + TA_CERT_CN_UNDERLINE_SIZE) {
        tloge!("invalid cn size: 0x%x\0", cn_size);
        return TeeResult::TEE_ERROR_GENERIC;
    }

    if perm_srv_convert_uuid_to_str(
        Some(&(info.uuid)),
        buff.as_mut_ptr(),
        TA_CERT_MAX_CN_INFO_LEN as _,
    ) != TeeResult::TEE_SUCCESS
    {
        return TeeResult::TEE_ERROR_GENERIC;
    }

    buff[UUID_STR_LEN] = b'_';
    let sls = &mut buff[(UUID_STR_LEN + TA_CERT_CN_UNDERLINE_SIZE)
        ..(UUID_STR_LEN + TA_CERT_CN_UNDERLINE_SIZE + info.service_name_len as usize)];
    sls.copy_from_slice(&info.service_name[0..info.service_name_len as usize]);

    if cn != &buff[0..cn_size] {
        tloge!("uuid or service name mismatch in TA cert and configs\0");
        return TeeResult::TEE_ERROR_GENERIC;
    }

    return TeeResult::TEE_SUCCESS;
}

fn check_contain_dynconf(permconfig: &perm_config) -> u32 {
    let strlen = DYN_CONFING_TAG.len() - 1;
    let dynconf_tag = DYN_CONFING_TAG;
    for i in 0..(permconfig.tlv_len as usize - strlen) {
        for j in 0..strlen {
            if unsafe { *((permconfig.tlv_buf as usize + i + j) as *const u8) } != dynconf_tag[j] {
                break;
            }
            if j == strlen - 1 {
                return permconfig.tlv_len - i as u32;
            }
        }
    }

    return 0;
}

pub(crate) fn parse_dyntlv_buf(
    uuid: &TeeUuid,
    permconfig: &perm_config,
    config: &config_info,
    dynconflen: u32,
) -> TeeResult {
    let mut dynconf_len = dynconflen;
    if dynconf_len != 0 {
        if config.uuid == *uuid {
            dynconf_len += 1;
            let mani = unsafe {
                let addr = permconfig.tlv_buf as usize + permconfig.tlv_len as usize;
                core::slice::from_raw_parts(addr as *const u8, dynconf_len as usize)
            };
            let res = tee_secure_img_parse_manifest(
                mani,
                &mut dynconf_len,
                false,
                config.manifest_info.target_type,
            );
            if res != TeeResult::TEE_SUCCESS {
                return TeeResult::TEE_ERROR_GENERIC;
            }
        } else {
            tloge!("different uuid from config and manifest_info\0");
            return TeeResult::TEE_ERROR_GENERIC;
        }
    }
    return TeeResult::TEE_SUCCESS;
}

fn parser_python_tlv_to_ta_config(buff: &[u8], config: &mut config_info) -> i32 {
    let dyn_conf = dyn_conf_t {
        dyn_conf_size: buff.len() as _,
        dyn_conf_buffer: buff.as_ptr() as _,
    };
    return register_conf(
        &dyn_conf,
        install_ta_config,
        config as *mut config_info as _,
        size_of::<config_info>() as u32,
    );
}

fn parse_tlv_to_ta_config(perm_config: &perm_config, config: &mut config_info) -> i32 {
    let ret;
    if (perm_config.policy_version & XML2TLV_PARSE_BIT_MAP) == XML2TLV_JAR_VALUE {
        let buff = unsafe {
            core::slice::from_raw_parts(perm_config.tlv_buf, perm_config.tlv_len as usize)
        };
        ret = parser_jar_tlv_to_config(buff, config);
    } else if (perm_config.policy_version & XML2TLV_PARSE_BIT_MAP) == XML2TLV_PY_VALUE {
        let buff = unsafe {
            core::slice::from_raw_parts(perm_config.tlv_buf, perm_config.tlv_len as usize)
        };
        ret = parser_python_tlv_to_ta_config(buff, config);
    } else {
        ret = TeeResult::TEE_ERROR_BAD_PARAMETERS.0 as i32;
    }
    if ret != PERMSRV_OK {
        tloge!(
            "parse failed for tlv type:%u, 0-jar, 2-python\0",
            perm_config.policy_version & XML2TLV_PARSE_BIT_MAP
        );
    }
    return ret;
}

fn parse_config_body_check(_uuid: &TeeUuid, perm_config: &perm_config) -> bool {
    perm_config.tlv_buf.is_null()
        || perm_config.tlv_len == 0
        || perm_config.policy_version == 0
        || perm_config.cn_size == 0
        || perm_config.tlv_len > TLV_MAX_LEN
        || (perm_config.tlv_len as usize) < DYN_CONFING_TAG.len() - 1
}

fn release_callee_info(info: u64) {
    let mut temp = info;

    while temp != 0 {
        let tmp = unsafe { &mut *(temp as *mut callee_ta_info) };
        if tmp.command_id != 0 {
            unsafe { TEE_Free(tmp.command_id as _) };
        }

        temp = tmp.next;
        unsafe { TEE_Free(temp as _) };
    }
}

pub(crate) fn perm_srv_update_config_by_same_uuid(new_config: &'static mut config_info) {
    let mut list = G_CONFIG_LIST.lock().unwrap();
    let mut cur = list.cursor_front_mut();
    while let Some(entry) = cur.current() {
        if (*entry).uuid == new_config.uuid {
            let old = cur.remove_current().unwrap();
            release_callee_info(old.control_info.callee_info);
            unsafe {
                TEE_Free(old as *mut config_info as _);
            }
            break;
        }
        cur.move_next();
    }
    list.push_back(new_config);
}

#[no_mangle]
pub extern "C" fn perm_srv_parse_config_body(
    iuuid: Option<&TeeUuid>,
    perm_config: Option<&mut perm_config>,
) -> TeeResult {
    let mut ret;
    let config_mem = match TeeMemory::malloc(size_of::<config_info>(), 0) {
        Ok(o) => o,
        Err(e) => return e,
    };
    let config = unsafe { &mut *(config_mem.addr() as *mut config_info) };

    if let (Some(uuid), Some(permconfig)) = (iuuid, perm_config) {
        if parse_config_body_check(uuid, permconfig) {
            tloge!("parse_config_body_check fail\0");
            return TeeResult::TEE_ERROR_BAD_PARAMETERS;
        }
        let dynconf_len = check_contain_dynconf(permconfig);
        permconfig.tlv_len = permconfig.tlv_len - dynconf_len;

        let res = parse_tlv_to_ta_config(permconfig, config);
        if res != PERMSRV_OK {
            return TeeResult::TEE_ERROR_GENERIC;
        }
        if permconfig.cert_type == TA_RELEASE_CERT {
            ret = check_cn_validation(&permconfig.cn[0..permconfig.cn_size], config);
            if ret != TeeResult::TEE_SUCCESS {
                return ret;
            }
            config.control_info.debug_info.valid_device = true;
        }
        config.version = permconfig.policy_version;
        if matches!(
            config.manifest_info.target_type,
            TA_TARGET_TYPE | SRV_TARGET_TYPE
        ) {
            ret = add_timer_perm_by_config(config);
            if ret != TeeResult::TEE_SUCCESS {
                return ret;
            }
        }
        ret = parse_dyntlv_buf(uuid, permconfig, config, dynconf_len);
        if ret != TeeResult::TEE_SUCCESS {
            return ret;
        }

        perm_srv_update_config_by_same_uuid(config);
        core::mem::forget(config_mem);

        return TeeResult::TEE_SUCCESS;
    } else {
        tloge!("param is null\0");
        return TeeResult::TEE_ERROR_BAD_PARAMETERS;
    }
}

#[no_mangle]
pub extern "C" fn perm_srv_convert_uuid_to_str(
    iuuid: Option<&TeeUuid>,
    buff: *mut u8,
    len: u32,
) -> TeeResult {
    let check = (iuuid.is_none()) || (len < UUID_FORMAT_STRLEN) || (buff.is_null());
    if check {
        tloge!("invalid parameter\0");
        return TeeResult::TEE_ERROR_GENERIC;
    }
    let uuid = iuuid.unwrap();

    let ret = unsafe {
        snprintf_s(
            buff,
            len as usize,
            len as usize - 1,
            b"%08x-%04x-%04x-%02x%02x-%02x%02x%02x%02x%02x%02x\0".as_ptr() as _,
            uuid.time_low,
            uuid.time_mid as u32,
            uuid.time_hi_and_version as u32,
            uuid.clock_seq_and_node[0] as u32,
            uuid.clock_seq_and_node[1] as u32,
            uuid.clock_seq_and_node[2] as u32,
            uuid.clock_seq_and_node[3] as u32,
            uuid.clock_seq_and_node[4] as u32,
            uuid.clock_seq_and_node[5] as u32,
            uuid.clock_seq_and_node[6] as u32,
            uuid.clock_seq_and_node[7] as u32,
        )
    };
    if ret <= 0 {
        tloge!("convert uuid to string failed %d\0", ret);
        return TeeResult::TEE_ERROR_GENERIC;
    }

    return TeeResult::TEE_SUCCESS;
}

fn raw_copy_config(config: &mut config_info, src_config: &config_info) {
    let c = unsafe {
        core::slice::from_raw_parts_mut(
            config as *mut config_info as *mut u8,
            size_of::<config_info>(),
        )
    };
    let src_c = unsafe {
        core::slice::from_raw_parts(
            src_config as *const config_info as *const u8,
            size_of::<config_info>(),
        )
    };
    c.copy_from_slice(src_c);
}

// NOTICE: make config_info as return value in full rust service
pub fn perm_srv_get_config_by_uuid(uuid: &TeeUuid, config: &mut config_info) -> TeeResult {
    let list = G_CONFIG_LIST.lock().unwrap();
    let it = list.iter();
    for entry in it {
        if (*entry).uuid == *uuid {
            raw_copy_config(config, *entry);
            return TeeResult::TEE_SUCCESS;
        }
    }

    return TeeResult::TEE_ERROR_GENERIC;
}

#[cfg(test)]
pub fn perm_srv_add_config(u: &TeeUuid, ci: &callee_ta_info) {
    let config_mem = match TeeMemory::malloc(size_of::<config_info>(), 0) {
        Ok(o) => o,
        Err(_) => return,
    };
    let config = unsafe { &mut *(config_mem.addr() as *mut config_info) };
    config.uuid = u.clone();
    config.control_info.callee_info = ci as *const callee_ta_info as _;
    config.version = 1;
    config.control_info.se_info.permissions = SE_OPEN_SESSION_PERMISSION;
    let mut list = G_CONFIG_LIST.lock().unwrap();
    list.push_back(config);
    core::mem::forget(config_mem);
}

// TODO: make config_info as return value in full rust service
#[no_mangle]
pub extern "C" fn perm_srv_get_config_by_taskid(
    taskid: u32,
    iconfig: Option<&mut config_info>,
) -> TeeResult {
    let mut sndr_uuid = SpawnUuid {
        uuid_valid: 0,
        uuid: TeeUuid {
            time_low: 0,
            time_mid: 0,
            time_hi_and_version: 0,
            clock_seq_and_node: [0u8; 8],
        },
    };
    if let Some(config) = iconfig {
        let ret = hm_getuuid(pid_to_hmpid!(taskid) as i32, &mut sndr_uuid);
        if ret != 0 {
            tloge!("get uuid failed\0");
            return TeeResult::TEE_ERROR_BAD_PARAMETERS;
        }

        let ret = perm_srv_get_config_by_uuid(&sndr_uuid.uuid, config);
        if ret != TeeResult::TEE_SUCCESS {
            tlogd!("cannot find target taskid\0");
        }
        return ret;
    } else {
        tloge!("get config by taskid bad parameter\0");
        return TeeResult::TEE_ERROR_BAD_PARAMETERS;
    }
}

fn check_perm_whitelist(entry: &mut config_info) {
    if check_tui_permission(&entry.uuid) {
        entry.control_info.tui_info.permissions |= TUI_PERMISSION;
    }

    if check_sem_permission(&entry.uuid) {
        entry.control_info.se_info.permissions |= SE_OPEN_SESSION_PERMISSION;
    }
}

fn rpmb_permission_check(entry: &mut config_info) {
    entry.control_info.rpmb_info.permissions = get_rpmb_permission(&entry.uuid);
}

fn rpmb_threshold_check(entry: &mut config_info) {
    entry.control_info.rpmb_info.size = get_rpmb_threshold(&entry.uuid);
}

#[no_mangle]
pub extern "C" fn perm_srv_set_ta_permissions(iuuid: Option<&TeeUuid>) -> TeeResult {
    if let Some(uuid) = iuuid {
        let mut list = G_CONFIG_LIST.lock().unwrap();
        let mut cur = list.cursor_front_mut();
        while let Some(entry) = cur.current() {
            if (*entry).uuid == *uuid {
                return TeeResult::TEE_SUCCESS;
            }
            cur.move_next();
        }
        let config_mem = match TeeMemory::malloc(size_of::<config_info>(), 0) {
            Ok(o) => o,
            Err(e) => return e,
        };
        let config = unsafe { &mut *(config_mem.addr() as *mut config_info) };
        config.uuid = *uuid;
        // config.version = 0;
        // config.control_info.callee_info = 0;

        check_perm_whitelist(config);
        rpmb_permission_check(config);
        rpmb_threshold_check(config);
        cur.push_back(config);
        core::mem::forget(config_mem);
        return TeeResult::TEE_SUCCESS;
    } else {
        return TeeResult::TEE_ERROR_BAD_PARAMETERS;
    }
}

fn del_timer_perm_by_config(config: &config_info) {
    /* there is no dynamic policy for Configs of version 0 */
    if config.version == 0 {
        return;
    }

    let permission = GENERAL_GROUP_PERMISSION;
    if (config.control_info.se_info.permissions & SE_OPEN_SESSION_PERMISSION) != 0 {
        if set_ta_timer_permission(&config.uuid, permission) != 0 {
            tloge!("fail to delete timer group permission\0");
        }
    }
}

#[no_mangle]
pub extern "C" fn perm_srv_clear_ta_permissions(iuuid: Option<&TeeUuid>) {
    if let Some(uuid) = iuuid {
        let mut list = G_CONFIG_LIST.lock().unwrap();
        let mut cur = list.cursor_front_mut();
        while let Some(current) = cur.current() {
            if (*current).uuid == *uuid {
                let old = cur.remove_current().unwrap();
                del_timer_perm_by_config(old);
                release_callee_info(old.control_info.callee_info);
                unsafe {
                    TEE_Free(old as *mut config_info as _);
                }
            }
            cur.move_next();
        }
    } else {
        return;
    }
}
