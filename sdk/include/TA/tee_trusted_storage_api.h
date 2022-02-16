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
 * @file tee_trusted_storage_api.h
 *
 * @brief ��ȫ�洢�ӿ�
 *
 * �����߿��Ե�����Щ�ӿ�ʵ�ְ�ȫ�洢��صĹ���
 *
 * @since 1
 */

#ifndef __TEE_TRUSTED_STORAGE_API_H
#define __TEE_TRUSTED_STORAGE_API_H

#include "tee_defines.h"
#include "tee_object_api.h"

/**
 * @brief ��������λ��ʼλ��ѡ�����TEE_SeekObjectData����
 */
enum __TEE_Whence {
    /** ����ʼλ�ö�λΪ����������ʼλ�� */
    TEE_DATA_SEEK_SET = 0, /* Position the starting position as the beginning of the data stream */
    /** ����ʼλ�ö�λΪ��ǰ������λ�� */
    TEE_DATA_SEEK_CUR,     /* Position the starting position as the current data stream position */
    /** ����ʼλ�ö�λ����������ĩβ */
    TEE_DATA_SEEK_END      /* Position the starting position at the end of the data stream */
};

struct __TEE_ObjectEnumHandle;
typedef struct __TEE_ObjectEnumHandle *TEE_ObjectEnumHandle;

typedef uint32_t TEE_Whence;

/**
 * @brief �洢ID�������ӦӦ�õĴ洢�ռ�
 */
enum Object_Storage_Constants {
    /** Ϊÿ��Ӧ�ó��򵥶�ʹ��˽�д洢�ռ� */
    TEE_OBJECT_STORAGE_PRIVATE = 0x00000001, /* Separate private storage space for each application */
    /** ����Ӧ�ó���ĵ������˴洢�ռ� */
    TEE_OBJECT_STORAGE_PERSO   = 0x00000002, /* Separate perso storage space for application */
    /** �����ʵ�ְ�ȫ����洢 */
    TEE_OBJECT_SEC_FLASH       = 0x80000000, /* Add for secure flash storage */
    /** �������rpmb�洢 */
    TEE_OBJECT_STORAGE_RPMB    = 0x80000001, /* Add for rpmb storage */
    /** ������ڴ洢ce */
    TEE_OBJECT_STORAGE_CE      = 0x80000002, /* Add for storage ce */
};

/**
 * @brief ϵͳ��ԴԼ������������λ��ָʾ���Բ�ȡ�����ֵ
 */
enum Miscellaneous_Constants {
    /** ��������λ��ָʾ������ռ�õ���󳤶� */
    TEE_DATA_MAX_POSITION = 0xFFFFFFFF, /* The maximum length that the position indicator of the data stream can take */
    /** objectID����󳤶ȣ�ʵ����չ��128�ֽ� */
    TEE_OBJECT_ID_MAX_LEN = 64,         /* The maximum length of objectID, which actually extends to 128 bytes */
};

/**
 * @brief �������ɴ洢����������ֽ���
 */
enum TEE_DATA_Size {
    /** �����������ɴ洢����������ֽ��� */
    TEE_DATA_OBJECT_MAX_SIZE = 0xFFFFFFFF /* The maximum bytes of data that the object data stream can store */
};

/**
 * @brief TEE_ObjectHandle��handleFlags������TEE_ObjectHandle�Զ����������ķ���Ȩ��
 */
enum Data_Flag_Constants {
    /** �����������ж�Ȩ�ޣ����Զ� */
    TEE_DATA_FLAG_ACCESS_READ = 0x00000001,
    /** ������������дȨ�ޣ�����д�ͽض� */
    TEE_DATA_FLAG_ACCESS_WRITE = 0x00000002,
    /** ������������д��_METAȨ�ޣ�����ɾ�������������� */
    TEE_DATA_FLAG_ACCESS_WRITE_META = 0x00000004,
    /** �����������й����Ȩ�ޣ������Դ򿪶��TEE_ObjectHandles���в����� */
    TEE_DATA_FLAG_SHARE_READ = 0x00000010,
    /** �����������й���д��Ȩ�ޣ����Դ򿪶��TEE_ObjectHandles����д�� */
    TEE_DATA_FLAG_SHARE_WRITE = 0x00000020,
    /** δʹ�� */
    TEE_DATA_FLAG_CREATE = 0x00000200,
    /** ����ͬ���������ļ������ͬ���ļ������ڣ��򴴽��µ������ļ������ͬ���ļ����ڣ��򱨴� */
    TEE_DATA_FLAG_EXCLUSIVE = 0x00000400,
    /** ����ͬ���������ļ������ͬ���ļ������ڣ��򴴽��µ������ļ������ͬ���ļ����ڣ��򱨴� */
    TEE_DATA_FLAG_OVERWRITE = 0x00000400,
    /** ���bit28����Ϊ1����ʾAES256�����Ϊ0����ʾAES128 */
    TEE_DATA_FLAG_AES256 =  0x10000000,
    /** ���bit29����Ϊ1�����ʾ�ȴ򿪵Ͱ汾 */
    TEE_DATA_FLAG_OPEN_AESC = 0x20000000,
};

/**
 * @brief ����һ���µĳ־û�����
 *
 * ����һ���µĳ־û����󣬿���ֱ�ӳ�ʼ����������TEE_Attribute���û�����ʹ�÷��صľ�����ʶ����TEE_Attribute��������
 *
 * @param storageID [IN]��Ӧ��ÿ��Ӧ�ó���ĵ����洢�ռ䣬ֵΪObject_Storage_Constants
 * @param ojbectID [IN]�����ʶ����Ҫ�����Ķ��������
 * @param objectIDLen [IN]�����ʶ���ĳ��ȣ����ֽڣ���������128�ֽ�
 * @param flags [IN]���󴴽���ı�־��ֵ������Data_Flag_Constant��Handle_Flag_Constant�е�һ������
 * @param attributes [IN]��ʱ�����TEE_ObjectHandle���ڳ�ʼ�������TEE_Attribute��������TEE_HANDLE_NULL
 * @param initialData [IN]��ʼ�����ݣ����ڳ�ʼ������������
 * @param initialDataLen [IN]��ʼ���ݳ��ȣ����ֽ�Ϊ��λ��
 * @param object [OUT]����ִ�гɹ��󷵻ص�TEE_ObjectHandle
 *
 * @return TEE_SUCCESS ָʾ�����ѳɹ�ִ��
 * @return TEE_ERROR_ITEM_NOT_FOUND storageID������
 * @return TEE_ERROR_ACCESS_CONFLICT ���ʳ�ͻ
 * @return TEE_ERROR_OUT_OF_MEMORY �ڴ治�㣬�޷���ɲ���
 * @return TEE_ERROR_STORAGE_NO_SPACE û���㹻�Ŀռ�����������
 *
 */
TEE_Result TEE_CreatePersistentObject(uint32_t storageID, const void *ojbectID, size_t objectIDLen, uint32_t flags,
                                      TEE_ObjectHandle attributes, const void *initialData, size_t initialDataLen,
                                      TEE_ObjectHandle *object);

/**
 * @brief �����е����ö���
 *
 * �����е����ö����û�����ʹ�÷��صľ�����ʶ����TEE_Attribute��������
 *
 * @param storageID [IN]��Ӧ��ÿ��Ӧ�ó���ĵ����洢�ռ䣬ֵΪObject_Storage_Constants
 * @param ojbectID [IN]�����ʶ����Ҫ�򿪵Ķ��������
 * @param objectIDLen [IN]�����ʶ���ĳ��ȣ����ֽڣ���������128�ֽ�
 * @param flags [IN]����򿪺�ı�־��ֵ������Data_Flag_Constants��Handle_Flag_Constants�е�һ������
 * @param object[OUT]����ִ�гɹ��󷵻ص�TEE_ObjectHandle
 *
 * @return TEE_SUCCESS ָʾ�����ѳɹ�ִ��
 * @return TEE_ERROR_ITEM_NOT_FOUND storageID�����ڻ��Ҳ��������ʶ��
 * @return TEE_ERROR_ACCESS_CONFLICT ���ʳ�ͻ
 * @return TEE_ERROR_OUT_OF_MEMORY �ڴ治�㣬�޷���ɲ���
 *
 */
TEE_Result TEE_OpenPersistentObject(uint32_t storageID, const void *ojbectID, size_t objectIDLen, uint32_t flags,
                                    TEE_ObjectHandle *object);

/**
 * @brief �Ӷ������������ȡ���ݵĴ�С�ֽڵ�������
 *
 * �Ӷ������������ȡ���ݵĴ�С�ֽڵ���������TEE_ObjectHandle��Ҫʹ��TEE_DATA_FLAG_ACCESS_READȨ�޴�
 *
 * @param ojbect [IN]Ҫ��ȡ��TEE_ObjectHandle
 * @param buffer [OUT]�洢�����ݵĻ�����
 * @param size [IN]���ֽڶ�ȡ�����ݴ�С
 * @param count [OUT]���ֽ�ʵ�ʶ�ȡ�����ݴ�С
 *
 * @return TEE_SUCCESS ָʾ�����ѳɹ�ִ��
 * @return TEE_ERROR_OUT_OF_MEMORY �ڴ治�㣬�޷���ɲ���
 *
 */
TEE_Result TEE_ReadObjectData(TEE_ObjectHandle ojbect, void *buffer, size_t size, uint32_t *count);

/**
 * @brief �����ݴӻ�����д�������������Ĵ�С�ֽ�
 *
 * �����ݴӻ�����д�������������Ĵ�С�ֽ�,TEE_ObjectHandle��Ҫʹ��TEE_DATA_FLAG_ACCESS_WRITEȨ�޴�
 *
 * @param ojbect [IN]Ҫд���TEE_ObjectHandle
 * @param buffer [IN]�洢Ҫд�������
 * @param size [IN]Ҫд������ݳ��ȣ���С������4096�ֽ�
 *
 * @return TEE_SUCCESS ָʾ�����ѳɹ�ִ��
 * @return TEE_ERROR_OUT_OF_MEMORY �ڴ治�㣬�޷���ɲ���
 * @return TEE_ERROR_STORAGE_NO_SPACE û���㹻�Ŀռ���ִ�в���
 *
 */
TEE_Result TEE_WriteObjectData(TEE_ObjectHandle ojbect, const void *buffer, size_t size);

/**
 * @brief �����������Ĵ�С
 *
 * �����СС�ڵ�ǰ�������Ĵ�С����ɾ�����ж�����ֽڡ������С���ڵ�ǰ�������Ĵ�С����ʹ�á�0����չTEE_ObjectHandle\n
 * ��Ҫ����TEE_DATA_FLAG_ACCESS_WRITEȨ�޴�
 *
 * @param object [IN]Ҫ�ضϵ�TEE_ObjectHandle
 * @param size [IN]���������³��ȣ���С������4096�ֽ�
 *
 * @return TEE_SUCCESS ָʾ�����ѳɹ�ִ��
 * @return TEE_ERROR_STORAGE_NO_SPACE û���㹻�Ŀռ���ִ�в���
 *
 */
TEE_Result TEE_TruncateObjectData(TEE_ObjectHandle object, size_t size);

/**
 * @brief
 * ����TEE_ObjectHandleָ���������λ��
 *
 * ����TEE_ObjectHandleָ���������λ�ã���������λ������Ϊ����ʼλ��+ƫ��������wherece����ƫ��������ʼλ�ã�\n
 * ��ֵ������TEE_Whence��ѡ�񣬺������£�\n
 * TEE_DATA_SEEK_SET��������ƫ��������ʼλ��Ϊ�ļ�ͷ��Ϊ0\n
 * TEE_DATA_SEEK_CUR��������ƫ�Ƶ���ʼλ��Ϊ��ǰλ��\n
 * TEE_DATA_SEEK_END��������ƫ��������ʼλ�����ļ���ĩβ������ƫ����Ϊ����ʱ�������ƫ�ƣ�������ƫ����Ϊ����ʱ������ǰƫ�ơ�
 *
 * @param object [IN]��Ҫ���õ�TEE_ObjectHandle
 * @param offset [IN]������λ���ƶ��Ĵ�С����С������4096�ֽ�
 * @param whence [IN]������ƫ�����ĳ�ʼλ��
 *
 * @return TEE_SUCCESS ָʾ�����ѳɹ�ִ��
 * @return TEE_ERROR_OVERFLOW �ò�������λ��ָʾ����ֵ������ϵͳ����TEE_DATA_MAX_POSIT
 *
 */
TEE_Result TEE_SeekObjectData(TEE_ObjectHandle object, int32_t offset, TEE_Whence whence);

/**
 * @brief ͬ���򿪵�TEE_ObjectHandle��ͬ����Ӧ�İ�ȫ�����ļ�������
 *
 * @param object [IN]��Ҫͬ����TEE_ObjectHandle
 *
 * @return TEE_SUCCESS ָʾ�����ѳɹ�ִ��
 *
 */
TEE_Result TEE_SyncPersistentObject(TEE_ObjectHandle object);

/**
 * @brief ���Ķ����ʶ��
 *
 * ��Ҫʹ��TEE_DATA_FLAG_ACCESS_WRITE_METAȨ�޴�TEE_ObjectHandle
 *
 * @param object [IN/OUT]Ҫ�޸ĵĶ�����
 * @param newObjectID [IN]�¶����ʶ��
 * @param newObjectIDLen [IN]�¶����ʶ������
 *
 * @return TEE_SUCCESS ָʾ�����ѳɹ�ִ��
 *
 */
TEE_Result TEE_RenamePersistentObject(TEE_ObjectHandle object, void *newObjectID, size_t newObjectIDLen);

/**
 * @brief ����δ��ʼ������ö�����ľ��
 *
 * @param obj_enumerator [OUT]ָ���´����Ķ���ö���������ָ��
 *
 * @return TEE_SUCCESS ָʾ�����ѳɹ�ִ��
 * @return TEE_ERROR_OUT_OF_MEMORY û���㹻���ڴ�������
 *
 */
TEE_Result TEE_AllocatePersistentObjectEnumerator(TEE_ObjectEnumHandle *obj_enumerator);

/**
 * @brief �ͷ��ѷ���Ķ���ö���������
 *
 * �������ú���ʧЧ�����з���ľ�������ͷţ���TEE_AllocatePersistentObjectEnumerator���ʹ��
 *
 * @param obj_enumerator [IN]��������TEE_ObjectEnumHandle
 *
 */
void TEE_FreePersistentObjectEnumerator(TEE_ObjectEnumHandle obj_enumerator);

/**
 * @brief ����ʱ����ö��������Ϊ���ʼ״̬����������״̬
 *
 * @param obj_enumerator [IN]��Ҫ���õĶ���ö������TEE_ObjectEnumHandle
 *
 */
void TEE_ResetPersistentObjectEnumerator(TEE_ObjectEnumHandle obj_enumerator);

/**
 * @brief ��ʼö�ٸ����洢�ռ��е����ж���
 *
 * �������Ϣ����ͨ��TEE_GetNextPersistentObject������ȡ
 *
 * @param obj_enumerator [IN]����Ķ���ö����TEE_ObjectEnumHandle
 * @param storage_id [IN]��Ӧ��ÿ��Ӧ�ó���ĵ����洢�ռ䣬ֵΪObject_Storage_Constants��\n
 * Ŀǰ��֧��TEE_STORAGE_PRIVATE
 *
 * @return TEE_SUCCESS ָʾ�����ѳɹ�ִ��
 * @return TEE_ITEM_NOT_FOUND storageID����TEE_STORAGE_PRIVATE���ߴ洢�ռ���û�ж���
 *
 */
TEE_Result TEE_StartPersistentObjectEnumerator(TEE_ObjectEnumHandle obj_enumerator, uint32_t storage_id);

/**
 * @brief ��ȡ����ö�����е���һ������
 *
 * ���ض����TEE_ObjectInfo��objectID��objectIDLen��Ϣ
 *
 * @param obj_enumerator [IN]��ʼ������ö����TEE_ObjectEnumHandle
 * @param object_info [IN]�洢��ȡ����TEE_ObjectInfo�ṹ��ָ��
 * @param object_id [IN]������ָ�룬���ڴ洢��ȡ��objectID
 * @param object_id_len[IN]���ڴ洢��ȡ���Ķ���IDLen
 *
 * @param TEE_SUCCESS ָʾ�����ѳɹ�ִ��
 * @param TEE_ITEM_NOT_FOUND ö����û�ж����ö������δ��ʼ��
 *
 */
TEE_Result TEE_GetNextPersistentObject(TEE_ObjectEnumHandle obj_enumerator,
    TEE_ObjectInfo *object_info, void *object_id, size_t *object_id_len);

/**
 * @brief �رմ򿪵�TEE_ObjectHandle��ɾ������
 *
 * �ö����ǳ־ö��󣬲�����Ҫʹ��TEE_DATA_FLAG_ACCESS_WRITE_METAȨ�޴�
 *
 * @param object [IN]��Ҫ�رպ�ɾ����TEE_ObjectHandle
 *
 * @return TEE_SUCCESS ָʾ�����ѳɹ�ִ��
 * @return TEE_ERROR_STORAGE_NOT_AVAILABLE �޷������ļ����ڵĴ洢����
 *
 */
TEE_Result TEE_CloseAndDeletePersistentObject1(TEE_ObjectHandle object);

#endif
