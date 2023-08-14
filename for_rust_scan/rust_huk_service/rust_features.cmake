set(rust_huk_features "")
if ("${CONFIG_TEE_UPGRADE}" STREQUAL "y")
    set(rust_huk_features "${rust_huk_features},config_tee_upgrade")
endif()
