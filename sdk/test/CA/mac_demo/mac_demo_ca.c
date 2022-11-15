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
static const TEEC_UUID g_macDemoUuid = {
    0x42a00d89, 0x1b0b, 0x4423,
    { 0x9a, 0x8d, 0x74, 0xe3, 0xae, 0x01, 0x6c, 0x8c }
};

#define DEMO_MAC_TA_PATH    "/vendor/bin/42a00d89-1b0b-4423-9a8d-74e3ae016c8c.sec"
#define OPERATION_START_FLAG 1

enum {
    CMD_SEND_CMD_AES_CBC_MAC_DEMO = 1,
    CMD_SEND_CMD_AES_CMAC_DEMO = 2,
};

TEEC_Context g_context;
TEEC_Session g_session;

static TEEC_Result teec_init(void)
{
    TEEC_Result ret;
    TEEC_Operation operation = {0};
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
        TEEC_NONE,
        TEEC_NONE);

    g_context.ta_path = (uint8_t *)DEMO_MAC_TA_PATH;
    ret = TEEC_OpenSession(&g_context, &g_session, &g_macDemoUuid,
        TEEC_LOGIN_IDENTIFY, NULL, &operation, &origin);
    if (ret != TEEC_SUCCESS) {
        printf("open session failed: result:%d, origin: %d\n", (int)ret, origin);
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

    operation.started = OPERATION_START_FLAG;
    operation.paramTypes = TEEC_PARAM_TYPES(
        TEEC_NONE,
        TEEC_NONE,
        TEEC_NONE,
        TEEC_NONE);

    ret = TEEC_InvokeCommand(&g_session, CMD_SEND_CMD_AES_CBC_MAC_DEMO, &operation, &origin);
    if (ret != TEEC_SUCCESS) {
        printf("AES-CBC_MAC failed, codes=0x%x, origin=0x%x\n", ret, origin);
    } else {
        printf("CA AES-CBC-MAC Demo Runs Successfully\n");
    }

    ret = TEEC_InvokeCommand(&g_session, CMD_SEND_CMD_AES_CMAC_DEMO, &operation, &origin);
    if (ret != TEEC_SUCCESS) {
        printf("AES-CMAC failed, codes=0x%x, origin=0x%x\n", ret, origin);
    } else {
        printf("CA AES-CMAC Demo Run Successfully\n");
    }

    TEEC_CloseSession(&g_session);
    TEEC_FinalizeContext(&g_context);
    return ret;
}
