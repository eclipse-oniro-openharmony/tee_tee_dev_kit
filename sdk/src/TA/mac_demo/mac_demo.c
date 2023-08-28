/*
 * Copyright (C) 2022 Huawei Technologies Co., Ltd.
 * Licensed under the Mulan PSL v2.
 * You can use this software according to the terms and conditions of the Mulan PSL v2.
 * You may obtain a copy of Mulan PSL v2 at:
 *     http://license.coscl.org.cn/MulanPSL2
 * THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND, EITHER EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR
 * PURPOSE.
 * See the Mulan PSL v2 for more details.
 */
#include "tee_crypto_api.h"
#include "tee_object_api.h"
#include "tee_ext_api.h"
#include "tee_log.h"
#include "securec.h"

#define TA_KEY_IV_LEN           16
#define TA_AES_KEY_LEN          32
#define TA_AES_KEY_LEN_BIT      (32 * 8)
#define TA_MAC_BUFFER_SIZE      32
#define TA_DATA_BUFFER_SIZE     64

static uint8_t g_data_plain_text[TA_DATA_BUFFER_SIZE] = { 0 };

enum {
    CMD_ID_AES_CBC_MAC_DEMO = 1,
    CMD_ID_AES_CMAC_DEMO = 2,
};

/*
 * Demo For AES-CBC-MAC
 * You can generate a key and use AES-CBC-MAC to get the MAC of TA data
 */
static TEE_Result crypto_api_aes_cbc_mac_test(void)
{
    TEE_Result ret;

    /* operation */
    TEE_OperationHandle operation_mac = NULL;
    uint8_t object_id[TA_KEY_IV_LEN];

    /* key object */
    TEE_ObjectHandle object = NULL;

    /* other params */
    uint8_t *plaintext = NULL;
    uint8_t mac_result[TA_MAC_BUFFER_SIZE] = { 0 };
    size_t mac_len = TA_MAC_BUFFER_SIZE;

    /* 1.Allocate Operation */
    ret = TEE_AllocateOperation(&operation_mac, TEE_ALG_AES_CBC_MAC_NOPAD, TEE_MODE_MAC, TA_AES_KEY_LEN_BIT);
    if (ret != TEE_SUCCESS) {
        tloge("Failed to allocate operation for mac, ret is 0x%x", ret);
        return ret;
    }

    /* 2. Generate key */
    ret = TEE_AllocateTransientObject(TEE_TYPE_AES, TA_AES_KEY_LEN_BIT, &object);
    if (ret != TEE_SUCCESS) {
        tloge("Failed to allocate object for AES key: ret=0x%x", ret);
        goto cleanup_1;
    }
    ret = TEE_GenerateKey(object, TA_AES_KEY_LEN_BIT, NULL, 0);
    if (ret != TEE_SUCCESS) {
        tloge("Failed to generate AES key:ret=0x%x", ret);
        goto cleanup_2;
    }

    /* 3.Set operation key */
    ret = TEE_SetOperationKey(operation_mac, object);
    if (ret != TEE_SUCCESS) {
        tloge("failed to set operation key for mac:ret=0x%x", ret);
        goto cleanup_2;
    }

    /* 4.Set object_id and generate random plaintext(In user case maybe the plaintext is the key data of the TA) */
    TEE_GenerateRandom(object_id, sizeof(object_id));
    plaintext = g_data_plain_text;
    TEE_GenerateRandom((void *)plaintext, sizeof(g_data_plain_text));

    /* 5.Use the Key and object_id to Do AES-CBC-MAC */
    TEE_MACInit(operation_mac, object_id, sizeof(object_id));
    TEE_MACUpdate(operation_mac, (void *)plaintext, (size_t)(sizeof(g_data_plain_text) - TA_MAC_BUFFER_SIZE));
    ret = TEE_MACComputeFinal(operation_mac,
        (void *)(plaintext + sizeof(g_data_plain_text) - TA_MAC_BUFFER_SIZE),
        TA_MAC_BUFFER_SIZE,
        (void *)(mac_result),
        &mac_len);
    if (ret != TEE_SUCCESS) {
        tloge("Failed to mac compute final do final:ret=0x%x", ret);
        goto cleanup_2;
    }
    tlogi("Succeed to do AES-CBC-MAC");

cleanup_2:
    TEE_FreeTransientObject(object);
cleanup_1:
    TEE_FreeOperation(operation_mac);
    return ret;
}

/*
 * Demo For AES-CMAC
 * You can generate a key and use AES-CMAC to get the MAC of TA data
 */
static TEE_Result crypto_api_aes_cmac_test(void)
{
    TEE_Result ret;

    /* operation */
    TEE_OperationHandle operation_mac = NULL;

    /* key object */
    TEE_ObjectHandle object = NULL;

    /* other params */
    uint8_t *plaintext = NULL;
    uint8_t mac_result[TA_MAC_BUFFER_SIZE] = { 0 };
    size_t mac_len = TA_MAC_BUFFER_SIZE;

    /* 1.Allocate Operation */
    ret = TEE_AllocateOperation(&operation_mac, TEE_ALG_AES_CMAC, TEE_MODE_MAC, TA_AES_KEY_LEN_BIT);
    if (ret != TEE_SUCCESS) {
        tloge("Failed to allocate operation for mac, ret is 0x%x", ret);
        return ret;
    }

    /* 2. Generate key */
    ret = TEE_AllocateTransientObject(TEE_TYPE_AES, TA_AES_KEY_LEN_BIT, &object);
    if (ret != TEE_SUCCESS) {
        tloge("Failed to allocate object for AES key: ret=0x%x", ret);
        goto cleanup_1;
    }
    ret = TEE_GenerateKey(object, TA_AES_KEY_LEN_BIT, NULL, 0);
    if (ret != TEE_SUCCESS) {
        tloge("Failed to generate AES key:ret=0x%x", ret);
        goto cleanup_2;
    }

    /* 3.Set operation key */
    ret = TEE_SetOperationKey(operation_mac, object);
    if (ret != TEE_SUCCESS) {
        tloge("failed to set operation key for mac:ret=0x%x", ret);
        goto cleanup_2;
    }

    /* 4.Set object_id and generate random plaintext(In user case maybe the plaintext is the key data of the TA) */
    plaintext = g_data_plain_text;
    TEE_GenerateRandom((void *)plaintext, sizeof(g_data_plain_text));

    /* 5.Use the Key and object_id to Do AES-CMAC(CMAC do not use object_id) */
    TEE_MACInit(operation_mac, NULL, 0);
    TEE_MACUpdate(operation_mac, (void *)plaintext, (size_t)(sizeof(g_data_plain_text) - TA_MAC_BUFFER_SIZE));
    ret = TEE_MACComputeFinal(operation_mac,
        (void *)(plaintext + sizeof(g_data_plain_text) - TA_MAC_BUFFER_SIZE),
        TA_MAC_BUFFER_SIZE,
        (void *)(mac_result),
        &mac_len);
    if (ret != TEE_SUCCESS) {
        tloge("Failed to mac compute final do final:ret=0x%x", ret);
        goto cleanup_2;
    }
    tlogi("Succeed to do AES-CMAC\n");
cleanup_2:
    TEE_FreeTransientObject(object);
cleanup_1:
    TEE_FreeOperation(operation_mac);
    return ret;
}

TEE_Result TA_CreateEntryPoint(void)
{
    TEE_Result ret;

    tlogd("----- TA entry point ----- ");
    /* When you develop your own CA, you need to change the name to your own path and CA name. */
    ret = AddCaller_CA_exec("/vendor/bin/mac_demo_ca", 0);
    if (ret == TEE_SUCCESS) {
        tlogd("TA entry point: add ca whitelist success");
    } else {
        tloge("TA entry point: add ca whitelist failed");
        return TEE_ERROR_GENERIC;
    }

    return TEE_SUCCESS;
}

TEE_Result TA_OpenSessionEntryPoint(uint32_t param_types, TEE_Param params[4], void** session_context)
{
    (void)param_types;
    (void)params;
    (void)session_context;
    tlogd("---- TA MAC Demo OpenSession EntryPoint -----");

    return TEE_SUCCESS;
}

TEE_Result TA_InvokeCommandEntryPoint(void* session_context, uint32_t cmd_id, uint32_t param_types, TEE_Param params[4])
{
    (void)session_context;
    (void)params;
    TEE_Result ret;
    if (!check_param_type(param_types,
        TEE_PARAM_TYPE_NONE,
        TEE_PARAM_TYPE_NONE,
        TEE_PARAM_TYPE_NONE,
        TEE_PARAM_TYPE_NONE)) {
        tloge("Bad expected parameter types");
        return TEE_ERROR_BAD_PARAMETERS;
    }

    tlogd("--- TA MAC Demo InvokeCommand EntryPoint ---");
    switch (cmd_id) {
    case CMD_ID_AES_CBC_MAC_DEMO:
        ret = crypto_api_aes_cbc_mac_test();
        break;
    case CMD_ID_AES_CMAC_DEMO:
        ret = crypto_api_aes_cmac_test();
        break;
    default:
        tloge("invalid cmd:%u", cmd_id);
        ret = TEE_ERROR_BAD_PARAMETERS;
        break;
    }
    if (ret != TEE_SUCCESS) {
        tloge("Cmd id %u Test Failed, ret is 0x%x", cmd_id, ret);
    } else {
        tlogd("Cmd id %u Test Pass", cmd_id);
    }
    return ret;
}

void TA_CloseSessionEntryPoint(void* session_context)
{
    (void)session_context;
    tlogd("---- TA MAC Demo CloseSession EntryPoint -----");
}

void TA_DestroyEntryPoint(void)
{
    tlogd("---- TA MAC Demo Destroy EntryPoint ----");
}
