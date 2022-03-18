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
 * @brief 扩展接口
 *
 * @since 1
 */
#ifndef TEE_HW_EXT_API_H
#define TEE_HW_EXT_API_H

#include "tee_defines.h"

/**
 * @brief 在TEE中获取设备唯一ID
 *
 * @param device_unique_id [IN]用于存储结果的缓冲区
 * @param length [IN/OUT]设备ID缓冲区长度
 *
 * @return TEE_SUCCESS 操作成功
 * @return others 操作失败
 *
 */
TEE_Result TEE_EXT_GetDeviceUniqueId(uint8_t *device_unique_id, uint32_t *length);

#endif
