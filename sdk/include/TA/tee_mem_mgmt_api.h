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

/**
 * @file tee_mem_mgmt_api.h
 *
 * @brief �ڴ�����ӿ�
 *
 * �����߿���ʹ����Щ�ӿ�ʵ�ֶ��ڴ������صĹ��ܡ�
 *
 * @since 1
 */
#ifndef TEE_MEM_MGMT_API_H
#define TEE_MEM_MGMT_API_H

#include "tee_defines.h"
#include "tee_mem_monitoring_api.h"

/*
 * below definitions are defined by Global Platform or Platform SDK released previously
 * for compatibility:
 * don't make any change to the content below
 */
#ifndef ZERO_SIZE_PTR
#define ZERO_SIZE_PTR       ((void *)16)
#define zero_or_null_ptr(x) ((unsigned long)(x) <= (unsigned long)ZERO_SIZE_PTR)
#endif

enum MALLOC_HINT {
    ZERO           = 0,
    NOT_ZERO       = 1,
    ALIGN_004      = 0x80000002, /* buf align */
    ALIGN_008      = 0x80000003,
    ALIGN_016      = 0x80000004,
    ALIGN_032      = 0x80000005,
    ALIGN_064      = 0x80000006,
    ALIGN_128      = 0x80000007,
    ALIGN_256      = 0x80000008,
    ALIGN_004_ZERO = 0x80000012, /* buf align and set to zero */
    ALIGN_008_ZERO = 0x80000013,
    ALIGN_016_ZERO = 0x80000014,
    ALIGN_032_ZERO = 0x80000015,
    ALIGN_064_ZERO = 0x80000016,
    ALIGN_128_ZERO = 0x80000017,
    ALIGN_256_ZERO = 0x80000018,
};

#define TEE_MALLOC_FILL_ZERO 0x00000000
#define TEE_MALLOC_NO_FILL   0x00000001
#define TEE_MALLOC_NO_SHARE  0x00000002

#define TEE_MEMORY_ACCESS_READ      0x00000001
#define TEE_MEMORY_ACCESS_WRITE     0x00000002
#define TEE_MEMORY_ACCESS_ANY_OWNER 0x00000004

/**
 * @brief ��x��仺�����ĵ�һ����С�ֽ�
 *
 * @param buffer [OUT]������ָ��
 * @param x [IN]���ֵ
 * @param size [IN]�ֽ���
 *
 */
#if defined(API_LEVEL) && (API_LEVEL >= API_LEVEL1_2)
void TEE_MemFill(void *buffer, uint8_t x, size_t size);
#else
void TEE_MemFill(void *buffer, uint32_t x, size_t size);
#endif

/**
 * @brief ����С�ֽڴ�src���Ƶ�dest
 *
 * @param dest [OUT]dest������ָ��
 * @param src [IN]src������ָ��
 * @param size [IN]�ֽ���
 *
 */
void TEE_MemMove(void *dest, const void *src, size_t size);

/**
 * @brief ʹ����ʾֵ�����С�ֽڵ��ڴ淵�ص�ָ�뽫�����κ�C������������
 *
 * @param size [IN]��������ڴ��С
 * @param hint [IN]��־��0��ʾ���ص��ڴ潫��䡰\0��
 *
 * @return ָ���·����ڴ��ָ��
 * @return NULL ��ʾ����ʱʧ��
 *
 */
void *TEE_Malloc(size_t size, uint32_t hint);

 /**
  * @brief �ͷ�TEE_Malloc������ڴ�
  *
  * �������������NULL����TEE_Free����ִ���κβ���\n
  * ������Ӧȷ������������TEE_Malloc��TEE_Realloc�����ģ����Ҳ�Ӧ�����ͷ�һ���ڴ棬�����������Ԥ��
  *
  * @param buffer [IN]ָ���ڴ��ָ��
  *
  */
void TEE_Free(void *buffer);

/**
 * @brief ���·����ڴ�
 *
 * ���new_size���ھ�size������ڴ�����ݲ�����ģ�ʣ���ڴ�������ֽ�\n
 * �޸��ڴ��Сʱ����һ���µķ������\n
 * �������ʧ�ܣ������ؾ��ڴ棬�˺���������NULL\n
 * �������������NULL����˺�����TEE_Malloc��ͬ
 *
 * @param buffer [IN]ָ���ڴ��ָ��
 * @param new_size [IN]���·���Ĵ�С
 *
 * @return ָ�����ڴ��ָ�룬��ӦΪNULL
 * @return NULL��ʾʧ��
 *
 */
void *TEE_Realloc(void *buffer, size_t new_size);

/**
 * @brief �ڴ����ݱȽ�
 *
 * @param buffer1 [IN]��һ��ָ��
 * @param buffer2 [IN]�ڶ���ָ��
 * @param size [IN]Ҫ�Ƚϵ��ֽڴ�С
 *
 * @return -1 buffer1 < buffer2
 * @return 0 buffer1 == buffer2
 * @return 1 buffer1 > buffer2
 *
 */
int32_t TEE_MemCompare(const void *buffer1, const void *buffer2, size_t size);

/**
 * @brief ��黺�����ķ���Ȩ��
 *
 * @param accessFlags [IN]�����ķ���Ȩ��
 * @param buffer [IN]ָ���ڴ��ָ��
 * @param size [IN]Ҫ�����ڴ��С
 *
 * @return TEE_SUCCESS ���з���Ȩ��
 * @return TEE_ERROR_ACCESS_DENIED û�з���Ȩ��
 */
TEE_Result TEE_CheckMemoryAccessRights(uint32_t accessFlags, const void *buffer, size_t size);

/**
 * @brief ������ͬһʵ���Ĳ�ͬ�Ự�й����ȫ�ֱ���
 *
 * @param instanceData [IN]ȫ�ֱ�����ַ
 *
 */
void TEE_SetInstanceData(void *instanceData);

/**
 * @brief ��ȡTEE_SetInstanceData���õ�ָ��
 *
 * @return ָ��TEE_SetInstanceData���õı�����ָ�룬ָ�벻ӦΪNULL
 * @return NULL δ����InstanceData
 *
 */
void *TEE_GetInstanceData(void);

#endif
