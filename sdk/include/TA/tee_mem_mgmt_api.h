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
 * @brief 内存操作接口
 *
 * 开发者可以使用这些接口实现对内存操作相关的功能。
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
 * @brief 用x填充缓冲区的第一个大小字节
 *
 * @param buffer [OUT]缓冲区指针
 * @param x [IN]填充值
 * @param size [IN]字节数
 *
 */
#if defined(API_LEVEL) && (API_LEVEL >= API_LEVEL1_2)
void TEE_MemFill(void *buffer, uint8_t x, size_t size);
#else
void TEE_MemFill(void *buffer, uint32_t x, size_t size);
#endif

/**
 * @brief 将大小字节从src复制到dest
 *
 * @param dest [OUT]dest缓冲区指针
 * @param src [IN]src缓冲区指针
 * @param size [IN]字节数
 *
 */
void TEE_MemMove(void *dest, const void *src, size_t size);

/**
 * @brief 使用提示值分配大小字节的内存返回的指针将兼容任何C基本数据类型
 *
 * @param size [IN]将分配的内存大小
 * @param hint [IN]标志，0表示返回的内存将填充“\0”
 *
 * @return 指向新分配内存的指针
 * @return NULL 表示分配时失败
 *
 */
void *TEE_Malloc(size_t size, uint32_t hint);

 /**
  * @brief 释放TEE_Malloc分配的内存
  *
  * 如果缓冲区等于NULL，则TEE_Free将不执行任何操作\n
  * 调用者应确保缓冲区是由TEE_Malloc或TEE_Realloc创建的，并且不应两次释放一个内存，操作结果不可预测
  *
  * @param buffer [IN]指向内存的指针
  *
  */
void TEE_Free(void *buffer);

/**
 * @brief 重新分配内存
 *
 * 如果new_size大于旧size，则旧内存的内容不会更改，剩余内存是随机字节\n
 * 修改内存大小时将有一个新的分配操作\n
 * 如果分配失败，将返回旧内存，此函数将返回NULL\n
 * 如果缓冲区等于NULL，则此函数与TEE_Malloc相同
 *
 * @param buffer [IN]指向内存的指针
 * @param new_size [IN]重新分配的大小
 *
 * @return 指向新内存的指针，不应为NULL
 * @return NULL表示失败
 *
 */
void *TEE_Realloc(void *buffer, size_t new_size);

/**
 * @brief 内存内容比较
 *
 * @param buffer1 [IN]第一个指针
 * @param buffer2 [IN]第二个指针
 * @param size [IN]要比较的字节大小
 *
 * @return -1 buffer1 < buffer2
 * @return 0 buffer1 == buffer2
 * @return 1 buffer1 > buffer2
 *
 */
int32_t TEE_MemCompare(const void *buffer1, const void *buffer2, size_t size);

/**
 * @brief 检查缓冲区的访问权限
 *
 * @param accessFlags [IN]待检查的访问权限
 * @param buffer [IN]指向内存的指针
 * @param size [IN]要检查的内存大小
 *
 * @return TEE_SUCCESS 具有访问权限
 * @return TEE_ERROR_ACCESS_DENIED 没有访问权限
 */
TEE_Result TEE_CheckMemoryAccessRights(uint32_t accessFlags, const void *buffer, size_t size);

/**
 * @brief 用于在同一实例的不同会话中共享的全局变量
 *
 * @param instanceData [IN]全局变量地址
 *
 */
void TEE_SetInstanceData(void *instanceData);

/**
 * @brief 获取TEE_SetInstanceData设置的指针
 *
 * @return 指向TEE_SetInstanceData设置的变量的指针，指针不应为NULL
 * @return NULL 未设置InstanceData
 *
 */
void *TEE_GetInstanceData(void);

#endif
