# tee_trusted_storage_api.h


## 概述

安全存储接口

开发者可以调用这些接口实现安全存储相关的功能

**起始版本：**

1

**相关模块：**

[TeeTrustedStorage](_tee_trusted_storage.md)


## 汇总


### 类型定义

| 名称 | 描述 | 
| -------- | -------- |
| **TEE_ObjectEnumHandle** |  | 
| **TEE_Whence** |  | 


### 枚举

| 名称 | 描述 | 
| -------- | -------- |
| [__TEE_Whence](_tee_trusted_storage.md#__tee_whence) { [TEE_DATA_SEEK_SET](_tee_trusted_storage.md) = 0, [TEE_DATA_SEEK_CUR](_tee_trusted_storage.md), [TEE_DATA_SEEK_END](_tee_trusted_storage.md) } | 数据流定位起始位置选项，用于TEE_SeekObjectData函数 | 
| [Object_Storage_Constants](_tee_trusted_storage.md#object_storage_constants) {<br/>[TEE_OBJECT_STORAGE_PRIVATE](_tee_trusted_storage.md) = 0x00000001, [TEE_OBJECT_STORAGE_PERSO](_tee_trusted_storage.md) = 0x00000002, [TEE_OBJECT_SEC_FLASH](_tee_trusted_storage.md) = 0x80000000, [TEE_OBJECT_STORAGE_RPMB](_tee_trusted_storage.md) = 0x80000001,<br/>[TEE_OBJECT_STORAGE_CE](_tee_trusted_storage.md) = 0x80000002<br/>} | 存储ID，定义对应应用的存储空间 | 
| [Miscellaneous_Constants](_tee_trusted_storage.md#miscellaneous_constants) { [TEE_DATA_MAX_POSITION](_tee_trusted_storage.md) = 0xFFFFFFFF, [TEE_OBJECT_ID_MAX_LEN](_tee_trusted_storage.md) = 64 } | 系统资源约束，如数据流位置指示可以采取的最大值 | 
| [TEE_DATA_Size](_tee_trusted_storage.md#tee_data_size) { [TEE_DATA_OBJECT_MAX_SIZE](_tee_trusted_storage.md) = 0xFFFFFFFF } | 数据流可存储的最大数据字节数 | 
| [Data_Flag_Constants](_tee_trusted_storage.md#data_flag_constants) {<br/>[TEE_DATA_FLAG_ACCESS_READ](_tee_trusted_storage.md) = 0x00000001, [TEE_DATA_FLAG_ACCESS_WRITE](_tee_trusted_storage.md) = 0x00000002, [TEE_DATA_FLAG_ACCESS_WRITE_META](_tee_trusted_storage.md) = 0x00000004, [TEE_DATA_FLAG_SHARE_READ](_tee_trusted_storage.md) = 0x00000010,<br/>[TEE_DATA_FLAG_SHARE_WRITE](_tee_trusted_storage.md) = 0x00000020, [TEE_DATA_FLAG_CREATE](_tee_trusted_storage.md) = 0x00000200, [TEE_DATA_FLAG_EXCLUSIVE](_tee_trusted_storage.md) = 0x00000400, [TEE_DATA_FLAG_OVERWRITE](_tee_trusted_storage.md) = 0x00000400,<br/>[TEE_DATA_FLAG_OPEN_AESC](_tee_trusted_storage.md) = 0x20000000<br/>} | TEE_ObjectHandle的handleFlags决定了TEE_ObjectHandle对对象数据流的访问权限 | 


### 函数

| 名称 | 描述 | 
| -------- | -------- |
| [TEE_CreatePersistentObject](_tee_trusted_storage.md#tee_createpersistentobject) (uint32_t storageID, constvoid \*ojbectID, size_t objectIDLen, uint32_t flags, TEE_ObjectHandle attributes, constvoid \*initialData, size_t initialDataLen, TEE_ObjectHandle \*object) | 创建一个新的持久化对象 | 
| [TEE_OpenPersistentObject](_tee_trusted_storage.md#tee_openpersistentobject) (uint32_t storageID, constvoid \*ojbectID, size_t objectIDLen, uint32_t flags, TEE_ObjectHandle \*object) | 打开现有的永久对象 | 
| [TEE_ReadObjectData](_tee_trusted_storage.md#tee_readobjectdata) (TEE_ObjectHandle ojbect, void \*buffer, size_t size, uint32_t \*count) | 从对象的数据流读取数据的大小字节到缓冲区 | 
| [TEE_WriteObjectData](_tee_trusted_storage.md#tee_writeobjectdata) (TEE_ObjectHandle ojbect, constvoid \*buffer, size_t size) | 将数据从缓冲区写入对象的数据流的大小字节 | 
| [TEE_TruncateObjectData](_tee_trusted_storage.md#tee_truncateobjectdata) (TEE_ObjectHandle object, size_t size) | 更改数据流的大小 | 
| [TEE_SeekObjectData](_tee_trusted_storage.md#tee_seekobjectdata) (TEE_ObjectHandle object, int32_t offset, TEE_Whence whence) | 设置TEE_ObjectHandle指向的数据流位置 | 
| [TEE_SyncPersistentObject](_tee_trusted_storage.md#tee_syncpersistentobject) (TEE_ObjectHandle object) | 同步打开的TEE_ObjectHandle并同步相应的安全属性文件到磁盘 | 
| [TEE_RenamePersistentObject](_tee_trusted_storage.md#tee_renamepersistentobject) (TEE_ObjectHandle object, void \*newObjectID, size_t newObjectIDLen) | 更改对象标识符 | 
| [TEE_AllocatePersistentObjectEnumerator](_tee_trusted_storage.md#tee_allocatepersistentobjectenumerator) (TEE_ObjectEnumHandle \*obj_enumerator) | 分配未初始化对象枚举器的句柄 | 
| [TEE_FreePersistentObjectEnumerator](_tee_trusted_storage.md#tee_freepersistentobjectenumerator) (TEE_ObjectEnumHandle obj_enumerator) | 释放已分配的对象枚举器句柄。 | 
| [TEE_ResetPersistentObjectEnumerator](_tee_trusted_storage.md#tee_resetpersistentobjectenumerator) (TEE_ObjectEnumHandle obj_enumerator) | 将临时对象枚举器重置为其初始状态，即分配后的状态 | 
| [TEE_StartPersistentObjectEnumerator](_tee_trusted_storage.md#tee_startpersistentobjectenumerator) (TEE_ObjectEnumHandle obj_enumerator, uint32_t storage_id) | 开始枚举给定存储空间中的所有对象 | 
| [TEE_GetNextPersistentObject](_tee_trusted_storage.md#tee_getnextpersistentobject) (TEE_ObjectEnumHandle obj_enumerator, TEE_ObjectInfo \*object_info, void \*object_id, size_t \*object_id_len) | 获取对象枚举器中的下一个对象 | 
| [TEE_CloseAndDeletePersistentObject1](_tee_trusted_storage.md#tee_closeanddeletepersistentobject1) (TEE_ObjectHandle object) | 关闭打开的TEE_ObjectHandle并删除对象 | 
