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
#include <stdio.h>
#include "tee_client_api.h"

/*
 * when developing your own CA, you need to generate a new UUID.
 */
#define SECSTORAGE_DEMO_TA_PATH    "/vendor/bin/bb9aefd4-ccef-4ff8-a1d5-2fb92fab7836.sec"
#define OPERATION_START_FLAG 1

enum {
    CMD_STORE_KEY_DEMO = 1,
    CMD_STORE_DATA_DEMO = 2,
    CMD_ENUMERATOR_DEMO = 3,
};

/* Secstorage_Demo_uuid:bb9aefd4-ccef-4ff8-a1d5-2fb92fab7836 */
static const TEEC_UUID g_secstorageDemoUuid = {
    0xbb9aefd4, 0xccef, 0x4ff8,
    { 0xa1, 0xd5, 0x2f, 0xb9, 0x2f, 0xab, 0x78, 0x36 }
};

TEEC_Context g_context;
TEEC_Session g_session;

static TEEC_Result teec_init(void)
{
    TEEC_Operation operation = {0};
    TEEC_Result ret;
    uint32_t origin = 0;

    ret = TEEC_InitializeContext(NULL, &g_context);
    if (ret != TEEC_SUCCESS) {
        printf("teec initial failed\n");
        return ret;
    }

    /* pass TA's FULLPATH to TEE, then OpenSession. CA MUST use the TEEC_LOGIN_IDENTIFY mode to login. */
    operation.started = OPERATION_START_FLAG;
    operation.paramTypes = TEEC_PARAM_TYPES(
        TEEC_NONE,
        TEEC_NONE,
        TEEC_MEMREF_TEMP_INPUT,
        TEEC_MEMREF_TEMP_INPUT);

    g_context.ta_path = (uint8_t *)SECSTORAGE_DEMO_TA_PATH;
    ret = TEEC_OpenSession(&g_context, &g_session, &g_secstorageDemoUuid,
        TEEC_LOGIN_IDENTIFY, NULL, &operation, &origin);
    if (ret != TEEC_SUCCESS) {
        printf("open session failed: ret=0x%x, origin=0x%x", ret, origin);
        TEEC_FinalizeContext(&g_context);
        return ret;
    }
    printf("teec init OK\n");
    return ret;
}

int main(void)
{
    TEEC_Result ret;
    TEEC_Operation operation = {0};
    uint32_t origin = 0;

    ret = teec_init();
    if (ret != TEEC_SUCCESS) {
        printf("teec_init Failed!\n");
        return -1;
    }

    operation.started = 1;
    operation.paramTypes = TEEC_PARAM_TYPES(
        TEEC_NONE,
        TEEC_NONE,
        TEEC_NONE,
        TEEC_NONE);
    ret = TEEC_InvokeCommand(&g_session, CMD_STORE_KEY_DEMO, &operation, &origin);
    if (ret != TEEC_SUCCESS) {
        printf("invoke failed, codes=0x%x, origin=0x%x", ret, origin);
    } else {
        printf("Succeed to run CMD_STORE_KEY_DEMO sample\n");
    }

    ret = TEEC_InvokeCommand(&g_session, CMD_STORE_DATA_DEMO, &operation, &origin);
    if (ret != TEEC_SUCCESS) {
        printf("invoke failed, codes=0x%x, origin=0x%x", ret, origin);
    } else {
        printf("Succeed to run CMD_STORE_DATA_DEMO sample\n");
    }

    ret = TEEC_InvokeCommand(&g_session, CMD_ENUMERATOR_DEMO, &operation, &origin);
    if (ret != TEEC_SUCCESS) {
        printf("invoke failed, codes=0x%x, origin=0x%x", ret, origin);
    } else {
        printf("Succeed to run CMD_ENUMERATOR_DEMO sample\n");
    }

    TEEC_CloseSession(&g_session);
    TEEC_FinalizeContext(&g_context);
    return ret;
}
