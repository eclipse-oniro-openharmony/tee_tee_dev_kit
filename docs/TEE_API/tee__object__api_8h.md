# tee_object_api.h


## 概述

安全存储接口

开发者可以使用这些接口实现安全存储的相关功能。

**起始版本：**

1

**相关模块：**

[TeeTrustedStorage](_tee_trusted_storage.md)


## 汇总


### 宏定义

| 名称 | 描述 | 
| -------- | -------- |
| [TEE_HANDLE_NULL](_tee_trusted_storage.md)   0x00000000 | HANDLE_NULL的定义，无效的对象句柄 | 
| [TEE_ATTR_FLAG_VALUE](_tee_trusted_storage.md)   0x20000000 | 属性标识符标志列表 | 
| **TEE_ATTR_FLAG_PUBLIC**   0x10000000 |  | 
| **TEE_ATTR_IS_BUFFER**(attribute_id)   ((((attribute_id) &lt;&lt; 2) &gt;&gt; 31) == 0) |  | 
| **TEE_ATTR_IS_VALUE**(attribute_id)   ((((attribute_id) &lt;&lt; 2) &gt;&gt; 31) == 1) |  | 
| **TEE_ATTR_IS_PROTECTED**(attribute_id)   ((((attribute_id) &lt;&lt; 3) &gt;&gt; 31) == 0) |  | 
| **TEE_ATTR_IS_PUBLIC**(attribute_id)   ((((attribute_id) &lt;&lt; 3) &gt;&gt; 31) == 1) |  | 


### 枚举

| 名称 | 描述 | 
| -------- | -------- |
| [Usage_Constants](_tee_trusted_storage.md#usage_constants) {<br/>[TEE_USAGE_EXTRACTABLE](_tee_trusted_storage.md) = 0x00000001, [TEE_USAGE_ENCRYPT](_tee_trusted_storage.md) = 0x00000002, [TEE_USAGE_DECRYPT](_tee_trusted_storage.md) = 0x00000004, [TEE_USAGE_MAC](_tee_trusted_storage.md) = 0x00000008,<br/>[TEE_USAGE_SIGN](_tee_trusted_storage.md) = 0x00000010, [TEE_USAGE_VERIFY](_tee_trusted_storage.md) = 0x00000020, [TEE_USAGE_DERIVE](_tee_trusted_storage.md) = 0x00000040, [TEE_USAGE_DEFAULT](_tee_trusted_storage.md) = 0xFFFFFFFF<br/>} | TEE_ObjectHandle的密钥使用方式，决定了对象密钥的使用情况 | 
| [Handle_Flag_Constants](_tee_trusted_storage.md#handle_flag_constants) { [TEE_HANDLE_FLAG_PERSISTENT](_tee_trusted_storage.md) = 0x00010000, [TEE_HANDLE_FLAG_INITIALIZED](_tee_trusted_storage.md) = 0x00020000, [TEE_HANDLE_FLAG_KEY_SET](_tee_trusted_storage.md) = 0x00040000, [TEE_HANDLE_FLAG_EXPECT_TWO_KEYS](_tee_trusted_storage.md) = 0x00080000 } | TEE_ObjectHandle的句柄标志指示对象的一些信息，是否为永久对象，是否已初始化等。 | 


### 函数

| 名称 | 描述 | 
| -------- | -------- |
| [TEE_GetObjectBufferAttribute](_tee_trusted_storage.md#tee_getobjectbufferattribute) (TEE_ObjectHandle object, uint32_t attributeID, void \*buffer, size_t \*size) | 在TEE_ObjectHandle指向的对象的TEE_Attribute结构中获取联合的缓冲区内容 | 
| [TEE_GetObjectValueAttribute](_tee_trusted_storage.md#tee_getobjectvalueattribute) (TEE_ObjectHandle object, uint32_t attributeID, uint32_t \*a, uint32_t \*b) | 在对象中的TEE_Attribute中获取联合的值 | 
| [TEE_CloseObject](_tee_trusted_storage.md#tee_closeobject) (TEE_ObjectHandle object) | 关闭打开的TEE_ObjectHandle对象 | 
| [TEE_AllocateTransientObject](_tee_trusted_storage.md#tee_allocatetransientobject) (uint32_t objectType, uint32_t maxObjectSize, TEE_ObjectHandle \*object) | 分配一个未初始化的对象来存储密钥或密钥对 | 
| [TEE_FreeTransientObject](_tee_trusted_storage.md#tee_freetransientobject) (TEE_ObjectHandle object) | 释放已分配的临时对象 | 
| [TEE_ResetTransientObject](_tee_trusted_storage.md#tee_resettransientobject) (TEE_ObjectHandle object) | 将瞬态对象重置为初始状态，即分配后的状态 | 
| [TEE_PopulateTransientObject](_tee_trusted_storage.md#tee_populatetransientobject) (TEE_ObjectHandle object, TEE_Attribute \*attrs, uint32_t attrCount) | 将参数attrs中的属性分配给未初始化的瞬态对象 | 
| [TEE_InitRefAttribute](_tee_trusted_storage.md#tee_initrefattribute) (TEE_Attribute \*attr, uint32_t attributeID, void \*buffer, size_t length) | 初始化缓冲区类型TEE_Attribute | 
| [TEE_InitValueAttribute](_tee_trusted_storage.md#tee_initvalueattribute) (TEE_Attribute \*attr, uint32_t attributeID, uint32_t a, uint32_t b) | 初始化TEE_Attribute | 
| [TEE_GenerateKey](_tee_trusted_storage.md#tee_generatekey) (TEE_ObjectHandle object, uint32_t keySize, TEE_Attribute \*params, uint32_t paramCount) | 此函数生成随机密钥或密钥对，并将其分配给临时对象 | 
| [TEE_GetObjectInfo1](_tee_trusted_storage.md#tee_getobjectinfo1) (TEE_ObjectHandle object, TEE_ObjectInfo \*objectInfo) | 获取对象的TEE_ObjectInfo | 
| [TEE_CopyObjectAttributes1](_tee_trusted_storage.md#tee_copyobjectattributes1) (TEE_ObjectHandle destObject, TEE_ObjectHandle srcObject) | 使用初始化对象将TEE_Attribute赋值给未初始化的对象 | 
| [TEE_RestrictObjectUsage1](_tee_trusted_storage.md#tee_restrictobjectusage1) (TEE_ObjectHandle object, uint32_t objectUsage) | 限制对象的objectUse位 | 
