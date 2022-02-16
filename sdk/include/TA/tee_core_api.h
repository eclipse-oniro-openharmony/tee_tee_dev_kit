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
 * @brief TA�Ự�����ӿ�
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
 * @brief �������ε�Ӧ�ó���ʵ������������\
 *
 * @param panicCode [IN]TA�������Ϣ�ԿֻŴ���
 *
 */
void TEE_Panic(TEE_Result panicCode);

/**
 * @brief ʹ��������Ӧ�ó�����»Ự
 *
 * @param destination [IN]ָ�����Ŀ��������Ӧ�ó����UUID��TEE_UUID�ṹ��ָ��
 * @param cancellationRequestTimeout [IN]�Ժ���Ϊ��λ�ĳ�ʱ������ֵ
 * @param paramTypes [IN]�����д��ݵ����в���������
 * @param params [IN]�����д��ݵĲ���
 * @param session [OUT]ָ�򽫽��տͻ��˻Ự����ı�����ָ��
 * @param returnOrigin [OUT]ָ�򽫰�������ԭ��ı�����ָ��
 *
 * @return TEE_SUCCESS �ɹ��򿪻Ự
 * @return TEE_ERROR_ITEM_NOT_FOUND ��TEE���Ҳ���Ŀ��TA
 * @return TEE_ERROR_ACCESS_DENIED ��Ŀ��������Ӧ�ó���ķ��ʱ��ܾ�
 *
 */
TEE_Result TEE_OpenTASession(const TEE_UUID *destination, uint32_t cancellationRequestTimeout, uint32_t paramTypes,
                             TEE_Param params[TEE_PARAMS_NUM], TEE_TASessionHandle *session, uint32_t *returnOrigin);

/**
 * @brief �ر���TEE_OpenTASession�򿪵Ŀͻ��˻Ự
 *
 * @param session [IN]TEE_OpenTASession�򿪵ĻỰ���
 *
 */
void TEE_CloseTASession(TEE_TASessionHandle session);

/**
 * @brief �ڿͻ���������Ӧ�ó���ʵ����Ŀ��������Ӧ�ó���ʵ��֮��򿪵ĻỰ�е�������
 *
 * @param session [IN]�򿪵ĻỰ���
 * @param cancellationRequestTimeout [IN]�Ժ���Ϊ��λ�ĳ�ʱ������ֵ
 * @param commandID [IN]Ҫ���õ�����ı�ʶ��
 * @param paramTypes [IN]�����д��ݵ����в���������
 * @param params [IN]�����д��ݵĲ���
 * @param returnOrigin [IN]ָ�򽫰�������ԭ��ı�����ָ��
 *
 * @return TEE_SUCCESS ���ò����ɹ�
 * @return TEE_ERROR_ACCESS_DENIED ��Ŀ��TA��������ܾ�
 *
 */
TEE_Result TEE_InvokeTACommand(TEE_TASessionHandle session, uint32_t cancellationRequestTimeout, uint32_t commandID,
                               uint32_t paramTypes, TEE_Param params[TEE_PARAMS_NUM], uint32_t *returnOrigin);

#endif
