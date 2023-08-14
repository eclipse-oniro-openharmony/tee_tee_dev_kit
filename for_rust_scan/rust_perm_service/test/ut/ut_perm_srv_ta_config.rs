use crate::{
    perm_srv_common_ffi::{malloc, TEE_SERVICE_SE},
    perm_srv_elf_verify::tee_elf_verify_ffi::SN_MAX_SIZE,
    perm_srv_ta_config::{
        asc2hex, byte_to_integer, config_tlv_parser_basic_info, config_tlv_parser_control_info,
        config_tlv_parser_info, config_tlv_parser_manifest_info, config_tlv_parser_rpmb_info,
        config_tlv_parser_rpmb_permission, config_tlv_value_to_manifest_info,
        get_byte_value_from_buff, get_tag_uuid, is_duplicate_callee, parse_dyntlv_buf,
        parser_callee_info, parser_callee_ta_command_id, parser_callee_ta_uuid, parser_fill_callee,
        parser_ta_manager, perm_srv_add_config, perm_srv_clear_ta_permissions,
        perm_srv_convert_str_to_uuid, perm_srv_convert_uuid_to_str, perm_srv_get_config_by_taskid,
        perm_srv_parse_config_body, perm_srv_set_ta_permissions,
        perm_srv_ta_config_cmd::perm_srv_query_ta2ta_perm,
        perm_srv_ta_config_ffi::{
            ipc_msg_snd, perm_config, TLV_TAG_HEAP_SIZE, TLV_TAG_INSTANCE_KEEP_ALIVE,
            TLV_TAG_MEM_PAGE_ALIGN, TLV_TAG_MULTI_SESSION, TLV_TAG_SINGLE_INSTANCE,
            TLV_TAG_STACK_SIZE, TLV_TAG_SYS_VERIFY_TA, TLV_TAG_TARGET_TYPE,
        },
        perm_srv_update_config_by_same_uuid,
        ta_config_builder_ffi::{
            callee_ta_info, config_info, RPMB_GENERAL_PERMISSION, RPMB_RESET_PERMISSION,
        },
        tlv_tag_rpmb_general, tlv_tag_rpmb_specific, ASN1_INTEGER, ASN1_SEQUENCE,
        TA_MANAGER_TRUSTONIC, XML2TLV_PY_VALUE,
    },
    permission_service_ffi::{
        CHECK_BY_UUID, PERM_TYPE_MANAGE_INFO, PERM_TYPE_RPMB_CAPABILITY, PERM_TYPE_RPMB_SIZE,
        PERM_TYPE_SE_CAPABILITY, PERM_TYPE_SFS_CAPABILITY, PERM_TYPE_TUI_CAPABILITY,
        TA_RELEASE_CERT,
    },
};
#[cfg(test)]
use crate::{
    perm_srv_ta_config::perm_srv_ta_config_cmd::perm_srv_query_perms,
    permission_service_ffi::{perm_srv_reply_msg_t, perm_srv_req_msg_t, CHECK_BY_TASKID},
};
#[cfg(test)]
use librust_service_ffi::{tee_defines::TeeUuid, TeeResult};
#[cfg(test)]
use std::mem::MaybeUninit;

#[test]
pub fn ut_perm_srv_query_perms() {
    let imsg = None;
    let sndr_taskid = 0;
    let mut isndr_uuid = unsafe { MaybeUninit::<TeeUuid>::zeroed().assume_init() };
    let irsp = None;
    let ret = perm_srv_query_perms(imsg, sndr_taskid, Some(&isndr_uuid), irsp);
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);
    println!("test for perm_srv_query_perms with invalid param succ");

    let mut irsp = unsafe { MaybeUninit::<perm_srv_reply_msg_t>::zeroed().assume_init() };
    let ret = perm_srv_query_perms(imsg, sndr_taskid, Some(&isndr_uuid), Some(&mut irsp));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);
    println!("test for perm_srv_query_perms with invalid param succ");

    let mut msg = unsafe { MaybeUninit::<perm_srv_req_msg_t>::zeroed().assume_init() };
    let ret = perm_srv_query_perms(Some(&msg), sndr_taskid, Some(&isndr_uuid), Some(&mut irsp));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_ACCESS_DENIED.0);
    println!("test for perm_srv_query_perms with invalid param succ");

    msg.req_msg.query_perms.checkby = 2;
    let ret = perm_srv_query_perms(Some(&msg), sndr_taskid, Some(&isndr_uuid), Some(&mut irsp));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);
    println!("test for perm_srv_query_perms with invalid param succ");

    msg.req_msg.query_perms.checkby = CHECK_BY_TASKID;
    let ret = perm_srv_query_perms(Some(&msg), sndr_taskid, Some(&isndr_uuid), Some(&mut irsp));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_ACCESS_DENIED.0);
    println!("test for perm_srv_query_perms with invalid param succ");

    isndr_uuid.time_low = 1;
    let _ = perm_srv_set_ta_permissions(Some(&isndr_uuid));
    isndr_uuid.time_low = 2;
    let _ = perm_srv_set_ta_permissions(Some(&isndr_uuid));
    isndr_uuid.time_low = 1;
    let _ = perm_srv_set_ta_permissions(Some(&isndr_uuid));
    let _ = perm_srv_set_ta_permissions(None);
    msg.req_msg.query_perms.checkby = CHECK_BY_UUID;
    msg.req_msg.query_perms.uuid = isndr_uuid.clone();
    let ret = perm_srv_query_perms(Some(&msg), sndr_taskid, Some(&isndr_uuid), Some(&mut irsp));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_ACCESS_DENIED.0);
    println!("test for perm_srv_query_perms with invalid param succ");

    msg.req_msg.query_perms.perm_type = PERM_TYPE_RPMB_SIZE;
    let ret = perm_srv_query_perms(Some(&msg), sndr_taskid, Some(&isndr_uuid), Some(&mut irsp));
    assert_eq!(ret.0, TeeResult::TEE_SUCCESS.0);
    println!("test for perm_srv_query_perms with right param succ");

    let ret = perm_srv_query_perms(Some(&msg), 1, Some(&isndr_uuid), Some(&mut irsp));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_ACCESS_DENIED.0);
    println!("test for perm_srv_query_perms with invalid param succ");

    msg.req_msg.query_perms.perm_type = PERM_TYPE_MANAGE_INFO;
    let ret = perm_srv_query_perms(Some(&msg), 1, Some(&isndr_uuid), Some(&mut irsp));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_ACCESS_DENIED.0);
    println!("test for perm_srv_query_perms with invalid param succ");

    let ret = perm_srv_query_perms(Some(&msg), sndr_taskid, Some(&isndr_uuid), Some(&mut irsp));
    assert_eq!(ret.0, TeeResult::TEE_SUCCESS.0);
    println!("test for perm_srv_query_perms with right param succ");

    msg.req_msg.query_perms.perm_type = PERM_TYPE_SE_CAPABILITY;
    let ret = perm_srv_query_perms(Some(&msg), sndr_taskid, Some(&isndr_uuid), Some(&mut irsp));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_ACCESS_DENIED.0);
    println!("test for perm_srv_query_perms with invalid param succ");

    isndr_uuid = TEE_SERVICE_SE;
    let ret = perm_srv_query_perms(Some(&msg), 1, Some(&isndr_uuid), Some(&mut irsp));
    assert_eq!(ret.0, TeeResult::TEE_SUCCESS.0);
    println!("test for perm_srv_query_perms with right param succ");

    msg.req_msg.query_perms.perm_type = PERM_TYPE_TUI_CAPABILITY;
    let ret = perm_srv_query_perms(Some(&msg), sndr_taskid, Some(&isndr_uuid), Some(&mut irsp));
    assert_eq!(ret.0, TeeResult::TEE_SUCCESS.0);
    println!("test for perm_srv_query_perms with right param succ");

    isndr_uuid = TEE_SERVICE_SE;
    let ret = perm_srv_query_perms(Some(&msg), 1, Some(&isndr_uuid), Some(&mut irsp));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_ACCESS_DENIED.0);
    println!("test for perm_srv_query_perms with invalid param succ");

    msg.req_msg.query_perms.perm_type = PERM_TYPE_RPMB_SIZE;
    let mut isndr_uuid = unsafe { MaybeUninit::<TeeUuid>::zeroed().assume_init() };
    isndr_uuid.time_low = 1;
    let ret = perm_srv_query_perms(Some(&msg), 0, Some(&isndr_uuid), Some(&mut irsp));
    assert_eq!(ret.0, TeeResult::TEE_SUCCESS.0);
    println!("test for perm_srv_query_perms with right param succ");

    msg.req_msg.query_perms.perm_type = PERM_TYPE_RPMB_CAPABILITY;
    let ret = perm_srv_query_perms(Some(&msg), 0, Some(&isndr_uuid), Some(&mut irsp));
    assert_eq!(ret.0, TeeResult::TEE_SUCCESS.0);
    println!("test for perm_srv_query_perms with right param succ");

    msg.req_msg.query_perms.perm_type = PERM_TYPE_SFS_CAPABILITY;
    let ret = perm_srv_query_perms(Some(&msg), 0, Some(&isndr_uuid), Some(&mut irsp));
    assert_eq!(ret.0, TeeResult::TEE_SUCCESS.0);
    println!("test for perm_srv_query_perms with right param succ");
}

#[test]
pub fn ut_perm_srv_query_ta2ta_perm() {
    let mut msg = unsafe { MaybeUninit::<perm_srv_req_msg_t>::zeroed().assume_init() };
    let sndr_taskid = 0;
    let mut isndr_uuid = unsafe { MaybeUninit::<TeeUuid>::zeroed().assume_init() };
    let mut irsp = unsafe { MaybeUninit::<perm_srv_reply_msg_t>::zeroed().assume_init() };

    let ret = perm_srv_query_ta2ta_perm(Some(&msg), sndr_taskid, Some(&isndr_uuid), None);
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);
    println!("test for perm_srv_query_perms with invalid param succ");

    let ret = perm_srv_query_ta2ta_perm(None, sndr_taskid, Some(&isndr_uuid), Some(&mut irsp));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);
    println!("test for perm_srv_query_perms with invalid param succ");

    let ret =
        perm_srv_query_ta2ta_perm(Some(&msg), sndr_taskid, Some(&isndr_uuid), Some(&mut irsp));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);
    println!("test for perm_srv_query_perms with invalid param succ");

    isndr_uuid.time_low = 1;
    let ccmd = unsafe { malloc(8) };
    let p = unsafe { core::slice::from_raw_parts_mut(ccmd as *mut u32, 2) };
    p.fill(1);
    let c = unsafe { malloc(core::mem::size_of::<callee_ta_info>()) };
    let ci = unsafe { &mut *(c as *mut callee_ta_info) };
    ci.next = 0;
    ci.uuid = isndr_uuid.clone();
    ci.command_num = 2;
    ci.command_id = p.as_ptr() as _;
    perm_srv_add_config(&isndr_uuid, ci);

    msg.req_msg.query_ta2ta_perm.uuid = isndr_uuid.clone();
    msg.req_msg.query_ta2ta_perm.cmd = 1;
    let ret =
        perm_srv_query_ta2ta_perm(Some(&msg), sndr_taskid, Some(&isndr_uuid), Some(&mut irsp));
    assert_eq!(ret.0, TeeResult::TEE_SUCCESS.0);
    println!("test for perm_srv_query_perms with invalid param succ");

    perm_srv_clear_ta_permissions(Some(&isndr_uuid));
    println!("test for perm_srv_clear_ta_permissions succ");

    let _ = perm_srv_get_config_by_taskid(0, None);
}

#[test]
fn ut_uuid_to_str() {
    let mut uuid = unsafe { MaybeUninit::<TeeUuid>::zeroed().assume_init() };
    uuid.time_low = 3;
    let buf = [0u8; 37];
    let ret = perm_srv_convert_uuid_to_str(Some(&uuid), buf.as_ptr() as _, 37);
    assert_eq!(ret.0, TeeResult::TEE_SUCCESS.0);
    println!("convert succ");

    let ret = perm_srv_convert_uuid_to_str(None, buf.as_ptr() as _, 37);
    assert_eq!(ret.0, TeeResult::TEE_ERROR_GENERIC.0);
    println!("convert fail");

    let ret = perm_srv_convert_uuid_to_str(Some(&uuid), buf.as_ptr() as _, 41);
    assert_eq!(ret.0, TeeResult::TEE_ERROR_GENERIC.0);
    println!("convert fail");
}

#[test]
pub fn ut_perm_srv_parse_config_body() {
    let mut ret = perm_srv_parse_config_body(None, None);
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);
    println!("test for perm_srv_parse_config_body with invalid param succ");

    let mut uuid = unsafe { MaybeUninit::<TeeUuid>::zeroed().assume_init() };
    uuid.time_low = 1;
    let mut config = unsafe { MaybeUninit::<perm_config>::zeroed().assume_init() };
    ret = perm_srv_parse_config_body(Some(&uuid), Some(&mut config));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);
    println!("test for perm_srv_parse_config_body with invalid param succ");

    let tlv = " {}gpd.ta.dynConf\0";
    config.tlv_buf = tlv.as_ptr() as _;
    config.tlv_len = tlv.len() as u32;
    config.policy_version = 1;
    config.cn_size = 64;
    config.cn = [1u8; SN_MAX_SIZE];
    ret = perm_srv_parse_config_body(Some(&uuid), Some(&mut config));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_GENERIC.0);
    println!("test for perm_srv_parse_config_body with invalid param succ");

    let tlv = b"{}gpd.ta.dynConf\0";
    config.tlv_buf = tlv.as_ptr() as _;
    config.tlv_len = tlv.len() as u32;
    ret = perm_srv_parse_config_body(Some(&uuid), Some(&mut config));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_GENERIC.0);
    println!("test for perm_srv_parse_config_body with invalid param succ");

    let tlv = [ASN1_SEQUENCE, b'{'];
    config.tlv_buf = tlv.as_ptr() as _;
    config.tlv_len = tlv.len() as u32;
    ret = perm_srv_parse_config_body(Some(&uuid), Some(&mut config));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_BAD_PARAMETERS.0);
    println!("test for perm_srv_parse_config_body with invalid param succ");

    let mut tlv: [u8; 17] = [
        b' ', b'{', b'}', b'g', b'p', b'd', b'.', b't', b'a', b'.', b'd', b'y', b'n', b'C', b'o',
        b'n', b'f',
    ];
    unsafe {
        *tlv.get_unchecked_mut(0) = ASN1_SEQUENCE as _;
    }
    config.tlv_buf = tlv.as_ptr() as _;
    config.tlv_len = tlv.len() as u32;
    ret = perm_srv_parse_config_body(Some(&uuid), Some(&mut config));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_GENERIC.0);
    println!("test for perm_srv_parse_config_body with invalid param succ");

    let tlv = [
        ASN1_SEQUENCE,
        0,
        64,
        b'{',
        b'}',
        b'g',
        b'p',
        b'd',
        b'.',
        b't',
        b'a',
        b'.',
        b'd',
        b'y',
        b'n',
        b'C',
        b'o',
        b'n',
        b'f',
    ];
    config.tlv_buf = tlv.as_ptr() as _;
    config.tlv_len = tlv.len() as u32;
    ret = perm_srv_parse_config_body(Some(&uuid), Some(&mut config));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_GENERIC.0);
    println!("test for perm_srv_parse_config_body with invalid param succ");

    let tlv = [
        ASN1_SEQUENCE,
        0,
        0,
        4,
        ASN1_SEQUENCE,
        2,
        b'd',
        b'd',
        b'g',
        b'p',
        b'd',
        b'.',
        b't',
        b'a',
        b'.',
        b'd',
        b'y',
        b'n',
        b'C',
        b'o',
        b'n',
        b'f',
    ];
    config.tlv_buf = tlv.as_ptr() as _;
    config.tlv_len = tlv.len() as u32;
    ret = perm_srv_parse_config_body(Some(&uuid), Some(&mut config));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_GENERIC.0);
    println!("test for perm_srv_parse_config_body with invalid param succ");

    let tlv = b"000003940050001b0062000130071000110\
    081000110270003c02800016029300040x1a0303000\
    211028000160303000263029300040x1e0310017403\
    2000390333002422222222-0000-0000-0000-22222\
    22222220343000563|1103200036033300241111111\
    1-0000-0000-0000-11111111111103430002630320\
    002c033300248a927bc8-c38e-42bb-a019-f3979f1\
    4594d0320002c033300245cb439af-a4bf-472c-ad9\
    4-d147af90444e03200036034300026303330024bb0\
    c0d5e-38b3-42de-8400-a8a982156b820320004703\
    330024cc1c47d6-28e4-40ff-901d-7fdaab7bf6140\
    343001363|11|1|2|3|4|5|6|7001001a9002000170\
    033000fdrv_test_module0020001800330010drv_t\
    est_module20020001900330011drv_test_module6\
    4002000270033001fdrv_test_module_copy321234\
    56789002000270033001fdrv_test_module_copy64\
    123456789002000270033001fdrv_Test_module_copy\
    12345678912002000270033001fdrv_test_module_copy\
    64888888888002000270033001fdrv_test_module_copy64\
    777777777002000270033001fdrv_test_module_copy64\
    666666666002000270033001fdrv_test_module_copy12345678912";
    config.policy_version = XML2TLV_PY_VALUE;
    config.tlv_buf = tlv.as_ptr() as _;
    config.tlv_len = tlv.len() as u32;
    ret = perm_srv_parse_config_body(Some(&uuid), Some(&mut config));
    assert_eq!(ret.0, TeeResult::TEE_SUCCESS.0);
    println!("test for perm_srv_parse_config_body with right param succ");
}

#[test]
pub fn test_perm_srv_parse_config_body1() {
    let mut uuid = unsafe { MaybeUninit::<TeeUuid>::zeroed().assume_init() };
    uuid.time_low = 1;
    let mut config = unsafe { MaybeUninit::<perm_config>::zeroed().assume_init() };
    config.policy_version = 1;
    let tlv = [
        ASN1_SEQUENCE,
        4,
        ASN1_SEQUENCE,
        ASN1_INTEGER,
        2,
        b'd',
        b'd',
        b'g',
        b'p',
        b'd',
        b'.',
        b't',
        b'a',
        b'.',
        b'd',
        b'y',
        b'n',
        b'C',
        b'o',
        b'n',
        b'f',
    ];
    config.tlv_buf = tlv.as_ptr() as _;
    config.cn_size = 64;
    config.cn = [1u8; SN_MAX_SIZE];
    config.tlv_len = tlv.len() as u32;
    let ret = perm_srv_parse_config_body(Some(&uuid), Some(&mut config));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_GENERIC.0);
    println!("test for perm_srv_parse_config_body with invalid param succ");
}

#[test]
pub fn test_perm_srv_parse_config_body2() {
    let mut uuid = unsafe { MaybeUninit::<TeeUuid>::zeroed().assume_init() };
    uuid.time_low = 1;
    let mut config = unsafe { MaybeUninit::<perm_config>::zeroed().assume_init() };
    config.policy_version = 1;
    let tlv = [
        ASN1_SEQUENCE,
        4,
        ASN1_SEQUENCE,
        ASN1_INTEGER,
        1,
        b'd',
        b'd',
        b'g',
        b'p',
        b'd',
        b'.',
        b't',
        b'a',
        b'.',
        b'd',
        b'y',
        b'n',
        b'C',
        b'o',
        b'n',
        b'f',
    ];
    config.tlv_buf = tlv.as_ptr() as _;
    config.cn_size = 64;
    config.cn = [1u8; SN_MAX_SIZE];
    config.tlv_len = tlv.len() as u32;
    let ret = perm_srv_parse_config_body(Some(&uuid), Some(&mut config));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_GENERIC.0);
    println!("test for perm_srv_parse_config_body with invalid param succ");

    let mut uuid = unsafe { MaybeUninit::<TeeUuid>::zeroed().assume_init() };
    uuid.time_low = 1;
    let mut config = unsafe { MaybeUninit::<perm_config>::zeroed().assume_init() };
    config.policy_version = 1;
    let tlv = [
        ASN1_SEQUENCE,
        4,
        ASN1_SEQUENCE,
        ASN1_INTEGER,
        3,
        b'd',
        b'd',
        b'g',
        b'p',
        b'd',
        b'.',
        b't',
        b'a',
        b'.',
        b'd',
        b'y',
        b'n',
        b'C',
        b'o',
        b'n',
        b'f',
    ];
    config.tlv_buf = tlv.as_ptr() as _;
    config.cn_size = 64;
    config.cn = [1u8; SN_MAX_SIZE];
    config.tlv_len = tlv.len() as u32;
    let ret = perm_srv_parse_config_body(Some(&uuid), Some(&mut config));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_GENERIC.0);
    println!("test for perm_srv_parse_config_body with invalid param succ");
}

#[test]
pub fn test_perm_srv_parse_config_body3() {
    let mut uuid = unsafe { MaybeUninit::<TeeUuid>::zeroed().assume_init() };
    uuid.time_low = 1;
    let mut config = unsafe { MaybeUninit::<perm_config>::zeroed().assume_init() };
    config.policy_version = 1;
    let tlv = [
        ASN1_SEQUENCE,
        b'g',
        b'p',
        b'd',
        b'.',
        b't',
        b'a',
        b'.',
        b'd',
        b'y',
        b'n',
        b'C',
        b'o',
        b'n',
        b'f',
        b'\0',
    ];
    config.tlv_buf = tlv.as_ptr() as _;
    config.cn_size = 64;
    config.cn = [1u8; SN_MAX_SIZE];
    config.tlv_len = tlv.len() as u32;
    let ret = perm_srv_parse_config_body(Some(&uuid), Some(&mut config));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_GENERIC.0);
    println!("test for perm_srv_parse_config_body with invalid param succ");
}

#[test]
pub fn test_perm_srv_parse_config_body4() {
    let mut uuid = unsafe { MaybeUninit::<TeeUuid>::zeroed().assume_init() };
    uuid.time_low = 1;
    let mut config = unsafe { MaybeUninit::<perm_config>::zeroed().assume_init() };
    config.policy_version = 1;
    let tlv = [
        ASN1_SEQUENCE,
        0,
        4,
        ASN1_INTEGER,
        2,
        b'd',
        b'd',
        b'g',
        b'p',
        b'd',
        b'.',
        b't',
        b'a',
        b'.',
        b'd',
        b'y',
        b'n',
        b'C',
        b'o',
        b'n',
        b'f',
    ];
    config.tlv_buf = tlv.as_ptr() as _;
    config.cn_size = 64;
    config.cn = [1u8; SN_MAX_SIZE];
    config.tlv_len = tlv.len() as u32;
    let ret = perm_srv_parse_config_body(Some(&uuid), Some(&mut config));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_GENERIC.0);
    println!("test for perm_srv_parse_config_body with invalid param succ");

    let mut uuid = unsafe { MaybeUninit::<TeeUuid>::zeroed().assume_init() };
    uuid.time_low = 1;
    let mut config = unsafe { MaybeUninit::<perm_config>::zeroed().assume_init() };
    config.policy_version = 1;
    let tlv = [
        ASN1_SEQUENCE,
        0,
        32,
        ASN1_INTEGER,
        2,
        b'd',
        b'd',
        b'g',
        b'p',
        b'd',
        b'.',
        b't',
        b'a',
        b'.',
        b'd',
        b'y',
        b'n',
        b'C',
        b'o',
        b'n',
        b'f',
    ];
    config.tlv_buf = tlv.as_ptr() as _;
    config.cn_size = 64;
    config.cn = [1u8; SN_MAX_SIZE];
    config.tlv_len = tlv.len() as u32;
    let ret = perm_srv_parse_config_body(Some(&uuid), Some(&mut config));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_GENERIC.0);
    println!("test for perm_srv_parse_config_body with invalid param succ");

    let mut uuid = unsafe { MaybeUninit::<TeeUuid>::zeroed().assume_init() };
    uuid.time_low = 1;
    let mut config = unsafe { MaybeUninit::<perm_config>::zeroed().assume_init() };
    config.policy_version = 1;
    let tlv = [
        ASN1_SEQUENCE,
        0,
        16,
        ASN1_INTEGER,
        2,
        b'd',
        b'd',
        b'g',
        b'p',
        b'd',
        b'.',
        b't',
        b'a',
        b'.',
        b'd',
        b'y',
        b'n',
        b'C',
        b'o',
        b'n',
        b'f',
    ];
    config.tlv_buf = tlv.as_ptr() as _;
    config.cn_size = 64;
    config.cn = [1u8; SN_MAX_SIZE];
    config.tlv_len = tlv.len() as u32;
    let ret = perm_srv_parse_config_body(Some(&uuid), Some(&mut config));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_GENERIC.0);
    println!("test for perm_srv_parse_config_body with invalid param succ");

    let mut uuid = unsafe { MaybeUninit::<TeeUuid>::zeroed().assume_init() };
    uuid.time_low = 1;
    let mut config = unsafe { MaybeUninit::<perm_config>::zeroed().assume_init() };
    config.policy_version = 1;
    let tlv = [
        ASN1_SEQUENCE,
        0,
        16,
        ASN1_INTEGER,
        2,
        b'd',
        b'd',
        ASN1_SEQUENCE,
        0,
        6,
        ASN1_INTEGER,
        2,
        b'd',
        b'y',
        b'n',
        b'C',
        b'o',
        b'n',
        b'f',
    ];
    config.tlv_buf = tlv.as_ptr() as _;
    config.cn_size = 64;
    config.cn = [1u8; SN_MAX_SIZE];
    config.tlv_len = tlv.len() as u32;
    let ret = perm_srv_parse_config_body(Some(&uuid), Some(&mut config));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_GENERIC.0);
    println!("test for perm_srv_parse_config_body with invalid param succ");
}

#[test]
pub fn test_perm_srv_parse_config_body5() {
    let mut uuid = unsafe { MaybeUninit::<TeeUuid>::zeroed().assume_init() };
    uuid.time_low = 1;
    let mut config = unsafe { MaybeUninit::<perm_config>::zeroed().assume_init() };
    config.policy_version = 1;
    let tlv = [
        ASN1_SEQUENCE,
        0,
        16,
        ASN1_INTEGER,
        2,
        b'd',
        b'd',
        ASN1_SEQUENCE,
        0,
        6,
        ASN1_INTEGER,
        2,
        b'd',
        b'y',
        b'n',
        b'C',
        b'o',
        b'n',
        b'f',
    ];
    config.tlv_buf = tlv.as_ptr() as _;
    config.cn_size = 64;
    config.cn = [1u8; SN_MAX_SIZE];
    config.tlv_len = tlv.len() as u32;
    let ret = perm_srv_parse_config_body(Some(&uuid), Some(&mut config));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_GENERIC.0);
    println!("test for perm_srv_parse_config_body with invalid param succ");

    let mut uuid = unsafe { MaybeUninit::<TeeUuid>::zeroed().assume_init() };
    uuid.time_low = 1;
    let mut config = unsafe { MaybeUninit::<perm_config>::zeroed().assume_init() };
    config.policy_version = 1;
    let tlv = [
        ASN1_SEQUENCE,
        0,
        16,
        ASN1_INTEGER,
        2,
        b'd',
        b'd',
        ASN1_SEQUENCE,
        0,
        9,
        ASN1_INTEGER,
        2,
        b'd',
        b'y',
        ASN1_INTEGER,
        2,
        b'o',
        b'n',
        b'x',
    ];
    config.tlv_buf = tlv.as_ptr() as _;
    config.cn_size = 64;
    config.cn = [1u8; SN_MAX_SIZE];
    config.tlv_len = tlv.len() as u32;
    let ret = perm_srv_parse_config_body(Some(&uuid), Some(&mut config));
    assert_eq!(ret.0, TeeResult::TEE_SUCCESS.0);
    println!("test for perm_srv_parse_config_body with invalid param succ");
}

#[test]
pub fn test_perm_srv_parse_config_body6() {
    let mut uuid = unsafe { MaybeUninit::<TeeUuid>::zeroed().assume_init() };
    uuid.time_low = 1;
    let mut config = unsafe { MaybeUninit::<perm_config>::zeroed().assume_init() };
    config.policy_version = 1;
    let tlv = [
        ASN1_SEQUENCE,
        0,
        16,
        ASN1_INTEGER,
        2,
        b'd',
        b'd',
        ASN1_SEQUENCE,
        0,
        9,
        ASN1_INTEGER,
        2,
        0,
        1,
        ASN1_INTEGER,
        2,
        b'o',
        b'n',
        b'x',
    ];
    config.tlv_buf = tlv.as_ptr() as _;
    config.cn_size = 64;
    config.cn = [1u8; SN_MAX_SIZE];
    config.tlv_len = tlv.len() as u32;
    let ret = perm_srv_parse_config_body(Some(&uuid), Some(&mut config));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_GENERIC.0);
    println!("test for perm_srv_parse_config_body with invalid param succ");
}

#[test]
pub fn test_perm_srv_parse_config_body7() {
    let mut uuid = unsafe { MaybeUninit::<TeeUuid>::zeroed().assume_init() };
    uuid.time_low = 1;
    let mut config = unsafe { MaybeUninit::<perm_config>::zeroed().assume_init() };
    config.policy_version = 1;
    config.cert_type = TA_RELEASE_CERT;
    let tlv = [
        ASN1_SEQUENCE,
        0,
        16, // len after total
        ASN1_INTEGER,
        2,
        b'd',
        b'd',
        ASN1_SEQUENCE,
        0,
        9,
        ASN1_INTEGER,
        2,
        b'b',
        b'd',
        ASN1_INTEGER,
        2,
        b'o',
        b'n',
        b'x',
    ];
    config.tlv_buf = tlv.as_ptr() as _;
    config.cn_size = 64;
    config.cn = [1u8; SN_MAX_SIZE];
    config.tlv_len = tlv.len() as u32;
    let ret = perm_srv_parse_config_body(Some(&uuid), Some(&mut config));
    assert_eq!(ret.0, TeeResult::TEE_ERROR_GENERIC.0);
    println!("test for perm_srv_parse_config_body with invalid param succ");
}

#[test]
pub fn ut_asc2hex() {
    let ret = asc2hex(b'1');
    assert_eq!(ret, 1);

    let ret = asc2hex(b'a');
    assert_eq!(ret, 10);

    let ret = asc2hex(b'B');
    assert_eq!(ret, 11);

    let ret = asc2hex(1);
    assert_eq!(ret, -1);
    println!("test for asc2hex succ");
}

#[test]
pub fn ut_byte_to_integer() {
    let ret = byte_to_integer(&[1, 2], 2);
    assert_eq!(ret, 258);

    let ret = byte_to_integer(&[1, 2], 3);
    assert_eq!(ret, 0);

    println!("test for byte_to_integer succ");
}

#[test]
pub fn ut_get_byte_value_from_buff() {
    let mut ans: u8 = 0;
    let ret = get_byte_value_from_buff(&[1], &mut ans);
    assert_eq!(ret, -1);

    let ret = get_byte_value_from_buff(&[1, b'a'], &mut ans);
    assert_eq!(ret, -1);

    let ret = get_byte_value_from_buff(&[b'a', b'a'], &mut ans);
    assert_eq!(ret, 0);

    println!("test for get_byte_value_from_buff succ");
}

#[test]
pub fn ut_perm_srv_convert_str_to_uuid() {
    let mut uuid = unsafe { MaybeUninit::<TeeUuid>::zeroed().assume_init() };
    let buffer = b"88888888-8888-8888-8888-888888888888";
    let ret = perm_srv_convert_str_to_uuid(buffer, &mut uuid);
    assert_eq!(ret, 0);

    let mut uuid = unsafe { MaybeUninit::<TeeUuid>::zeroed().assume_init() };
    let buffer = b"Z8888888-8888-8888-8888-888888888888";
    let ret = perm_srv_convert_str_to_uuid(buffer, &mut uuid);
    assert_eq!(ret, -1);

    let mut uuid = unsafe { MaybeUninit::<TeeUuid>::zeroed().assume_init() };
    let buffer = b"88888888-8888-8888-8888-8888-88888888";
    let ret = perm_srv_convert_str_to_uuid(buffer, &mut uuid);
    assert_eq!(ret, -1);

    let mut uuid = unsafe { MaybeUninit::<TeeUuid>::zeroed().assume_init() };
    let buffer = b"88888888-8888-8888-8888-8888-888888888";
    let ret = perm_srv_convert_str_to_uuid(buffer, &mut uuid);
    assert_eq!(ret, -1);

    println!("test for perm_srv_convert_str_to_uuid succ");
}

#[test]
pub fn ut_parser_callee_ta_uuid() {
    let mut uuid = unsafe { MaybeUninit::<TeeUuid>::zeroed().assume_init() };
    let buffer = b"\0\x2488888888-8888-8888-8888-888888888888\n";
    let ret = parser_callee_ta_uuid(buffer, &mut uuid);
    assert_eq!(ret, 0);

    let mut uuid = unsafe { MaybeUninit::<TeeUuid>::zeroed().assume_init() };
    let buffer = b"\0\x2588888888-8888-8888-8888-888888888888\n";
    let ret = parser_callee_ta_uuid(buffer, &mut uuid);
    assert_eq!(ret, -1);

    let mut uuid = unsafe { MaybeUninit::<TeeUuid>::zeroed().assume_init() };
    let buffer = b"\0\x2588888888-8888-8888-8888-888888888888";
    let ret = parser_callee_ta_uuid(buffer, &mut uuid);
    assert_eq!(ret, -1);
    println!("test for parser_callee_ta_uuid succ");
}

#[test]
pub fn ut_parser_callee_ta_command_id() {
    let mut info = unsafe { MaybeUninit::<callee_ta_info>::zeroed().assume_init() };
    let buffer = b"";
    let ret = parser_callee_ta_command_id(buffer, &mut info);
    assert_eq!(ret, 0);

    let mut info = unsafe { MaybeUninit::<callee_ta_info>::zeroed().assume_init() };
    let buffer = b"zzz";
    let ret = parser_callee_ta_command_id(buffer, &mut info);
    assert_eq!(ret, -1);
    println!("test parser_callee_ta_command_id with invalid param succ");

    let mut info = unsafe { MaybeUninit::<callee_ta_info>::zeroed().assume_init() };
    let buffer = b"000\x02\x02\x01\x800000";
    let ret = parser_callee_ta_command_id(buffer, &mut info);
    assert_eq!(ret, -1);
    println!("test parser_callee_ta_command_id with invalid param succ");

    let mut info = unsafe { MaybeUninit::<callee_ta_info>::zeroed().assume_init() };
    let buffer = b"000\x02\x02\x01\x800000000\x02\x02\x01\x800000";
    let ret = parser_callee_ta_command_id(buffer, &mut info);
    assert_eq!(ret, 0);
    println!("test parser_callee_ta_command_id with right param succ");

    let mut info = unsafe { MaybeUninit::<callee_ta_info>::zeroed().assume_init() };
    let buffer = b"000\x02\x02\x01\x800000000\x02\x02\x01\x500000";
    let ret = parser_callee_ta_command_id(buffer, &mut info);
    assert_eq!(ret, -1);
    println!("test parser_callee_ta_command_id with invalid param succ");

    let mut info = unsafe { MaybeUninit::<callee_ta_info>::zeroed().assume_init() };
    let buffer = b"000\x02\x02\x01\x200000";
    let ret = parser_callee_ta_command_id(buffer, &mut info);
    assert_eq!(ret, -1);
    println!("test parser_callee_ta_command_id with invalid param succ");
}

#[test]
pub fn ut_is_duplicate_callee() {
    let mut callee_info = unsafe { MaybeUninit::<callee_ta_info>::zeroed().assume_init() };
    callee_info.uuid.time_low = 1;
    let callee_info2 = unsafe { MaybeUninit::<callee_ta_info>::zeroed().assume_init() };
    callee_info.next = &callee_info2 as *const callee_ta_info as _;
    let mut config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    let ret = is_duplicate_callee(&config, &callee_info);
    assert_eq!(ret, false);

    config.control_info.callee_info = &callee_info as *const callee_ta_info as _;
    let ret = is_duplicate_callee(&config, &callee_info2);
    assert_eq!(ret, true);

    println!("test for is_duplicate_callee succ");
}

#[test]
pub fn ut_parser_fill_callee() {
    let mut config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    let value_len = 10;
    let buff = b"";
    let ret = parser_fill_callee(buff, value_len, &mut config);
    assert_eq!(ret, -1);

    let mut config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    let value_len = 10;
    let buff = b"000\x02\x0288";
    let ret = parser_fill_callee(buff, value_len, &mut config);
    assert_eq!(ret, -1);

    let mut config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    let value_len = 43;
    let buff = b"000\x02\x02\0\x410000000000000000000000000000000000000000";
    let ret = parser_fill_callee(buff, value_len, &mut config);
    assert_eq!(ret, -1);

    let mut config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    let value_len = 43;
    let buff = b"000\x02\x02\0\x41\0\x2488888888-8888-8888-8888-888888888888000";
    let ret = parser_fill_callee(buff, value_len, &mut config);
    assert_eq!(ret, -1);
    println!("test invalid params succ");

    let mut config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    let value_len = 64;
    let buff = b"000\x02\x02\0\x41\0\x2488888888-8888-8888-8888-888888888888000\x02\x02\x01\x800000000\x02\x02\x01\x800000";
    let ret = parser_fill_callee(buff, value_len, &mut config);
    assert_eq!(ret, 0);
    println!("test for parser_fill_callee succ");
}

#[test]
pub fn ut_parser_callee_info() {
    let buff = b"000\x02\x02\0\x41\0\x2488888888-8888-8888-8888-888888888888000\x02\x02\x01\x800000000\x02\x02\x01\x800000";
    let mut config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    let ret = parser_callee_info(buff, &mut config);
    assert_eq!(ret, -1);

    let buff = b"xx";
    let mut config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    let ret = parser_callee_info(buff, &mut config);
    assert_eq!(ret, -1);

    let buff = b"\x10\x43000\x02\x02\0\x41\0\x2488888888-8888-8888-8888-888888888888000\x02\x02\x01\x800000000\x02\x02\x01\x800000";
    let mut config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    let ret = parser_callee_info(buff, &mut config);
    assert_eq!(ret, -1);

    let buff = b"\x10\0\0\0000\x02\x02\0\x41\0\x2488888888-8888-8888-8888-888888888888000\x02\x02\x01\x800000000\x02\x02\x01\x800000";
    let mut config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    let ret = parser_callee_info(buff, &mut config);
    assert_eq!(ret, -1);
    println!("test for invalid len succ");

    let buff = b"\x10\0\x40\x02\x02\0\x41\0\x2488888888-8888-8888-8888-888888888888000\x02\x02\x01\x800000000\x02\x02\x01\x800000";
    let mut config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    let ret = parser_callee_info(buff, &mut config);
    assert_eq!(ret, 0);
}

#[test]
pub fn ut_get_tag_uuid() {
    let buff = b"";
    let mut config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    let ret = get_tag_uuid(buff, &mut config);
    assert_eq!(ret, -1);

    let buff = b"000000000000000000000000000000000000";
    let mut config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    let ret = get_tag_uuid(buff, &mut config);
    assert_eq!(ret, -1);

    let buff = b"88888888-8888-8888-8888-888888888888";
    let mut config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    let ret = get_tag_uuid(buff, &mut config);
    assert_eq!(ret, 0);
    println!("test for get_tag_uuid succ");
}

#[test]
pub fn ut_config_tlv_parser_basic_info() {
    let buff = b"\x02";
    let mut config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    let ret = config_tlv_parser_basic_info(buff, &mut config);
    assert_eq!(ret, -1);
    println!("test for config_tlv_parser_basic_info succ");

    let buff = b"\x02\x02\x00\x99";
    let mut config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    let ret = config_tlv_parser_basic_info(buff, &mut config);
    assert_eq!(ret, -1);
    println!("test for config_tlv_parser_basic_info succ");

    let buff = b"\x02\x00\x10\x02\x02\x10\x00000000000000";
    let mut config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    let ret = config_tlv_parser_basic_info(buff, &mut config);
    assert_eq!(ret, 0);
    println!("test for config_tlv_parser_basic_info succ");

    let buff = b"\x02\x00\x10\x02\x02\x01\x00000000000000";
    let mut config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    let ret = config_tlv_parser_basic_info(buff, &mut config);
    assert_eq!(ret, -1);
    println!("test for config_tlv_parser_basic_info succ");

    let buff = b"\x02\x00\x10\x02\x02\x01\x00000000000000";
    let mut config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    let ret = config_tlv_parser_basic_info(buff, &mut config);
    assert_eq!(ret, -1);
    println!("test for config_tlv_parser_basic_info succ");

    let buff = b"\x02\x00\x28\x02\x02\x01\x0088888888-8888-8888-8888-888888888888";
    let mut config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    let ret = config_tlv_parser_basic_info(buff, &mut config);
    assert_eq!(ret, 0);
    println!("test for config_tlv_parser_basic_info succ");

    let buff = b"\x02\x00\x28\x02\x02\x01\x0188888888-8888-8888-8888-888888888888";
    let mut config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    let ret = config_tlv_parser_basic_info(buff, &mut config);
    assert_eq!(ret, 0);
    println!("test for config_tlv_parser_basic_info succ");

    let buff = b"\x02\x00\x48\x02\x02\x01\x0188888888-8888-8888-8888-88888888888888888888-8888-8888-8888-88888888";
    let mut config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    let ret = config_tlv_parser_basic_info(buff, &mut config);
    assert_eq!(ret, -1);
    println!("test for config_tlv_parser_basic_info succ");
}

#[test]
pub fn ut_config_tlv_value_to_manifest_info() {
    let buff = b"\x02\x00\x48\x02\x02\x01\x0188888888-8888-8888-8888-88888888888888888888-8888-8888-8888-88888888";
    let mut config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    let mut tag = 0;
    config_tlv_value_to_manifest_info(buff, tag, &mut config);

    tag = TLV_TAG_SINGLE_INSTANCE;
    config_tlv_value_to_manifest_info(buff, tag, &mut config);

    tag = TLV_TAG_HEAP_SIZE;
    config_tlv_value_to_manifest_info(buff, tag, &mut config);

    tag = TLV_TAG_MULTI_SESSION;
    config_tlv_value_to_manifest_info(buff, tag, &mut config);

    tag = TLV_TAG_STACK_SIZE;
    config_tlv_value_to_manifest_info(buff, tag, &mut config);

    tag = TLV_TAG_INSTANCE_KEEP_ALIVE;
    config_tlv_value_to_manifest_info(buff, tag, &mut config);

    tag = TLV_TAG_MEM_PAGE_ALIGN;
    config_tlv_value_to_manifest_info(buff, tag, &mut config);

    tag = TLV_TAG_TARGET_TYPE;
    config_tlv_value_to_manifest_info(buff, tag, &mut config);

    tag = TLV_TAG_SYS_VERIFY_TA;
    config_tlv_value_to_manifest_info(buff, tag, &mut config);
}

#[test]
pub fn ut_config_tlv_parser_manifest_info() {
    let buff = b"\x02\x00\x48\x02\x02\x01\x0188888888-8888-8888-8888-88888888888888888888-8888-8888-8888-88888888";
    let mut config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    let ret = config_tlv_parser_manifest_info(buff, &mut config);
    assert_eq!(ret, 0);

    let buff = b"\x02\x00\x48\x02\x02\x01\x1388888888-8888-8888-8888-88888888888888888888-8888-8888-8888-88888888";
    let mut config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    let ret = config_tlv_parser_manifest_info(buff, &mut config);
    assert_eq!(ret, -1);

    let buff = b"\x22\x20\x28\x02\x02\x01\x1388888888-8888-8888-8888-88888888888888888888-8888-8888-8888-88888888";
    let mut config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    let ret = config_tlv_parser_manifest_info(buff, &mut config);
    assert_eq!(ret, -1);
    println!("test for parser manifest succ");
}

#[test]
pub fn ut_tlv_tag_rpmb_general() {
    let buff = b"\0";
    let mut config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    tlv_tag_rpmb_general(buff, &mut config);
    assert_eq!(config.control_info.rpmb_info.permissions, 0);

    let buff = b"\x01";
    tlv_tag_rpmb_general(buff, &mut config);
    assert_eq!(
        config.control_info.rpmb_info.permissions,
        RPMB_GENERAL_PERMISSION
    );
    println!("test for tlv_tag_rpmb_general succ");
}

#[test]
pub fn ut_tlv_tag_rpmb_specific() {
    let buff = b"\0";
    let mut config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    tlv_tag_rpmb_specific(buff, &mut config);
    assert_eq!(config.control_info.rpmb_info.permissions, 0);

    let buff = b"\x01";
    tlv_tag_rpmb_specific(buff, &mut config);
    assert_eq!(
        config.control_info.rpmb_info.permissions,
        RPMB_RESET_PERMISSION
    );
    println!("test for tlv_tag_rpmb_specific succ");
}

#[test]
pub fn ut_config_tlv_parser_rpmb_permission() {
    let buff = b"\0";
    let mut config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    let ret = config_tlv_parser_rpmb_permission(buff, &mut config);
    assert_eq!(ret, -1);

    let buff = b"\x02\x00\x48\x02\x02\x01\x0188888888-8888-8888-8888-88888888888888888888-8888-8888-8888-88888888";
    let mut config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    let ret = config_tlv_parser_rpmb_permission(buff, &mut config);
    assert_eq!(ret, 0);

    let buff = b"\x02\x00\x48\x02\x02\x01\x2288888888-8888-8888-8888-88888888888888888888-8888-8888-8888-88888888";
    let mut config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    let ret = config_tlv_parser_rpmb_permission(buff, &mut config);
    assert_eq!(ret, 0);

    let buff = b"\x02\x00\x48\x02\x02\x01\x2388888888-8888-8888-8888-88888888888888888888-8888-8888-8888-88888888";
    let mut config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    let ret = config_tlv_parser_rpmb_permission(buff, &mut config);
    assert_eq!(ret, 0);
}

#[test]
pub fn ut_config_tlv_parser_rpmb_info() {
    let buff = b"\x02\x00\x48\x02\x02\x01\x2388888888-8888-8888-8888-88888888888888888888-8888-8888-8888-88888888";
    let mut config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    let ret = config_tlv_parser_rpmb_info(buff, &mut config);
    assert_eq!(ret, 0);

    let buff = b"\x22\x00\x28\x02\x02\x01\x2388888888-8888-8888-8888-88888888888888888888-8888-8888-8888-88888888";
    let mut config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    let ret = config_tlv_parser_rpmb_info(buff, &mut config);
    assert_eq!(ret, -1);

    let buff = b"\x02\x00\x48\x02\x02\x00\x7188888888-8888-8888-8888-88888888888888888888-8888-8888-8888-88888888";
    let mut config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    let ret = config_tlv_parser_rpmb_info(buff, &mut config);
    assert_eq!(ret, -1);
    println!("test for config_tlv_parser_rpmb_info succ");

    let buff = b"\x02\x00\x4f\x02\x02\x00\x71\x02\x00\x48\x02\x02\x01\x7188888888-8888-8888-8888-88888888888888888888-8888-8888-8888-88888888";
    let mut config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    let ret = config_tlv_parser_rpmb_info(buff, &mut config);
    assert_eq!(ret, 0);

    let buff = b"\x02\x00\x48\x02\x02\x01\x2088888888-8888-8888-8888-88888888888888888888-8888-8888-8888-88888888";
    let mut config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    let ret = config_tlv_parser_rpmb_info(buff, &mut config);
    assert_eq!(ret, -1);

    let buff = b"\x02\x00\x08\x02\x02\x01\x208888";
    let mut config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    let ret = config_tlv_parser_rpmb_info(buff, &mut config);
    assert_eq!(ret, 0);
    println!("test for config_tlv_parser_rpmb_info succ");
}

#[test]
pub fn ut_config_tlv_parser_info() {
    let buff = b"\x22\x20\x28\x02\x02\x01\x208888";
    let mut config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    let ret = config_tlv_parser_info(buff, &mut config);
    assert_eq!(ret, -1);

    let buff = b"\x02\x00\x08\x02\x02\x01\x208888";
    let mut config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    let ret = config_tlv_parser_info(buff, &mut config);
    assert_eq!(ret, 0);

    let buff = b"\x02\x00\x08\x02\x02\x01\x408888";
    let mut config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    let ret = config_tlv_parser_info(buff, &mut config);
    assert_eq!(ret, 0);

    let buff = b"\x02\x00\x08\x02\x02\x01\x608888";
    let mut config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    let ret = config_tlv_parser_info(buff, &mut config);
    assert_eq!(ret, 0);

    let buff = b"\x02\x00\x08\x02\x02\x00\x738888";
    let mut config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    let ret = config_tlv_parser_info(buff, &mut config);
    assert_eq!(ret, 0);

    let buff = b"\x02\x00\x08\x02\x02\x01\x518888";
    let mut config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    let ret = config_tlv_parser_info(buff, &mut config);
    assert_eq!(ret, 0);
    println!("test for config_tlv_parser_info succ");
}

#[test]
pub fn ut_parser_ta_manager() {
    let buff = b"Trustonic";
    let mut config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    parser_ta_manager(buff, &mut config);
    assert_eq!(config.control_info.ta_manager, TA_MANAGER_TRUSTONIC);

    let buff = b"Trustonicccc";
    let mut config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    parser_ta_manager(buff, &mut config);
    assert_eq!(config.control_info.ta_manager, 0);
    println!("test for parser_ta_manager succ");
}

#[test]
pub fn ut_config_tlv_parser_control_info() {
    let buff = b"\x22\x30\x28\x02\x02\x01\x518888";
    let mut config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    let ret = config_tlv_parser_control_info(buff, &mut config);
    assert_eq!(ret, -1);

    let buff = b"\x02\x00\x08\x02\x02\x01\x518888";
    let mut config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    let ret = config_tlv_parser_control_info(buff, &mut config);
    assert_eq!(ret, 0);

    let buff = b"\x02\x00\x08\x02\x02\x00\x318888";
    let mut config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    let ret = config_tlv_parser_control_info(buff, &mut config);
    assert_eq!(ret, -1);

    let buff = b"\x02\x00\x56\x02\x02\x00\x31\x02\x00\x4f\x02\x02\x00\x71\x02\x00\x48\x02\x02\x01\x7188888888-8888-8888-8888-88888888888888888888-8888-8888-8888-88888888";
    let mut config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    let ret = config_tlv_parser_control_info(buff, &mut config);
    assert_eq!(ret, 0);

    let buff = b"\x02\x00\x56\x02\x02\x00\x36\x02\x00\x4f\x02\x02\x00\x71\x02\x00\x48\x02\x02\x01\x7188888888-8888-8888-8888-88888888888888888888-8888-8888-8888-88888888";
    let mut config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    let ret = config_tlv_parser_control_info(buff, &mut config);
    assert_eq!(ret, 0);

    let buff = b"\x02\x00\x56\x02\x02\x00\x33\x02\x00\x4f\x02\x02\x00\x71\x02\x00\x48\x02\x02\x01\x7188888888-8888-8888-8888-88888888888888888888-8888-8888-8888-88888888";
    let mut config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    let ret = config_tlv_parser_control_info(buff, &mut config);
    assert_eq!(ret, 0);

    let buff = b"\x02\x00\x47\x02\x02\x00\x04\x10\0\x40\x02\x02\0\x41\0\x2488888888-8888-8888-8888-888888888888000\x02\x02\x01\x800000000\x02\x02\x01\x800000";
    let mut config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    let ret = config_tlv_parser_control_info(buff, &mut config);
    assert_eq!(ret, 0);

    let buff = b"\x02\x00\x47\x02\x02\x01\x70\x10\0\x40\x02\x02\0\x41\0\x2488888888-8888-8888-8888-888888888888000\x02\x02\x01\x800000000\x02\x02\x01\x800000";
    let mut config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    let ret = config_tlv_parser_control_info(buff, &mut config);
    assert_eq!(ret, 0);
}

#[test]
pub fn ut_perm_srv_update_config_by_same_uuid() {
    let config =
        unsafe { &mut *(malloc(core::mem::size_of::<config_info>() as _) as *mut config_info) };
    config.uuid.time_low = 1;
    perm_srv_update_config_by_same_uuid(config as _);
    let config1 =
        unsafe { &mut *(malloc(core::mem::size_of::<config_info>() as _) as *mut config_info) };
    config1.uuid.time_low = 2;
    perm_srv_update_config_by_same_uuid(config1 as _);
    let config2 =
        unsafe { &mut *(malloc(core::mem::size_of::<config_info>() as _) as *mut config_info) };
    config2.uuid.time_low = 2;
    perm_srv_update_config_by_same_uuid(config2 as _);
    println!("test for updata same uuid succ");
}

#[test]
pub fn ut_parse_dyntlv_buf() {
    let uuid = unsafe { MaybeUninit::<TeeUuid>::zeroed().assume_init() };
    let permconfig = unsafe { MaybeUninit::<perm_config>::zeroed().assume_init() };
    let config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    let dynconflen = 16;
    let ret = parse_dyntlv_buf(&uuid, &permconfig, &config, dynconflen);
    assert_eq!(ret.0, TeeResult::TEE_ERROR_GENERIC.0);

    let uuid = unsafe { MaybeUninit::<TeeUuid>::zeroed().assume_init() };
    let permconfig = unsafe { MaybeUninit::<perm_config>::zeroed().assume_init() };
    let config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    let dynconflen = 0;
    let ret = parse_dyntlv_buf(&uuid, &permconfig, &config, dynconflen);
    assert_eq!(ret.0, TeeResult::TEE_SUCCESS.0);

    let mut uuid = unsafe { MaybeUninit::<TeeUuid>::zeroed().assume_init() };
    uuid.time_low = 1;
    let permconfig = unsafe { MaybeUninit::<perm_config>::zeroed().assume_init() };
    let config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    let dynconflen = 16;
    let ret = parse_dyntlv_buf(&uuid, &permconfig, &config, dynconflen);
    assert_eq!(ret.0, TeeResult::TEE_ERROR_GENERIC.0);

    let uuid = unsafe { MaybeUninit::<TeeUuid>::zeroed().assume_init() };
    let permconfig = unsafe { MaybeUninit::<perm_config>::zeroed().assume_init() };
    let config = unsafe { MaybeUninit::<config_info>::zeroed().assume_init() };
    let dynconflen = 16;
    let ret = parse_dyntlv_buf(&uuid, &permconfig, &config, dynconflen);
    assert_eq!(ret.0, TeeResult::TEE_ERROR_GENERIC.0);
    println!("test for parse_dyntlv_buf succ");
}

#[test]
pub fn test_ipc() {
    let msg = b"msg";
    let ret = ipc_msg_snd(0, 0, msg.as_ptr() as _, msg.len() as _);
    assert_eq!(ret, 0);
}
