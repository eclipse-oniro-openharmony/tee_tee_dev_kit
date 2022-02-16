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
 * @file tee_time_api.h
 *
 * @brief 安全时间接口
 *
 * 开发者可以使用这些接口实现安全时间相关的功能。
 *
 * @since 1
 */
#ifndef __TEE_TIME_API_H
#define __TEE_TIME_API_H

#include "tee_defines.h"

/*
 * Get current TEE system rtc time
 *
 * @param time [OUT] current system rtc time
 * @return void
 */

/**
 * @brief 获取当前TEE系统时间
 *
 * @param time [OUT]当前系统时间
 *
 */
void TEE_GetSystemTime(TEE_Time *time);

/**
 * @brief 等待指定的毫秒数
 *
 * @param timeout [IN]指定的毫秒数
 *
 * @return TEE_SUCCESS 成功
 * @return TEE_ERROR_CANCEL 等待已取消
 * @return TEE_ERROR_OUT_OF_MEMORY 没有足够的内存来完成操作
 *
 */
TEE_Result TEE_Wait(uint32_t timeout);

/**
 * @brief 检索受信任应用程序的持久时间
 *
 * @param time [IN]受信任应用程序的持久时间
 *
 * @return TEE_SUCCESS 成功
 * @return TEE_ERROR_TIME_NOT_SET 持久时间尚未设置
 * @return TEE_ERROR_TIME_NEEDS_RESET 永久时间已设置，但可能已损坏，不得再信任
 * @return TEE_ERROR_OVERFLOW TA持续时间中的秒数超过了uint32_t的范围
 * @return TEE_ERROR_OUT_OF_MEMORY 没有足够的内存来完成操作
 *
 */
TEE_Result TEE_GetTAPersistentTime(TEE_Time *time);

/**
 * @brief 设置当前受信任应用程序的持久化时间
 *
 * @param time [IN]受信任应用程序的持久时间
 *
 * @return TEE_SUCCESS 成功
 * @return TEE_ERROR_OUT_OF_MEMORY 没有足够的内存来完成操作
 * @return TEE_ERROR_STORAGE_NO_SPACE 没有足够的存储空间来完成操作
 *
 */
TEE_Result TEE_SetTAPersistentTime(TEE_Time *time);

/**
 * @brief 获取当前REE系统时间
 *
 * @param time [OUT]当前REE系统时间
 *
 */
void TEE_GetREETime(TEE_Time *time);

#endif
