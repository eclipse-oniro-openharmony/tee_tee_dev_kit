use std::{
    mem::{size_of, MaybeUninit},
    ptr::{null, null_mut},
};

use librust_service_ffi::{tee_defines::TeeUuid, TeeResult};

use crate::{
    perm_srv_common_ffi::{
        dyn_conf_t, get_img_info, malloc, CLIENT_TARGET_TYPE, DRV_TARGET_TYPE, SRV_TARGET_TYPE,
        TA_TARGET_TYPE,
    },
    perm_srv_elf_verify::{
        perm_srv_elf_verify_cmd::{perm_srv_elf_verify, perm_srv_ta_run_authorization_check},
        ta_lib_img_unpack_ffi::{
            ta_cipher_layer_t, ta_payload_layer_t, ta_property_t, CIPHER_LAYER_KEY_V1,
            CIPHER_LAYER_KEY_V2, CIPHER_LAYER_KEY_V3, ECC256_SIGNATURE_SIZE,
            HARDWARE_ENGINE_CRYPTO, HARDWARE_TIMER_MGR, RSA4096_SIGNATURE_SIZE,
            SIGNATURE_SIZE_INVALID, SIGN_ALGO_ECC_256, SIGN_ALGO_RSA_2048, SIGN_ALGO_RSA_4096,
            SIGN_ALG_HASH_MASK,
        },
        tee_comm_elf_verify_ffi::{
            drv_mani_t, sign_config_t, TeeObjectHandleVar, PUB_KEY_RELEASE, SIGN_SEC_ALG_DEFAULT,
            SIGN_SEC_ALG_ECDSA,
        },
        tee_comm_elf_verify_v3v5::{
            alloc_name_buffer_copy_mani_conf, check_img_format_valid, check_manifest_alloc_name,
            do_ta_image_verify, free_global_res, get_img_header, get_sign_config,
            get_signature_verify_key, get_sub_cert_from_certchain, get_ta_cipher_layer_len,
            handle_cipher_layer_len, handle_drv_mani, handle_dyn_conf_buffer, overflow_check,
            secure_img_copy_rsp_v3v5, set_cipher_layer, set_drv_manifest,
            tee_sec_img_payload_decrypt_ops, tee_secure_get_img_header_v3v5,
            tee_secure_img_decrypt_payload, tee_secure_img_get_signature_size,
            tee_secure_img_header_check_v3v5, tee_secure_img_parse_cipher_layer,
            tee_secure_img_parse_payload, tee_secure_img_proc_cipher_layer,
            tee_secure_img_proc_payload, tee_secure_img_signature_verify,
        },
        tee_elf_verify_ffi::{elf_verify_reply, elf_verify_req, leaf_cert},
    },
    perm_srv_ta_config::{
        perm_srv_add_config,
        ta_config_builder_ffi::{callee_ta_info, MAX_SERVICE_NAME_LEN},
    },
    permission_service_ffi::{cert_param_t, perm_srv_reply_msg_t, perm_srv_req_msg_t},
    ut::stub::set_payload_format,
};

use super::stub::set_img_info_v5;

#[test]
pub fn ut_perm_srv_elf_verify() {
    let ret = perm_srv_elf_verify(None, 0, None, None);
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);

    let msg = unsafe { MaybeUninit::<perm_srv_req_msg_t>::zeroed().assume_init() };
    let mut rsp = unsafe { MaybeUninit::<perm_srv_reply_msg_t>::zeroed().assume_init() };
    let uuid = unsafe { MaybeUninit::<TeeUuid>::zeroed().assume_init() };
    let ret = perm_srv_elf_verify(Some(&msg), 1, Some(&uuid), Some(&mut rsp));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);

    let msg = unsafe { MaybeUninit::<perm_srv_req_msg_t>::zeroed().assume_init() };
    let mut rsp = unsafe { MaybeUninit::<perm_srv_reply_msg_t>::zeroed().assume_init() };
    let uuid = unsafe { MaybeUninit::<TeeUuid>::zeroed().assume_init() };
    let ret = perm_srv_elf_verify(Some(&msg), 0, Some(&uuid), Some(&mut rsp));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);
    println!("test for perm_srv_elf_verify succ 1");

    let mut msg = unsafe { MaybeUninit::<perm_srv_req_msg_t>::zeroed().assume_init() };
    let mut rsp = unsafe { MaybeUninit::<perm_srv_reply_msg_t>::zeroed().assume_init() };
    let uuid = unsafe { MaybeUninit::<TeeUuid>::zeroed().assume_init() };
    msg.header.send.msg_size = core::mem::size_of::<elf_verify_req>() as _;
    let ret = perm_srv_elf_verify(Some(&msg), 0, Some(&uuid), Some(&mut rsp));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_GENERIC.0);

    let mut msg = unsafe { MaybeUninit::<perm_srv_req_msg_t>::zeroed().assume_init() };
    let mut rsp = unsafe { MaybeUninit::<perm_srv_reply_msg_t>::zeroed().assume_init() };
    let uuid = unsafe { MaybeUninit::<TeeUuid>::zeroed().assume_init() };
    msg.header.send.msg_size = core::mem::size_of::<elf_verify_req>() as _;
    let mut auth = [0u8; 32];
    msg.req_msg.verify_req.auth_share_addr = &mut auth as *mut u8 as _;
    msg.req_msg.verify_req.auth_share_size = 32;
    let ret = perm_srv_elf_verify(Some(&msg), 0, Some(&uuid), Some(&mut rsp));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_GENERIC.0);

    let mut msg = unsafe { MaybeUninit::<perm_srv_req_msg_t>::zeroed().assume_init() };
    let mut rsp = unsafe { MaybeUninit::<perm_srv_reply_msg_t>::zeroed().assume_init() };
    let uuid = unsafe { MaybeUninit::<TeeUuid>::zeroed().assume_init() };
    msg.header.send.msg_size = core::mem::size_of::<elf_verify_req>() as _;
    let mut auth = [0u8; 32];
    msg.req_msg.verify_req.auth_share_addr = &mut auth as *mut u8 as _;
    msg.req_msg.verify_req.auth_share_size = 32;
    msg.req_msg.verify_req.reply_share_addr = &mut auth as *mut u8 as _;
    msg.req_msg.verify_req.reply_share_size = 32;
    let ret = perm_srv_elf_verify(Some(&msg), 2, Some(&uuid), Some(&mut rsp));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_COMMUNICATION.0);

    let mut msg = unsafe { MaybeUninit::<perm_srv_req_msg_t>::zeroed().assume_init() };
    let mut rsp = unsafe { MaybeUninit::<perm_srv_reply_msg_t>::zeroed().assume_init() };
    let uuid = unsafe { MaybeUninit::<TeeUuid>::zeroed().assume_init() };
    msg.header.send.msg_size = core::mem::size_of::<elf_verify_req>() as _;
    let mut auth = [0u8; 32];
    msg.req_msg.verify_req.auth_share_addr = &mut auth as *mut u8 as _;
    msg.req_msg.verify_req.auth_share_size = 32;
    msg.req_msg.verify_req.reply_share_addr = &mut auth as *mut u8 as _;
    msg.req_msg.verify_req.reply_share_size = 32;
    let ret = perm_srv_elf_verify(Some(&msg), 0, Some(&uuid), Some(&mut rsp));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);

    let mut msg = unsafe { MaybeUninit::<perm_srv_req_msg_t>::zeroed().assume_init() };
    let mut rsp = unsafe { MaybeUninit::<perm_srv_reply_msg_t>::zeroed().assume_init() };
    let uuid = unsafe { MaybeUninit::<TeeUuid>::zeroed().assume_init() };
    msg.header.send.msg_size = core::mem::size_of::<elf_verify_req>() as _;
    let mut auth = [0u8; 32];
    msg.req_msg.verify_req.auth_share_addr = &mut auth as *mut u8 as _;
    msg.req_msg.verify_req.auth_share_size = 32;
    msg.req_msg.verify_req.reply_share_addr = &mut auth as *mut u8 as _;
    msg.req_msg.verify_req.reply_share_size = 32;
    msg.req_msg.verify_req.img_size = 1;
    let ret = perm_srv_elf_verify(Some(&msg), 0, Some(&uuid), Some(&mut rsp));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_ACCESS_DENIED.0);
    println!("test for perm_srv_elf_verify succ");

    let mut msg = unsafe { MaybeUninit::<perm_srv_req_msg_t>::zeroed().assume_init() };
    let mut rsp = unsafe { MaybeUninit::<perm_srv_reply_msg_t>::zeroed().assume_init() };
    let uuid = unsafe { MaybeUninit::<TeeUuid>::zeroed().assume_init() };
    msg.header.send.msg_size = core::mem::size_of::<elf_verify_req>() as _;
    let mut auth = [0u8; 32];
    msg.req_msg.verify_req.auth_share_addr = &mut auth as *mut u8 as _;
    msg.req_msg.verify_req.auth_share_size = 32;
    msg.req_msg.verify_req.reply_share_addr = &mut auth as *mut u8 as _;
    msg.req_msg.verify_req.reply_share_size = 32;
    msg.req_msg.verify_req.img_size = 1;
    msg.req_msg.verify_req.version = 2;
    let ret = perm_srv_elf_verify(Some(&msg), 0, Some(&uuid), Some(&mut rsp));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_ACCESS_DENIED.0);
    println!("test for perm_srv_elf_verify succ");

    let mut msg = unsafe { MaybeUninit::<perm_srv_req_msg_t>::zeroed().assume_init() };
    let mut rsp = unsafe { MaybeUninit::<perm_srv_reply_msg_t>::zeroed().assume_init() };
    let uuid = unsafe { MaybeUninit::<TeeUuid>::zeroed().assume_init() };
    msg.header.send.msg_size = core::mem::size_of::<elf_verify_req>() as _;
    let mut auth = [0u8; 32];
    msg.req_msg.verify_req.auth_share_addr = &mut auth as *mut u8 as _;
    msg.req_msg.verify_req.auth_share_size = 32;
    msg.req_msg.verify_req.reply_share_addr = &mut auth as *mut u8 as _;
    msg.req_msg.verify_req.reply_share_size = 32;
    msg.req_msg.verify_req.img_size = 1;
    let ret = perm_srv_elf_verify(Some(&msg), 0, Some(&uuid), Some(&mut rsp));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_ACCESS_DENIED.0);
    println!("test for perm_srv_elf_verify succ");

    let mut msg = unsafe { MaybeUninit::<perm_srv_req_msg_t>::zeroed().assume_init() };
    let mut rsp = unsafe { MaybeUninit::<perm_srv_reply_msg_t>::zeroed().assume_init() };
    let uuid = unsafe { MaybeUninit::<TeeUuid>::zeroed().assume_init() };
    msg.header.send.msg_size = core::mem::size_of::<elf_verify_req>() as _;
    let mut auth = [0u8; 32];
    msg.req_msg.verify_req.auth_share_addr = &mut auth as *mut u8 as _;
    msg.req_msg.verify_req.auth_share_size = 32;
    msg.req_msg.verify_req.reply_share_addr = &mut auth as *mut u8 as _;
    msg.req_msg.verify_req.reply_share_size = 32;
    msg.req_msg.verify_req.img_size = 1;
    msg.req_msg.verify_req.version = 2;
    let ret = perm_srv_elf_verify(Some(&msg), 0, Some(&uuid), Some(&mut rsp));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_ACCESS_DENIED.0);
    println!("test for perm_srv_elf_verify succ");
}

#[test]
pub fn ut_perm_srv_ta_run_authorization_check() {
    let uuid = TeeUuid {
        time_low: 0,
        time_mid: 0,
        time_hi_and_version: 0,
        clock_seq_and_node: [0u8; 8],
    };
    let manifest = unsafe { MaybeUninit::<ta_property_t>::zeroed().assume_init() };
    let target_version = 0;
    let mem_page_align = false;
    let ret = perm_srv_ta_run_authorization_check(&uuid, &manifest, target_version, mem_page_align);
    assert_eq!(ret.0, TeeResult::TEE_ERROR_GENERIC.0);

    let uuid = TeeUuid {
        time_low: 0,
        time_mid: 0,
        time_hi_and_version: 0,
        clock_seq_and_node: [0u8; 8],
    };
    let manifest = unsafe { MaybeUninit::<ta_property_t>::zeroed().assume_init() };
    let target_version = 2;
    let mem_page_align = false;
    let ret = perm_srv_ta_run_authorization_check(&uuid, &manifest, target_version, mem_page_align);
    assert_eq!(ret.0, TeeResult::TEE_ERROR_GENERIC.0);

    let uuid = TeeUuid {
        time_low: 0,
        time_mid: 0,
        time_hi_and_version: 0,
        clock_seq_and_node: [0u8; 8],
    };
    let manifest = unsafe { MaybeUninit::<ta_property_t>::zeroed().assume_init() };
    let target_version = 1;
    let mem_page_align = false;
    let ccmd = unsafe { malloc(8) };
    let p = unsafe { core::slice::from_raw_parts_mut(ccmd as *mut u32, 2) };
    p.fill(1);
    let c = unsafe { malloc(core::mem::size_of::<callee_ta_info>()) };
    let ci = unsafe { &mut *(c as *mut callee_ta_info) };
    ci.next = 0;
    ci.uuid = uuid.clone();
    ci.command_num = 2;
    ci.command_id = p.as_ptr() as _;
    perm_srv_add_config(&uuid, ci);
    let ret = perm_srv_ta_run_authorization_check(&uuid, &manifest, target_version, mem_page_align);
    assert_eq!(ret.0, 0);

    let uuid = TeeUuid {
        time_low: 0,
        time_mid: 0,
        time_hi_and_version: 0,
        clock_seq_and_node: [0u8; 8],
    };
    let mut manifest = unsafe { MaybeUninit::<ta_property_t>::zeroed().assume_init() };
    let target_version = 1;
    let mem_page_align = false;
    manifest.heap_size = 5000;
    let ret = perm_srv_ta_run_authorization_check(&uuid, &manifest, target_version, mem_page_align);
    assert_eq!(ret.0, TeeResult::TEE_ERROR_GENERIC.0);
    println!("test for perm_srv_ta_run_authorization_check succ");
}

#[test]
pub fn ut_get_sub_cert_from_certchain() {
    let ret = get_sub_cert_from_certchain(null(), 0, None, None);
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);

    let cert = b"xxxxxxxx";
    let ret = get_sub_cert_from_certchain(cert.as_ptr(), cert.len() as _, None, None);
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);

    let mut ilc = unsafe { MaybeUninit::<leaf_cert>::zeroed().assume_init() };
    let mut sec = unsafe { MaybeUninit::<leaf_cert>::zeroed().assume_init() };
    let ret = get_sub_cert_from_certchain(
        cert.as_ptr(),
        cert.len() as _,
        Some(&mut ilc),
        Some(&mut sec),
    );
    assert_eq!(ret.0, TeeResult::TEE_ERROR_OUT_OF_MEMORY.0);

    let mut ilc = unsafe { MaybeUninit::<leaf_cert>::zeroed().assume_init() };
    ilc.cert_buf = cert.as_ptr() as _;
    ilc.cert_buf_len = cert.len() as _;
    let mut sec = unsafe { MaybeUninit::<leaf_cert>::zeroed().assume_init() };
    sec.cert_buf = cert.as_ptr() as _;
    sec.cert_buf_len = cert.len() as _;
    let o = [0u8; 64];
    let ret = get_sub_cert_from_certchain(o.as_ptr(), 64, Some(&mut ilc), Some(&mut sec));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_OUT_OF_MEMORY.0);

    let mut ilc = unsafe { MaybeUninit::<leaf_cert>::zeroed().assume_init() };
    ilc.cert_buf = cert.as_ptr() as _;
    ilc.cert_buf_len = cert.len() as _;
    let mut sec = unsafe { MaybeUninit::<leaf_cert>::zeroed().assume_init() };
    sec.cert_buf = cert.as_ptr() as _;
    sec.cert_buf_len = cert.len() as _;
    let o = b"\x00\x00\x00\x100000000000000000\x00\x00\x00\x0800000000";
    let ret = get_sub_cert_from_certchain(o.as_ptr(), 64, Some(&mut ilc), Some(&mut sec));
    assert_eq!(ret.0, TeeResult::TEE_SUCCESS.0);

    let mut ilc = unsafe { MaybeUninit::<leaf_cert>::zeroed().assume_init() };
    ilc.cert_buf = cert.as_ptr() as _;
    ilc.cert_buf_len = cert.len() as _;
    let mut sec = unsafe { MaybeUninit::<leaf_cert>::zeroed().assume_init() };
    sec.cert_buf = cert.as_ptr() as _;
    sec.cert_buf_len = cert.len() as _;
    let o = b"\x00\x00\x00\x100000000000000000\x00\x00\x20\x0800000000";
    let ret = get_sub_cert_from_certchain(o.as_ptr(), o.len() as _, Some(&mut ilc), Some(&mut sec));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_GENERIC.0);

    let mut ilc = unsafe { MaybeUninit::<leaf_cert>::zeroed().assume_init() };
    ilc.cert_buf = cert.as_ptr() as _;
    ilc.cert_buf_len = cert.len() as _;
    let mut sec = unsafe { MaybeUninit::<leaf_cert>::zeroed().assume_init() };
    sec.cert_buf = cert.as_ptr() as _;
    sec.cert_buf_len = cert.len() as _;
    let o = b"\x00\x00\x00\x100000000000000000";
    let ret = get_sub_cert_from_certchain(o.as_ptr(), o.len() as _, Some(&mut ilc), Some(&mut sec));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_GENERIC.0);

    let mut ilc = unsafe { MaybeUninit::<leaf_cert>::zeroed().assume_init() };
    ilc.cert_buf = cert.as_ptr() as _;
    ilc.cert_buf_len = cert.len() as _;
    let mut sec = unsafe { MaybeUninit::<leaf_cert>::zeroed().assume_init() };
    sec.cert_buf = cert.as_ptr() as _;
    sec.cert_buf_len = cert.len() as _;
    let o = b"\x00\x00\x00\x100000000000000000\x00\x00\x00\x000000";
    let ret = get_sub_cert_from_certchain(o.as_ptr(), 64, Some(&mut ilc), Some(&mut sec));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_OUT_OF_MEMORY.0);
}

#[test]
pub fn ut_secure_img_copy_rsp_v3v5() {
    let ret = secure_img_copy_rsp_v3v5(None);
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);

    let mut ireq = unsafe { MaybeUninit::<elf_verify_reply>::zeroed().assume_init() };
    let ret = secure_img_copy_rsp_v3v5(Some(&mut ireq));
    assert_eq!(ret.0, 0);

    let mut ireq = unsafe { MaybeUninit::<elf_verify_reply>::zeroed().assume_init() };
    ireq.otrp_ta = true;
    let ret = secure_img_copy_rsp_v3v5(Some(&mut ireq));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_GENERIC.0);
    println!("test for secure_img_copy_rsp_v3v5 succ");
}

#[test]
pub fn ut_free_global_res() {
    free_global_res();

    set_cipher_layer(0, 8, 8);
    free_global_res();
    println!("test for free_global_res succ");
}

#[test]
pub fn ut_tee_secure_img_signature_verify() {
    let plain = [0u8; 16];
    let mut sig = [0u8; 16];
    let mut hash_data = [0u8; 16];
    let ret = tee_secure_img_signature_verify(
        null(),
        plain.len() as _,
        sig.as_mut_ptr() as _,
        sig.len() as _,
        hash_data.as_mut_ptr() as _,
    );
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);

    let plain = [0u8; 16];
    let mut sig = [0u8; 16];
    let mut hash_data = [0u8; 16];
    let ret = tee_secure_img_signature_verify(
        plain.as_ptr() as _,
        0,
        sig.as_mut_ptr() as _,
        sig.len() as _,
        hash_data.as_mut_ptr() as _,
    );
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);

    let plain = [0u8; 16];
    let sig = [0u8; 16];
    let mut hash_data = [0u8; 16];
    let ret = tee_secure_img_signature_verify(
        plain.as_ptr() as _,
        plain.len() as _,
        null_mut(),
        sig.len() as _,
        hash_data.as_mut_ptr() as _,
    );
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);

    let plain = [0u8; 16];
    let mut sig = [0u8; 16];
    let mut hash_data = [0u8; 16];
    let ret = tee_secure_img_signature_verify(
        plain.as_ptr() as _,
        plain.len() as _,
        sig.as_mut_ptr() as _,
        0,
        hash_data.as_mut_ptr() as _,
    );
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);

    let mut plain = [0u8; 16];
    let mut sig = [0u8; 16];
    let mut hash_data = [0u8; 16];
    let ret = tee_secure_img_signature_verify(
        plain.as_mut_ptr() as _,
        plain.len() as _,
        sig.as_mut_ptr() as _,
        sig.len() as _,
        hash_data.as_mut_ptr() as _,
    );
    assert_eq!(ret.0, 0);

    set_cipher_layer(SIGN_ALGO_RSA_2048 as _, 8, 8);
    let mut plain = [0u8; 16];
    let mut sig = [0u8; 16];
    let mut hash_data = [0u8; 16];
    let ret = tee_secure_img_signature_verify(
        plain.as_mut_ptr() as _,
        plain.len() as _,
        sig.as_mut_ptr() as _,
        sig.len() as _,
        hash_data.as_mut_ptr() as _,
    );
    assert_eq!(ret.0, 0);

    set_cipher_layer(0 as _, 8, 8);
    let mut plain = [0u8; 16];
    let mut sig = [0u8; 16];
    let mut hash_data = [0u8; 16];
    let ret = tee_secure_img_signature_verify(
        plain.as_mut_ptr() as _,
        plain.len() as _,
        sig.as_mut_ptr() as _,
        sig.len() as _,
        hash_data.as_mut_ptr() as _,
    );
    assert_eq!(ret.0, 0);
}

#[test]
pub fn ut_do_ta_image_verify() {
    let mut sig = [0u8; 32];
    let mut hash = [0u8; 32];
    let mut config = unsafe { MaybeUninit::<sign_config_t>::zeroed().assume_init() };
    let mut key: u64 = 32;
    let ret = do_ta_image_verify(&mut sig, &mut hash, &config, &mut key);
    assert_eq!(ret.0, TeeResult::TEE_SUCCESS.0);

    config.sign_ta_alg = SIGN_SEC_ALG_ECDSA;
    let ret = do_ta_image_verify(&mut sig, &mut hash, &config, &mut key);
    assert_eq!(ret.0, TeeResult::TEE_ERROR_GENERIC.0);

    config.sign_ta_alg = 0;
    let sig = unsafe {
        let addr = 1 as u64 as *mut u8;
        core::slice::from_raw_parts_mut(addr, 16)
    };
    let ret = do_ta_image_verify(sig, &mut hash, &config, &mut key);
    assert_eq!(ret.0, TeeResult::TEE_ERROR_GENERIC.0);
    println!("test for do_ta_image_verify succ");
}

#[test]
pub fn ut_get_signature_verify_key() {
    set_img_info_v5();
    let mut cert_param = unsafe { MaybeUninit::<cert_param_t>::zeroed().assume_init() };
    let config = unsafe { MaybeUninit::<sign_config_t>::zeroed().assume_init() };
    let mut key: u64 = 32;
    let mut dyyn = false;
    let ret = get_signature_verify_key(&mut key, &config, &mut cert_param, &mut dyyn);
    assert_eq!(ret.0, TeeResult::TEE_SUCCESS.0);
    println!("test for get_signature_verify_key succ");
}

#[test]
pub fn ut_git_sign_config() {
    let _ = get_img_header();
    let ret = get_ta_cipher_layer_len();
    assert_eq!(ret, 0);
    let mut config = unsafe { MaybeUninit::<sign_config_t>::zeroed().assume_init() };
    get_sign_config(&mut config);

    set_cipher_layer(SIGN_ALGO_RSA_4096, 8, 8);
    get_sign_config(&mut config);

    set_cipher_layer(SIGN_ALGO_ECC_256, 8, 8);
    get_sign_config(&mut config);

    set_cipher_layer(SIGN_ALGO_ECC_256 | SIGN_ALG_HASH_MASK, 8, 8);
    println!("test for get_sign_config succ");
}

#[test]
pub fn ut_overflow_check() {
    let ret = overflow_check(1, 1);
    assert_eq!(ret, false);
    let ret = overflow_check(u32::MAX, u32::MAX);
    assert_eq!(ret, true);
    println!("test for overflow_check succ");
}

#[test]
pub fn ut_handle_cipher_layer_len() {
    let ver = CIPHER_LAYER_KEY_V1;
    let ret = handle_cipher_layer_len(ver);
    assert_eq!(ret.0, TeeResult::TEE_SUCCESS.0);

    let ver = CIPHER_LAYER_KEY_V2;
    let ret = handle_cipher_layer_len(ver);
    assert_eq!(ret.0, TeeResult::TEE_SUCCESS.0);

    let ver = CIPHER_LAYER_KEY_V3;
    let ret = handle_cipher_layer_len(ver);
    assert_eq!(ret.0, TeeResult::TEE_SUCCESS.0);

    let ver = 0xff;
    let ret = handle_cipher_layer_len(ver);
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);
    println!("test for handle_cipher_layer_len succ");
}

#[test]
pub fn ut_check_img_format_valid() {
    let mut config = unsafe { MaybeUninit::<sign_config_t>::zeroed().assume_init() };
    let ret = check_img_format_valid(&mut config);
    assert_eq!(ret, true);

    set_payload_format();

    let ret = check_img_format_valid(&config);
    assert_eq!(ret, false);

    config.key_style = PUB_KEY_RELEASE;
    config.sign_ta_alg = SIGN_SEC_ALG_DEFAULT;
    let ret = check_img_format_valid(&config);
    assert_eq!(ret, false);
    println!("test for check_img_format_valid succ");
}

#[test]
pub fn ut_tee_secure_img_header_check_v3v5() {
    let ret = tee_secure_img_header_check_v3v5();
    assert_eq!(ret.0, TeeResult::TEE_SUCCESS.0);
    println!("test for tee_secure_img_header_check_v3v5 succ");
}

#[test]
pub fn ut_get_img_header() {
    let ret = tee_secure_get_img_header_v3v5(null(), 0);
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);

    let buf_len = 0;
    let buf = b"xxxxx";
    let ret = tee_secure_get_img_header_v3v5(buf.as_ptr(), buf_len);
    assert_eq!(ret.0, TeeResult::TEE_ERROR_GENERIC.0);

    let buf = b"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx";
    let buf_len = buf.len() as u32;
    let ret = tee_secure_get_img_header_v3v5(buf.as_ptr(), buf_len);
    assert_eq!(ret.0, TeeResult::TEE_SUCCESS.0);
    println!("test for tee_secure_get_img_header_v3v5 succ");
}

#[test]
pub fn ut_tee_secure_img_parse_cipher_layer() {
    let plain = [0u8; 64];
    let mut ci = unsafe { MaybeUninit::<ta_cipher_layer_t>::zeroed().assume_init() };
    let ret = tee_secure_img_parse_cipher_layer(&plain, &mut ci);
    assert_eq!(ret.0, 0);

    let plain = [0u8; 2];
    let mut ci = unsafe { MaybeUninit::<ta_cipher_layer_t>::zeroed().assume_init() };
    let ret = tee_secure_img_parse_cipher_layer(&plain, &mut ci);
    assert_eq!(ret.0, TeeResult::TEE_ERROR_GENERIC.0);

    let mut plain = [0u8; 64];
    let mut ci = unsafe { MaybeUninit::<ta_cipher_layer_t>::zeroed().assume_init() };
    ci.cipher_hdr.key_size = u32::MAX - 1;
    unsafe {
        let addr = &ci as *const ta_cipher_layer_t as u64;
        let slice = core::slice::from_raw_parts(
            addr as *const u8,
            core::mem::size_of::<ta_cipher_layer_t>(),
        );
        (&mut plain[0..slice.len()]).copy_from_slice(slice);
    }
    let ret = tee_secure_img_parse_cipher_layer(&plain, &mut ci);
    assert_eq!(ret.0, TeeResult::TEE_ERROR_GENERIC.0);

    let mut plain = [0u8; 64];
    let mut ci = unsafe { MaybeUninit::<ta_cipher_layer_t>::zeroed().assume_init() };
    ci.cipher_hdr.key_size = 64;
    unsafe {
        let addr = &ci as *const ta_cipher_layer_t as u64;
        let slice = core::slice::from_raw_parts(
            addr as *const u8,
            core::mem::size_of::<ta_cipher_layer_t>(),
        );
        (&mut plain[0..slice.len()]).copy_from_slice(slice);
    }
    let ret = tee_secure_img_parse_cipher_layer(&plain, &mut ci);
    assert_eq!(ret.0, TeeResult::TEE_ERROR_GENERIC.0);

    let mut plain = [0u8; 64];
    let mut ci = unsafe { MaybeUninit::<ta_cipher_layer_t>::zeroed().assume_init() };
    ci.cipher_hdr.key_size = 8;
    ci.cipher_hdr.iv_size = u32::MAX - 1;
    unsafe {
        let addr = &ci as *const ta_cipher_layer_t as u64;
        let slice = core::slice::from_raw_parts(
            addr as *const u8,
            core::mem::size_of::<ta_cipher_layer_t>(),
        );
        (&mut plain[0..slice.len()]).copy_from_slice(slice);
    }
    let ret = tee_secure_img_parse_cipher_layer(&plain, &mut ci);
    assert_eq!(ret.0, TeeResult::TEE_ERROR_GENERIC.0);

    let mut plain = [0u8; 64];
    let mut ci = unsafe { MaybeUninit::<ta_cipher_layer_t>::zeroed().assume_init() };
    ci.cipher_hdr.key_size = 8;
    ci.cipher_hdr.iv_size = 64;
    unsafe {
        let addr = &ci as *const ta_cipher_layer_t as u64;
        let slice = core::slice::from_raw_parts(
            addr as *const u8,
            core::mem::size_of::<ta_cipher_layer_t>(),
        );
        (&mut plain[0..slice.len()]).copy_from_slice(slice);
    }
    let ret = tee_secure_img_parse_cipher_layer(&plain, &mut ci);
    assert_eq!(ret.0, TeeResult::TEE_ERROR_GENERIC.0);
}

#[test]
pub fn ut_tee_secure_img_proc_cipher_layer() {
    let img_size = 0;
    let mut off: u32 = 0;
    let mut layer = 0u32;
    let ret =
        tee_secure_img_proc_cipher_layer(null_mut(), img_size, &mut off as _, &mut layer as _);
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);

    let mut buf = [0u8; 128];
    let ret = tee_secure_img_proc_cipher_layer(
        buf.as_mut_ptr() as _,
        buf.len() as _,
        &mut off as _,
        &mut layer as _,
    );
    assert_eq!(ret.0, TeeResult::TEE_ERROR_GENERIC.0);

    let mut buf = [0u8; 128];
    let mut off = 8u32;
    let ret =
        tee_secure_img_proc_cipher_layer(buf.as_mut_ptr() as _, 1, &mut off as _, &mut layer as _);
    assert_eq!(ret.0, TeeResult::TEE_ERROR_GENERIC.0);

    let mut buf = [0u8; 64];
    let mut off: u32 = 0;
    let mut layer = 64u32;
    let ret = tee_secure_img_proc_cipher_layer(
        buf.as_mut_ptr() as _,
        buf.len() as _,
        &mut off as _,
        &mut layer as _,
    );
    assert_eq!(ret.0, 0);
    println!("test for tee_secure_img_proc_cipher_layer succ");
}

#[test]
pub fn ut_tee_secure_img_get_signature_size() {
    let sig = [0u8; 64];
    let ret =
        tee_secure_img_get_signature_size(SIGN_ALGO_RSA_2048, sig.as_ptr() as _, sig.len() as _);
    assert_eq!(ret, 256);

    let sig = [0u8; 64];
    let ret = tee_secure_img_get_signature_size(SIGN_ALGO_RSA_2048, null(), sig.len() as _);
    assert_eq!(ret, SIGNATURE_SIZE_INVALID);

    let sig = [0u8; 64];
    let ret =
        tee_secure_img_get_signature_size(SIGN_ALGO_RSA_4096, sig.as_ptr() as _, sig.len() as _);
    assert_eq!(ret, RSA4096_SIGNATURE_SIZE);

    let sig = [0u8; 64];
    let ret =
        tee_secure_img_get_signature_size(SIGN_ALGO_ECC_256, sig.as_ptr() as _, sig.len() as _);
    assert_eq!(ret, ECC256_SIGNATURE_SIZE);

    let sig = [0u8; 64];
    let ret = tee_secure_img_get_signature_size(0xff, sig.as_ptr() as _, sig.len() as _);
    assert_eq!(ret, SIGNATURE_SIZE_INVALID);
    println!("test for tee_secure_img_get_signature_size succ");
}

#[test]
pub fn ut_tee_sec_img_payload_decrypt_ops() {
    let mut key_obj = unsafe { MaybeUninit::<TeeObjectHandleVar>::zeroed().assume_init() };
    let src = [0u8; 4096];
    let mut dst = [0u8; 4096];
    let mut len = dst.len() as u32;
    let ret = tee_sec_img_payload_decrypt_ops(&mut key_obj, &src, &mut dst, &mut len);
    assert_eq!(ret.0, TeeResult::TEE_SUCCESS.0);
    println!("test for tee_sec_img_payload_decrypt_ops succ");
}

#[test]
pub fn ut_tee_secure_img_decrypt_payload() {
    let text = [0u8; 4096];
    let mut dst = [0u8; 4096];
    let mut len = 4096;
    let ret = tee_secure_img_decrypt_payload(&text, &mut dst, &mut len);
    assert_eq!(ret.0, 0);

    let text = [0u8; 4096];
    let mut dst = [0u8; 4096];
    let mut len = 0;
    let ret = tee_secure_img_decrypt_payload(&text, &mut dst, &mut len);
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);

    println!("test for tee_secure_img_decrypt_payload succ");
}

#[test]
pub fn ut_alloc_name_buffer_copy_mani_conf() {
    let mut mani = [0u8; 128];
    let ret = alloc_name_buffer_copy_mani_conf(&mani);
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_FORMAT.0);

    let mut p = unsafe { MaybeUninit::<ta_property_t>::zeroed().assume_init() };
    p.single_instance = 1;
    let tap = &p as *const ta_property_t as *const u8;
    let sls = unsafe { core::slice::from_raw_parts(tap, core::mem::size_of::<ta_property_t>()) };
    (&mut mani[104..(104 + core::mem::size_of::<ta_property_t>())]).copy_from_slice(sls);

    let ret = alloc_name_buffer_copy_mani_conf(&mani);
    assert_eq!(ret.0, 0);
    println!("test for alloc_name_buffer_copy_mani_conf succ");
}

#[test]
pub fn ut_handle_drv_mani() {
    let mut mani = unsafe { MaybeUninit::<drv_mani_t>::zeroed().assume_init() };
    handle_drv_mani(&mut mani);

    mani.hardware_type = HARDWARE_ENGINE_CRYPTO;
    handle_drv_mani(&mut mani);

    mani.hardware_type = HARDWARE_TIMER_MGR;
    handle_drv_mani(&mut mani);
    println!("test for handle_drv_mani succ");
}

#[test]
pub fn ut_set_drv_manifest() {
    let mut mani = unsafe { MaybeUninit::<drv_mani_t>::zeroed().assume_init() };
    let ret = set_drv_manifest(&mut mani);
    assert_eq!(ret.0, 0);
    mani.hardware_type = HARDWARE_ENGINE_CRYPTO;
    let ret = set_drv_manifest(&mut mani);
    assert_eq!(ret.0, 0);
    mani.hardware_type = HARDWARE_TIMER_MGR;
    let ret = set_drv_manifest(&mut mani);
    assert_eq!(ret.0, 0);
    println!("test for set_drv_manifest succ");
}

#[test]
pub fn ut_check_manifest_alloc_name() {
    let mani = [0u8; 2];
    let ret = check_manifest_alloc_name(&mani);
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);

    let mani = [0u8; 256];
    let ret = check_manifest_alloc_name(&mani);
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);

    let mani = [0u8; size_of::<TeeUuid>() + size_of::<ta_property_t>() + MAX_SERVICE_NAME_LEN - 1];
    let ret = check_manifest_alloc_name(&mani);
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);

    let mut mani =
        [0u8; size_of::<TeeUuid>() + size_of::<ta_property_t>() + MAX_SERVICE_NAME_LEN - 1];
    let mut p = unsafe { MaybeUninit::<ta_property_t>::zeroed().assume_init() };
    p.single_instance = 1;
    let tap = &p as *const ta_property_t as *const u8;
    let sls = unsafe { core::slice::from_raw_parts(tap, core::mem::size_of::<ta_property_t>()) };
    (&mut mani[55..(55 + core::mem::size_of::<ta_property_t>())]).copy_from_slice(sls);
    let ret = check_manifest_alloc_name(&mani);
    assert_eq!(ret.0, 0);
    println!("test for check_manifest_alloc_name succ");
}

#[test]
pub fn ut_handle_dyn_conf_buffer() {
    let mut d = unsafe { MaybeUninit::<dyn_conf_t>::zeroed().assume_init() };
    let mut ext_size = size_of::<dyn_conf_t>() as u32;
    let ret = handle_dyn_conf_buffer(&mut d, &mut ext_size);
    assert_eq!(ret, 0);

    let mut d = unsafe { MaybeUninit::<dyn_conf_t>::zeroed().assume_init() };
    d.dyn_conf_size = 1024;
    let mut ext_size = size_of::<dyn_conf_t>() as u32;
    let ret = handle_dyn_conf_buffer(&mut d, &mut ext_size);
    assert_eq!(ret, TeeResult::TEE_ERROR_GENERIC.0 as i32);

    let mut imginfo = unsafe { &mut *get_img_info() };
    imginfo.manifest.ext.target_type = DRV_TARGET_TYPE as _;
    let mut d = unsafe { MaybeUninit::<dyn_conf_t>::zeroed().assume_init() };
    let mut ext_size = size_of::<dyn_conf_t>() as u32;
    let ret = handle_dyn_conf_buffer(&mut d, &mut ext_size);
    assert_eq!(ret, 0);

    imginfo.manifest.ext.target_type = TA_TARGET_TYPE as _;
    let mut d = unsafe { MaybeUninit::<dyn_conf_t>::zeroed().assume_init() };
    let mut ext_size = size_of::<dyn_conf_t>() as u32;
    let ret = handle_dyn_conf_buffer(&mut d, &mut ext_size);
    assert_eq!(ret, 0);

    imginfo.manifest.ext.target_type = SRV_TARGET_TYPE as _;
    let mut d = unsafe { MaybeUninit::<dyn_conf_t>::zeroed().assume_init() };
    let mut ext_size = size_of::<dyn_conf_t>() as u32;
    let ret = handle_dyn_conf_buffer(&mut d, &mut ext_size);
    assert_eq!(ret, 0);

    imginfo.manifest.ext.target_type = CLIENT_TARGET_TYPE as _;
    let mut d = unsafe { MaybeUninit::<dyn_conf_t>::zeroed().assume_init() };
    let mut ext_size = size_of::<dyn_conf_t>() as u32;
    let ret = handle_dyn_conf_buffer(&mut d, &mut ext_size);
    assert_eq!(ret, 0);

    imginfo.manifest.ext.target_type = 0xff;
    let mut d = unsafe { MaybeUninit::<dyn_conf_t>::zeroed().assume_init() };
    let mut ext_size = size_of::<dyn_conf_t>() as u32;
    let ret = handle_dyn_conf_buffer(&mut d, &mut ext_size);
    assert_eq!(ret, TeeResult::TEE_ERROR_GENERIC.0 as i32);
    println!("test for handle_dyn_conf_buffer succ");
}

#[test]
pub fn ut_tee_secure_img_parse_manifest() {
    let mut mani = [0u8; 1024];
    let mut size = 1024;
    let ret = crate::perm_srv_elf_verify::tee_comm_elf_verify_v3v5::tee_secure_img_parse_manifest(
        mani.as_mut_ptr() as _,
        &mut size,
        false,
        1,
    );
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);

    let mut mani = [0u8; 1024];
    let mut size = 1024;
    let ret = crate::perm_srv_elf_verify::tee_comm_elf_verify_v3v5::tee_secure_img_parse_manifest(
        mani.as_mut_ptr() as _,
        &mut size,
        true,
        0,
    );
    assert_eq!(ret.0, TeeResult::TEE_SUCCESS.0);

    let mut mani = [0u8; 1024];
    let mut size = 1024;
    let ret = crate::perm_srv_elf_verify::tee_comm_elf_verify_v3v5::tee_secure_img_parse_manifest(
        mani.as_mut_ptr() as _,
        &mut size,
        true,
        0,
    );
    assert_eq!(ret.0, TeeResult::TEE_SUCCESS.0);

    let mut imginfo = unsafe { &mut *get_img_info() };
    imginfo.manifest.service_name = unsafe { malloc(10) as _ };
    imginfo.manifest.mani_info.service_name_len = 10;
    let mut mani = [0u8; 1023];
    let mut size = 1023;
    let ret = crate::perm_srv_elf_verify::tee_comm_elf_verify_v3v5::tee_secure_img_parse_manifest(
        mani.as_mut_ptr() as _,
        &mut size,
        true,
        0,
    );
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);

    let mut mani = [0u8; 1025];
    let mut size = 1025;
    let ret = crate::perm_srv_elf_verify::tee_comm_elf_verify_v3v5::tee_secure_img_parse_manifest(
        mani.as_mut_ptr() as _,
        &mut size,
        true,
        0,
    );
    assert_eq!(ret.0, TeeResult::TEE_SUCCESS.0);
    println!("test for tee_secure_img_parse_manifest succ");
}

#[test]
pub fn ut_tee_secure_img_parse_payload() {
    let mut plain = [0u8; 4096];
    let mut pay = unsafe { MaybeUninit::<ta_payload_layer_t>::zeroed().assume_init() };
    let ret = tee_secure_img_parse_payload(&mut plain, &mut pay);
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);

    let mut plain = [0u8; 10];
    let mut pay = unsafe { MaybeUninit::<ta_payload_layer_t>::zeroed().assume_init() };
    let ret = tee_secure_img_parse_payload(&mut plain, &mut pay);
    assert_eq!(ret.0, TeeResult::TEE_ERROR_GENERIC.0);

    let mut plain = [0u8; 50];
    let mut pay = unsafe { MaybeUninit::<ta_payload_layer_t>::zeroed().assume_init() };
    let ret = tee_secure_img_parse_payload(&mut plain, &mut pay);
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);

    let mut plain = [0u8; 100];
    let mut pay = unsafe { MaybeUninit::<ta_payload_layer_t>::zeroed().assume_init() };
    let ret = tee_secure_img_parse_payload(&mut plain, &mut pay);
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);

    let mut plain = [0u8; 150];
    let mut pay = unsafe { MaybeUninit::<ta_payload_layer_t>::zeroed().assume_init() };
    let ret = tee_secure_img_parse_payload(&mut plain, &mut pay);
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);

    let mut plain = [0u8; 200];
    let mut pay = unsafe { MaybeUninit::<ta_payload_layer_t>::zeroed().assume_init() };
    let ret = tee_secure_img_parse_payload(&mut plain, &mut pay);
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);
    println!("test succ");

    let mut mani = [0u8; 99];
    let mut p = unsafe { MaybeUninit::<ta_property_t>::zeroed().assume_init() };
    p.single_instance = 1;
    let tap = &p as *const ta_property_t as *const u8;
    let sls = unsafe { core::slice::from_raw_parts(tap, core::mem::size_of::<ta_property_t>()) };
    (&mut mani[75..(75 + core::mem::size_of::<ta_property_t>())]).copy_from_slice(sls);
    let mut pay = unsafe { MaybeUninit::<ta_payload_layer_t>::zeroed().assume_init() };
    pay.payload_hdr.mani_info_size = 79;
    let ret = tee_secure_img_parse_payload(&mut mani, &mut pay);
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);
    println!("tst for tee_secure_img_parse_payload succ");
}

#[test]
pub fn ut_tee_secure_img_proc_payload() {
    let mut img = [0u8; 2048];
    let mut ps = 0u32;
    let ret = tee_secure_img_proc_payload(img.as_mut_ptr() as _, img.len() as _, 0, 192, &mut ps);
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_FORMAT.0);

    let img = [0u8; 2048];
    let mut ps = 0u32;
    let ret = tee_secure_img_proc_payload(null_mut(), img.len() as _, 0, 192, &mut ps);
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);

    let mut img = [0u8; 2048];
    let mut ps = 192u32;
    let ret = tee_secure_img_proc_payload(img.as_mut_ptr() as _, 10, 0, 192, &mut ps);
    assert_eq!(ret.0, TeeResult::TEE_ERROR_GENERIC.0);

    let mut mani = [0u8; 1024];
    let mut payh = unsafe { MaybeUninit::<ta_payload_layer_t>::zeroed().assume_init() };
    payh.payload_hdr.mani_info_size = 100;
    payh.payload_hdr.mani_ext_size = 100;
    let s = unsafe {
        let addr = &mut payh as *mut ta_payload_layer_t as *mut u8;
        core::slice::from_raw_parts_mut(addr, size_of::<ta_payload_layer_t>())
    };
    (&mut mani[0..size_of::<ta_payload_layer_t>()]).copy_from_slice(s);
    let ret = tee_secure_img_proc_payload(mani.as_mut_ptr() as _, mani.len() as _, 0, 0, &mut ps);
    assert_eq!(ret.0, 4294901760);
    println!("tst for tee_secure_img_parse_payload succ");
}
