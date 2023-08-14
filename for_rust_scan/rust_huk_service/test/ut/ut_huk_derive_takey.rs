use core::{mem::MaybeUninit, ptr::null_mut};

use librust_service_ffi::{tee_defines::TeeUuid, TeeResult};

use crate::{
    huk_derive_takey::{
        huk_srv_map_from_task, huk_srv_task_unmap, huk_task_derive_takey,
        huk_task_takey_param_check, INNER_ITER_MAX_NUM, OUTER_ITER_MAX_NUM,
    },
    huk_derive_takey2::huk_task_derive_takey2_iter,
    huk_service_ffi::{huk_srv_msg, huk_srv_rsp, CMAC_DERV_MAX_DATA_IN_SIZE},
};

#[test]
pub fn ut_huk_task_takey_param_check() {
    let uuid0 = TeeUuid::default();
    let mut msg = unsafe { MaybeUninit::<huk_srv_msg>::zeroed().assume_init() };

    let mut ret: TeeResult = huk_task_takey_param_check(&msg, None);
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);
    println!("test for invalid uuid succ");

    ret = huk_task_takey_param_check(&msg, Some(&uuid0));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);
    println!("test for invalid salt_buf succ");

    msg.data.takey_msg.salt_buf = 1;
    msg.data.takey_msg.salt_size = CMAC_DERV_MAX_DATA_IN_SIZE;
    ret = huk_task_takey_param_check(&msg, Some(&uuid0));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);
    println!("test for invalid key_buf succ");

    msg.data.takey_msg.key_buf = 1;
    msg.data.takey_msg.key_size = CMAC_DERV_MAX_DATA_IN_SIZE;
    msg.data.takey_msg.outer_iter_num = 0xffffffff;
    msg.data.takey_msg.inner_iter_num = 0xffffffff;
    ret = huk_task_takey_param_check(&msg, Some(&uuid0));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);
    println!("test for invalid iter_num succ");

    msg.data.takey_msg.outer_iter_num = OUTER_ITER_MAX_NUM;
    msg.data.takey_msg.inner_iter_num = INNER_ITER_MAX_NUM;
    ret = huk_task_takey_param_check(&msg, Some(&uuid0));
    assert_eq!(ret.0, TeeResult::TEE_SUCCESS.0);
    println!("test for huk_task_takey_param_check succ");
}

#[test]
pub fn ut_huk_srv_map_from_task() {
    let u = [0u8; 64];
    let mut vaddr: u64 = 0;
    let ret = huk_srv_map_from_task(0, (&u).as_ptr() as _, 64, &mut vaddr);
    assert_eq!(ret, 0);
    println!("test for huk_srv_map_from_task with right param succ");
    let ret = huk_srv_map_from_task(0, (&u).as_ptr() as _, 0, &mut vaddr);
    assert_eq!(ret, TeeResult::TEE_ERROR_BAD_PARAMETERS.0 as i32);
    println!("test for huk_srv_map_from_task with invalid size succ");
    let ret = huk_srv_map_from_task(0, (&u).as_ptr() as _, 0, null_mut());
    assert_eq!(ret, TeeResult::TEE_ERROR_BAD_PARAMETERS.0 as i32);
    println!("test for huk_srv_map_from_task with invalid vaddr succ");
}

#[test]
pub fn ut_huk_srv_task_unmap() {
    let mut virt_addr = 0;
    let mut size = 1;
    huk_srv_task_unmap(virt_addr, size);
    println!("test for huk_srv_task_unmap with invalid vaddr succ");
    virt_addr = 1;
    size = 0;
    huk_srv_task_unmap(virt_addr, size);
    println!("test for huk_srv_task_unmap with invalid size succ");
    virt_addr = 1;
    size = 64;
    huk_srv_task_unmap(virt_addr, size);
    println!("test for huk_srv_task_unmap succ");
}

#[test]
pub fn ut_huk_task_derive_takey() {
    let mut uuid0 = TeeUuid::default();
    let mut msg = unsafe { MaybeUninit::<huk_srv_msg>::zeroed().assume_init() };
    let mut rsp = unsafe { MaybeUninit::<huk_srv_rsp>::zeroed().assume_init() };

    let ret = huk_task_derive_takey(None, None, 0, Some(&uuid0));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);
    println!("test for huk_task_derive_takey with invalid msg succ");

    msg.data.takey_msg.salt_size = 0;
    let ret = huk_task_derive_takey(Some(&msg), Some(&mut rsp), 0, Some(&uuid0));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);
    println!("test for huk_task_derive_takey with invalid paramcheck succ");

    uuid0.time_low = 1;
    msg.data.takey_msg.key_buf = 1;
    msg.data.takey_msg.key_size = 1;
    let ret = huk_task_derive_takey(Some(&msg), Some(&mut rsp), 0, Some(&uuid0));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);
    println!("test for huk_task_derive_takey with invalid salt succ");

    msg.data.takey_msg.key_buf = 0;
    msg.data.takey_msg.key_size = 0;
    msg.data.takey_msg.salt_size = 1;
    msg.data.takey_msg.salt_buf = 1;
    let ret = huk_task_derive_takey(Some(&msg), Some(&mut rsp), 0, Some(&uuid0));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);
    println!("test for huk_task_derive_takey with invalid key succ");

    let key = [0u8; 64];
    let salt = [0u8; 64];
    msg.data.takey_msg.key_buf = key.as_ptr() as _;
    msg.data.takey_msg.key_size = 64;
    msg.data.takey_msg.salt_size = 64;
    msg.data.takey_msg.salt_buf = salt.as_ptr() as _;
    let ret = huk_task_derive_takey(Some(&msg), Some(&mut rsp), 0, Some(&uuid0));
    assert_eq!(ret.0, 0);
    println!("test for huk_task_derive_takey with right param succ");

    let key = [0u8; 64];
    let salt = [0u8; 64];
    msg.data.takey_msg.key_buf = key.as_ptr() as _;
    msg.data.takey_msg.key_size = 64;
    msg.data.takey_msg.salt_size = 64;
    msg.header.send.msg_id = 1;
    msg.data.takey_msg.salt_buf = salt.as_ptr() as _;
    let ret = huk_task_derive_takey(Some(&msg), Some(&mut rsp), 0, Some(&uuid0));
    assert_eq!(ret.0, 0);
    println!("test for huk_task_derive_takey with diff msg cmd succ");

    msg.header.send.msg_id = 0;
    let key = [0u8; 64];
    let salt = [0u8; 16];
    msg.data.takey_msg.key_buf = key.as_ptr() as _;
    msg.data.takey_msg.key_size = 64;
    msg.data.takey_msg.salt_size = CMAC_DERV_MAX_DATA_IN_SIZE + 1;
    msg.data.takey_msg.salt_buf = salt.as_ptr() as _;
    let ret = huk_task_derive_takey(Some(&msg), Some(&mut rsp), 0, Some(&uuid0));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);
    println!("test for huk_task_derive_takey with invalid salt size succ");

    let key = [0u8; 64];
    let salt = [0u8; 64];
    msg.data.takey_msg.key_buf = key.as_ptr() as _;
    msg.data.takey_msg.key_size = 64;
    msg.data.takey_msg.salt_size = 64;
    msg.data.takey_msg.salt_buf = salt.as_ptr() as _;
    msg.data.takey_msg.inner_iter_num = 0xfff;
    msg.data.takey_msg.outer_iter_num = 0xfff;
    let ret = huk_task_derive_takey(Some(&msg), Some(&mut rsp), 0, Some(&uuid0));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);
    println!("test for huk_task_derive_takey with invalid iter succ");

    let key = [0u8; 64];
    let salt = [0u8; 64];
    msg.data.takey_msg.key_buf = key.as_ptr() as _;
    msg.data.takey_msg.key_size = 64;
    msg.data.takey_msg.salt_size = 2;
    msg.data.takey_msg.salt_buf = salt.as_ptr() as _;
    msg.data.takey_msg.inner_iter_num = 0;
    msg.data.takey_msg.outer_iter_num = 0;
    let ret = huk_task_derive_takey(Some(&msg), Some(&mut rsp), 0, Some(&uuid0));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_GENERIC.0);
    println!("test for huk_task_derive_takey with invalid map salt size succ");

    let key = [0u8; 64];
    let salt = [0u8; 64];
    msg.data.takey_msg.key_buf = key.as_ptr() as _;
    msg.data.takey_msg.key_size = 2;
    msg.data.takey_msg.salt_size = 64;
    msg.data.takey_msg.salt_buf = salt.as_ptr() as _;
    let ret = huk_task_derive_takey(Some(&msg), Some(&mut rsp), 0, Some(&uuid0));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_GENERIC.0);
    println!("test for huk_task_derive_takey with invalid map key size succ");
}

#[test]
pub fn ut_huk_task_derive_takey2_iter() {
    let mut uuid0 = TeeUuid::default();
    let mut msg = unsafe { MaybeUninit::<huk_srv_msg>::zeroed().assume_init() };
    let mut rsp = unsafe { MaybeUninit::<huk_srv_rsp>::zeroed().assume_init() };

    let ret = huk_task_derive_takey2_iter(None, None, 0, Some(&uuid0));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);
    println!("test for huk_task_derive_takey2_iter with invalid msg succ");

    msg.data.takey_msg.salt_size = 0;
    let ret = huk_task_derive_takey2_iter(Some(&msg), Some(&mut rsp), 0, Some(&uuid0));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);
    println!("test for huk_task_derive_takey2_iter with invalid paramcheck succ");

    uuid0.time_low = 1;
    msg.data.takey_msg.key_buf = 1;
    msg.data.takey_msg.key_size = 1;
    let ret = huk_task_derive_takey2_iter(Some(&msg), Some(&mut rsp), 0, Some(&uuid0));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);
    println!("test for huk_task_derive_takey2_iter with invalid salt succ");

    msg.data.takey_msg.key_buf = 0;
    msg.data.takey_msg.key_size = 0;
    msg.data.takey_msg.salt_size = 1;
    msg.data.takey_msg.salt_buf = 1;
    let ret = huk_task_derive_takey2_iter(Some(&msg), Some(&mut rsp), 0, Some(&uuid0));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);
    println!("test for huk_task_derive_takey2_iter with invalid key succ");

    let key = [0u8; 64];
    let salt = [0u8; 64];
    msg.data.takey_msg.key_buf = key.as_ptr() as _;
    msg.data.takey_msg.key_size = 64;
    msg.data.takey_msg.salt_size = 64;
    msg.data.takey_msg.salt_buf = salt.as_ptr() as _;
    let ret = huk_task_derive_takey2_iter(Some(&msg), Some(&mut rsp), 0, Some(&uuid0));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_GENERIC.0);
    println!("test for huk_task_derive_takey2_iter with right invalid iter succ");

    let key = [0u8; 64];
    let salt = [0u8; 64];
    msg.data.takey_msg.key_buf = key.as_ptr() as _;
    msg.data.takey_msg.key_size = 64;
    msg.data.takey_msg.salt_size = 64;
    msg.data.takey_msg.salt_buf = salt.as_ptr() as _;
    msg.data.takey_msg.inner_iter_num = 2;
    msg.data.takey_msg.outer_iter_num = 2;
    let ret = huk_task_derive_takey2_iter(Some(&msg), Some(&mut rsp), 0, Some(&uuid0));
    assert_eq!(ret.0, 0);
    println!("test for huk_task_derive_takey2_iter with right iter succ");

    let key = [0u8; 64];
    let salt = [0u8; 64];
    msg.data.takey_msg.key_buf = key.as_ptr() as _;
    msg.data.takey_msg.key_size = 64;
    msg.data.takey_msg.salt_size = 64;
    msg.data.takey_msg.salt_buf = salt.as_ptr() as _;
    msg.data.takey_msg.inner_iter_num = 0x7;
    msg.data.takey_msg.outer_iter_num = 2;
    let ret = huk_task_derive_takey2_iter(Some(&msg), Some(&mut rsp), 0, Some(&uuid0));
    assert_eq!(ret.0, 0xffffffff);
    println!("test for huk_task_derive_takey2_iter with invalid inner iter succ");

    let key = [0u8; 64];
    let salt = [0u8; 64];
    msg.data.takey_msg.key_buf = key.as_ptr() as _;
    msg.data.takey_msg.key_size = 64;
    msg.data.takey_msg.salt_size = 32;
    msg.data.takey_msg.salt_buf = salt.as_ptr() as _;
    msg.data.takey_msg.inner_iter_num = 2;
    msg.data.takey_msg.outer_iter_num = 2;
    let ret = huk_task_derive_takey2_iter(Some(&msg), Some(&mut rsp), 0, Some(&uuid0));
    assert_eq!(ret.0, 0);
    println!("test for huk_task_derive_takey2_iter with right salt_size succ");

    let key = [0u8; 64];
    let salt = [0u8; 64];
    msg.data.takey_msg.key_buf = key.as_ptr() as _;
    msg.data.takey_msg.key_size = 64;
    msg.data.takey_msg.salt_size = 63;
    msg.data.takey_msg.salt_buf = salt.as_ptr() as _;
    msg.data.takey_msg.inner_iter_num = 2;
    msg.data.takey_msg.outer_iter_num = 2;
    let ret = huk_task_derive_takey2_iter(Some(&msg), Some(&mut rsp), 0, Some(&uuid0));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_OUT_OF_MEMORY.0);
    println!("test for huk_task_derive_takey2_iter with invalid salt_size succ");

    let key = [0u8; 64];
    let salt = [0u8; 64];
    msg.data.takey_msg.key_buf = key.as_ptr() as _;
    msg.data.takey_msg.key_size = 63;
    msg.data.takey_msg.salt_size = 64;
    msg.data.takey_msg.salt_buf = salt.as_ptr() as _;
    msg.data.takey_msg.inner_iter_num = 2;
    msg.data.takey_msg.outer_iter_num = 2;
    let ret = huk_task_derive_takey2_iter(Some(&msg), Some(&mut rsp), 0, Some(&uuid0));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_OUT_OF_MEMORY.0);
    println!("test for huk_task_derive_takey2_iter with invalid key_size succ");

    let key = [0u8; 64];
    let salt = [0u8; 16];
    msg.data.takey_msg.key_buf = key.as_ptr() as _;
    msg.data.takey_msg.key_size = 64;
    msg.data.takey_msg.salt_size = CMAC_DERV_MAX_DATA_IN_SIZE + 1;
    msg.data.takey_msg.salt_buf = salt.as_ptr() as _;
    let ret = huk_task_derive_takey2_iter(Some(&msg), Some(&mut rsp), 0, Some(&uuid0));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);
    println!("test for huk_task_derive_takey2_iter with invalid salt size succ");

    let key = [0u8; 64];
    let salt = [0u8; 64];
    msg.data.takey_msg.key_buf = key.as_ptr() as _;
    msg.data.takey_msg.key_size = 64;
    msg.data.takey_msg.salt_size = 64;
    msg.data.takey_msg.salt_buf = salt.as_ptr() as _;
    msg.data.takey_msg.inner_iter_num = 0xfff;
    msg.data.takey_msg.outer_iter_num = 0xfff;
    let ret = huk_task_derive_takey2_iter(Some(&msg), Some(&mut rsp), 0, Some(&uuid0));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);
    println!("test for huk_task_derive_takey2_iter with invalid iter succ");

    let key = [0u8; 64];
    let salt = [0u8; 64];
    msg.data.takey_msg.key_buf = key.as_ptr() as _;
    msg.data.takey_msg.key_size = 64;
    msg.data.takey_msg.salt_size = 2;
    msg.data.takey_msg.salt_buf = salt.as_ptr() as _;
    msg.data.takey_msg.inner_iter_num = 0;
    msg.data.takey_msg.outer_iter_num = 0;
    let ret = huk_task_derive_takey2_iter(Some(&msg), Some(&mut rsp), 0, Some(&uuid0));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_GENERIC.0);
    println!("test for huk_task_derive_takey2_iter with invalid map salt size succ");

    let key = [0u8; 64];
    let salt = [0u8; 64];
    msg.data.takey_msg.key_buf = key.as_ptr() as _;
    msg.data.takey_msg.key_size = 2;
    msg.data.takey_msg.salt_size = 64;
    msg.data.takey_msg.salt_buf = salt.as_ptr() as _;
    let ret = huk_task_derive_takey2_iter(Some(&msg), Some(&mut rsp), 0, Some(&uuid0));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);
    println!("test for huk_task_derive_takey2_iter with invalid map key size succ");

    let key = [0u8; 64];
    let salt = [0u8; 64];
    msg.data.takey_msg.key_buf = key.as_ptr() as _;
    msg.data.takey_msg.key_size = 65;
    msg.data.takey_msg.salt_size = 64;
    msg.data.takey_msg.salt_buf = salt.as_ptr() as _;
    let ret = huk_task_derive_takey2_iter(Some(&msg), Some(&mut rsp), 0, Some(&uuid0));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_GENERIC.0);
    println!("test for huk_task_derive_takey2_iter with invalid map key size succ");
}
