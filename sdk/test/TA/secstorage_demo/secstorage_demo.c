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

#include "tee_trusted_storage_api.h"
#include "tee_mem_mgmt_api.h"
#include "tee_log.h"
#include "securec.h"
#include "tee_ext_api.h"

#define PARAM_COUNT             4
#define MAX_AES_KEY_SIZE        256
#define ARRAY_SIZE              32

enum {
    CMD_STORE_KEY_DEMO = 1,
    CMD_STORE_DATA_DEMO = 2,
    CMD_ENUMERATOR_DEMO = 3,
};

const char *g_write_buffer = "It is a test for persist object!";

TEE_Result store_key_sample(void)
{
    uint32_t storage_id = TEE_OBJECT_STORAGE_PRIVATE;
    TEE_ObjectHandle transient_key = NULL;
    TEE_ObjectHandle persistent_key = NULL;
    TEE_Result ret;
    uint32_t w_flags = TEE_DATA_FLAG_ACCESS_WRITE;
    uint32_t r_flags = TEE_DATA_FLAG_ACCESS_READ;
    char attr_buffer[ARRAY_SIZE] = "test save attribute\n";
    void *object_id = "store_key_sample.txt";
    void *aes_key = NULL;

    tlogd("store_key_sample start:============");

    /* Allocating Storage Space for Keys */
    ret = TEE_AllocateTransientObject(TEE_TYPE_AES, MAX_AES_KEY_SIZE, (&transient_key));
    if (ret != TEE_SUCCESS) {
        tloge("Failed to execute allocate transient object:ret = 0x%x", ret);
        return ret;
    }

    TEE_Attribute *aes_attribute = (TEE_Attribute *)TEE_Malloc(sizeof(TEE_Attribute), 0);
    if (aes_attribute == NULL) {
        tloge("Failed to malloc");
        TEE_FreeTransientObject(transient_key);
        return TEE_ERROR_OUT_OF_MEMORY;
    }

    TEE_InitRefAttribute(aes_attribute, TEE_ATTR_SECRET_VALUE, attr_buffer, sizeof(attr_buffer));
    ret = TEE_PopulateTransientObject(transient_key, aes_attribute, 1);
    if (ret != TEE_SUCCESS) {
        tloge("Failed to execute populate transient object:ret = %d", ret);
        goto cleanup_1;
    }

    /* Create a persistent file to store the key */
    ret = TEE_CreatePersistentObject(storage_id, object_id, strlen(object_id), w_flags, transient_key, NULL, 0,
        (&persistent_key));
    if (ret != TEE_SUCCESS) {
        tloge("Failed to create object:ret = 0x%x", ret);
        goto cleanup_1;
    }
    TEE_CloseObject(persistent_key);

    /* Open the object to obtain key. */
    ret = TEE_OpenPersistentObject(storage_id, object_id, strlen(object_id),
        r_flags | TEE_DATA_FLAG_ACCESS_WRITE_META, (&persistent_key));
    if (ret != TEE_SUCCESS) {
        tloge("Failed to execute open persistent object:ret = %x", ret);
        TEE_Free(aes_key);
        return ret;
    }
    TEE_CloseObject(persistent_key);
    persistent_key = NULL;
cleanup_1:
    TEE_Free(aes_attribute);
    TEE_FreeTransientObject(transient_key);
    return ret;
}

TEE_Result check_store_data(uint32_t storage_id, void *create_object_id)
{
    char *read_buffer = NULL;
    uint32_t count = 0;
    TEE_ObjectHandle persistent_data = NULL;
    TEE_ObjectInfo object_info = {0};
    uint32_t r_flags = TEE_DATA_FLAG_ACCESS_READ;
    TEE_Result ret;

    /* Open the created file and read data. */
    ret = TEE_OpenPersistentObject(storage_id, create_object_id, strlen(create_object_id),
        r_flags, (&persistent_data));
    if (ret != TEE_SUCCESS) {
        tloge("Failed to open file:ret = 0x%x", ret);
        return ret;
    }

    ret = TEE_GetObjectInfo1(persistent_data, &object_info);
    if (ret != TEE_SUCCESS) {
        tloge("Failed to open file:ret = 0x%x", ret);
        TEE_CloseObject(persistent_data);
        return ret;
    }

    read_buffer = TEE_Malloc(object_info.dataSize + 1, 0);
    if (read_buffer == NULL) {
        tloge("Failed to open file:ret = 0x%x", ret);
        TEE_CloseObject(persistent_data);
        return ret;
    }

    /* Read data that has been stored in secure storage */
    ret = TEE_ReadObjectData(persistent_data, read_buffer, object_info.dataSize, &count);
    if (ret != TEE_SUCCESS) {
        TEE_CloseObject(persistent_data);
        TEE_Free(read_buffer);
        return ret;
    }
    if (TEE_MemCompare(g_write_buffer, read_buffer, strlen(g_write_buffer)) != 0) {
        TEE_CloseObject(persistent_data);
        TEE_Free(read_buffer);
        return TEE_FAIL;
    }
    tlogi("Succeed to Compare Date");
    /* close permanent object */
    TEE_CloseObject(persistent_data);
    TEE_Free(read_buffer);
    return TEE_SUCCESS;
}

TEE_Result store_data_sample(void)
{
    uint32_t storage_id = TEE_OBJECT_STORAGE_PRIVATE;
    uint32_t w_flags = TEE_DATA_FLAG_ACCESS_WRITE;
    void *create_object_id = "store_data_sample.txt";
    TEE_ObjectHandle persistent_data = NULL;
    TEE_Result ret;

    tlogd("store_data_sample start:============");

    /*
     * Create a permanent object: The flag is a write operation.
     * (The initialData, objectinfo, and attributes are written to the physical media.)
     */
    ret = TEE_CreatePersistentObject(storage_id, create_object_id, strlen(create_object_id), w_flags,
        TEE_HANDLE_NULL, NULL, 0, (&persistent_data));
    if (ret != TEE_SUCCESS) {
        tloge("Failed to create file: ret = 0x%x", ret);
        return ret;
    }

    ret = TEE_WriteObjectData(persistent_data, g_write_buffer, strlen(g_write_buffer));
    if (ret != TEE_SUCCESS) {
        tloge("Failed to write file: ret = 0x%x", ret);
        /*
         * After a file is opened or created, the exception branch must close the file.
         * Otherwise, memory leakage occurs.
         */
        TEE_CloseObject(persistent_data);
        return ret;
    }
    TEE_CloseObject(persistent_data);

    return check_store_data(storage_id, create_object_id);
}

#define HASH_LEN 32
#define DIR_LEN 64
#define HASH_NAME_BUFF_LEN (2 * HASH_LEN + 1 + DIR_LEN)

TEE_Result enumerator_sample()
{
    TEE_Result ret;
    TEE_ObjectEnumHandle object_enumerator = NULL;
    TEE_ObjectInfo obj_info = {0};
    uint8_t object_id[HASH_NAME_BUFF_LEN] = {0};
    size_t object_id_len = HASH_NAME_BUFF_LEN;

    ret = TEE_AllocatePersistentObjectEnumerator(&object_enumerator);
    if (ret != TEE_SUCCESS) {
        tloge("Failed to allocate objectEnumerator");
        return ret;
    }

    ret = TEE_StartPersistentObjectEnumerator(object_enumerator, TEE_OBJECT_STORAGE_PRIVATE);
    if (ret != TEE_SUCCESS) {
        tloge("Failed to start object_enumerator");
        goto clean;
    }

    ret = TEE_GetNextPersistentObject(object_enumerator, &obj_info, object_id, &object_id_len);
    if (ret != TEE_SUCCESS)
        tloge("Failed get next persistent object");
clean:
    TEE_FreePersistentObjectEnumerator(object_enumerator);
    return ret;
}

TEE_Result TA_CreateEntryPoint(void)
{
    TEE_Result ret;

    tlogd("----- TA entry point ----- ");
    /* When you develop your own CA, you need to change the name to your own path and CA name. */
    ret = AddCaller_CA_exec("/vendor/bin/secstorage_demo_ca", 0);
    if (ret == TEE_SUCCESS) {
        tlogd("TA entry point: add ca whitelist success");
    } else {
        tloge("TA entry point: add ca whitelist failed");
        return TEE_ERROR_GENERIC;
    }
    return TEE_SUCCESS;
}

TEE_Result TA_OpenSessionEntryPoint(uint32_t parm_type,
    TEE_Param params[PARAM_COUNT], void** session_context)
{
    (void)parm_type;
    (void)params;
    (void)session_context;
    tlogd("---- TA open session -------- ");

    return TEE_SUCCESS;
}

TEE_Result TA_InvokeCommandEntryPoint(void* session_context, uint32_t cmd_id, uint32_t parm_type,
    TEE_Param params[PARAM_COUNT])
{
    TEE_Result ret;
    (void)session_context;
    (void)params;
    tlogd("---- TA invoke command ----------- ");
    if (!check_param_type(parm_type,
        TEE_PARAM_TYPE_NONE,
        TEE_PARAM_TYPE_NONE,
        TEE_PARAM_TYPE_NONE,
        TEE_PARAM_TYPE_NONE)) {
        tloge("Bad expected parameter types");
        return TEE_ERROR_BAD_PARAMETERS;
    }

    switch (cmd_id) {
    case CMD_STORE_KEY_DEMO:
        ret = store_key_sample();
        if (ret != TEE_SUCCESS) {
            tloge("InvokeCommand Failed 0x%x. cmd is %u", ret, cmd_id);
            return ret;
        }
        break;
    case CMD_STORE_DATA_DEMO:
        ret = store_data_sample();
        if (ret != TEE_SUCCESS) {
            tloge("InvokeCommand Failed 0x%x. cmd is %u", ret, cmd_id);
            return ret;
        }
        break;
    case CMD_ENUMERATOR_DEMO:
        ret = enumerator_sample();
        if (ret != TEE_SUCCESS) {
            tloge("InvokeCommand Failed 0x%x. cmd is %u", ret, cmd_id);
            return ret;
        }
        break;
    default:
        tloge("Unknown cmd is %u", cmd_id);
        ret = TEE_ERROR_BAD_PARAMETERS;
    }

    return  ret;
}

void TA_CloseSessionEntryPoint(void* session_context)
{
    (void)session_context;
    tlogd("---- close session ----- ");
}

void TA_DestroyEntryPoint(void)
{
    tlogd("---- destory TA ---- ");
}
