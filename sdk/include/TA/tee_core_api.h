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
 * @file tee_core_api.h
 *
 * @brief Provides APIs for managing trusted application (TA) sessions.
 *
 * @since 1
 */
#ifndef __TEE_CORE_API_H
#define __TEE_CORE_API_H

#include "tee_defines.h"
#ifndef _TEE_TA_SESSION_HANDLE
#define _TEE_TA_SESSION_HANDLE
typedef uint32_t TEE_TASessionHandle;
#endif

/**
 * @brief Raises a panic in the TA instance.
 *
 * @param panicCode Indicates an informative panic code defined by the TA.
 *
 */
void TEE_Panic(TEE_Result panicCode);

/**
 * @brief Opens a new session with a TA.
 *
 * @param destination Indicates the pointer to the <b>TEE_UUID</b> structure that contains
 * the Universal Unique Identifier (UUID) of the target TA.
 * @param cancellationRequestTimeout Indicates the timeout period in milliseconds or a special value
 * if there is no timeout.
 * @param paramTypes Indicates the types of all parameters passed in the operation.
 * @param params Indicates the parameters passed in the operation.
 * @param session Indicates the pointer to the variable that will receive the client session handle.
 * @param returnOrigin Indicates the pointer to the variable that holds the return origin.
 *
 * @return Returns <b>TEE_SUCCESS</b> if the session is opened.
 * @return Returns <b>TEE_ERROR_ITEM_NOT_FOUND</b> if the TA cannot be found in the Trusted Execution Environment (TEE).
 * @return Returns <b>TEE_ERROR_ACCESS_DENIED</b> if the access request to the TA is denied.
 *
 */
TEE_Result TEE_OpenTASession(const TEE_UUID *destination, uint32_t cancellationRequestTimeout, uint32_t paramTypes,
                             TEE_Param params[TEE_PARAMS_NUM], TEE_TASessionHandle *session, uint32_t *returnOrigin);

/**
 * @brief Closes a client session.
 *
 * @param session Indicates the handle of the session to close.
 *
 */
void TEE_CloseTASession(TEE_TASessionHandle session);

/**
 * @brief Invokes a command in a session opened between this client TA instance and a target TA instance.
 *
 * @param session Indicates the handle of the opened session.
 * @param cancellationRequestTimeout Indicates the timeout period in milliseconds or a special value
 * if there is no timeout.
 * @param commandID Indicates the identifier of the command to invoke.
 * @param paramTypes Indicates the types of all parameters passed in the operation.
 * @param params Indicates the parameters passed in the operation.
 * @param returnOrigin Indicates the pointer to the variable that holds the return origin.
 *
 * @return Returns <b>TEE_SUCCESS</b> if the operation is successful.
 * @return Returns <b>TEE_ERROR_ACCESS_DENIED</b> if the command fails to be invoked.
 *
 */
TEE_Result TEE_InvokeTACommand(TEE_TASessionHandle session, uint32_t cancellationRequestTimeout, uint32_t commandID,
                               uint32_t paramTypes, TEE_Param params[TEE_PARAMS_NUM], uint32_t *returnOrigin);

#endif
