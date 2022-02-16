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
 * @brief TA会话操作接口
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
 * @brief 在受信任的应用程序实例中引发死机\
 *
 * @param panicCode [IN]TA定义的信息性恐慌代码
 *
 */
void TEE_Panic(TEE_Result panicCode);

/**
 * @brief 使用受信任应用程序打开新会话
 *
 * @param destination [IN]指向包含目标受信任应用程序的UUID的TEE_UUID结构的指针
 * @param cancellationRequestTimeout [IN]以毫秒为单位的超时或特殊值
 * @param paramTypes [IN]操作中传递的所有参数的类型
 * @param params [IN]操作中传递的参数
 * @param session [OUT]指向将接收客户端会话句柄的变量的指针
 * @param returnOrigin [OUT]指向将包含返回原点的变量的指针
 *
 * @return TEE_SUCCESS 成功打开会话
 * @return TEE_ERROR_ITEM_NOT_FOUND 在TEE中找不到目标TA
 * @return TEE_ERROR_ACCESS_DENIED 对目标受信任应用程序的访问被拒绝
 *
 */
TEE_Result TEE_OpenTASession(const TEE_UUID *destination, uint32_t cancellationRequestTimeout, uint32_t paramTypes,
                             TEE_Param params[TEE_PARAMS_NUM], TEE_TASessionHandle *session, uint32_t *returnOrigin);

/**
 * @brief 关闭由TEE_OpenTASession打开的客户端会话
 *
 * @param session [IN]TEE_OpenTASession打开的会话句柄
 *
 */
void TEE_CloseTASession(TEE_TASessionHandle session);

/**
 * @brief 在客户端受信任应用程序实例和目标受信任应用程序实例之间打开的会话中调用命令
 *
 * @param session [IN]打开的会话句柄
 * @param cancellationRequestTimeout [IN]以毫秒为单位的超时或特殊值
 * @param commandID [IN]要调用的命令的标识符
 * @param paramTypes [IN]操作中传递的所有参数的类型
 * @param params [IN]操作中传递的参数
 * @param returnOrigin [IN]指向将包含返回原点的变量的指针
 *
 * @return TEE_SUCCESS 调用操作成功
 * @return TEE_ERROR_ACCESS_DENIED 向目标TA调用命令被拒绝
 *
 */
TEE_Result TEE_InvokeTACommand(TEE_TASessionHandle session, uint32_t cancellationRequestTimeout, uint32_t commandID,
                               uint32_t paramTypes, TEE_Param params[TEE_PARAMS_NUM], uint32_t *returnOrigin);

#endif
