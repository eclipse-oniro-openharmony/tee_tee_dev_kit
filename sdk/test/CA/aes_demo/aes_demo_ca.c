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
#include <stdlib.h>
#include <string.h>
#include "tee_client_api.h"
#include "securec.h"

/* AES CRYPTO DEMO uuid: b3cb259b-a92c-478a-94e1-4ccb7406d25a */
static const TEEC_UUID g_aesCryptoUuid = {
    0xb3cb259b, 0xa92c, 0x478a,
    { 0x94, 0xe1, 0x4c, 0xcb, 0x74, 0x06, 0xd2, 0x5a }
};

enum DEMO_AES_CRYPTO_CMD {
    CMD_AES_CBC_ENCRYPT_DECRYPT = 0,
    CMD_AES_CCM_ENCRYPT_DECRYPT,
};

int main(void)
{
    TEEC_Result result;
    TEEC_Context context;
    TEEC_Operation operation;
    TEEC_Session session;
    uint32_t origin;

    result = TEEC_InitializeContext(NULL, &context);
    if (result != TEEC_SUCCESS) {
        printf("AES demo teec initial failed.\n");
        return result;
    }

    if (memset_s(&operation, sizeof(operation), 0x00, sizeof(operation)) != 0) {
        printf("AES demo mem set failed.\n");
        goto cleanup_1;
    }
    operation.started = 1;
    operation.paramTypes = TEEC_PARAM_TYPES(TEEC_NONE, TEEC_NONE, TEEC_MEMREF_TEMP_INPUT, TEEC_MEMREF_TEMP_INPUT);

    result = TEEC_OpenSession(&context, &session, &g_aesCryptoUuid,
        TEEC_LOGIN_IDENTIFY, NULL,  &operation, &origin);
    if (result != TEEC_SUCCESS) {
        printf("AES demo teec open session failed.\n");
        goto cleanup_1;
    } else {
        printf("AES demo teec open session successed.\n");
    }

    operation.started = 1;
    operation.paramTypes = TEEC_PARAM_TYPES(TEEC_NONE, TEEC_NONE, TEEC_NONE, TEEC_NONE);

    result = TEEC_InvokeCommand(&session, CMD_AES_CBC_ENCRYPT_DECRYPT, &operation, &origin);
    if (result != TEEC_SUCCESS) {
        printf("AES CBC encrypt and decrypt test failed, result=0x%x, origin=0x%x.\n", result, origin);
        goto cleanup_2;
    } else {
        printf("AES CBC encrypt and decrypt test successed.\n");
    }
    result = TEEC_InvokeCommand(&session, CMD_AES_CCM_ENCRYPT_DECRYPT, &operation, &origin);
    if (result != TEEC_SUCCESS) {
        printf("AES CCM encrypt and decrypt test failed, result=0x%x, origin=0x%x.\n", result, origin);
        goto cleanup_2;
    } else {
        printf("AES CCM encrypt and decrypt test successed.\n");
    }

cleanup_2:
    TEEC_CloseSession(&session);
cleanup_1:
    TEEC_FinalizeContext(&context);
    return result;
}