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
 * @brief 扩展接口
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
 * @brief TA可以调用此API添加调用者信息，允许调用此TA。此API用于CA，以二进制可执行文件的形式
 *
 * @param ca_name[IN]CA调用方的进程名称
 * @param ca_uid[IN]CA调用方的uid
 *
 * @return TEE_SUCCESS 操作成功
 * @return others 无法为目标CA添加主叫方信息
 *
 */
TEE_Result AddCaller_CA_exec(const char *ca_name, uint32_t ca_uid);

/**
 * @brief 获取当前会话类型
 *
 * @return 当前会话的会话类型
 *
 */
uint32_t TEE_GetSessionType(void);

#ifdef __cplusplus
#if __cplusplus
}
#endif /* __cpluscplus */
#endif /* __cpluscplus */

#endif
