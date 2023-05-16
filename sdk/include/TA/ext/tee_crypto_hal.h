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
 * @brief Provides APIs for cryptographic operations.
 *
 * You can use these APIs to implement encryption and decryption.
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
 * @brief Sets the encryption and decryption engines to an operation.
 *
 * @param operation Indicates the handle of the operation to set.
 * @param crypto Indicates the engines to set.
 *
 * @return Returns <b>TEE_SUCCESS</b> if the operation is successful.
 * @return Returns <b>TEE_ERROR_BAD_PARAMETERS</b> if <b>operation</b> is null or <b>crypto</b> is invalid.
 *
 */
TEE_Result TEE_SetCryptoFlag(TEE_OperationHandle operation, uint32_t crypto);

/**
 * @brief Sets the encryption and decryption engines to an object.
 *
 * @param object Indicates the handle of the object to set.
 * @param crypto Indicates the engines to set.
 *
 * @return Returns <b>TEE_SUCCESS</b> if the operation is successful.
 * @return Returns <b>TEE_ERROR_BAD_PARAMETERS</b> if <b>object</b> is null or <b>crypto</b> is invalid.
 *
 */
TEE_Result TEE_SetObjectFlag(TEE_ObjectHandle object, uint32_t crypto);
#endif
