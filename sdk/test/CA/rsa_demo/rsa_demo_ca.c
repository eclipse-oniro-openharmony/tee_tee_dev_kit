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

/* AES CRYPTO DEMO uuid: c5fefe99-002d-48d1-a22e-55fc82684638 */
static const TEEC_UUID g_rsaCryptoUuid = {
    0xc5fefe99, 0x002d, 0x48d1,
    { 0xa2, 0x2e, 0x55, 0xfc, 0x82, 0x68, 0x46, 0x38 }
};

enum {
    CMD_RSA_ENCRYPT_DECRYPT = 0,
    CMD_RSA_SIGN_VERIFY,
};

int main(void)
{
    TEEC_Context context;
    TEEC_Session session;
    TEEC_Operation operation;
    uint32_t origin;
    TEEC_Result result;

    result = TEEC_InitializeContext(NULL, &context);
    if (result != TEEC_SUCCESS) {
        printf("RSA demo teec initial failed.\n");
        return result;
    }

    if (memset_s(&operation, sizeof(operation), 0x00, sizeof(operation)) != 0) {
        printf("RSA demo mem set failed.\n");
        goto cleanup_1;
    }
    operation.started = 1;
    operation.paramTypes = TEEC_PARAM_TYPES(TEEC_NONE, TEEC_NONE, TEEC_MEMREF_TEMP_INPUT, TEEC_MEMREF_TEMP_INPUT);

    result = TEEC_OpenSession(&context, &session, &g_rsaCryptoUuid,
        TEEC_LOGIN_IDENTIFY, NULL,  &operation, &origin);
    if (result != TEEC_SUCCESS) {
        printf("RSA demo teec open session failed.\n");
        goto cleanup_1;
    } else {
        printf("RSA demo teec open session successed.\n");
    }

    operation.started = 1;
    operation.paramTypes = TEEC_PARAM_TYPES(TEEC_NONE, TEEC_NONE, TEEC_NONE, TEEC_NONE);

    result = TEEC_InvokeCommand(&session, CMD_RSA_ENCRYPT_DECRYPT, &operation, &origin);
    if (result != TEEC_SUCCESS) {
        printf("RSA encrypt and decrypt test failed, result=0x%x, origin=0x%x.\n", result, origin);
        goto cleanup_2;
    } else {
        printf("RSA encrypt and decrypt test successed.\n");
    }

    result = TEEC_InvokeCommand(&session, CMD_RSA_SIGN_VERIFY, &operation, &origin);
    if (result != TEEC_SUCCESS) {
        printf("RSA sign and verify test failed, result=0x%x, origin=0x%x.\n", result, origin);
        goto cleanup_2;
    } else {
        printf("RSA signature and verify test successed.\n");
    }

cleanup_2:
    TEEC_CloseSession(&session);
cleanup_1:
    TEEC_FinalizeContext(&context);
    return result;
}