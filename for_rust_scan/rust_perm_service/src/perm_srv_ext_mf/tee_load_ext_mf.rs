//!
//! Copyright (c) Huawei Technologies Co., Ltd. 2022-2023. All rights reserved.
//! Description: perm service load ext manifest
//! Create: 2023-03-30
//!

use core::mem::size_of;
use librust_service_ffi::{core::TEE_Malloc, TeeResult};

use crate::{
    perm_srv_common_ffi::{dyn_conf_t, malloc, MAX_IMAGE_LEN},
    perm_srv_elf_verify::{
        ta_lib_img_unpack_ffi::{
            manifest_extension_t, ta_payload_layer_t, CA_CALLER_HASH_LEN, DECIMAL_BASE, HEX_BASE,
            TA_AUTH_XML_HEADER_SIZE, TA_MAX_CALLER_NUM,
        },
        tee_elf_verify_ffi::{get_img_info, get_ta_payload},
    },
    tloge,
};

use super::tee_load_ext_mf_ffi::*;

#[repr(C)]
struct config<'a> {
    key: &'a [u8],
    ty: i32,
}
unsafe impl Sync for config<'_> {}

// Add all valid configurations
static G_VALID_CONFIG: &[config] = &[
    config {
        key: b"gpd.ta.distribution",
        ty: TA_DISTRIBUTION,
    },
    config {
        key: b"gpd.ta.api_level",
        ty: TA_API_LEVEL,
    },
    config {
        key: b"gpd.sdk.version",
        ty: SDK_VERSION,
    },
    config {
        key: b"gpd.ta.is_lib",
        ty: IS_LIB,
    },
    config {
        key: b"gpd.ta.objectEnumEnable",
        ty: SSA_ENUM_ENABLE,
    },
    #[cfg(feature = "config_otrp_support")]
    config {
        key: b"gpd.ta.otrp_flag",
        ty: OTRP_FLAG,
    },
    config {
        key: b"gpd.ta.dynConf",
        ty: IS_DYN_CONF,
    },
    config {
        key: b"gpd.ta.target_type",
        ty: TARGET_TYPE,
    },
    config {
        key: b"gpd.ta.sys_verify_ta",
        ty: SYS_VERIFY_TA,
    },
    config {
        key: b"gpd.elf.target_version",
        ty: TARGET_VERSION,
    },
    #[cfg(feature = "config_adapt_big_memory")]
    config {
        key: b"gpd.ta.mem_page_align",
        ty: MEM_PAGE_ALIGN,
    },
    config {
        key: b"gpd.ta.hardWareType",
        ty: HARD_WARE_TYPE,
    },
    config {
        key: b"gpd.ta.auth",
        ty: TA_AUTH_CONF,
    },
    config {
        key: b"gpd.srv.is_need_release_ta_res",
        ty: SRV_RELEASE_TA_RES,
    },
    config {
        key: b"gpd.srv.crash_callback",
        ty: SRV_CRASH_CALLBACK,
    },
    config {
        key: b"gpd.srv.is_need_create_msg",
        ty: SRV_NEED_CREATE_MSG,
    },
    config {
        key: b"gpd.srv.is_need_release_msg",
        ty: SRV_NEED_RELEASE_MSG,
    },
    #[cfg(feature = "config_livepatch_enable")]
    config {
        key: b"gpd.patch.type",
        ty: PATCH_TYPE,
    },
    #[cfg(feature = "config_livepatch_enable")]
    config {
        key: b"gpd.patch.elf_name",
        ty: ELF_NAME,
    },
    #[cfg(feature = "config_livepatch_enable")]
    config {
        key: b"gpd.patch.livepatch_version",
        ty: LIVEPATCH_VERSION,
    },
    #[cfg(feature = "config_livepatch_enable")]
    config {
        key: b"gpd.patch.module_name",
        ty: MODULE_NAME,
    },
    #[cfg(feature = "config_livepatch_enable")]
    config {
        key: b"gpd.patch.release_version",
        ty: RELEASE_VERSION,
    },
    // config {
    //     key: null(),
    //     ty: UNSUPPORTED,
    // },
];

fn get_conf_type(key: &[u8]) -> i32 {
    for c in G_VALID_CONFIG {
        if c.key == key {
            return c.ty;
        }
    }

    return UNSUPPORTED;
}

fn str_to_bool(s: &[u8], value: &mut bool) -> TeeResult {
    if !matches!(s.len(), 4 | 5) {
        return TeeResult::TEE_ERROR_BAD_PARAMETERS;
    }
    if s.len() == 4
        && matches!(s[0], b't' | b'T')
        && matches!(s[1], b'r' | b'R')
        && matches!(s[2], b'u' | b'U')
        && matches!(s[3], b'e' | b'E')
    {
        *value = true;
    } else {
        *value = false;
    }

    TeeResult::TEE_SUCCESS
}

fn str_to_uint16(s: &[u8], value: &mut u16, base: i32) -> TeeResult {
    if s.len() == 0 {
        return TeeResult::TEE_ERROR_BAD_PARAMETERS;
    }

    let val = strtol(s.as_ptr() as _, 0 as _, base);

    let temp_check = val < 0 || val > 0xffff; // Max value of uint16_t
    if temp_check {
        tloge!("Invalid string for type uint16_t\0");
        return TeeResult::TEE_ERROR_GENERIC;
    }
    *value = val as u16;

    return TeeResult::TEE_SUCCESS;
}

fn is_service_type(ty: i32) -> bool {
    return ty == SRV_RELEASE_TA_RES
        || ty == SRV_CRASH_CALLBACK
        || ty == SRV_NEED_CREATE_MSG
        || ty == SRV_NEED_RELEASE_MSG
        || ty == SYS_VERIFY_TA;
}

fn parse_service_manifest_item(
    ty: i32,
    value: &[u8],
    mani_ext: &mut manifest_extension_t,
) -> TeeResult {
    return match ty {
        SRV_RELEASE_TA_RES => str_to_bool(value, &mut mani_ext.is_need_release_ta_res),
        SRV_CRASH_CALLBACK => str_to_bool(value, &mut mani_ext.crash_callback),
        SRV_NEED_CREATE_MSG => str_to_bool(value, &mut mani_ext.is_need_create_msg),
        SRV_NEED_RELEASE_MSG => str_to_bool(value, &mut mani_ext.is_need_release_msg),
        SYS_VERIFY_TA => str_to_bool(value, &mut mani_ext.sys_verify_ta),
        _ => TeeResult::TEE_ERROR_GENERIC,
    };
}

fn parse_ext_item(ty: i32, value: &[u8], mani_ext: &mut manifest_extension_t) -> TeeResult {
    if is_service_type(ty) {
        return parse_service_manifest_item(ty, value, mani_ext);
    }
    #[cfg(feature = "config_livepatch_enable")]
    if is_livepatch_type(ty) {
        return parse_livepatch_manifest_item(ty, value);
    }

    return TeeResult::TEE_SUCCESS;
}

fn check_auth_info_caller_num(
    caller_num: u32,
    off: u32,
    size: &mut u32,
    left_size: u32,
) -> TeeResult {
    if caller_num > TA_MAX_CALLER_NUM {
        tloge!("invalid ta caller num %u\0", caller_num);
        return TeeResult::TEE_ERROR_BAD_PARAMETERS;
    }

    if caller_num * CA_CALLER_HASH_LEN < (*size - off - size_of::<u32>() as u32) {
        tloge!(
            "caller info hash too long, %u, %u, %u\0",
            caller_num,
            *size,
            *size - off - size_of::<u32>() as u32
        );
        return TeeResult::TEE_ERROR_GENERIC;
    }

    if caller_num * CA_CALLER_HASH_LEN > (*size - off - size_of::<u32>() as u32) {
        /* modify auth info size when hash include '\n' */
        if left_size >= off + size_of::<u32>() as u32 + caller_num * CA_CALLER_HASH_LEN {
            *size = off + size_of::<u32>() as u32 + caller_num * CA_CALLER_HASH_LEN;
        } else {
            tloge!(
                "caller info hash too short, %u, %u, %u\0",
                caller_num,
                *size,
                *size - off - size_of::<u32>() as u32
            );
            return TeeResult::TEE_ERROR_GENERIC;
        }
    }

    return TeeResult::TEE_SUCCESS;
}

fn set_ta_auth_info(start: &[u8], size: &mut u32, left_size: u32) -> TeeResult {
    let img_info = unsafe { &mut *get_img_info() };

    let mut off: u32 = 0;
    /* gpd.ta.auth: || auth_enable || caller_num || caller_hash */
    if *size < TA_AUTH_KEY.len() as u32 + TA_AUTH_XML_HEADER_SIZE {
        tloge!("ta auth info too short, %u\0", *size);
        return TeeResult::TEE_ERROR_GENERIC;
    }

    off += TA_AUTH_KEY.len() as u32;
    let auth_enable: u32 = unsafe { *((start.as_ptr() as u64 + off as u64) as *const u32) };
    if auth_enable != 0 && auth_enable != 1 {
        tloge!("invalid ta auth config, %u\0", auth_enable);
        return TeeResult::TEE_ERROR_GENERIC;
    }

    off += size_of::<u32>() as u32;
    let caller_num: u32 = unsafe { *((start.as_ptr() as u64 + off as u64) as *const u32) };
    if check_auth_info_caller_num(caller_num, off, size, left_size) != TeeResult::TEE_SUCCESS {
        tloge!("invalid caller num\0");
        return TeeResult::TEE_ERROR_GENERIC;
    }

    /* auth info is sent to TA by shared memory, not included in manifest_ext buf and ta_property.other_buff. */
    let ta_payload: &mut ta_payload_layer_t = unsafe { &mut *get_ta_payload() };
    if *size > ta_payload.payload_hdr.mani_ext_size {
        tloge!(
            "auth info is larger than manifest ext size, %u, %u\0",
            *size,
            ta_payload.payload_hdr.mani_ext_size
        );
        return TeeResult::TEE_ERROR_GENERIC;
    }
    ta_payload.payload_hdr.mani_ext_size -= *size;

    if caller_num == 0 {
        img_info.manifest.ta_auth.auth_enable = auth_enable != 0;
        img_info.manifest.ta_auth.caller_num = caller_num;
        img_info.manifest.ta_auth.caller_hash = 0 as _;
        return TeeResult::TEE_SUCCESS;
    }

    let caller_hash_len = (caller_num * CA_CALLER_HASH_LEN) as usize;
    /* caller_hash is freed in free_verify. */
    img_info.manifest.ta_auth.caller_hash = unsafe { TEE_Malloc(caller_hash_len, 0) } as _;
    if img_info.manifest.ta_auth.caller_hash.is_null() {
        tloge!("failed to malloc memory for ta auth\0");
        return TeeResult::TEE_ERROR_OUT_OF_MEMORY;
    }

    off += size_of::<u32>() as u32;
    let dest: &mut [u8] = unsafe {
        core::slice::from_raw_parts_mut(img_info.manifest.ta_auth.caller_hash, caller_hash_len)
    };
    dest.copy_from_slice(&start[off as usize..(off as usize + caller_hash_len)]);

    img_info.manifest.ta_auth.auth_enable = auth_enable != 0;
    img_info.manifest.ta_auth.caller_num = caller_num;

    return TeeResult::TEE_SUCCESS;
}

fn set_dyn_conf_size(idyn_conf: &mut Option<&mut dyn_conf_t>) -> TeeResult {
    /* if dyn_conf is NULL, means that is not support dyn conf */
    if let Some(dyn_conf) = idyn_conf {
        dyn_conf.dyn_conf_size = 1;
    }
    return TeeResult::TEE_SUCCESS;
}

fn index_of_slice(buff: &[u8], c: u8) -> usize {
    for index in 0..buff.len() {
        if buff[index] == c {
            return index;
        }
    }
    return buff.len();
}

pub const EXTENSION_MAX: u32 = 64;
fn tee_secure_img_parse_manifest_item(
    start: &[u8],
    size: &mut u32,
    left_size: u32,
    mani_ext: &mut manifest_extension_t,
    dyn_conf: &mut Option<&mut dyn_conf_t>,
) -> TeeResult {
    // Skip the empty line of manifest extension configuration
    if *size == 0 {
        return TeeResult::TEE_SUCCESS;
    }
    let s = u32::min(*size, EXTENSION_MAX - 1);
    let item = &start[0..s as usize];

    let mut index = index_of_slice(&item, b':');
    // Get the key of the item
    let name = &item[0..index];
    let ty = get_conf_type(name);
    // Get the value of the item
    if index == item.len() && ty != TA_AUTH_CONF {
        return TeeResult::TEE_ERROR_GENERIC;
    }
    if index != item.len() {
        index += 1;
    }
    let value = &item[index..item.len()];

    match ty {
        TA_DISTRIBUTION => return str_to_uint16(value, &mut mani_ext.distribution, HEX_BASE),
        TA_API_LEVEL => return str_to_uint16(value, &mut mani_ext.api_level, HEX_BASE),
        SDK_VERSION => return str_to_uint16(value, &mut mani_ext.sdk_version, HEX_BASE),
        IS_LIB => return str_to_bool(value, &mut mani_ext.is_lib),
        SSA_ENUM_ENABLE => return str_to_bool(value, &mut mani_ext.ssa_enum_enable),
        OTRP_FLAG => return str_to_bool(value, &mut mani_ext.otrp_flag),
        IS_DYN_CONF => return set_dyn_conf_size(dyn_conf),
        TARGET_TYPE => return str_to_uint16(value, &mut mani_ext.target_type, HEX_BASE),
        TARGET_VERSION => {
            let mut ret = str_to_uint16(value, &mut mani_ext.target_version, DECIMAL_BASE);
            if ret == TeeResult::TEE_SUCCESS && mani_ext.target_version == 0 {
                tloge!("target version : 0 is not valid\0");
                ret = TeeResult::TEE_ERROR_BAD_PARAMETERS;
            }
            return ret;
        }
        MEM_PAGE_ALIGN => return str_to_bool(value, &mut mani_ext.mem_page_align),
        HARD_WARE_TYPE => return str_to_uint16(value, &mut mani_ext.hardware_type, DECIMAL_BASE),
        TA_AUTH_CONF => return set_ta_auth_info(start, size, left_size),
        _ => return parse_ext_item(ty, value, mani_ext),
    }
}

fn set_dyn_conf(start: &[u8], conf: &mut Option<&mut dyn_conf_t>) -> TeeResult {
    if let Some(dyn_conf) = conf {
        /*
         * if dyn_conf is NULL, means that is not support dyn conf
         * if dyn_conf->dyn_conf_buffer is not NULL, means dyn_conf_buffer has already been set_dyn_conf
         * if dyn_conf_size is 0, means we haven't find gpd.ta.dynConf flag yet
         */
        if !dyn_conf.dyn_conf_buffer.is_null() || dyn_conf.dyn_conf_size == 0 {
            return TeeResult::TEE_SUCCESS;
        }
        let len = start.len();

        if len <= DYN_CONF_START.len() || len >= MAX_IMAGE_LEN as usize {
            tloge!("dyn conf size is invalied\0");
            return TeeResult::TEE_ERROR_GENERIC;
        }

        /* end - start is dyn_conf total size, we must del 'gpd.ta.dynConf:' from it */
        dyn_conf.dyn_conf_size = (len - DYN_CONF_START.len()) as u32;
        dyn_conf.dyn_conf_buffer = unsafe { malloc(dyn_conf.dyn_conf_size as _) } as _;
        if dyn_conf.dyn_conf_buffer.is_null() {
            tloge!("failed to load dyn conf buffer\0");
            return TeeResult::TEE_ERROR_GENERIC;
        }

        let dest = unsafe {
            core::slice::from_raw_parts_mut(dyn_conf.dyn_conf_buffer, dyn_conf.dyn_conf_size as _)
        };
        /* copy the dyn conf buffer, we should ignore 'gpd.ta.dynConf:', and copy rest of it */
        dest.copy_from_slice(
            &start[DYN_CONF_START.len()..(DYN_CONF_START.len() + dyn_conf.dyn_conf_size as usize)],
        );

        return TeeResult::TEE_SUCCESS;
    } else {
        return TeeResult::TEE_SUCCESS;
    }
}

fn check_extention_process_params(extension_size: u32) -> bool {
    if extension_size > MAX_IMAGE_LEN {
        tloge!("manifest extent size too large: %u\0", extension_size);
        return true;
    }

    if extension_size == 0 {
        return true;
    }

    return false;
}

#[no_mangle]
pub extern "C" fn tee_secure_img_parse_manifest_extension(
    iextension: *const u8,
    extension_size: u32,
    imani_ext: Option<&mut manifest_extension_t>,
    dyn_conf: Option<&mut dyn_conf_t>,
) -> TeeResult {
    let mut ret;

    if check_extention_process_params(extension_size) || iextension.is_null() {
        return TeeResult::TEE_ERROR_BAD_PARAMETERS;
    }

    let mut conf: Option<&mut dyn_conf_t> = match dyn_conf {
        Some(c) => Some(c),
        None => None,
    };

    if let Some(mani_ext) = imani_ext {
        let extension = unsafe { core::slice::from_raw_parts(iextension, extension_size as usize) };

        let mut start: usize = 0;
        let mut end = index_of_slice(extension, b'\n');
        while (end as u32) < extension_size {
            let mut size: u32 = (end - start) as u32;

            ret = tee_secure_img_parse_manifest_item(
                &extension[start..extension_size as usize],
                &mut size,
                extension_size - start as u32,
                mani_ext,
                &mut conf,
            );
            if ret != TeeResult::TEE_SUCCESS {
                tloge!(
                    "Failed to parse manifest extension item: %.*s\0",
                    size,
                    (&extension[start..end]).as_ptr() as u64
                );
                return ret;
            }
            ret = set_dyn_conf(&extension[start..end], &mut conf);
            if ret != TeeResult::TEE_SUCCESS {
                return ret;
            }

            start += size as usize + 1;
            if start as u32 >= extension_size {
                return TeeResult::TEE_SUCCESS;
            }

            end = index_of_slice(&extension[start..extension_size as usize], b'\n') + start;
        }

        return TeeResult::TEE_SUCCESS;
    } else {
        return TeeResult::TEE_ERROR_BAD_PARAMETERS;
    }
}
