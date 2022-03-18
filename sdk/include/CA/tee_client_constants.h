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

#ifndef TEE_CLIENT_CONSTANTS_H
#define TEE_CLIENT_CONSTANTS_H
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
 * @file tee_client_constants.h
 *
 * @brief �������ݼ��������塣
 *
 * @since 8
 */

/**
 * @brief ����TEEC_Operation��TEEC_Parameter������
 *
 * @since 8
 */
#define TEEC_PARAM_NUM 4 /* teec param max number */

/**
 * @brief �������صĴ�����
 *
 * @since 8
 */
enum TEEC_ReturnCode {
    TEEC_SUCCESS = 0x0,                       /* �������سɹ� */
    TEEC_ERROR_INVALID_CMD,                   /* �Ƿ������ȫӦ�ò�֧�ֵ����� */
    TEEC_ERROR_SERVICE_NOT_EXIST,             /* ��ȫӦ�ò����� */
    TEEC_ERROR_SESSION_NOT_EXIST,             /* �ͻ���Ӧ���밲ȫӦ�õ����Ӳ����� */
    TEEC_ERROR_SESSION_MAXIMUM,               /* ��ȫӦ�õ����������� */
    TEEC_ERROR_REGISTER_EXIST_SERVICE,        /* ע���Ѿ����ڵİ�ȫӦ�� */
    TEEC_ERROR_TAGET_DEAD_FATAL,              /* ��ȫOS��ܴ��� */
    TEEC_ERROR_READ_DATA,                     /* ��ȡ�ļ����� */
    TEEC_ERROR_WRITE_DATA,                    /* д���ļ����� */
    TEEC_ERROR_TRUNCATE_OBJECT,               /* �ض��ļ����� */
    TEEC_ERROR_SEEK_DATA,                     /* �����ļ����� */
    TEEC_ERROR_FSYNC_DATA,                    /* ͬ���ļ����� */
    TEEC_ERROR_RENAME_OBJECT,                 /* �������ļ����� */
    TEEC_ERROR_TRUSTED_APP_LOAD_ERROR,        /* �򿪻Ựʱ�����ذ�ȫӦ��ʧ�� */
    TEEC_ERROR_GENERIC = 0xFFFF0000,          /* ͨ�ô��󣬳�ʼ����ȫӦ��ʧ�� */
    TEEC_ERROR_ACCESS_DENIED = 0xFFFF0001,    /* Ȩ��У��ʧ�ܣ���TEE�������򿪻Ự�ͷ�����������Ȩ�޵�У�飬
                                                 Ȩ��У�鲻ͨ���᷵�ش������ */
    TEEC_ERROR_CANCEL = 0xFFFF0002,           /* ������ȡ�������������ȡ����־λ����λ��
                                                 �ٶԴ˲������в���ʱ���ش������ */
    TEEC_ERROR_ACCESS_CONFLICT = 0xFFFF0003,  /* �������ʵ���Ȩ�޳�ͻ��
                                                 ��ȫ�洢�����ж��ļ��Ĳ������ʿ��ܻ����������� */
    TEEC_ERROR_EXCESS_DATA = 0xFFFF0004,      /* ��������������̫�� ����ȫӦ���޷����� */
    TEEC_ERROR_BAD_FORMAT = 0xFFFF0005,       /* ���ݸ�ʽ����ȷ���ͻ���Ӧ�����Ĳ�����ʽ������ͻ���Ӧ����
                                                 ��ȫӦ�õ�ͨ��Э�飬��ȫӦ���޷����� */
    TEEC_ERROR_BAD_PARAMETERS = 0xFFFF0006,   /* ������Ч�����Ϊ�ջ�Ƿ��ȴ��� */
    TEEC_ERROR_BAD_STATE = 0xFFFF0007,        /* ��ǰ״̬�µĲ�����Ч������ȫ�洢�������ʱ��
                                                 ���û�г�ʼ����ȫ�洢���񣬻᷵�ش������ */
    TEEC_ERROR_ITEM_NOT_FOUND = 0xFFFF0008,   /* ���������δ�ҵ� */
    TEEC_ERROR_NOT_IMPLEMENTED = 0xFFFF0009,  /* ����Ĳ������ڵ���δʵ�֣�����ȡ������ʱ���ش������ */
    TEEC_ERROR_NOT_SUPPORTED = 0xFFFF000A,    /* ����Ĳ�����Ч��δ֧�֣�����ȫ�ӽ��ܷ����һЩ�㷨,
                                                 ��DSA��ʱ���ش������ */
    TEEC_ERROR_NO_DATA = 0xFFFF000B,          /* ���ݴ��� ������Ĳ����Ҳ�����Ӧ������ */
    TEEC_ERROR_OUT_OF_MEMORY = 0xFFFF000C,    /* ϵͳ������Դ���㣬�ڴ�����ʧ�ܻ᷵�ش������ */
    TEEC_ERROR_BUSY = 0xFFFF000D,             /* ϵͳ��æ��ϵͳ�������ڶ�ռһЩ��Դ */
    TEEC_ERROR_COMMUNICATION = 0xFFFF000E,    /* �ǰ�ȫ����Ӧ�ó����밲ȫӦ��ͨ��ʱ�������� */
    TEEC_ERROR_SECURITY = 0xFFFF000F,         /* ��⵽��ȫ���󣬰�ȫ���緢������ */
    TEEC_ERROR_SHORT_BUFFER = 0xFFFF0010,     /* �ڴ����볤��С��������ȣ�
                                                 ʹ������Ϊ#TEEC_MEMREF_TEMP_OUTPUTʱ��Ҫע�������� */
    TEEC_ERROR_MAC_INVALID = 0xFFFF3071,      /* MACֵУ����� */
    TEEC_ERROR_TARGET_DEAD = 0xFFFF3024,      /* ��ȫӦ�ñ��� */
    TEEC_FAIL = 0xFFFF5002                    /* ͨ�ô��� */
};

/**
 * @brief �������ش��������Դ
 *
 * @since 8
 */
enum TEEC_ReturnCodeOrigin {
    TEEC_ORIGIN_API = 0x1,          /* ���������Կͻ���API */
    TEEC_ORIGIN_COMMS = 0x2,        /* ���������Էǰ�ȫ�����밲ȫ�����ͨ�� */
    TEEC_ORIGIN_TEE = 0x3,          /* ���������԰�ȫ���� */
    TEEC_ORIGIN_TRUSTED_APP = 0x4,  /* ���������԰�ȫӦ�� */
};

/**
 * @brief �����ڴ��ʶ
 *
 * @since 8
 */
enum TEEC_SharedMemCtl {
    TEEC_MEM_INPUT = 0x1,        /* �����ڴ���������Ǵӿͻ���Ӧ�õ���ȫӦ�� */
    TEEC_MEM_OUTPUT = 0x2,       /* �����ڴ���������ǴӰ�ȫӦ�õ��ͻ���Ӧ�� */
    TEEC_MEM_INOUT = 0x3,        /* �����ڴ���ڿͻ���Ӧ���밲ȫӦ��֮��˫�򴫵� */
};

/**
 * @brief �������Ͷ���
 *
 * @since 8
 */
enum TEEC_ParamType {
    TEEC_NONE = 0x0,                  /* ����û��ʹ�� */
    TEEC_VALUE_INPUT = 0x01,          /* ������#TEEC_Value���Ӧ��ֻ����Ϊ���룬
                                         �������Ǵӿͻ���Ӧ�õ���ȫӦ�� */
    TEEC_VALUE_OUTPUT = 0x02,         /* ������#TEEC_Value���Ӧ��ֻ����Ϊ�����
                                         �������ǴӰ�ȫӦ�õ��ͻ���Ӧ�� */
    TEEC_VALUE_INOUT = 0x03,          /* ������#TEEC_Value���Ӧ���ȿ�����Ҳ����� */
    TEEC_MEMREF_TEMP_INPUT = 0x05,    /* ������#TEEC_TempMemoryReference���Ӧ��
                                         ֻ����Ϊ���룬�������Ǵӿͻ���Ӧ�õ���ȫӦ�� */
    TEEC_MEMREF_TEMP_OUTPUT = 0x06,   /* ������#TEEC_TempMemoryReference���Ӧ��
                                         ֻ����Ϊ������������ǴӰ�ȫӦ�õ��ͻ���Ӧ�� */
    TEEC_MEMREF_TEMP_INOUT = 0x07,    /* ������#TEEC_TempMemoryReference���Ӧ���ȿ�����Ҳ�������
                                         ���ڿͻ���Ӧ���밲ȫӦ��֮��˫�򴫵� */
    TEEC_MEMREF_WHOLE = 0xc,          /* ������#TEEC_RegisteredMemoryReference���Ӧ�����������ڴ棬
                                         ����������ָ��Ĺ����ڴ�ı�ʶ#TEEC_SharedMemCtlһ�� */
    TEEC_MEMREF_PARTIAL_INPUT = 0xd,  /* ������#TEEC_RegisteredMemoryReference���Ӧ��ֻ����Ϊ���룬
                                         �������Ǵӿͻ���Ӧ�õ���ȫӦ�� */
    TEEC_MEMREF_PARTIAL_OUTPUT = 0xe, /* ������#TEEC_RegisteredMemoryReference���Ӧ��ֻ����Ϊ�����
                                         �������ǴӰ�ȫӦ�õ��ͻ���Ӧ�� */
    TEEC_MEMREF_PARTIAL_INOUT = 0xf   /* ������#TEEC_RegisteredMemoryReference���Ӧ���ȿ�����Ҳ�������
                                         ���ڿͻ���Ӧ���밲ȫӦ��֮��˫�򴫵� */
};

/**
 * @brief Login��ʽ
 *
 * @since 8
*/
enum TEEC_LoginMethod {
    TEEC_LOGIN_PUBLIC = 0x0,            /* ����ҪLogin���� */
    TEEC_LOGIN_USER,                    /* �ṩ�û����пͻ���Ӧ�õ�Login���� */
    TEEC_LOGIN_GROUP,                   /* �ṩ���û����пͻ���Ӧ�õ�Login���� */
    TEEC_LOGIN_APPLICATION = 0x4,       /* �ṩ�ͻ���Ӧ���Լ���Login���� */
    TEEC_LOGIN_USER_APPLICATION = 0x5,  /* �ṩ�û����пͻ���Ӧ�õ�Login���ݣ�
                                           �Լ��ͻ���Ӧ���Լ���Login���� */
    TEEC_LOGIN_GROUP_APPLICATION = 0x6, /* �ṩ���û����пͻ���Ӧ�õ�Login���ݣ�
                                           �Լ��ͻ���Ӧ���Լ���Login���� */
    TEEC_LOGIN_IDENTIFY = 0x7,          /* TEEOSԤ��LoginMethod */
};

/** @} */
#endif
