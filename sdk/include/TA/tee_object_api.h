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
 * @brief 安全存储接口
 *
 * 开发者可以使用这些接口实现安全存储的相关功能。
 *
 * @since 1
 */
#ifndef __TEE_OBJECT_API_H
#define __TEE_OBJECT_API_H

#include "tee_defines.h"

/**
 * @brief HANDLE_NULL的定义，无效的对象句柄
 */
#define TEE_HANDLE_NULL 0x00000000

/**
 * @brief TEE_ObjectHandle的密钥使用方式，决定了对象密钥的使用情况
 */
enum Usage_Constants {
    /** 可以提取对象的密钥 */
    TEE_USAGE_EXTRACTABLE = 0x00000001,
    /** 对象的密钥可以用于加密 */
    TEE_USAGE_ENCRYPT     = 0x00000002,
    /** 对象的密钥可以用于解密 */
    TEE_USAGE_DECRYPT     = 0x00000004,
    /** 对象的密钥可以用于哈希计算 */
    TEE_USAGE_MAC         = 0x00000008,
    /** 对象的密钥可以用于签名 */
    TEE_USAGE_SIGN        = 0x00000010,
    /** 对象的密钥可以用来验签 */
    TEE_USAGE_VERIFY      = 0x00000020,
    /** 对象的密钥可用于派生 */
    TEE_USAGE_DERIVE      = 0x00000040,
    /** 对象初始化，默认分配所有权限 */
    TEE_USAGE_DEFAULT     = 0xFFFFFFFF,
};

/**
 * @brief TEE_ObjectHandle的句柄标志指示对象的一些信息，是否为永久对象，是否已初始化等。
 */
enum Handle_Flag_Constants {
    /** 持久化对象 */
    TEE_HANDLE_FLAG_PERSISTENT      = 0x00010000,
    /** 对象已初始化 */
    TEE_HANDLE_FLAG_INITIALIZED     = 0x00020000,
    /** 未使用 */
    TEE_HANDLE_FLAG_KEY_SET         = 0x00040000,
    /** 未使用 */
    TEE_HANDLE_FLAG_EXPECT_TWO_KEYS = 0x00080000,
};

/**
 * @brief 属性标识符标志列表
 */
#define TEE_ATTR_FLAG_VALUE  0x20000000
#define TEE_ATTR_FLAG_PUBLIC 0x10000000

#define TEE_ATTR_IS_BUFFER(attribute_id) ((((attribute_id) << 2) >> 31) == 0)
#define TEE_ATTR_IS_VALUE(attribute_id)  ((((attribute_id) << 2) >> 31) == 1)

#define TEE_ATTR_IS_PROTECTED(attribute_id) ((((attribute_id) << 3) >> 31) == 0)
#define TEE_ATTR_IS_PUBLIC(attribute_id)    ((((attribute_id) << 3) >> 31) == 1)

/**
 * @brief 在TEE_ObjectHandle指向的对象的TEE_Attribute结构中获取联合的缓冲区内容
 *
 * TEE_Attribute结构中的联合成员需要是ref。如果TEE_Attribute是私有的，则对象的使用常数必须包括TEE_USAGE_EXTRACTABLE
 *
 * @param object [IN]源TEE_ObjectHandle
 * @param attributeID [IN]要获取的属性ID，如TEE_ObjectAttribute，也可以自定义
 * @param buffer [OUT]指针，指向的缓冲区用于存储获取的缓冲区的内容
 * @param size [IN/OUT]指针，存储内容字节长度
 *
 * @return TEE_SUCCESS 指示函数已成功执行
 * @return TEE_ERROR_ITEM_NOT_FOUND 在对象中找不到要查找的TEE_Attribute，或者对象未初始化
 * @return TEE_ERROR_SHORT_BUFFER提供的缓冲区太小，无法存储获取的内容
 *
 */
TEE_Result TEE_GetObjectBufferAttribute(TEE_ObjectHandle object, uint32_t attributeID, void *buffer, size_t *size);

/**
 * @brief 在对象中的TEE_Attribute中获取联合的值
 *
 * TEE_Attribute结构中联合的成员必须为value。如果TEE_Attribute是私有的，则对象的Usage_Constants需要包括TEE_USAGE_EXTRACTABLE
 *
 * @param object [IN]源TEE_ObjectHandle
 * @param attributeID [IN]需要获取的属性ID，如TEE_ObjectAttribute，也可以自定义
 * @param a [OUT]指针，指向的空间用于存储
 * @param b [OUT]指针，指向的空间用于存储b
 *
 * @return TEE_SUCCESS 指示函数已成功执行
 * @return TEE_ERROR_ITEM_NOT_FOUND 在对象中找不到要查找的TEE_Attribute，或者对象未初始化
 * @return TEE_ERROR_ACCESS_DENIED 尝试获取私有TEE_Attribute，但未设置TEE_USAGE_EXTRACTABLE
 *
 */
TEE_Result TEE_GetObjectValueAttribute(TEE_ObjectHandle object, uint32_t attributeID, uint32_t *a, uint32_t *b);

/**
 * @brief 关闭打开的TEE_ObjectHandle对象
 *
 * 对象可以是持久对象，也可以是临时对象
 *
 * @param object [IN]待关闭的TEE_ObjectHandle对象
 *
 */
void TEE_CloseObject(TEE_ObjectHandle object);

/**
 * @brief 分配一个未初始化的对象来存储键
 *
 * objectType和maxObjectSize需要指定以预分配
 *
 * @param objectType [IN]待创建对象的类型，取值为TEE_ObjectType
 * @param maxObjectSize [IN]对象的最大字节数
 * @param object [OUT]指向新创建对象句柄的指针
 *
 * @return TEE_SUCCESS 指示函数已成功执行
 * @return TEE_ERROR_OUT_OF_MEMORY 内存不足，无法分配
 * @return TEE_ERROR_NOT_SUPPORTED 不支持对象提供的字节
 *
 */
TEE_Result TEE_AllocateTransientObject(uint32_t objectType, uint32_t maxObjectSize, TEE_ObjectHandle *object);

/**
 * @brief 释放已分配的临时对象
 *
 * 函数调用后，句柄失效，所有分配的都被释放。与TEE_AllocateTransientObject配对
 *
 * @param object[IN]需要释放的TEE_ObjectHandle
 *
 */
void TEE_FreeTransientObject(TEE_ObjectHandle object);

/**
 * @brief 将瞬态对象重置为初始状态，即分配后的状态
 *
 * 可以重用已分配但未存储密钥的未初始化对象来存储密钥
 *
 * @param object [IN]需要重置的TEE_ObjectHandle
 *
 */
void TEE_ResetTransientObject(TEE_ObjectHandle object);

/**
 * @brief 将参数attrs中的属性分配给未初始化的瞬态对象
 *
 * 确保对象仍未初始化\n
 * 参数attrs由可信应用程序提供
 *
 * @param object [IN/OUT]TEE_ObjectHandle已创建但未初始化
 * @param attrs [IN]对象属性数组，可以是一个或多个TEE_Attribute
 * @param attrCount [IN]数组成员数
 *
 * @return TEE_SUCCESS 指示函数已成功执行
 * @return TEE_ERROR_BAD_PARAMETERS 属性不正确或不一致
 *
 */
TEE_Result TEE_PopulateTransientObject(TEE_ObjectHandle object, TEE_Attribute *attrs, uint32_t attrCount);

/**
 * @brief 初始化缓冲区类型TEE_Attribute
 *
 * TEE_Attribute结构中的联合成员需要是ref
 *
 * @param attr [OUT]要初始化的TEE_Attribute
 * @param attributeID [IN]分配给TEE_Attribute的ID
 * @param buffer [IN]缓冲区存储要分配的内容
 * @param length [IN]赋值内容的字节长度
 *
 */
void TEE_InitRefAttribute(TEE_Attribute *attr, uint32_t attributeID, void *buffer, size_t length);

/**
 * @brief 初始化TEE_Attribute
 *
 * @param attr [OUT]要初始化的TEE_Attribute
 * @param attributeID [IN]分配给TEE_Attribute的ID
 * @param a [IN]将值赋值给TEE_Attribute中的联合的成员值a
 * @param b [IN]将值赋值给TEE_Attribute中的联合的成员值b
 *
 */
void TEE_InitValueAttribute(TEE_Attribute *attr, uint32_t attributeID, uint32_t a, uint32_t b);

/**
 * @brief 此函数生成随机密钥或密钥对，并将其分配给临时对象
 *
 * @param object [IN]瞬态对象，用于存储生成的密钥
 * @param keySize [IN]所需密钥的字节数
 * @param params [IN]密钥生成参数说明
 * @param paramCount [IN]生成密钥所需的参数数
 *
 * @return TEE_SUCCESS 指示函数已成功执行
 * @return TEE_ERROR_BAD_PARAMETERS 生成的密钥与临时对象可以存储的密钥类型不一致
 *
 */
TEE_Result TEE_GenerateKey(TEE_ObjectHandle object, uint32_t keySize, TEE_Attribute *params, uint32_t paramCount);

/**
 * @brief 获取对象的TEE_ObjectInfo
 *
 * 获取对象的TEE_ObjectInfo，并将其复制到参数objectInfo指向的空间中，该空间由用户预分配
 *
 * @param object [IN]源TEE_ObjectHandle
 * @param objectInfo [OUT]用于存储TEE_ObjectInfo的结构体指针
 *
 * @return TEE_SUCCESS 指示函数已成功执行
 * @return TEE_ERROR_CORRUPT_OBJECT 文件损坏，文件句柄将被关闭
 * @return TEE_ERROR_STORAGE_NOT_AVAILABLE 无法访问文件所在的存储区域
 *
 */
TEE_Result TEE_GetObjectInfo1(TEE_ObjectHandle object, TEE_ObjectInfo *objectInfo);

/**
 * @brief 使用初始化对象将TEE_Attribute赋值给未初始化的对象
 *
 * 该函数使用初始化对象将TEE_Attribute赋值给未初始化的对象，相当于将srcobject的TEE_Attribute复制到destobject中\n
 * 两个对象的TEE_Attribute类型和编号必须匹配
 *
 * @param destObject [IN]要分配的未初始化的TEE_ObjectHandle
 * @param srcObject [IN]初始化的TEE_ObjectHandle用于给另一个对象赋值
 *
 * @return TEE_SUCCESS 指示函数已成功执行
 * @return TEE_ERROR_CORRUPT_OBJECT 文件损坏，文件句柄将被关闭
 * @return TEE_ERROR_STORAGE_NOT_AVAILABLE 无法访问文件所在的存储区域
 *
 */
TEE_Result TEE_CopyObjectAttributes1(TEE_ObjectHandle destObject, TEE_ObjectHandle srcObject);

/**
 * @brief 限制对象的objectUse位
 *
 * 此位决定对象中密钥的使用情况。取值范围为“使用量_常量”。对于参数objectUse的标志位：\n
 * 如果此位设置为1，则对象的使用标志不会改变\n
 * 当该参数设置为0时，清除该对象对应的对象使用标志。\n
 * 新创建的对象将包含所有的使用量_常量，并且使用量标志只能清除，不能设置
 *
 * @param object [IN]需要限制的TEE_ObjectHandle
 * @param objectUsage [IN]用户希望更改的objectUsage
 *
 * @return TEE_SUCCESS 指示函数已成功执行
 * @return TEE_ERROR_CORRUPT_OBJECT 文件损坏，文件句柄将被关闭
 * @return TEE_ERROR_STORAGE_NOT_AVAILABLE 无法访问文件所在的存储区域
 *
 */
TEE_Result TEE_RestrictObjectUsage1(TEE_ObjectHandle object, uint32_t objectUsage);
#endif
