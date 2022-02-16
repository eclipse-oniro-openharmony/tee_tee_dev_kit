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
 * @file tee_time_api.h
 *
 * @brief ��ȫʱ��ӿ�
 *
 * �����߿���ʹ����Щ�ӿ�ʵ�ְ�ȫʱ����صĹ��ܡ�
 *
 * @since 1
 */
#ifndef __TEE_TIME_API_H
#define __TEE_TIME_API_H

#include "tee_defines.h"

/*
 * Get current TEE system rtc time
 *
 * @param time [OUT] current system rtc time
 * @return void
 */

/**
 * @brief ��ȡ��ǰTEEϵͳʱ��
 *
 * @param time [OUT]��ǰϵͳʱ��
 *
 */
void TEE_GetSystemTime(TEE_Time *time);

/**
 * @brief �ȴ�ָ���ĺ�����
 *
 * @param timeout [IN]ָ���ĺ�����
 *
 * @return TEE_SUCCESS �ɹ�
 * @return TEE_ERROR_CANCEL �ȴ���ȡ��
 * @return TEE_ERROR_OUT_OF_MEMORY û���㹻���ڴ�����ɲ���
 *
 */
TEE_Result TEE_Wait(uint32_t timeout);

/**
 * @brief ����������Ӧ�ó���ĳ־�ʱ��
 *
 * @param time [IN]������Ӧ�ó���ĳ־�ʱ��
 *
 * @return TEE_SUCCESS �ɹ�
 * @return TEE_ERROR_TIME_NOT_SET �־�ʱ����δ����
 * @return TEE_ERROR_TIME_NEEDS_RESET ����ʱ�������ã����������𻵣�����������
 * @return TEE_ERROR_OVERFLOW TA����ʱ���е�����������uint32_t�ķ�Χ
 * @return TEE_ERROR_OUT_OF_MEMORY û���㹻���ڴ�����ɲ���
 *
 */
TEE_Result TEE_GetTAPersistentTime(TEE_Time *time);

/**
 * @brief ���õ�ǰ������Ӧ�ó���ĳ־û�ʱ��
 *
 * @param time [IN]������Ӧ�ó���ĳ־�ʱ��
 *
 * @return TEE_SUCCESS �ɹ�
 * @return TEE_ERROR_OUT_OF_MEMORY û���㹻���ڴ�����ɲ���
 * @return TEE_ERROR_STORAGE_NO_SPACE û���㹻�Ĵ洢�ռ�����ɲ���
 *
 */
TEE_Result TEE_SetTAPersistentTime(TEE_Time *time);

/**
 * @brief ��ȡ��ǰREEϵͳʱ��
 *
 * @param time [OUT]��ǰREEϵͳʱ��
 *
 */
void TEE_GetREETime(TEE_Time *time);

#endif
