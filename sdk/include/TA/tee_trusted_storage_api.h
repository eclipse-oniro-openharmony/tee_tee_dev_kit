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
 * @brief 安全存储接口
 *
 * 开发者可以调用这些接口实现安全存储相关的功能
 *
 * @since 1
 */

#ifndef __TEE_TRUSTED_STORAGE_API_H
#define __TEE_TRUSTED_STORAGE_API_H

#include "tee_defines.h"
#include "tee_object_api.h"

/**
 * @brief 数据流定位起始位置选项，用于TEE_SeekObjectData函数
 */
enum __TEE_Whence {
    /** 将起始位置定位为数据流的起始位置 */
    TEE_DATA_SEEK_SET = 0, /* Position the starting position as the beginning of the data stream */
    /** 将起始位置定位为当前数据流位置 */
    TEE_DATA_SEEK_CUR,     /* Position the starting position as the current data stream position */
    /** 将起始位置定位在数据流的末尾 */
    TEE_DATA_SEEK_END      /* Position the starting position at the end of the data stream */
};

struct __TEE_ObjectEnumHandle;
typedef struct __TEE_ObjectEnumHandle *TEE_ObjectEnumHandle;

typedef uint32_t TEE_Whence;

/**
 * @brief 存储ID，定义对应应用的存储空间
 */
enum Object_Storage_Constants {
    /** 为每个应用程序单独使用私有存储空间 */
    TEE_OBJECT_STORAGE_PRIVATE = 0x00000001, /* Separate private storage space for each application */
    /** 用于应用程序的单独个人存储空间 */
    TEE_OBJECT_STORAGE_PERSO   = 0x00000002, /* Separate perso storage space for application */
    /** 添加以实现安全闪存存储 */
    TEE_OBJECT_SEC_FLASH       = 0x80000000, /* Add for secure flash storage */
    /** 添加用于rpmb存储 */
    TEE_OBJECT_STORAGE_RPMB    = 0x80000001, /* Add for rpmb storage */
    /** 添加用于存储ce */
    TEE_OBJECT_STORAGE_CE      = 0x80000002, /* Add for storage ce */
};

/**
 * @brief 系统资源约束，如数据流位置指示可以采取的最大值
 */
enum Miscellaneous_Constants {
    /** 数据流的位置指示符可以占用的最大长度 */
    TEE_DATA_MAX_POSITION = 0xFFFFFFFF, /* The maximum length that the position indicator of the data stream can take */
    /** objectID的最大长度，实际扩展到128字节 */
    TEE_OBJECT_ID_MAX_LEN = 64,         /* The maximum length of objectID, which actually extends to 128 bytes */
};

/**
 * @brief 数据流可存储的最大数据字节数
 */
enum TEE_DATA_Size {
    /** 对象数据流可存储的最大数据字节数 */
    TEE_DATA_OBJECT_MAX_SIZE = 0xFFFFFFFF /* The maximum bytes of data that the object data stream can store */
};

/**
 * @brief TEE_ObjectHandle的handleFlags决定了TEE_ObjectHandle对对象数据流的访问权限
 */
enum Data_Flag_Constants {
    /** 对数据流具有读权限，可以读 */
    TEE_DATA_FLAG_ACCESS_READ = 0x00000001,
    /** 对数据流具有写权限，可以写和截断 */
    TEE_DATA_FLAG_ACCESS_WRITE = 0x00000002,
    /** 对数据流具有写入_META权限，可以删除和重命名操作 */
    TEE_DATA_FLAG_ACCESS_WRITE_META = 0x00000004,
    /** 对数据流具有共享读权限，您可以打开多个TEE_ObjectHandles进行并发读 */
    TEE_DATA_FLAG_SHARE_READ = 0x00000010,
    /** 对数据流具有共享写入权限，可以打开多个TEE_ObjectHandles并发写入 */
    TEE_DATA_FLAG_SHARE_WRITE = 0x00000020,
    /** 未使用 */
    TEE_DATA_FLAG_CREATE = 0x00000200,
    /** 保护同名的现有文件。如果同名文件不存在，则创建新的数据文件；如果同名文件存在，则报错 */
    TEE_DATA_FLAG_EXCLUSIVE = 0x00000400,
    /** 保护同名的现有文件。如果同名文件不存在，则创建新的数据文件；如果同名文件存在，则报错 */
    TEE_DATA_FLAG_OVERWRITE = 0x00000400,
    /** 如果bit28设置为1，表示AES256，如果为0，表示AES128 */
    TEE_DATA_FLAG_AES256 =  0x10000000,
    /** 如果bit29设置为1，则表示先打开低版本 */
    TEE_DATA_FLAG_OPEN_AESC = 0x20000000,
};

/**
 * @brief 创建一个新的持久化对象
 *
 * 创建一个新的持久化对象，可以直接初始化数据流和TEE_Attribute，用户可以使用返回的句柄访问对象的TEE_Attribute和数据流
 *
 * @param storageID [IN]对应于每个应用程序的单独存储空间，值为Object_Storage_Constants
 * @param ojbectID [IN]对象标识符，要创建的对象的名称
 * @param objectIDLen [IN]对象标识符的长度（按字节），不超过128字节
 * @param flags [IN]对象创建后的标志，值可以是Data_Flag_Constant或Handle_Flag_Constant中的一个或多个
 * @param attributes [IN]临时对象的TEE_ObjectHandle用于初始化对象的TEE_Attribute，可以是TEE_HANDLE_NULL
 * @param initialData [IN]初始化数据，用于初始化数据流数据
 * @param initialDataLen [IN]初始数据长度（以字节为单位）
 * @param object [OUT]函数执行成功后返回的TEE_ObjectHandle
 *
 * @return TEE_SUCCESS 指示函数已成功执行
 * @return TEE_ERROR_ITEM_NOT_FOUND storageID不存在
 * @return TEE_ERROR_ACCESS_CONFLICT 访问冲突
 * @return TEE_ERROR_OUT_OF_MEMORY 内存不足，无法完成操作
 * @return TEE_ERROR_STORAGE_NO_SPACE 没有足够的空间来创建对象
 *
 */
TEE_Result TEE_CreatePersistentObject(uint32_t storageID, const void *ojbectID, size_t objectIDLen, uint32_t flags,
                                      TEE_ObjectHandle attributes, const void *initialData, size_t initialDataLen,
                                      TEE_ObjectHandle *object);

/**
 * @brief 打开现有的永久对象
 *
 * 打开现有的永久对象，用户可以使用返回的句柄访问对象的TEE_Attribute和数据流
 *
 * @param storageID [IN]对应于每个应用程序的单独存储空间，值为Object_Storage_Constants
 * @param ojbectID [IN]对象标识符，要打开的对象的名称
 * @param objectIDLen [IN]对象标识符的长度（按字节），不超过128字节
 * @param flags [IN]对象打开后的标志，值可以是Data_Flag_Constants或Handle_Flag_Constants中的一个或多个
 * @param object[OUT]函数执行成功后返回的TEE_ObjectHandle
 *
 * @return TEE_SUCCESS 指示函数已成功执行
 * @return TEE_ERROR_ITEM_NOT_FOUND storageID不存在或找不到对象标识符
 * @return TEE_ERROR_ACCESS_CONFLICT 访问冲突
 * @return TEE_ERROR_OUT_OF_MEMORY 内存不足，无法完成操作
 *
 */
TEE_Result TEE_OpenPersistentObject(uint32_t storageID, const void *ojbectID, size_t objectIDLen, uint32_t flags,
                                    TEE_ObjectHandle *object);

/**
 * @brief 从对象的数据流读取数据的大小字节到缓冲区
 *
 * 从对象的数据流读取数据的大小字节到缓冲区，TEE_ObjectHandle需要使用TEE_DATA_FLAG_ACCESS_READ权限打开
 *
 * @param ojbect [IN]要读取的TEE_ObjectHandle
 * @param buffer [OUT]存储读数据的缓冲区
 * @param size [IN]按字节读取的数据大小
 * @param count [OUT]按字节实际读取的数据大小
 *
 * @return TEE_SUCCESS 指示函数已成功执行
 * @return TEE_ERROR_OUT_OF_MEMORY 内存不足，无法完成操作
 *
 */
TEE_Result TEE_ReadObjectData(TEE_ObjectHandle ojbect, void *buffer, size_t size, uint32_t *count);

/**
 * @brief 将数据从缓冲区写入对象的数据流的大小字节
 *
 * 将数据从缓冲区写入对象的数据流的大小字节,TEE_ObjectHandle需要使用TEE_DATA_FLAG_ACCESS_WRITE权限打开
 *
 * @param ojbect [IN]要写入的TEE_ObjectHandle
 * @param buffer [IN]存储要写入的数据
 * @param size [IN]要写入的数据长度，大小不超过4096字节
 *
 * @return TEE_SUCCESS 指示函数已成功执行
 * @return TEE_ERROR_OUT_OF_MEMORY 内存不足，无法完成操作
 * @return TEE_ERROR_STORAGE_NO_SPACE 没有足够的空间来执行操作
 *
 */
TEE_Result TEE_WriteObjectData(TEE_ObjectHandle ojbect, const void *buffer, size_t size);

/**
 * @brief 更改数据流的大小
 *
 * 如果大小小于当前数据流的大小，则删除所有多余的字节。如果大小大于当前数据流的大小，则使用“0”扩展TEE_ObjectHandle\n
 * 需要具有TEE_DATA_FLAG_ACCESS_WRITE权限打开
 *
 * @param object [IN]要截断的TEE_ObjectHandle
 * @param size [IN]数据流的新长度，大小不超过4096字节
 *
 * @return TEE_SUCCESS 指示函数已成功执行
 * @return TEE_ERROR_STORAGE_NO_SPACE 没有足够的空间来执行操作
 *
 */
TEE_Result TEE_TruncateObjectData(TEE_ObjectHandle object, size_t size);

/**
 * @brief
 * 设置TEE_ObjectHandle指向的数据流位置
 *
 * 设置TEE_ObjectHandle指向的数据流位置，将数据流位置设置为：起始位置+偏移量参数wherece控制偏移量的起始位置，\n
 * 该值可以在TEE_Whence中选择，含义如下：\n
 * TEE_DATA_SEEK_SET，数据流偏移量的起始位置为文件头，为0\n
 * TEE_DATA_SEEK_CUR，数据流偏移的起始位置为当前位置\n
 * TEE_DATA_SEEK_END，数据流偏移量的起始位置是文件的末尾当参数偏移量为正数时，它向后偏移，当参数偏移量为负数时，它向前偏移。
 *
 * @param object [IN]需要设置的TEE_ObjectHandle
 * @param offset [IN]数据流位置移动的大小，大小不超过4096字节
 * @param whence [IN]数据流偏移量的初始位置
 *
 * @return TEE_SUCCESS 指示函数已成功执行
 * @return TEE_ERROR_OVERFLOW 该操作导致位置指示器的值超过其系统限制TEE_DATA_MAX_POSIT
 *
 */
TEE_Result TEE_SeekObjectData(TEE_ObjectHandle object, int32_t offset, TEE_Whence whence);

/**
 * @brief 同步打开的TEE_ObjectHandle并同步相应的安全属性文件到磁盘
 *
 * @param object [IN]需要同步的TEE_ObjectHandle
 *
 * @return TEE_SUCCESS 指示函数已成功执行
 *
 */
TEE_Result TEE_SyncPersistentObject(TEE_ObjectHandle object);

/**
 * @brief 更改对象标识符
 *
 * 需要使用TEE_DATA_FLAG_ACCESS_WRITE_META权限打开TEE_ObjectHandle
 *
 * @param object [IN/OUT]要修改的对象句柄
 * @param newObjectID [IN]新对象标识符
 * @param newObjectIDLen [IN]新对象标识符长度
 *
 * @return TEE_SUCCESS 指示函数已成功执行
 *
 */
TEE_Result TEE_RenamePersistentObject(TEE_ObjectHandle object, void *newObjectID, size_t newObjectIDLen);

/**
 * @brief 分配未初始化对象枚举器的句柄
 *
 * @param obj_enumerator [OUT]指向新创建的对象枚举器句柄的指针
 *
 * @return TEE_SUCCESS 指示函数已成功执行
 * @return TEE_ERROR_OUT_OF_MEMORY 没有足够的内存来分配
 *
 */
TEE_Result TEE_AllocatePersistentObjectEnumerator(TEE_ObjectEnumHandle *obj_enumerator);

/**
 * @brief 释放已分配的对象枚举器句柄。
 *
 * 函数调用后句柄失效，所有分配的句柄都被释放，与TEE_AllocatePersistentObjectEnumerator配对使用
 *
 * @param obj_enumerator [IN]待发布的TEE_ObjectEnumHandle
 *
 */
void TEE_FreePersistentObjectEnumerator(TEE_ObjectEnumHandle obj_enumerator);

/**
 * @brief 将临时对象枚举器重置为其初始状态，即分配后的状态
 *
 * @param obj_enumerator [IN]需要重置的对象枚举器的TEE_ObjectEnumHandle
 *
 */
void TEE_ResetPersistentObjectEnumerator(TEE_ObjectEnumHandle obj_enumerator);

/**
 * @brief 开始枚举给定存储空间中的所有对象
 *
 * 对象的信息可以通过TEE_GetNextPersistentObject函数获取
 *
 * @param obj_enumerator [IN]分配的对象枚举器TEE_ObjectEnumHandle
 * @param storage_id [IN]对应于每个应用程序的单独存储空间，值为Object_Storage_Constants，\n
 * 目前仅支持TEE_STORAGE_PRIVATE
 *
 * @return TEE_SUCCESS 指示函数已成功执行
 * @return TEE_ITEM_NOT_FOUND storageID不是TEE_STORAGE_PRIVATE或者存储空间中没有对象
 *
 */
TEE_Result TEE_StartPersistentObjectEnumerator(TEE_ObjectEnumHandle obj_enumerator, uint32_t storage_id);

/**
 * @brief 获取对象枚举器中的下一个对象
 *
 * 返回对象的TEE_ObjectInfo、objectID、objectIDLen信息
 *
 * @param obj_enumerator [IN]初始化对象枚举器TEE_ObjectEnumHandle
 * @param object_info [IN]存储获取到的TEE_ObjectInfo结构体指针
 * @param object_id [IN]缓冲区指针，用于存储获取的objectID
 * @param object_id_len[IN]用于存储获取到的对象IDLen
 *
 * @param TEE_SUCCESS 指示函数已成功执行
 * @param TEE_ITEM_NOT_FOUND 枚举器没有对象或枚举器尚未初始化
 *
 */
TEE_Result TEE_GetNextPersistentObject(TEE_ObjectEnumHandle obj_enumerator,
    TEE_ObjectInfo *object_info, void *object_id, size_t *object_id_len);

/**
 * @brief 关闭打开的TEE_ObjectHandle并删除对象
 *
 * 该对象是持久对象，并且需要使用TEE_DATA_FLAG_ACCESS_WRITE_META权限打开
 *
 * @param object [IN]需要关闭和删除的TEE_ObjectHandle
 *
 * @return TEE_SUCCESS 指示函数已成功执行
 * @return TEE_ERROR_STORAGE_NOT_AVAILABLE 无法访问文件所在的存储区域
 *
 */
TEE_Result TEE_CloseAndDeletePersistentObject1(TEE_ObjectHandle object);

#endif
