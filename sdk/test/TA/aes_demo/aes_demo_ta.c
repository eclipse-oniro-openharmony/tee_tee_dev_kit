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

enum DEMO_AES_CRYPTO_CMD {
    CMD_AES_CBC_ENCRYPT_DECRYPT = 0,
    CMD_AES_CCM_ENCRYPT_DECRYPT,
};

TEE_Result TA_CreateEntryPoint(void)
{
    TEE_Result ret;
    tlogd("----- AES TA create entry point -----\n");

    /* When you develop your own CA, you need to change the name to your own path and CA name. */
    ret = AddCaller_CA_exec("/vendor/bin/aes_demo_ca", 0);
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
    tlogd("---- AES TA open session entry point --------\n");
    (void)param_types;
    (void)params;
    (void)session_context;
    return TEE_SUCCESS;
}

#define AES_KEY_SIZE_MAX 128     // AES key size only can be 128, 192, 256
#define BIT_TO_BYTE 8
#define UPDATE_BLOCK_SIZE 16
#define AES_CBC_CIPHER_LEN 48
static TEE_Result demo_aes_cbc_pkcs5_encrypt(TEE_ObjectHandle key_obj, uint8_t *cipher, size_t *cipher_size)
{
    uint8_t iv[] = {
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
        0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f};
    size_t iv_size = sizeof(iv);
    uint8_t plant[] = {
        0x39, 0xf2, 0x33, 0x69, 0xa9, 0xd9, 0xba, 0xcf,
        0xa5, 0x30, 0xe2, 0x63, 0x04, 0x23, 0x14, 0x61,
        0xb2, 0xeb, 0x05, 0xe2, 0xc3, 0x9b, 0xe9, 0xfc,
        0xda, 0x6c, 0x19, 0x07, 0x8c, 0x6a, 0x9d, 0x1b};
    uint32_t plant_size = sizeof(plant);
    TEE_Result ret;
    TEE_OperationHandle oper_enc = NULL;

    ret = TEE_AllocateOperation(&oper_enc, TEE_ALG_AES_CBC_PKCS5, TEE_MODE_ENCRYPT, AES_KEY_SIZE_MAX);
    if (ret != TEE_SUCCESS) {
        tloge("Allocate aes cbc encrypted operation fail, ret 0x%x\n", ret);
        return ret;
    }
    ret = TEE_SetOperationKey(oper_enc, key_obj);
    if (ret != TEE_SUCCESS) {
        tloge("Set aes cbc encrypted operation key fail, ret 0x%x\n", ret);
        goto error;
    }

    TEE_CipherInit(oper_enc, iv, iv_size);
    ret = TEE_CipherDoFinal(oper_enc, plant, plant_size, cipher, cipher_size);
    if (ret != TEE_SUCCESS || *cipher_size != AES_CBC_CIPHER_LEN) {
        tloge("Do final aes cbc encrypt fail, ret=0x%x, plant_size=0x%x, dst_len=0x%x\n",
            ret, plant_size, *cipher_size);
        ret = TEE_FAIL;
        goto error;
    }

error:
    TEE_FreeOperation(oper_enc);
    return ret;
}

static TEE_Result demo_aes_cbc_pkcs5_decrypt(TEE_ObjectHandle key_obj,
    uint8_t *cipher, size_t cipher_size, uint8_t *decrypted, size_t *dec_size)
{
    uint8_t iv[] = {
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
        0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f};
    size_t iv_size = sizeof(iv);
    TEE_Result ret;
    TEE_OperationHandle oper_dec = NULL;

    ret = TEE_AllocateOperation(&oper_dec, TEE_ALG_AES_CBC_PKCS5, TEE_MODE_DECRYPT, AES_KEY_SIZE_MAX);
    if (ret != TEE_SUCCESS) {
        tloge("Allocate aes cbc decrypted operation fail, ret 0x%x\n", ret);
        return ret;
    }

    ret = TEE_SetOperationKey(oper_dec, key_obj);
    if (ret != TEE_SUCCESS) {
        tloge("Set aes cbc decrypted operation key fail, ret 0x%x\n", ret);
        goto error;
    }

    TEE_CipherInit(oper_dec, iv, iv_size);
    ret = TEE_CipherDoFinal(oper_dec, cipher, cipher_size, decrypted, dec_size);
    if (ret != TEE_SUCCESS || *dec_size != (cipher_size - iv_size)) {
        tloge("Do final aes cbc decrypt fail, ret=0x%x, cipher_size=0x%x, dst_len=0x%x\n", ret, cipher_size, *dec_size);
        ret = TEE_FAIL;
        goto error;
    }
error:
    TEE_FreeOperation(oper_dec);
    return ret;
}

static TEE_Result test_demo_aes_cbc_pkcs5()
{
    uint8_t plant[] = {
        0x39, 0xf2, 0x33, 0x69, 0xa9, 0xd9, 0xba, 0xcf,
        0xa5, 0x30, 0xe2, 0x63, 0x04, 0x23, 0x14, 0x61,
        0xb2, 0xeb, 0x05, 0xe2, 0xc3, 0x9b, 0xe9, 0xfc,
        0xda, 0x6c, 0x19, 0x07, 0x8c, 0x6a, 0x9d, 0x1b};
    uint32_t plant_size = sizeof(plant);
    uint8_t cipher[AES_CBC_CIPHER_LEN] = {0};
    uint8_t decrypted[AES_CBC_CIPHER_LEN] = {0};
    size_t cipher_size = AES_CBC_CIPHER_LEN;
    size_t dec_size = AES_CBC_CIPHER_LEN;
    TEE_Result ret;
    TEE_ObjectHandle key_obj = NULL;

    ret = TEE_AllocateTransientObject(TEE_TYPE_AES, AES_KEY_SIZE_MAX, &key_obj);
    if (ret != TEE_SUCCESS) {
        tloge("Allocate transient object fail\n");
        return ret;
    }
    ret = TEE_GenerateKey(key_obj, AES_KEY_SIZE_MAX, NULL, 0);
    if (ret != TEE_SUCCESS) {
        tloge("Failed to generate aes cbc key, ret=0x%x", ret);
        goto error;
    }
    ret = demo_aes_cbc_pkcs5_encrypt(key_obj, cipher, &cipher_size);
    if (ret != TEE_SUCCESS) {
        tloge("Demo aes cbc pkcs5 encrypt test fail\n");
        goto error;
    }
    ret = demo_aes_cbc_pkcs5_decrypt(key_obj, cipher, cipher_size, decrypted, &dec_size);
    if (ret != TEE_SUCCESS) {
        tloge("Demo aes cbc pkcs5 decrypt test fail\n");
        goto error;
    }
    if (memcmp(plant, decrypted, plant_size) != 0) {
        tloge("AES CBC decrypted result incorrect\n");
        ret = TEE_FAIL;
        goto error;
    }
    tlogd("Demo aes cbc pkcs5 test pass!\n");
error:
    TEE_FreeTransientObject(key_obj);
    return ret;
}

#define AES_CCM_CIPHER_LEN 16
#define AES_CCM_NONCE_LEN 7
#define AES_CCM_TAG_LEN 32
static TEE_Result demo_aes_ccm_encrypt(TEE_ObjectHandle key_obj,
    uint8_t *cipher, size_t *cipher_size, uint8_t *tag, size_t *tag_size)
{
    TEE_Result ret;
    TEE_OperationHandle oper_enc = NULL;
    uint8_t nonce[AES_CCM_NONCE_LEN] = {0x14, 0xD7, 0xF6, 0xC9, 0x3D, 0x1F, 0x53};
    const uint32_t nonce_size = AES_CCM_NONCE_LEN;
    uint8_t aad[AES_CCM_CIPHER_LEN] = {
        0x06, 0x55, 0xAD, 0xA9, 0x1B, 0xFC, 0x6D, 0xD2,
        0x88, 0x27, 0xB7, 0x4A, 0x76, 0x9B, 0xDB, 0xD6};
    const uint32_t aad_size = AES_CCM_CIPHER_LEN;
    uint8_t plant[AES_CCM_CIPHER_LEN] = {
        0xF7, 0x44, 0x6E, 0xB2, 0x73, 0xED, 0x03, 0xE8,
        0x1D, 0x24, 0x14, 0xFA, 0xB8, 0x7D, 0xC6, 0xCB};
    const uint32_t plant_size = AES_CCM_CIPHER_LEN;

    ret = TEE_AllocateOperation(&oper_enc, TEE_ALG_AES_CCM, TEE_MODE_ENCRYPT, AES_KEY_SIZE_MAX);
    if (ret != TEE_SUCCESS) {
        tloge("Allocate aes ccm encrypted operation fail, ret 0x%x\n", ret);
        return ret;
    }

    ret = TEE_SetOperationKey(oper_enc, key_obj);
    if (ret != TEE_SUCCESS) {
        tloge("Set aes ccm encrypted operation key fail, ret 0x%x\n", ret);
        goto error;
    }

    ret = TEE_AEInit(oper_enc, nonce, nonce_size, *tag_size, aad_size, plant_size);
    if (ret != TEE_SUCCESS) {
        tloge("AE init aes ccm encrypted operation fail, ret 0x%x\n", ret);
        goto error;
    }

    /* Could be called many times */
    TEE_AEUpdateAAD(oper_enc, aad, aad_size);

    /* We ignore TEE_AEUpdate because we do it on TEE_AEEncryptFinal or TEE_AEDecryptFinal */
    ret = TEE_AEEncryptFinal(oper_enc, plant, plant_size, cipher, cipher_size, tag, tag_size);
    if (ret != TEE_SUCCESS  || *cipher_size != AES_CCM_CIPHER_LEN) {
        tloge("AE final aes ccm encrypt fail, ret 0x%x\n, cipher_size %u, tag_size %u", ret, *cipher_size, *tag_size);
        ret = TEE_FAIL;
        goto error;
    }

error:
    TEE_FreeOperation(oper_enc);
    return ret;
}

static TEE_Result demo_aes_ccm_decrypt(TEE_ObjectHandle key_obj, uint8_t *cipher, size_t cipher_size,
    uint8_t *decryted, size_t *dec_size, uint8_t *tag, size_t tag_size)
{
    TEE_Result ret;
    TEE_OperationHandle oper_dec = NULL;
    uint8_t nonce[AES_CCM_NONCE_LEN] = {0x14, 0xD7, 0xF6, 0xC9, 0x3D, 0x1F, 0x53};
    const uint32_t nonce_size = AES_CCM_NONCE_LEN;
    uint8_t aad[AES_CCM_CIPHER_LEN] = {
        0x06, 0x55, 0xAD, 0xA9, 0x1B, 0xFC, 0x6D, 0xD2,
        0x88, 0x27, 0xB7, 0x4A, 0x76, 0x9B, 0xDB, 0xD6};
    const uint32_t aad_size = AES_CCM_CIPHER_LEN;
    const uint32_t plant_size = AES_CCM_CIPHER_LEN;

    ret = TEE_AllocateOperation(&oper_dec, TEE_ALG_AES_CCM, TEE_MODE_DECRYPT, AES_KEY_SIZE_MAX);
    if (ret != TEE_SUCCESS) {
        tloge("Allocate aes ccm decrypted operation fail, ret 0x%x\n", ret);
        return ret;
    }

    ret = TEE_SetOperationKey(oper_dec, key_obj);
    if (ret != TEE_SUCCESS) {
        tloge("Set aes ccm decrypted operation key fail, ret 0x%x\n", ret);
        goto error;
    }

    ret = TEE_AEInit(oper_dec, nonce, nonce_size, tag_size * BIT_TO_BYTE, aad_size, plant_size);
    if (ret != TEE_SUCCESS) {
        tloge("AE init aes ccm decrypted operation fail, ret 0x%x\n", ret);
        goto error;
    }

    /* Could be called many times */
    TEE_AEUpdateAAD(oper_dec, aad, aad_size);
    ret = TEE_AEDecryptFinal(oper_dec, cipher, cipher_size, decryted, dec_size, tag, tag_size);
    if (ret != TEE_SUCCESS  || *dec_size != AES_CCM_CIPHER_LEN) {
        tloge("AE final aes ccm decrypt fail, ret 0x%x\n", ret);
        ret = TEE_FAIL;
        goto error;
    }

error:
    TEE_FreeOperation(oper_dec);
    return ret;
}

static TEE_Result test_demo_aes_ccm()
{
    TEE_Result ret;
    TEE_ObjectHandle key_obj = NULL;
    uint8_t plant[AES_CCM_CIPHER_LEN] = {
        0xF7, 0x44, 0x6E, 0xB2, 0x73, 0xED, 0x03, 0xE8,
        0x1D, 0x24, 0x14, 0xFA, 0xB8, 0x7D, 0xC6, 0xCB};
    const uint32_t plant_size = AES_CCM_CIPHER_LEN;
    uint8_t cipher[AES_CCM_CIPHER_LEN] = {0};
    size_t cipher_size = AES_CCM_CIPHER_LEN;
    uint8_t decryted[AES_CCM_CIPHER_LEN] = {0};
    size_t dec_size = AES_CCM_CIPHER_LEN;
    size_t tag_size = AES_CCM_TAG_LEN;
    uint8_t tag[AES_CCM_TAG_LEN] = {0};
    ret = TEE_AllocateTransientObject(TEE_TYPE_AES, AES_KEY_SIZE_MAX, &key_obj);
    if (ret != TEE_SUCCESS) {
        tloge("Allocate transient object fail\n");
        return ret;
    }
    ret = TEE_GenerateKey(key_obj, AES_KEY_SIZE_MAX, NULL, 0);
    if (ret != TEE_SUCCESS) {
        tloge("Failed to generate aes ccm key, ret=0x%x", ret);
        goto error;
    }
    ret = demo_aes_ccm_encrypt(key_obj, cipher, &cipher_size, tag, &tag_size);
    if (ret != TEE_SUCCESS) {
        tloge("Demo aes ccm encrypt test fail\n");
        goto error;
    }
    ret = demo_aes_ccm_decrypt(key_obj, cipher, cipher_size, decryted, &dec_size, tag, tag_size);
    if (ret != TEE_SUCCESS) {
        tloge("Demo aes ccm decrypt test fail\n");
        goto error;
    }
    if (memcmp(plant, decryted, plant_size) != 0) {
        tloge("AES CCM decrypted result incorrect\n");
        ret = TEE_FAIL;
        goto error;
    }
    tlogd("Demo aes ccm test pass!\n");
error:
    TEE_FreeTransientObject(key_obj);
    return ret;
}

TEE_Result TA_InvokeCommandEntryPoint(void *session_context, uint32_t cmd_id, uint32_t param_types, TEE_Param params[4])
{
    (void)session_context;
    (void)param_types;
    (void)params;
    TEE_Result ret;

    tlogd("---- AES TA invoke command entry point ----cmdid: %u-------\n", cmd_id);
    switch (cmd_id) {
    case CMD_AES_CBC_ENCRYPT_DECRYPT:
        ret = test_demo_aes_cbc_pkcs5();
        break;
    case CMD_AES_CCM_ENCRYPT_DECRYPT:
        ret = test_demo_aes_ccm();
        break;
    default:
        tloge("AES unknown command id: %u\n", cmd_id);
        ret = TEE_FAIL;
    }
    return ret;
}

void TA_CloseSessionEntryPoint(void *session_context)
{
    (void)session_context;
    tlogd("---- AES TA Close session entry point -----\n");
}

void TA_DestroyEntryPoint(void)
{
    tlogd("---- AES TA destroy entry point ----\n");
}
