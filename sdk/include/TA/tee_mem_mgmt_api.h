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
 * @brief Provides APIs for memory management.
 *
 *
 *
 * @since 1
 */
#ifndef TEE_MEM_MGMT_API_H
#define TEE_MEM_MGMT_API_H

#include "tee_defines.h"
#include "tee_mem_monitoring_api.h"

/*
 * The definitions below are defined by Global Platform or Platform SDK released previously
 * for compatibility.
 * Do not make any change to the content below.
 */
#ifndef ZERO_SIZE_PTR
#define ZERO_SIZE_PTR       ((void *)16)
#define zero_or_null_ptr(x) ((unsigned long)(x) <= (unsigned long)ZERO_SIZE_PTR)
#endif

enum MALLOC_HINT {
    ZERO           = 0,
    NOT_ZERO       = 1,
    ALIGN_004      = 0x80000002, /* Buffer alignment */
    ALIGN_008      = 0x80000003,
    ALIGN_016      = 0x80000004,
    ALIGN_032      = 0x80000005,
    ALIGN_064      = 0x80000006,
    ALIGN_128      = 0x80000007,
    ALIGN_256      = 0x80000008,
    ALIGN_004_ZERO = 0x80000012, /* The buffer is 4-byte aligned and initialized to zeros. */
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
 * @brief Fills <b>x</b> into the first <b>size</b> bytes of the buffer.
 *
 * @param buffer Indicates the pointer to the buffer.
 * @param x Indicates the value to fill.
 * @param size Indicates the number of bytes to fill.
 *
 */
#if defined(API_LEVEL) && (API_LEVEL >= API_LEVEL1_2)
void TEE_MemFill(void *buffer, uint8_t x, size_t size);
#else
void TEE_MemFill(void *buffer, uint32_t x, size_t size);
#endif

/**
 * @brief Copies bytes.
 *
 * @param dest Indicates the pointer to the buffer that holds the bytes copied.
 * @param src Indicates the pointer to the buffer that holds the bytes to copy.
 * @param size Indicates the number of bytes to copy.
 *
 */
void TEE_MemMove(void *dest, const void *src, size_t size);

/**
 * @brief Allocates space of the specified size for an object.
 *
 * @param size Indicates the size of the memory to be allocated.
 * @param hint Indicates a hint to the allocator. The value <b>0</b> indicates that the memory block
 * returned is filled with "\0".
 *
 * @return Returns a pointer to the newly allocated space if the operation is successful.
 * @return Returns a <b>NULL</b> pointer if the allocation fails.
 *
 */
void *TEE_Malloc(size_t size, uint32_t hint);

 /**
  * @brief Releases the memory allocated by <b>TEE_Malloc</b>.
  *
  * If the buffer is a <b>NULL</b> pointer, <b>TEE_Free</b> does nothing.
  * The buffer to be released must have been allocated by <b>TEE_Malloc</b> or <b>TEE_Realloc</b> and cannot be
  * released repeatedly. Otherwise, unexpected result may be caused.
  *
  * @param buffer Indicates the pointer to the memory to release.
  *
  */
void TEE_Free(void *buffer);

/**
 * @brief Reallocates memory.
 *
 * If <b>new_size</b> is greater than the old size, the content of the original memory does not change
 * and the space in excess of the old size contains unspecified content.
 * If the new size of the memory object requires movement of the object, the space for the previous
 * instantiation of the object is deallocated.
 * If the space cannot be allocated, the original object remains allocated and this function
 * returns a <b>NULL</b> pointer.
 * If the buffer is <b>NULL</b>, this function is equivalent to <b>TEE_Malloc</b>.
 *
 * @param buffer Indicates the pointer to the memory to reallocate.
 * @param new_size Indicates the new size required.
 *
 * @return Returns a pointer to the allocated memory if the operation is successful.
 * @return Returns a <b>NULL</b> pointer if the operation fails.
 *
 */
void *TEE_Realloc(void *buffer, size_t new_size);

/**
 * @brief Compares memory content from the beginning.
 *
 * @param buffer1 Indicates the pointer to the first buffer.
 * @param buffer2 Indicates the pointer to the second buffer.
 * @param size Indicates the number of the bytes to compare.
 *
 * @return Returns <b>–1</b> if buffer1 < buffer2.
 * @return Returns <b>0</b> if buffer1 == buffer2.
 * @return Returns <b>1</b> if buffer1 > buffer2.
 *
 */
int32_t TEE_MemCompare(const void *buffer1, const void *buffer2, size_t size);

/**
 * @brief Checks whether this TA has the requested permissions to access a buffer.
 *
 * @param accessFlags Indicates the access permissions to check.
 * @param buffer Indicates the pointer to the target buffer.
 * @param size Indicates the size of the buffer to check.
 *
 * @return Returns <b>TEE_SUCCESS</b> if the TA has the requested permissions.
 * @return Returns <b>TEE_ERROR_ACCESS_DENIED</b> otherwise.
 */
TEE_Result TEE_CheckMemoryAccessRights(uint32_t accessFlags, const void *buffer, size_t size);

/**
 * @brief Sets the TA instance data pointer.
 *
 * @param instanceData Indicates the pointer to the global TA instance data.
 *
 */
void TEE_SetInstanceData(void *instanceData);

/**
 * @brief Obtains the instance data pointer set by the TA using <b>TEE_SetInstanceData</b>.
 *
 * @return Returns the pointer to the instance data set by <b>TEE_SetInstanceData</b>
 * @return or <b>NULL</b> if no instance data pointer has been set.
 *
 */
void *TEE_GetInstanceData(void);

#endif
