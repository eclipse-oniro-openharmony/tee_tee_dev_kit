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
 * @file tee_hw_ext_api.h
 *
 * @brief Provides extended interfaces.
 *
 * @since 1
 */
#ifndef TEE_HW_EXT_API_H
#define TEE_HW_EXT_API_H

#include "tee_defines.h"

/**
 * @brief Obtains the unique device ID from the TEE.
 *
 * @param device_unique_id Indicates the pointer to the buffer for storing the device ID.
 * @param length Indicates the pointer to the buffer length.
 *
 * @return Returns <b>TEE_SUCCESS</b> if the operation is successful.
 * @return Returns other information otherwise.
 *
 */
TEE_Result TEE_EXT_GetDeviceUniqueId(uint8_t *device_unique_id, uint32_t *length);

#endif
