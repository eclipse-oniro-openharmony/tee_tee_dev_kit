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

#ifndef TEE_CLIENT_API_H
#define TEE_CLIENT_API_H
/**
 * @addtogroup TeeClient
 * @{
 *
 * @brief TEEC_API �ͻ���(�ǰ�ȫ��)�ӿڡ�
 *
 * �ṩ�ǰ�ȫ��(����ģʽ)�¿ͻ��˳�����ʰ�ȫģʽ�°�ȫӦ����ؽӿڡ�
 *
 * @since 8
 */

/**
 * @file tee_client_api.h
 *
 * @brief �ͻ���Ӧ�÷��ʰ�ȫӦ����ؽӿڶ��塣
 *
 * <p>ʹ��ʾ����
 * <p> 1.��TEE����������TEEC_InitializeContext��ʼ��TEE������
 * <p> 2.�򿪻Ự������TEEC_OpenSession������Ϊ��ȫӦ��TA��UUID��
 * <p> 3.�����������TEEC_InvokeCommand��ȫӦ�÷������
 * <p> 4.�رջỰ�����ýӿ�TEEC_CloseSession���رջỰ��
 * <p> 5.�ر�TEE���������ýӿ�TEEC_FinalizeContext���ر�TEE������
 *
 * @since 8
 */

#include <string.h>
#include "tee_client_type.h"

#ifdef __cplusplus
extern "C" {
#endif

/**
 * @brief ���ڼ���ǰ�ȫ�����밲ȫ���紫�ݲ�������ֵ
 *
 * @since 8
 */
#define TEEC_PARAM_TYPES(param0Type, param1Type, param2Type, param3Type) \
    ((param3Type) << 12 | (param2Type) << 8 | (param1Type) << 4 | (param0Type))

/**
 * @brief ���ڼ���paramTypes���ֶ�index����ֵ
 *
 * @since 8
 */
#define TEEC_PARAM_TYPE_GET(paramTypes, index) \
    (((paramTypes) >> (4*(index))) & 0x0F)

/**
 * @brief ��ʼ��TEE����
 *
 * @par ����:
 * ��ʼ��·��Ϊname��TEE����������name����Ϊ�գ�
 * ��ʼ��TEE�����Ǵ򿪻Ự����������Ļ�����
 * ��ʼ���ɹ��󣬿ͻ���Ӧ����TEE����һ�����ӡ�
 *
 * @param name [IN] TEE����·��
 * @param context [IN/OUT] contextָ�룬��ȫ���绷�����
 *
 * @return #TEEC_SUCCESS ��ʼ��TEE�����ɹ�
 *         #TEEC_ERROR_BAD_PARAMETERS ��������ȷ��name����ȷ��contextΪ��
 *         #TEEC_ERROR_GENERIC ϵͳ������Դ�����ԭ��
 * @since 8
 */
TEEC_Result TEEC_InitializeContext(const char *name, TEEC_Context *context);

/**
 * @brief �ر�TEE����
 *
 * @par ����:
 * �ر�contextָ���TEE�������Ͽ��ͻ���Ӧ����TEE����������
 *
 * @param context [IN/OUT] ָ���ѳ�ʼ���ɹ���TEE����
 *
 * @return ��
 * @since 8
 */
void TEEC_FinalizeContext(TEEC_Context *context);

/**
 * @brief �򿪻Ự
 *
 * @par ����:
 * ��ָ����TEE����context�£�Ϊ�ͻ���Ӧ����UUIDΪdestination�İ�ȫӦ�ý���һ�����ӣ�
 * ���ӷ�ʽ��connectionMethod������������connectionData�����ݵ����ݰ�����opetation�
 * �򿪻Ự�ɹ����������session�ǶԸ����ӵ�һ��������
 * ����򿪻Ựʧ�ܣ��������returnOriginΪ������Դ��
 *
 * @param context [IN/OUT] ָ���ѳ�ʼ���ɹ���TEE����
 * @param session [OUT] ָ��Ự��ȡֵ����Ϊ��
 * @param destination [IN] ��ȫӦ�õ�UUID��һ����ȫӦ��ӵ��Ψһ��UUID
 * @param connectionMethod [IN] ���ӷ�ʽ��ȡֵ��ΧΪ#TEEC_LoginMethod
 * @param connectionData [IN] �����ӷ�ʽ���Ӧ���������ݣ�
 * ������ӷ�ʽΪ#TEEC_LOGIN_PUBLIC��#TEEC_LOGIN_USER��
 * #TEEC_LOGIN_USER_APPLICATION��#TEEC_LOGIN_GROUP_APPLICATION����������ȡֵ����Ϊ�գ�
 * ������ӷ�ʽΪ#TEEC_LOGIN_GROUP��#TEEC_LOGIN_GROUP_APPLICATION��
 * �������ݱ���ָ������Ϊuint32_t�����ݣ������ݱ�ʾ�ͻ���Ӧ���������ӵ����û�
 * @param operation [IN/OUT] �ͻ���Ӧ���밲ȫӦ�ô��ݵ�����
 * @param returnOrigin [IN/OUT] ������Դ��ȡֵ��ΧΪ#TEEC_ReturnCodeOrigin
 *
 * @return #TEEC_SUCCESS �򿪻Ự�ɹ�
 *         #TEEC_ERROR_BAD_PARAMETERS ��������ȷ������contextΪ�ջ�sessionΪ�ջ�destinationΪ��
 *         #TEEC_ERROR_ACCESS_DENIED ϵͳ����Ȩ�޷���ʧ��
 *         #TEEC_ERROR_OUT_OF_MEMORY ϵͳ������Դ����
 *         #TEEC_ERROR_TRUSTED_APP_LOAD_ERROR ���ذ�ȫӦ��ʧ��
 *         ��������ֵ�ο� #TEEC_ReturnCode
 * @since 8
 */
TEEC_Result TEEC_OpenSession(TEEC_Context *context, TEEC_Session *session, const TEEC_UUID *destination,
    uint32_t connectionMethod, const void *connectionData, TEEC_Operation *operation, uint32_t *returnOrigin);

/**
 * @brief �رջỰ
 *
 * @par ����:
 * �ر�sessionָ��ĻỰ���Ͽ��ͻ���Ӧ���밲ȫӦ�õ�����
 *
 * @param session [IN/OUT] ָ���ѳɹ��򿪵ĻỰ
 *
 * @return ��
 * @since 8
 */
void TEEC_CloseSession(TEEC_Session *session);

/**
 * @brief ��������
 *
 * @par ����:
 * ��ָ���ĻỰsession��ɿͻ���Ӧ����ȫӦ�÷�������commandID��
 * ���͵�����Ϊoperation�������������ʧ�ܣ��������returnOriginΪ������Դ
 *
 * @param session [IN/OUT] ָ���Ѵ򿪳ɹ��ĻỰ
 * @param commandID [IN] ��ȫӦ��֧�ֵ�����ID���ɰ�ȫӦ�ö���
 * @param operation [IN/OUT] �����˿ͻ���Ӧ����ȫӦ�÷��͵���������
 * @param returnOrigin [IN/OUT] ������Դ��ȡֵ��ΧΪ#TEEC_ReturnCodeOrigin
 *
 * @return #TEEC_SUCCESS ��������ɹ�
 *         #TEEC_ERROR_BAD_PARAMETERS ��������ȷ������sessionΪ�ջ����operation��ʽ����ȷ
 *         #TEEC_ERROR_ACCESS_DENIED ϵͳ����Ȩ�޷���ʧ��
 *         #TEEC_ERROR_OUT_OF_MEMORY ϵͳ������Դ����
 *         ��������ֵ�ο� #TEEC_ReturnCode
 * @since 8
 */
TEEC_Result TEEC_InvokeCommand(TEEC_Session *session, uint32_t commandID,
    TEEC_Operation *operation, uint32_t *returnOrigin);

/**
 * @brief ע�Ṳ���ڴ�
 *
 * @par ����:
 * ��ָ����TEE����context��ע�Ṳ���ڴ�sharedMem��
 * ͨ��ע��ķ�ʽ��ȡ�����ڴ���ʵ���㿽������Ҫ����ϵͳ��֧�֣�
 * Ŀǰ��ʵ���У��÷�ʽ����ʵ���㿽��
 *
 * @param context [IN/OUT] �ѳ�ʼ���ɹ���TEE����
 * @param sharedMem [IN/OUT] �����ڴ�ָ�룬�����ڴ���ָ����ڴ治��Ϊ�ա���С����Ϊ��
 *
 * @return #TEEC_SUCCESS ��������ɹ�
 *         #TEEC_ERROR_BAD_PARAMETERS ��������ȷ������contextΪ�ջ�sharedMemΪ�գ������ڴ���ָ����ڴ�Ϊ��
 * @since 8
 */
TEEC_Result TEEC_RegisterSharedMemory(TEEC_Context *context, TEEC_SharedMemory *sharedMem);

/**
 * @brief ���빲���ڴ�
 *
 * @par ����:
 * ��ָ����TEE����context�����빲���ڴ�sharedMem��
 * ͨ�������ڴ����ʵ�ַǰ�ȫ�����밲ȫ���紫������ʱ���㿽������Ҫ����ϵͳ��֧�֣�
 * Ŀǰ��ʵ���У��÷�ʽ����ʵ���㿽��
 *
 * @attention ������sharedMem��size������Ϊ0�������᷵�سɹ������޷�ʹ�����
 * �����ڴ棬��Ϊ����ڴ��û�е�ַҲû�д�С
 * @param context [IN/OUT] �ѳ�ʼ���ɹ���TEE����
 * @param sharedMem [IN/OUT] �����ڴ�ָ�룬�����ڴ�Ĵ�С����Ϊ��
 *
 * @return #TEEC_SUCCESS ��������ɹ�
 *         #TEEC_ERROR_BAD_PARAMETERS ��������ȷ������contextΪ�ջ�sharedMemΪ��
 *         #TEEC_ERROR_OUT_OF_MEMORY ϵͳ������Դ���㣬����ʧ��
 * @since 8
 */
TEEC_Result TEEC_AllocateSharedMemory(TEEC_Context *context, TEEC_SharedMemory *sharedMem);

/**
 * @brief �ͷŹ����ڴ�
 *
 * @par ����:
 * �ͷ���ע��ɹ��ĵĻ�������ɹ��Ĺ����ڴ�sharedMem
 *
 * @attention �����ͨ��#TEEC_AllocateSharedMemory��ʽ��ȡ�Ĺ����ڴ棬
 * �ͷ�ʱ���������ڴ棻�����ͨ��#TEEC_RegisterSharedMemory��ʽ
 * ��ȡ�Ĺ����ڴ棬�ͷ�ʱ������չ����ڴ���ָ��ı����ڴ�
 * @param sharedMem [IN/OUT] ָ����ע��ɹ�������ɹ��Ĺ����ڴ�
 *
 * @return ��
 * @since 8
 */
void TEEC_ReleaseSharedMemory(TEEC_SharedMemory *sharedMem);

/**
 * @brief cancel API
 *
 * @par ����:
 * ȡ����һ���������е�open Session������һ��invoke command
 * ����һ��cancel��signal����������
 *
 * @attention �˲��������Ƿ���һ��cancel����Ϣ���Ƿ����cancel������TEE��TA������ĿǰΪ��ʵ��
 * @param operation [IN/OUT] �����˿ͻ���Ӧ����ȫӦ�÷��͵���������
 *
 * @return ��
 * @since 8
 */
void TEEC_RequestCancellation(TEEC_Operation *operation);

#ifdef __cplusplus
}
#endif
/** @} */
#endif
