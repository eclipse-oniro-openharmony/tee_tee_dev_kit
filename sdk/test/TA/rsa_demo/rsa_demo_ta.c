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
#include <string.h>

#define RSA_KEY_SIZE        2048
#define RSA_MASSAGE_SIZE    100
#define MSG_DIGEST_SIZE     32

enum {
    CMD_RSA_ENCRYPT_DECRYPT = 0,
    CMD_RSA_SIGN_VERIFY,
};

TEE_Result TA_CreateEntryPoint(void)
{
    TEE_Result ret;

    tlogd("----- RSA TA creat entry point -----\n");

    /* When you develop your own CA, you need to change the name to your own path and CA name. */
    ret = AddCaller_CA_exec("/vendor/bin/rsa_demo_ca", 0);
    if (ret == TEE_SUCCESS) {
        tlogd("TA entry point: add ca whitelist success");
    } else {
        tloge("TA entry point: add ca whitelist failed");
        return TEE_ERROR_GENERIC;
    }

    return TEE_SUCCESS;
}

TEE_Result TA_OpenSessionEntryPoint(uint32_t param_types, TEE_Param params[4], void **session_context)
{
    tlogd("---- RSA TA open session entry point --------\n");
    (void)param_types;
    (void)params;
    (void)session_context;
    return TEE_SUCCESS;
}

TEE_Result generate_random(uint8_t *msg_buf, uint32_t *msg_len)
{
    TEE_Result ret;
    uint32_t len;

    if (msg_buf == NULL || msg_len == NULL || *msg_len == 0) {
        tloge("Generating random bad parameters\n");
        return TEE_ERROR_BAD_PARAMETERS;
    }
    len = *msg_len;
    TEE_GenerateRandom(msg_buf, len);

    uint8_t *buf = (uint8_t *)TEE_Malloc(len, 0);
    if (buf == NULL) {
        tloge("Malloc fail\n");
        return TEE_ERROR_OUT_OF_MEMORY;
    }
    if (memcmp(msg_buf, buf, len) == 0) {
        *msg_len = 0;
        ret = TEE_ERROR_GENERIC;
        tloge("Generated random error, all bytes equal 0\n");
    } else {
        ret = TEE_SUCCESS;
    }
    TEE_Free(buf);
    return ret;
}

TEE_Result demo_rsa_encrypt(TEE_ObjectHandle rsa_key_obj,
    uint8_t *msg_buffer, size_t msg_len, uint8_t *encrypted_buffer, size_t *enc_len)
{
    TEE_Result ret;
    TEE_OperationHandle oper_enc = NULL;

    ret = TEE_AllocateOperation(&oper_enc, TEE_ALG_RSAES_PKCS1_OAEP_MGF1_SHA512, TEE_MODE_ENCRYPT, RSA_KEY_SIZE);
    if (ret != TEE_SUCCESS) {
        tloge("Fail to allocate rsa encrypt operation, ret 0x%x\n", ret);
        return ret;
    }

    ret = TEE_SetOperationKey(oper_enc, rsa_key_obj);
    if (ret != TEE_SUCCESS) {
        tloge("Fail to set rsa encrypt key, ret 0x%x\n", ret);
        TEE_FreeOperation(oper_enc);
        return ret;
    }

    ret = TEE_AsymmetricEncrypt(oper_enc, NULL, 0, msg_buffer, msg_len, encrypted_buffer, enc_len);
    if (ret != TEE_SUCCESS)
        tloge("Fail to do rsa encrypt, ret 0x%x\n", ret);

    TEE_FreeOperation(oper_enc);
    return ret;
}

TEE_Result demo_rsa_decrypt(TEE_ObjectHandle rsa_key_obj,
    uint8_t *encrypted_buffer, size_t enc_len, uint8_t *decrypted_buffer, size_t *dec_len)
{
    TEE_Result ret;
    TEE_OperationHandle oper_dec = NULL;

    ret = TEE_AllocateOperation(&oper_dec, TEE_ALG_RSAES_PKCS1_OAEP_MGF1_SHA512, TEE_MODE_DECRYPT, RSA_KEY_SIZE);
    if (ret != TEE_SUCCESS) {
        tloge("Fail to allocate rsa decrypt operation, ret 0x%x\n", ret);
        return ret;
    }
    ret = TEE_SetOperationKey(oper_dec, rsa_key_obj);
    if (ret != TEE_SUCCESS) {
        tloge("Fail to set rsa decrypt key, ret 0x%x\n", ret);
        TEE_FreeOperation(oper_dec);
        return ret;
    }
    ret = TEE_AsymmetricDecrypt(oper_dec, NULL, 0, encrypted_buffer, enc_len, decrypted_buffer, dec_len);
    if (ret != TEE_SUCCESS)
        tloge("Fail to do rsa decrypt, ret 0x%x\n", ret);

    TEE_FreeOperation(oper_dec);
    return ret;
}

TEE_Result test_demo_rsa_crypto()
{
    uint8_t msg_buffer[RSA_MASSAGE_SIZE] = {0};
    uint32_t msg_len = RSA_MASSAGE_SIZE;
    uint8_t encrypted_buffer[RSA_KEY_SIZE] = {0};
    size_t enc_len = RSA_KEY_SIZE;
    uint8_t decrypted_buffer[RSA_KEY_SIZE] = {0};
    size_t dec_len = RSA_KEY_SIZE;
    TEE_Result ret;
    TEE_ObjectHandle rsa_key_obj = NULL;

    ret = TEE_AllocateTransientObject(TEE_TYPE_RSA_KEYPAIR, RSA_KEY_SIZE, &rsa_key_obj);
    if (ret != TEE_SUCCESS) {
        tloge("Fail to allocate rsa transient object, ret 0x%x\n", ret);
        return ret;
    }

    ret = TEE_GenerateKey(rsa_key_obj, RSA_KEY_SIZE, NULL, 0);
    if (ret != TEE_SUCCESS) {
        tloge("Fail to generate rsa key, ret 0x%x\n", ret);
        goto error;
    }
    ret = generate_random(msg_buffer, &msg_len);
    if (ret != TEE_SUCCESS) {
        tloge("Fail to allocate rsa encrypt operation, ret 0x%x\n", ret);
        goto error;
    }
    ret = demo_rsa_encrypt(rsa_key_obj, msg_buffer, msg_len, encrypted_buffer, &enc_len);
    if (ret != TEE_SUCCESS) {
        tloge("Fail to test rsa encrypt demo, ret 0x%x\n", ret);
        goto error;
    }
    ret = demo_rsa_decrypt(rsa_key_obj, encrypted_buffer, enc_len, decrypted_buffer, &dec_len);
    if (ret != TEE_SUCCESS) {
        tloge("Fail to test rsa decrypt demo, ret 0x%x\n", ret);
        goto error;
    }
    if (memcmp(msg_buffer, decrypted_buffer, msg_len) != 0) {
        tloge("RSA decrypted result incorrect\n");
        ret = TEE_FAIL;
        goto error;
    }
    tlogd("Demo rsa crypto test pass!\n");

error:
    TEE_FreeTransientObject(rsa_key_obj);
    return ret;
}

TEE_Result demo_rsa_sign(TEE_ObjectHandle rsa_key_obj,
    uint8_t *buf, size_t buf_len, uint8_t *signature, size_t *sign_len)
{
    TEE_Result ret;
    TEE_OperationHandle oper_sign = NULL;

    ret = TEE_AllocateOperation(&oper_sign, TEE_ALG_RSASSA_PKCS1_PSS_MGF1_SHA256, TEE_MODE_SIGN, RSA_KEY_SIZE);
    if (ret != TEE_SUCCESS) {
        tloge("Fail to allocate rsa signature operation, ret 0x%x\n", ret);
        return ret;
    }

    ret = TEE_SetOperationKey(oper_sign, rsa_key_obj);
    if (ret != TEE_SUCCESS) {
        tloge("Fail to set rsa signature key, ret 0x%x\n", ret);
        goto clear;
    }
    ret = TEE_AsymmetricSignDigest(oper_sign, NULL, 0, buf, buf_len, signature, sign_len);
    if (ret != TEE_SUCCESS) {
        tloge("Fail to do rsa signature\n");
        ret = TEE_FAIL;
        goto clear;
    }

clear:
    TEE_FreeOperation(oper_sign);
    return ret;
}

TEE_Result demo_rsa_verify(TEE_ObjectHandle rsa_key_obj,
    uint8_t *buf, size_t buf_len, uint8_t *signature, size_t sign_len)
{
    TEE_Result ret;
    TEE_OperationHandle oper_verify = NULL;

    ret = TEE_AllocateOperation(&oper_verify, TEE_ALG_RSASSA_PKCS1_PSS_MGF1_SHA256, TEE_MODE_VERIFY, RSA_KEY_SIZE);
    if (ret != TEE_SUCCESS) {
        tloge("Fail to allocate rsa verify operation, ret 0x%x\n", ret);
        return ret;
    }

    ret = TEE_SetOperationKey(oper_verify, rsa_key_obj);
    if (ret != TEE_SUCCESS) {
        tloge("Fail to set rsa verify key, ret 0x%x\n", ret);
        goto clear;
    }

    ret = TEE_AsymmetricVerifyDigest(oper_verify, NULL, 0, buf, buf_len, signature, sign_len);
    if (ret != TEE_SUCCESS) {
        tloge("Fail to do rsa verify, ret 0x%x\n", ret);
        goto clear;
    }

clear:
    TEE_FreeOperation(oper_verify);
    return ret;
}

TEE_Result demo_message_do_digest(TEE_ObjectHandle rsa_key_obj, uint8_t *dig_buf, size_t *dig_len)
{
    uint8_t msg_buffer[RSA_MASSAGE_SIZE] = {0};
    uint32_t msg_len = RSA_MASSAGE_SIZE;
    TEE_Result ret;
    TEE_OperationHandle oper_digest = NULL;

    ret = TEE_GenerateKey(rsa_key_obj, RSA_KEY_SIZE, NULL, 0);
    if (ret != TEE_SUCCESS) {
        tloge("Fail to generate rsa key, ret 0x%x\n", ret);
        return ret;
    }
    ret = generate_random(msg_buffer, &msg_len);
    if (ret != TEE_SUCCESS) {
        tloge("Fail to generate massage random, ret 0x%x\n", ret);
        return ret;
    }
    /* Do sha256 digest for message, not required */
    ret = TEE_AllocateOperation(&oper_digest, TEE_ALG_SHA256, TEE_MODE_DIGEST, 0);
    if (ret != TEE_SUCCESS) {
        tloge("Fail to allocate sha256 digest operation, ret 0x%x\n", ret);
        return ret;
    }
    ret = TEE_DigestDoFinal(oper_digest, msg_buffer, msg_len, dig_buf, dig_len);
    if (ret != TEE_SUCCESS) {
        tloge("Fail to do final sha256 digest, ret 0x%x\n", ret);
        TEE_FreeOperation(oper_digest);
        return ret;
    }

    TEE_FreeOperation(oper_digest);
    return TEE_SUCCESS;
}

TEE_Result test_demo_rsa_sign_verify()
{
    uint8_t signature[RSA_KEY_SIZE] = {0};
    size_t sign_len = RSA_KEY_SIZE;
    TEE_Result ret;
    uint8_t dig_buf[MSG_DIGEST_SIZE] = {0};
    size_t dig_len = MSG_DIGEST_SIZE;
    TEE_ObjectHandle rsa_key_obj = NULL;

    ret = TEE_AllocateTransientObject(TEE_TYPE_RSA_KEYPAIR, RSA_KEY_SIZE, &rsa_key_obj);
    if (ret != TEE_SUCCESS) {
        tloge("Fail to allocate rsa key pair object, ret 0x%x\n", ret);
        return ret;
    }

    ret = demo_message_do_digest(rsa_key_obj, dig_buf, &dig_len);
    if (ret != TEE_SUCCESS) {
        tloge("Fail to do digest for message, ret 0x%x\n", ret);
        goto clear_obj;
    }
    ret = demo_rsa_sign(rsa_key_obj, dig_buf, dig_len, signature, &sign_len);
    if (ret != TEE_SUCCESS) {
        tloge("Fail to test rsa signature demo, ret 0x%x\n", ret);
        goto clear_obj;
    }
    ret = demo_rsa_verify(rsa_key_obj, dig_buf, dig_len, signature, sign_len);
    if (ret != TEE_SUCCESS) {
        tloge("Fail to test rsa verify demo, ret 0x%x\n", ret);
        goto clear_obj;
    }
    tlogd("Demo rsa sign verify test pass!\n");

clear_obj:
    TEE_FreeTransientObject(rsa_key_obj);
    return ret;
}

TEE_Result TA_InvokeCommandEntryPoint(void *session_context, uint32_t cmd_id, uint32_t param_types, TEE_Param params[4])
{
    (void)session_context;
    (void)param_types;
    (void)params;
    TEE_Result ret;

    tlogd("---- RSA TA invoke command entry point ----cmdid: %u-------\n", cmd_id);
    switch (cmd_id) {
    case CMD_RSA_ENCRYPT_DECRYPT:
        ret = test_demo_rsa_crypto();
        break;
    case CMD_RSA_SIGN_VERIFY:
        ret = test_demo_rsa_sign_verify();
        break;
    default:
        tloge("RSA unknown command id: %u\n", cmd_id);
        ret = TEE_FAIL;
    }
    return  ret;
}

void TA_CloseSessionEntryPoint(void *session_context)
{
    (void)session_context;
    tlogd("---- RSA TA Close session entry point -----\n");
}

void TA_DestroyEntryPoint(void)
{
    tlogd("---- RSA TA destroy entry point ----\n");
}
