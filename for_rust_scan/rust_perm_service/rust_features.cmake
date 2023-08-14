set(rust_perm_features "")

if ("${CONFIG_DYN_IMPORT_CERT}" STREQUAL "y")
	set(rust_perm_features "${rust_perm_features},config_dyn_import_cert")
endif()

if ("${CONFIG_REMOTE_ATTESTATION}" STREQUAL "y")
	set(rust_perm_features "${rust_perm_features},config_remote_attestation")
endif()

if ("${CONFIG_REMOTE_ATTESTATION_A32}" STREQUAL "y")
	set(rust_perm_features "${rust_perm_features},config_remote_attestation_a32")
endif()

if ("${CONFIG_LIVEPATCH_ENABLE}" STREQUAL "y")
	set(rust_perm_features "${rust_perm_features},config_livepatch_enable")
endif()

if ("${CONFIG_OTRP_SUPPORT}" STREQUAL "y")
	set(rust_perm_features "${rust_perm_features},config_otrp_support")
endif()

if ("${CONFIG_ADAPT_BIG_MEMORY}" STREQUAL "y")
	set(rust_perm_features "${rust_perm_features},config_adapt_big_memory")
endif()

if ("${CONFIG_TA_AUTH_MAX_CA_CALLER_NUM}" STREQUAL "32")
	set(rust_perm_features "${rust_perm_features},config_ta_auth_max_ca_caller_num32")
endif()

if ("${CONFIG_TA_CMS_SIGNATURE}" STREQUAL "y")
	set(rust_perm_features "${rust_perm_features},config_ta_cms_signature")
endif()

if ("${CONFIG_TIMER_DISABLE}" STREQUAL "y")
	set(rust_perm_features "${rust_perm_features},config_timer_disable")
endif()

if ("${CONFIG_OFF_DRV_TIMER}" STREQUAL "y")
	set(rust_perm_features "${rust_perm_features},config_off_drv_timer")
endif()

if ("${CONFIG_DYN_TA_FORMAT}" STREQUAL "1")
	set(rust_perm_features "${rust_perm_features},dyn_ta_support_v1,dyn_ta_support_v2,dyn_ta_support_v3,dyn_ta_support_v5")
elseif ("${CONFIG_DYN_TA_FORMAT}" STREQUAL "2")
	set(rust_perm_features "${rust_perm_features},dyn_ta_support_v2,dyn_ta_support_v3,dyn_ta_support_v5")
elseif ("${CONFIG_DYN_TA_FORMAT}" STREQUAL "3")
	set(rust_perm_features "${rust_perm_features},dyn_ta_support_v3,dyn_ta_support_v5")
elseif ("${CONFIG_DYN_TA_FORMAT}" STREQUAL "5")
	set(rust_perm_features "${rust_perm_features},dyn_ta_support_v5")
endif()

if ("${CONFIG_ELF_DECRYPT_ENABLE}" STREQUAL "y")
    if ("${CONFIG_CRYPTO_SOFT_ENGINE}" STREQUAL "mbedtls")
		set(rust_perm_features "${rust_perm_features},mbedtls_enable")
    elseif ("${CONFIG_CRYPTO_SOFT_ENGINE}" STREQUAL "openssl" OR "${CONFIG_CRYPTO_SOFT_ENGINE}" STREQUAL "openEuler_openssl")
		set(rust_perm_features "${rust_perm_features},openssl_enable")
    elseif ("${CONFIG_CRYPTO_SOFT_ENGINE}" STREQUAL "hitls")
		set(rust_perm_features "${rust_perm_features},hitls_enable")
    endif()
endif()

if ("${CONFIG_TA_CMS_RSA_SIGNATURE}" STREQUAL "y")
    set(rust_perm_features "${rust_perm_features},config_ta_cms_rsa_signature")
endif()
