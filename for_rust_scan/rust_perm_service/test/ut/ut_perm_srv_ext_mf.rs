// Copyright (C) 2023 Huawei Technologies Co., Ltd.
// Licensed under the Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2.
// You may obtain a copy of Mulan PSL v2 at:
//     http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
#[cfg(test)]
use crate::{
    perm_srv_common_ffi::dyn_conf_t,
    perm_srv_elf_verify::ta_lib_img_unpack_ffi::manifest_extension_t,
    perm_srv_ext_mf::tee_load_ext_mf::tee_secure_img_parse_manifest_extension,
};
#[cfg(test)]
use librust_service_ffi::TeeResult;
#[cfg(test)]
use std::{mem::MaybeUninit, ptr::null};

#[test]
pub fn ut_tee_secure_img_parse_manifest_extension() {
    let iextension = null();
    let extension_size = 10;
    let mut imani_ext = unsafe { MaybeUninit::<manifest_extension_t>::zeroed().assume_init() };
    let mut dyn_conf = unsafe { MaybeUninit::<dyn_conf_t>::zeroed().assume_init() };
    let ret = tee_secure_img_parse_manifest_extension(
        iextension,
        extension_size,
        Some(&mut imani_ext),
        Some(&mut dyn_conf),
    );
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);
    println!("test tee_secure_img_parse_manifest_extension with null input succ");

    let test_mani = "gpd.ta.target_type: 0\ngpd.elf.target_version: 1\n\0";
    let iext = test_mani.as_ptr() as *const u8;
    let ret = tee_secure_img_parse_manifest_extension(
        iext,
        test_mani.len() as _,
        None,
        Some(&mut dyn_conf),
    );
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);
    println!("test tee_secure_img_parse_manifest_extension with invalid imani succ");

    let test_mani = "gpd.ta.target_type: 0\ngpd.ta.mem_page_align:true\ngpd.ta.hardWareType:1\n\
    gpd.elf.target_version: 1\ngpd.ta.otrp_flag:false\ngpd.ta.dynConf: {sdhf: hzxdf}\n\ngpd.ta.distribution: 1\n\
    gpd.ta.api_level: \ngpd.sdk.version: 10\ngpd.ta.is_lib:true\ngpd.ta.objectEnumEnable: true\n\0";
    let iext = test_mani.as_ptr() as *const u8;
    let ret = tee_secure_img_parse_manifest_extension(
        iext,
        test_mani.len() as _,
        Some(&mut imani_ext),
        Some(&mut dyn_conf),
    );
    assert_eq!(ret.0, TeeResult::TEE_SUCCESS.0);
    println!("test tee_secure_img_parse_manifest_extension with right params succ");

    let test_mani = "gpd.ta.oooo: true\n\0";
    let iext = test_mani.as_ptr() as *const u8;
    let ret = tee_secure_img_parse_manifest_extension(
        iext,
        test_mani.len() as _,
        Some(&mut imani_ext),
        Some(&mut dyn_conf),
    );
    assert_eq!(ret.0, TeeResult::TEE_SUCCESS.0);
    println!("test tee_secure_img_parse_manifest_extension with unknown params succ");

    let test_mani = "gpd.ta.is_lib:xxxxxx\n\n\0";
    let iext = test_mani.as_ptr() as *const u8;
    let ret = tee_secure_img_parse_manifest_extension(
        iext,
        test_mani.len() as _,
        Some(&mut imani_ext),
        Some(&mut dyn_conf),
    );
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);
    println!("test tee_secure_img_parse_manifest_extension with too long params succ");

    let test_mani = "gpd.elf.target_version:\n\n\0";
    let iext = test_mani.as_ptr() as *const u8;
    let ret = tee_secure_img_parse_manifest_extension(
        iext,
        test_mani.len() as _,
        Some(&mut imani_ext),
        Some(&mut dyn_conf),
    );
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);
    println!("test tee_secure_img_parse_manifest_extension with invalid params succ");

    let test_mani = "gpd.elf.target_version:99999999\n\n\0";
    let iext = test_mani.as_ptr() as *const u8;
    let ret = tee_secure_img_parse_manifest_extension(
        iext,
        test_mani.len() as _,
        Some(&mut imani_ext),
        Some(&mut dyn_conf),
    );
    assert_eq!(ret.0, TeeResult::TEE_ERROR_GENERIC.0);
    println!("test tee_secure_img_parse_manifest_extension with invalid params succ");

    let test_mani = "gpd.elf.autdaf\n99999999\n\n\0";
    let iext = test_mani.as_ptr() as *const u8;
    let ret = tee_secure_img_parse_manifest_extension(
        iext,
        test_mani.len() as _,
        Some(&mut imani_ext),
        Some(&mut dyn_conf),
    );
    assert_eq!(ret.0, TeeResult::TEE_ERROR_GENERIC.0);
    println!("test tee_secure_img_parse_manifest_extension with invalid params succ");

    let test_mani = "gpd.elf.target_version:0\n\n\0";
    let iext = test_mani.as_ptr() as *const u8;
    let ret = tee_secure_img_parse_manifest_extension(
        iext,
        test_mani.len() as _,
        Some(&mut imani_ext),
        Some(&mut dyn_conf),
    );
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);
    println!("test tee_secure_img_parse_manifest_extension with invalid params succ");

    let test_mani = "gpd.ta.auth\n12345678\n\0";
    let iext = test_mani.as_ptr() as *const u8;
    let ret = tee_secure_img_parse_manifest_extension(
        iext,
        test_mani.len() as _,
        Some(&mut imani_ext),
        Some(&mut dyn_conf),
    );
    assert_eq!(ret.0, TeeResult::TEE_ERROR_GENERIC.0);
    println!("test tee_secure_img_parse_manifest_extension with invalid params succ");

    let test_mani = "gpd.ta.auth: 1234567888887491023865412734908\n\0";
    let iext = test_mani.as_ptr() as *const u8;
    let ret = tee_secure_img_parse_manifest_extension(
        iext,
        test_mani.len() as _,
        Some(&mut imani_ext),
        Some(&mut dyn_conf),
    );
    assert_eq!(ret.0, TeeResult::TEE_ERROR_GENERIC.0);
    println!("test tee_secure_img_parse_manifest_extension with invalid params succ");

    let test_mani = "gpd.srv.is_need_release_ta_res:false\ngpd.ta.sys_verify_ta:false\ngpd.ta.auth:\u{1000}71892347192638510273492\n\0";
    let iext = test_mani.as_ptr() as *const u8;
    let ret = tee_secure_img_parse_manifest_extension(
        iext,
        test_mani.len() as _,
        Some(&mut imani_ext),
        Some(&mut dyn_conf),
    );
    assert_eq!(ret.0, TeeResult::TEE_ERROR_GENERIC.0);
    println!("test tee_secure_img_parse_manifest_extension with invalid params succ");

    let test_mani = "gpd.srv.is_need_release_msg:false\ngpd.srv.is_need_create_msg:false\ngpd.srv.crash_callback:true\ngpd.ta.sys_verify_ta:false\ngpd.ta.auth:\u{1000}71892347192638510273492\n\0";
    let iext = test_mani.as_ptr() as *const u8;
    let ret = tee_secure_img_parse_manifest_extension(
        iext,
        test_mani.len() as _,
        Some(&mut imani_ext),
        Some(&mut dyn_conf),
    );
    assert_eq!(ret.0, TeeResult::TEE_ERROR_GENERIC.0);
    println!("test tee_secure_img_parse_manifest_extension with invalid params succ");

    let test_mani = "gpd.srv.is_need_release_msg:false\n\0";
    let iext = test_mani.as_ptr() as *const u8;
    let ret =
        tee_secure_img_parse_manifest_extension(iext, 0, Some(&mut imani_ext), Some(&mut dyn_conf));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);
    println!("test tee_secure_img_parse_manifest_extension with invalid params succ");

    let test_mani = "gpd.srv.is_need_create_msg:false\n\0";
    let iext = test_mani.as_ptr() as *const u8;
    let ret = tee_secure_img_parse_manifest_extension(
        iext,
        test_mani.len() as _,
        Some(&mut imani_ext),
        None,
    );
    assert_eq!(ret.0, TeeResult::TEE_SUCCESS.0);
    println!("test tee_secure_img_parse_manifest_extension with right params succ 1");

    let test_mani = "gpd.ta.auth:";
    let mut buffer = [0u8; 64];
    let pre = &mut buffer[0..test_mani.len()];
    pre.copy_from_slice(test_mani.as_bytes());
    let next = &mut buffer[test_mani.len()..64];
    let len = next.len();
    let mut number = 1;
    let es = &number as *const i32 as *const u8;
    let es_sls = unsafe { core::slice::from_raw_parts(es, 4) };
    (&mut next[0..4]).copy_from_slice(es_sls);
    number = 4;
    (&mut next[4..8]).copy_from_slice(es_sls);
    (&mut next[8..len]).fill(1);
    unsafe {
        *next.get_unchecked_mut(len - 2) = b'\n';
        *next.get_unchecked_mut(len - 1) = 0;
    }

    let iext = buffer.as_ptr() as *const u8;
    let ret = tee_secure_img_parse_manifest_extension(
        iext,
        buffer.len() as _,
        Some(&mut imani_ext),
        Some(&mut dyn_conf),
    );
    assert_eq!(ret.0, TeeResult::TEE_ERROR_GENERIC.0);
    println!(
        "test tee_secure_img_parse_manifest_extension with invalid params succ {}",
        number
    );

    let test_mani = "gpd.ta.auth:";
    let mut buffer = [0u8; 150];
    let pre = &mut buffer[0..test_mani.len()];
    pre.copy_from_slice(test_mani.as_bytes());
    let next = &mut buffer[test_mani.len()..150];
    let len = next.len();
    let mut number = 1;
    let es = &number as *const i32 as *const u8;
    let es_sls = unsafe { core::slice::from_raw_parts(es, 4) };
    (&mut next[0..4]).copy_from_slice(es_sls);
    number = 4;
    (&mut next[4..8]).copy_from_slice(es_sls);
    (&mut next[8..len]).fill(1);
    unsafe {
        *next.get_unchecked_mut(len - 2) = b'\n';
        *next.get_unchecked_mut(len - 1) = 0;
    }

    let iext = buffer.as_ptr() as *const u8;
    let ret = tee_secure_img_parse_manifest_extension(
        iext,
        buffer.len() as _,
        Some(&mut imani_ext),
        Some(&mut dyn_conf),
    );
    assert_eq!(ret.0, TeeResult::TEE_SUCCESS.0);
    println!(
        "test tee_secure_img_parse_manifest_extension with right params succ {}",
        number
    );
}
