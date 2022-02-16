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
 * @file tee_crypto_hal.h
 *
 * @brief 加解密接口
 *
 * 开发者可以使用这些接口实现加解密的相关功能。
 *
 * @since 1
 */
#ifndef TEE_CRYPTO_HAL_H
#define TEE_CRYPTO_HAL_H
#include "tee_crypto_api.h"
enum CRYPTO_ENGINE {
    SOFT_CRYPTO = 2,
    CRYPTO_ENGINE_MAX = 1024,
};

/**
 * @brief 将加密和解密引擎设置为运行
 *
 * @param operation [IN/OUT]操作的句柄
 * @param crypto [IN]要设置的engine
 *
 * @return TEE_SUCCESS 设置加密引擎成功
 * @return TEE_ERROR_BAD_PARAMETERS 操作为NULLi或加密无效
 *
 */
TEE_Result TEE_SetCryptoFlag(TEE_OperationHandle operation, uint32_t crypto);

/**
 * @brief 设置加解密引擎为object
 *
 * @param operation [IN/OUT]对象的句柄
 * @param crypto [IN]要设置的engine
 *
 * @return TEE_SUCCESS 设置加密引擎成功
 * @return TEE_ERROR_BAD_PARAMETERS 操作为NULLi或加密无效
 *
 */
TEE_Result TEE_SetObjectFlag(TEE_ObjectHandle object, uint32_t crypto);
#endif
