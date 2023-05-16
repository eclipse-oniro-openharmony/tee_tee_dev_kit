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
 * @file tee_ext_api.h
 *
 * @brief Provides extended interfaces.
 *
 * @since 1
 */
#ifndef TEE_EXT_API_H
#define TEE_EXT_API_H

#include "tee_defines.h"
#include "tee_hw_ext_api.h"

#ifdef __cplusplus
#if __cplusplus
extern "C" {
#endif /* __cpluscplus */
#endif /* __cpluscplus */


/**
 * @brief Adds information about a caller that can invoke this TA.
 * This API applies to the client applications (CAs) in the binary executable file format.
 *
 * @param ca_name Indicates the process name of the caller of the CA.
 * @param ca_uid Indicates the UID of the caller.
 *
 * @return Returns <b>TEE_SUCCESS</b> if the operation is successful.
 * @return Returns other information otherwise.
 *
 */
TEE_Result AddCaller_CA_exec(const char *ca_name, uint32_t ca_uid);

/**
 * @brief Obtains the session type.
 *
 * @return Returns the session type obtained.
 *
 */
uint32_t TEE_GetSessionType(void);

#ifdef __cplusplus
#if __cplusplus
}
#endif /* __cpluscplus */
#endif /* __cpluscplus */

#endif
