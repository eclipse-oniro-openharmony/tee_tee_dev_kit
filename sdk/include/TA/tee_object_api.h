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
 * @file tee_object_api.h
 *
 * @brief ��ȫ�洢�ӿ�
 *
 * �����߿���ʹ����Щ�ӿ�ʵ�ְ�ȫ�洢����ع��ܡ�
 *
 * @since 1
 */
#ifndef __TEE_OBJECT_API_H
#define __TEE_OBJECT_API_H

#include "tee_defines.h"

/**
 * @brief HANDLE_NULL�Ķ��壬��Ч�Ķ�����
 */
#define TEE_HANDLE_NULL 0x00000000

/**
 * @brief TEE_ObjectHandle����Կʹ�÷�ʽ�������˶�����Կ��ʹ�����
 */
enum Usage_Constants {
    /** ������ȡ�������Կ */
    TEE_USAGE_EXTRACTABLE = 0x00000001,
    /** �������Կ�������ڼ��� */
    TEE_USAGE_ENCRYPT     = 0x00000002,
    /** �������Կ�������ڽ��� */
    TEE_USAGE_DECRYPT     = 0x00000004,
    /** �������Կ�������ڹ�ϣ���� */
    TEE_USAGE_MAC         = 0x00000008,
    /** �������Կ��������ǩ�� */
    TEE_USAGE_SIGN        = 0x00000010,
    /** �������Կ����������ǩ */
    TEE_USAGE_VERIFY      = 0x00000020,
    /** �������Կ���������� */
    TEE_USAGE_DERIVE      = 0x00000040,
    /** �����ʼ����Ĭ�Ϸ�������Ȩ�� */
    TEE_USAGE_DEFAULT     = 0xFFFFFFFF,
};

/**
 * @brief TEE_ObjectHandle�ľ����־ָʾ�����һЩ��Ϣ���Ƿ�Ϊ���ö����Ƿ��ѳ�ʼ���ȡ�
 */
enum Handle_Flag_Constants {
    /** �־û����� */
    TEE_HANDLE_FLAG_PERSISTENT      = 0x00010000,
    /** �����ѳ�ʼ�� */
    TEE_HANDLE_FLAG_INITIALIZED     = 0x00020000,
    /** δʹ�� */
    TEE_HANDLE_FLAG_KEY_SET         = 0x00040000,
    /** δʹ�� */
    TEE_HANDLE_FLAG_EXPECT_TWO_KEYS = 0x00080000,
};

/**
 * @brief ���Ա�ʶ����־�б�
 */
#define TEE_ATTR_FLAG_VALUE  0x20000000
#define TEE_ATTR_FLAG_PUBLIC 0x10000000

#define TEE_ATTR_IS_BUFFER(attribute_id) ((((attribute_id) << 2) >> 31) == 0)
#define TEE_ATTR_IS_VALUE(attribute_id)  ((((attribute_id) << 2) >> 31) == 1)

#define TEE_ATTR_IS_PROTECTED(attribute_id) ((((attribute_id) << 3) >> 31) == 0)
#define TEE_ATTR_IS_PUBLIC(attribute_id)    ((((attribute_id) << 3) >> 31) == 1)

/**
 * @brief ��TEE_ObjectHandleָ��Ķ����TEE_Attribute�ṹ�л�ȡ���ϵĻ���������
 *
 * TEE_Attribute�ṹ�е����ϳ�Ա��Ҫ��ref�����TEE_Attribute��˽�еģ�������ʹ�ó����������TEE_USAGE_EXTRACTABLE
 *
 * @param object [IN]ԴTEE_ObjectHandle
 * @param attributeID [IN]Ҫ��ȡ������ID����TEE_ObjectAttribute��Ҳ�����Զ���
 * @param buffer [OUT]ָ�룬ָ��Ļ��������ڴ洢��ȡ�Ļ�����������
 * @param size [IN/OUT]ָ�룬�洢�����ֽڳ���
 *
 * @return TEE_SUCCESS ָʾ�����ѳɹ�ִ��
 * @return TEE_ERROR_ITEM_NOT_FOUND �ڶ������Ҳ���Ҫ���ҵ�TEE_Attribute�����߶���δ��ʼ��
 * @return TEE_ERROR_SHORT_BUFFER�ṩ�Ļ�����̫С���޷��洢��ȡ������
 *
 */
TEE_Result TEE_GetObjectBufferAttribute(TEE_ObjectHandle object, uint32_t attributeID, void *buffer, size_t *size);

/**
 * @brief �ڶ����е�TEE_Attribute�л�ȡ���ϵ�ֵ
 *
 * TEE_Attribute�ṹ�����ϵĳ�Ա����Ϊvalue�����TEE_Attribute��˽�еģ�������Usage_Constants��Ҫ����TEE_USAGE_EXTRACTABLE
 *
 * @param object [IN]ԴTEE_ObjectHandle
 * @param attributeID [IN]��Ҫ��ȡ������ID����TEE_ObjectAttribute��Ҳ�����Զ���
 * @param a [OUT]ָ�룬ָ��Ŀռ����ڴ洢
 * @param b [OUT]ָ�룬ָ��Ŀռ����ڴ洢b
 *
 * @return TEE_SUCCESS ָʾ�����ѳɹ�ִ��
 * @return TEE_ERROR_ITEM_NOT_FOUND �ڶ������Ҳ���Ҫ���ҵ�TEE_Attribute�����߶���δ��ʼ��
 * @return TEE_ERROR_ACCESS_DENIED ���Ի�ȡ˽��TEE_Attribute����δ����TEE_USAGE_EXTRACTABLE
 *
 */
TEE_Result TEE_GetObjectValueAttribute(TEE_ObjectHandle object, uint32_t attributeID, uint32_t *a, uint32_t *b);

/**
 * @brief �رմ򿪵�TEE_ObjectHandle����
 *
 * ��������ǳ־ö���Ҳ��������ʱ����
 *
 * @param object [IN]���رյ�TEE_ObjectHandle����
 *
 */
void TEE_CloseObject(TEE_ObjectHandle object);

/**
 * @brief ����һ��δ��ʼ���Ķ������洢��
 *
 * objectType��maxObjectSize��Ҫָ����Ԥ����
 *
 * @param objectType [IN]��������������ͣ�ȡֵΪTEE_ObjectType
 * @param maxObjectSize [IN]���������ֽ���
 * @param object [OUT]ָ���´�����������ָ��
 *
 * @return TEE_SUCCESS ָʾ�����ѳɹ�ִ��
 * @return TEE_ERROR_OUT_OF_MEMORY �ڴ治�㣬�޷�����
 * @return TEE_ERROR_NOT_SUPPORTED ��֧�ֶ����ṩ���ֽ�
 *
 */
TEE_Result TEE_AllocateTransientObject(uint32_t objectType, uint32_t maxObjectSize, TEE_ObjectHandle *object);

/**
 * @brief �ͷ��ѷ������ʱ����
 *
 * �������ú󣬾��ʧЧ�����з���Ķ����ͷš���TEE_AllocateTransientObject���
 *
 * @param object[IN]��Ҫ�ͷŵ�TEE_ObjectHandle
 *
 */
void TEE_FreeTransientObject(TEE_ObjectHandle object);

/**
 * @brief ��˲̬��������Ϊ��ʼ״̬����������״̬
 *
 * ���������ѷ��䵫δ�洢��Կ��δ��ʼ���������洢��Կ
 *
 * @param object [IN]��Ҫ���õ�TEE_ObjectHandle
 *
 */
void TEE_ResetTransientObject(TEE_ObjectHandle object);

/**
 * @brief ������attrs�е����Է����δ��ʼ����˲̬����
 *
 * ȷ��������δ��ʼ��\n
 * ����attrs�ɿ���Ӧ�ó����ṩ
 *
 * @param object [IN/OUT]TEE_ObjectHandle�Ѵ�����δ��ʼ��
 * @param attrs [IN]�����������飬������һ������TEE_Attribute
 * @param attrCount [IN]�����Ա��
 *
 * @return TEE_SUCCESS ָʾ�����ѳɹ�ִ��
 * @return TEE_ERROR_BAD_PARAMETERS ���Բ���ȷ��һ��
 *
 */
TEE_Result TEE_PopulateTransientObject(TEE_ObjectHandle object, TEE_Attribute *attrs, uint32_t attrCount);

/**
 * @brief ��ʼ������������TEE_Attribute
 *
 * TEE_Attribute�ṹ�е����ϳ�Ա��Ҫ��ref
 *
 * @param attr [OUT]Ҫ��ʼ����TEE_Attribute
 * @param attributeID [IN]�����TEE_Attribute��ID
 * @param buffer [IN]�������洢Ҫ���������
 * @param length [IN]��ֵ���ݵ��ֽڳ���
 *
 */
void TEE_InitRefAttribute(TEE_Attribute *attr, uint32_t attributeID, void *buffer, size_t length);

/**
 * @brief ��ʼ��TEE_Attribute
 *
 * @param attr [OUT]Ҫ��ʼ����TEE_Attribute
 * @param attributeID [IN]�����TEE_Attribute��ID
 * @param a [IN]��ֵ��ֵ��TEE_Attribute�е����ϵĳ�Աֵa
 * @param b [IN]��ֵ��ֵ��TEE_Attribute�е����ϵĳ�Աֵb
 *
 */
void TEE_InitValueAttribute(TEE_Attribute *attr, uint32_t attributeID, uint32_t a, uint32_t b);

/**
 * @brief �˺������������Կ����Կ�ԣ�������������ʱ����
 *
 * @param object [IN]˲̬�������ڴ洢���ɵ���Կ
 * @param keySize [IN]������Կ���ֽ���
 * @param params [IN]��Կ���ɲ���˵��
 * @param paramCount [IN]������Կ����Ĳ�����
 *
 * @return TEE_SUCCESS ָʾ�����ѳɹ�ִ��
 * @return TEE_ERROR_BAD_PARAMETERS ���ɵ���Կ����ʱ������Դ洢����Կ���Ͳ�һ��
 *
 */
TEE_Result TEE_GenerateKey(TEE_ObjectHandle object, uint32_t keySize, TEE_Attribute *params, uint32_t paramCount);

/**
 * @brief ��ȡ�����TEE_ObjectInfo
 *
 * ��ȡ�����TEE_ObjectInfo�������临�Ƶ�����objectInfoָ��Ŀռ��У��ÿռ����û�Ԥ����
 *
 * @param object [IN]ԴTEE_ObjectHandle
 * @param objectInfo [OUT]���ڴ洢TEE_ObjectInfo�Ľṹ��ָ��
 *
 * @return TEE_SUCCESS ָʾ�����ѳɹ�ִ��
 * @return TEE_ERROR_CORRUPT_OBJECT �ļ��𻵣��ļ���������ر�
 * @return TEE_ERROR_STORAGE_NOT_AVAILABLE �޷������ļ����ڵĴ洢����
 *
 */
TEE_Result TEE_GetObjectInfo1(TEE_ObjectHandle object, TEE_ObjectInfo *objectInfo);

/**
 * @brief ʹ�ó�ʼ������TEE_Attribute��ֵ��δ��ʼ���Ķ���
 *
 * �ú���ʹ�ó�ʼ������TEE_Attribute��ֵ��δ��ʼ���Ķ����൱�ڽ�srcobject��TEE_Attribute���Ƶ�destobject��\n
 * ���������TEE_Attribute���ͺͱ�ű���ƥ��
 *
 * @param destObject [IN]Ҫ�����δ��ʼ����TEE_ObjectHandle
 * @param srcObject [IN]��ʼ����TEE_ObjectHandle���ڸ���һ������ֵ
 *
 * @return TEE_SUCCESS ָʾ�����ѳɹ�ִ��
 * @return TEE_ERROR_CORRUPT_OBJECT �ļ��𻵣��ļ���������ر�
 * @return TEE_ERROR_STORAGE_NOT_AVAILABLE �޷������ļ����ڵĴ洢����
 *
 */
TEE_Result TEE_CopyObjectAttributes1(TEE_ObjectHandle destObject, TEE_ObjectHandle srcObject);

/**
 * @brief ���ƶ����objectUseλ
 *
 * ��λ������������Կ��ʹ�������ȡֵ��ΧΪ��ʹ����_�����������ڲ���objectUse�ı�־λ��\n
 * �����λ����Ϊ1��������ʹ�ñ�־����ı�\n
 * ���ò�������Ϊ0ʱ������ö����Ӧ�Ķ���ʹ�ñ�־��\n
 * �´����Ķ��󽫰������е�ʹ����_����������ʹ������־ֻ���������������
 *
 * @param object [IN]��Ҫ���Ƶ�TEE_ObjectHandle
 * @param objectUsage [IN]�û�ϣ�����ĵ�objectUsage
 *
 * @return TEE_SUCCESS ָʾ�����ѳɹ�ִ��
 * @return TEE_ERROR_CORRUPT_OBJECT �ļ��𻵣��ļ���������ر�
 * @return TEE_ERROR_STORAGE_NOT_AVAILABLE �޷������ļ����ڵĴ洢����
 *
 */
TEE_Result TEE_RestrictObjectUsage1(TEE_ObjectHandle object, uint32_t objectUsage);
#endif
