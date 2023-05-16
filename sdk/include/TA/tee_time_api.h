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
 * @brief Provides APIs for managing the Trusted Execution Environment (TEE) time.
 *
 * You can use these APIs to implement time-related features in a TEE.
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
 * @brief Obtains the current TEE system time.
 *
 * @param time Indicates the pointer to the current system time obtained.
 *
 */
void TEE_GetSystemTime(TEE_Time *time);

/**
 * @brief Waits for the specified period of time, in milliseconds.
 *
 * @param timeout Indicates the period of time to wait, in milliseconds.
 *
 * @return Returns <b>TEE_SUCCESS</b> if the operation is successful.
 * @return Returns <b>TEE_ERROR_CANCEL</b> if the wait is canceled.
 * @return Returns <b>TEE_ERROR_OUT_OF_MEMORY</b> if the memory is not sufficient to complete the operation.
 *
 */
TEE_Result TEE_Wait(uint32_t timeout);

/**
 * @brief Obtains the persistent time of this trusted application (TA).
 *
 * @param time Indicates the pointer to the persistent time of the TA.
 *
 * @return Returns <b>TEE_SUCCESS</b> if the operation is successful.
 * @return Returns <b>TEE_ERROR_TIME_NOT_SET</b> if the persistent time has not been set.
 * @return Returns <b>TEE_ERROR_TIME_NEEDS_RESET</b> if the persistent time is corrupted and
 * the application is not longer trusted.
 * @return Returns <b>TEE_ERROR_OVERFLOW</b> if the number of seconds in the TA persistent time
 * exceeds the range of <b>uint32_t</b>.
 * @return Returns <b>TEE_ERROR_OUT_OF_MEMORY</b> if the memory is not sufficient to complete the operation.
 *
 */
TEE_Result TEE_GetTAPersistentTime(TEE_Time *time);

/**
 * @brief Sets the persistent time for this TA.
 *
 * @param time Indicates the pointer to the persistent time of the TA.
 *
 * @return Returns <b>TEE_SUCCESS</b> if the operation is successful.
 * @return Returns <b>TEE_ERROR_OUT_OF_MEMORY</b> if the memory is not sufficient to complete the operation.
 * @return Returns <b>TEE_ERROR_STORAGE_NO_SPACE</b> if the storage space is not sufficient to complete the operation.
 *
 */
TEE_Result TEE_SetTAPersistentTime(TEE_Time *time);

/**
 * @brief Obtains the current Rich Execution Environment (REE) system time.
 *
 * @param time Indicates the pointer to the REE system time obtained.
 *
 */
void TEE_GetREETime(TEE_Time *time);

#endif
