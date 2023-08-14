//!
//! Copyright (c) Huawei Technologies Co., Ltd. 2022-2023. All rights reserved.
//! Description: perm service mani ext ffi
//! Create: 2023-03-30
//!

use librust_service_ffi::TeeResult;
pub const TA_AUTH_KEY: &[u8] = b"gpd.ta.auth\0";
pub const DYN_CONF_START: &[u8] = b"gpd.ta.dynConf\0";

// enum for mani ext
pub const UNSUPPORTED: i32 = 0;
pub const TA_DISTRIBUTION: i32 = 1;
pub const TA_API_LEVEL: i32 = 2;
pub const SDK_VERSION: i32 = 3;
pub const IS_LIB: i32 = 4;
pub const SSA_ENUM_ENABLE: i32 = 5;
pub const OTRP_FLAG: i32 = 6;
pub const IS_DYN_CONF: i32 = 7;
pub const TARGET_TYPE: i32 = 8;
pub const TARGET_VERSION: i32 = 9;
pub const SYS_VERIFY_TA: i32 = 10;
pub const MEM_PAGE_ALIGN: i32 = 11;
pub const HARD_WARE_TYPE: i32 = 12;
pub const TA_AUTH_CONF: i32 = 13;
pub const SRV_RELEASE_TA_RES: i32 = 14;
pub const SRV_CRASH_CALLBACK: i32 = 15;
pub const SRV_NEED_CREATE_MSG: i32 = 16;
pub const SRV_NEED_RELEASE_MSG: i32 = 17;
pub const PATCH_TYPE: i32 = 18;
pub const ELF_NAME: i32 = 19;
pub const LIVEPATCH_VERSION: i32 = 20;
pub const MODULE_NAME: i32 = 21;
pub const RELEASE_VERSION: i32 = 22;

pub fn strtol(restrict: *const u8, __restrict: *const *const u8, base: i32) -> i64 {
    unsafe { crate::perm_srv_common_ffi::strtol(restrict as _, __restrict as _, base) }
}

pub fn is_livepatch_type(ty: i32) -> bool {
    unsafe { crate::perm_srv_common_ffi::is_livepatch_type(ty) }
}

pub fn parse_livepatch_manifest_item(ty: i32, value: &[u8]) -> TeeResult {
    unsafe {
        crate::perm_srv_common_ffi::parse_livepatch_manifest_item(
            ty,
            value.as_ptr() as _,
            value.len() as _,
        )
    }
}
