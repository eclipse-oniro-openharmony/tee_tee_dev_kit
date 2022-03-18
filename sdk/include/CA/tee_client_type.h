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

#ifndef TEE_CLIENT_TYPE_H
#define TEE_CLIENT_TYPE_H
/**
 * @addtogroup TeeClient
 * @{
 *
 * @brief TEEC_API �ͻ���(�ǰ�ȫ��)�ӿڡ�
 *
 * �ṩ�ǰ�ȫ��(����ģʽ)�¿ͻ��˳�����ʰ�ȫģʽ�°�ȫӦ����ؽӿڡ�
 *
 * @since 8
 */

/**
 * @file tee_client_type.h
 *
 * @brief �����������ͺ����ݽṹ���塣
 *
 * @since 8
 */

#include <semaphore.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include "tee_client_constants.h"

/**
 * @brief �������Ͷ���
 *
 * @since 8
 */
struct ListNode {
    struct ListNode *next;  /* point to next node  */
    struct ListNode *prev;  /* point to prev node */
};

/**
 * @brief ��������ֵ���Ͷ��塣
 *
 * @since 8
 */
typedef enum TEEC_ReturnCode TEEC_Result;

/**
 * @brief UID�������Ͷ��壬��ѭRFC4122 [2]�����ڱ�ʶ��ȫӦ�á�
 *
 * @since 8
 */
typedef struct {
    uint32_t timeLow;
    uint16_t timeMid;
    uint16_t timeHiAndVersion;
    uint8_t clockSeqAndNode[8];
} TEEC_UUID;

/**
 * @brief �����ͻ���Ӧ���밲ȫ����֮�佨�������ӻ�����
 *
 * @since 8
 */
typedef struct {
    int32_t fd;
    uint8_t *ta_path;
    struct ListNode session_list;
    struct ListNode shrd_mem_list;
    union {
        struct {
            void *buffer;
            sem_t buffer_barrier;
        } share_buffer;
        uint64_t imp;
    };
} TEEC_Context;

/**
 * @brief �����ͻ���Ӧ���밲ȫ����֮�佨���ĻỰ��
 *
 * @since 8
 */
typedef struct {
    uint32_t session_id;
    TEEC_UUID service_id;
    uint32_t ops_cnt;
    union {
        struct ListNode head;
        uint64_t imp;
    };
    TEEC_Context *context;
} TEEC_Session;

/**
 * @brief ����һ�鹲���ڴ棬����ע�ᣬҲ���Է��䡣
 *
 * @since 8
 */
typedef struct {
    void *buffer;
    uint32_t size;
    uint32_t flags;         /* TEEC_SharedMemCtl */
    uint32_t ops_cnt;
    bool is_allocated;
    union {
        struct ListNode head;
        void* imp;
    };
    TEEC_Context *context;
} TEEC_SharedMemory;

/**
 * @brief ����һ����ʱ������ָ�롣
 *
 * @since 8
 */
typedef struct {
    void *buffer;
    uint32_t size;
} TEEC_TempMemoryReference;

/**
 * @brief ���������ڴ�ָ�룬ָ������ע������õĹ����ڴ档
 *
 * @since 8
 */
typedef struct {
    TEEC_SharedMemory *parent;
    uint32_t size;
    uint32_t offset;
} TEEC_RegisteredMemoryReference;

/**
 * @brief �����������ݡ�
 *
 * @since 8
 */
typedef struct {
    uint32_t a;
    uint32_t b;
} TEEC_Value;

/**
 * @brief ����#TEEC_Operation����Ӧ�Ĳ������͡�
 *
 * @since 8
 */
typedef union {
    TEEC_TempMemoryReference tmpref;
    TEEC_RegisteredMemoryReference memref;
    TEEC_Value value;
} TEEC_Parameter;

/**
 * @brief �򿪻Ự��������ʱ�Ĳ�����
 *
 * @since 8
 */
typedef struct {
    uint32_t started;     /* 0 ����ȡ�������������ʾִ�и����� */
    uint32_t paramTypes;  /* ʹ�� TEEC_PARAM_TYPES �������ò��� */
    TEEC_Parameter params[TEEC_PARAM_NUM];
    TEEC_Session *session;
    bool cancel_flag;
} TEEC_Operation;

/** @} */
#endif
