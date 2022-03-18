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
 * @brief �ӽ��ܽӿ�
 *
 * �����߿���ʹ����Щ�ӿ�ʵ�ּӽ��ܵ���ع��ܡ�
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
 * @brief �����ܺͽ�����������Ϊ����
 *
 * @param operation [IN/OUT]�����ľ��
 * @param crypto [IN]Ҫ���õ�engine
 *
 * @return TEE_SUCCESS ���ü�������ɹ�
 * @return TEE_ERROR_BAD_PARAMETERS ����ΪNULLi�������Ч
 *
 */
TEE_Result TEE_SetCryptoFlag(TEE_OperationHandle operation, uint32_t crypto);

/**
 * @brief ���üӽ�������Ϊobject
 *
 * @param operation [IN/OUT]����ľ��
 * @param crypto [IN]Ҫ���õ�engine
 *
 * @return TEE_SUCCESS ���ü�������ɹ�
 * @return TEE_ERROR_BAD_PARAMETERS ����ΪNULLi�������Ч
 *
 */
TEE_Result TEE_SetObjectFlag(TEE_ObjectHandle object, uint32_t crypto);
#endif
