# TeeTrustedStorage


## 概述

TeeTrustedStorage提供了一组安全存储接口。

开发者可以使用这些接口实现安全存储的相关功能。

**起始版本：**

1


## 汇总


### 文件

| 名称 | 描述 | 
| -------- | -------- |
| [tee_object_api.h](tee__object__api_8h.md) | 安全存储接口 | 
| [tee_trusted_storage_api.h](tee__trusted__storage__api_8h.md) | 安全存储接口 | 


### 宏定义

| 名称 | 描述 | 
| -------- | -------- |
| TEE_HANDLE_NULL    0x00000000 | HANDLE_NULL的定义，无效的对象句柄 | 
| TEE_ATTR_FLAG_VALUE    0x20000000 | 属性标识符标志列表 | 
| **TEE_ATTR_FLAG_PUBLIC**   0x10000000 |  | 
| **TEE_ATTR_IS_BUFFER**(attribute_id)   ((((attribute_id) &lt;&lt; 2) &gt;&gt; 31) == 0) |  | 
| **TEE_ATTR_IS_VALUE**(attribute_id)   ((((attribute_id) &lt;&lt; 2) &gt;&gt; 31) == 1) |  | 
| **TEE_ATTR_IS_PROTECTED**(attribute_id)   ((((attribute_id) &lt;&lt; 3) &gt;&gt; 31) == 0) |  | 
| **TEE_ATTR_IS_PUBLIC**(attribute_id)   ((((attribute_id) &lt;&lt; 3) &gt;&gt; 31) == 1) |  | 


### 类型定义

| 名称 | 描述 | 
| -------- | -------- |
| **TEE_ObjectEnumHandle** |  | 
| **TEE_Whence** |  | 


### 枚举

| 名称 | 描述 | 
| -------- | -------- |
| [Usage_Constants](#usage_constants) {<br/>TEE_USAGE_EXTRACTABLE = 0x00000001, TEE_USAGE_ENCRYPT = 0x00000002, TEE_USAGE_DECRYPT = 0x00000004, TEE_USAGE_MAC = 0x00000008,<br/>TEE_USAGE_SIGN = 0x00000010, TEE_USAGE_VERIFY = 0x00000020, TEE_USAGE_DERIVE = 0x00000040, TEE_USAGE_DEFAULT = 0xFFFFFFFF<br/>} | TEE_ObjectHandle的密钥使用方式，决定了对象密钥的使用情况 | 
| [Handle_Flag_Constants](#handle_flag_constants) { TEE_HANDLE_FLAG_PERSISTENT = 0x00010000, TEE_HANDLE_FLAG_INITIALIZED = 0x00020000, TEE_HANDLE_FLAG_KEY_SET = 0x00040000, TEE_HANDLE_FLAG_EXPECT_TWO_KEYS = 0x00080000 } | TEE_ObjectHandle的句柄标志指示对象的一些信息，是否为永久对象，是否已初始化等。 | 
| [__TEE_Whence](#__tee_whence) { TEE_DATA_SEEK_SET = 0, TEE_DATA_SEEK_CUR, TEE_DATA_SEEK_END } | 数据流定位起始位置选项，用于TEE_SeekObjectData函数 | 
| [Object_Storage_Constants](#object_storage_constants) {<br/>TEE_OBJECT_STORAGE_PRIVATE = 0x00000001, TEE_OBJECT_STORAGE_PERSO = 0x00000002, TEE_OBJECT_SEC_FLASH = 0x80000000, TEE_OBJECT_STORAGE_RPMB = 0x80000001,<br/>TEE_OBJECT_STORAGE_CE = 0x80000002<br/>} | 存储ID，定义对应应用的存储空间 | 
| [Miscellaneous_Constants](#miscellaneous_constants) { TEE_DATA_MAX_POSITION = 0xFFFFFFFF, TEE_OBJECT_ID_MAX_LEN = 64 } | 系统资源约束，如数据流位置指示可以采取的最大值 | 
| [TEE_DATA_Size](#tee_data_size) { TEE_DATA_OBJECT_MAX_SIZE = 0xFFFFFFFF } | 数据流可存储的最大数据字节数 | 
| [Data_Flag_Constants](#data_flag_constants) {<br/>TEE_DATA_FLAG_ACCESS_READ = 0x00000001, TEE_DATA_FLAG_ACCESS_WRITE = 0x00000002, TEE_DATA_FLAG_ACCESS_WRITE_META = 0x00000004, TEE_DATA_FLAG_SHARE_READ = 0x00000010,<br/>TEE_DATA_FLAG_SHARE_WRITE = 0x00000020, TEE_DATA_FLAG_CREATE = 0x00000200, TEE_DATA_FLAG_EXCLUSIVE = 0x00000400, TEE_DATA_FLAG_OVERWRITE = 0x00000400,<br/>TEE_DATA_FLAG_OPEN_AESC = 0x20000000<br/>} | TEE_ObjectHandle的handleFlags决定了TEE_ObjectHandle对对象数据流的访问权限 | 


### 函数

| 名称 | 描述 | 
| -------- | -------- |
| [TEE_GetObjectBufferAttribute](#tee_getobjectbufferattribute) (TEE_ObjectHandle object, uint32_t attributeID, void \*buffer, size_t \*size) | 在TEE_ObjectHandle指向的对象的TEE_Attribute结构中获取联合的缓冲区内容 | 
| [TEE_GetObjectValueAttribute](#tee_getobjectvalueattribute) (TEE_ObjectHandle object, uint32_t attributeID, uint32_t \*a, uint32_t \*b) | 在对象中的TEE_Attribute中获取联合的值 | 
| [TEE_CloseObject](#tee_closeobject) (TEE_ObjectHandle object) | 关闭打开的TEE_ObjectHandle对象 | 
| [TEE_AllocateTransientObject](#tee_allocatetransientobject) (uint32_t objectType, uint32_t maxObjectSize, TEE_ObjectHandle \*object) | 分配一个未初始化的对象来存储密钥或密钥对 | 
| [TEE_FreeTransientObject](#tee_freetransientobject) (TEE_ObjectHandle object) | 释放已分配的临时对象 | 
| [TEE_ResetTransientObject](#tee_resettransientobject) (TEE_ObjectHandle object) | 将瞬态对象重置为初始状态，即分配后的状态 | 
| [TEE_PopulateTransientObject](#tee_populatetransientobject) (TEE_ObjectHandle object, TEE_Attribute \*attrs, uint32_t attrCount) | 将参数attrs中的属性分配给未初始化的瞬态对象 | 
| [TEE_InitRefAttribute](#tee_initrefattribute) (TEE_Attribute \*attr, uint32_t attributeID, void \*buffer, size_t length) | 初始化缓冲区类型TEE_Attribute | 
| [TEE_InitValueAttribute](#tee_initvalueattribute) (TEE_Attribute \*attr, uint32_t attributeID, uint32_t a, uint32_t b) | 初始化TEE_Attribute | 
| [TEE_GenerateKey](#tee_generatekey) (TEE_ObjectHandle object, uint32_t keySize, TEE_Attribute \*params, uint32_t paramCount) | 此函数生成随机密钥或密钥对，并将其分配给临时对象 | 
| [TEE_GetObjectInfo1](#tee_getobjectinfo1) (TEE_ObjectHandle object, TEE_ObjectInfo \*objectInfo) | 获取对象的TEE_ObjectInfo | 
| [TEE_CopyObjectAttributes1](#tee_copyobjectattributes1) (TEE_ObjectHandle destObject, TEE_ObjectHandle srcObject) | 使用初始化对象将TEE_Attribute赋值给未初始化的对象 | 
| [TEE_RestrictObjectUsage1](#tee_restrictobjectusage1) (TEE_ObjectHandle object, uint32_t objectUsage) | 限制对象的objectUse位 | 
| [TEE_CreatePersistentObject](#tee_createpersistentobject) (uint32_t storageID, constvoid \*ojbectID, size_t objectIDLen, uint32_t flags, TEE_ObjectHandle attributes, constvoid \*initialData, size_t initialDataLen, TEE_ObjectHandle \*object) | 创建一个新的持久化对象 | 
| [TEE_OpenPersistentObject](#tee_openpersistentobject) (uint32_t storageID, constvoid \*ojbectID, size_t objectIDLen, uint32_t flags, TEE_ObjectHandle \*object) | 打开现有的永久对象 | 
| [TEE_ReadObjectData](#tee_readobjectdata) (TEE_ObjectHandle ojbect, void \*buffer, size_t size, uint32_t \*count) | 从对象的数据流读取数据的大小字节到缓冲区 | 
| [TEE_WriteObjectData](#tee_writeobjectdata) (TEE_ObjectHandle ojbect, constvoid \*buffer, size_t size) | 将数据从缓冲区写入对象的数据流的大小字节 | 
| [TEE_TruncateObjectData](#tee_truncateobjectdata) (TEE_ObjectHandle object, size_t size) | 更改数据流的大小 | 
| [TEE_SeekObjectData](#tee_seekobjectdata) (TEE_ObjectHandle object, int32_t offset, TEE_Whence whence) | 设置TEE_ObjectHandle指向的数据流位置 | 
| [TEE_SyncPersistentObject](#tee_syncpersistentobject) (TEE_ObjectHandle object) | 同步打开的TEE_ObjectHandle并同步相应的安全属性文件到磁盘 | 
| [TEE_RenamePersistentObject](#tee_renamepersistentobject) (TEE_ObjectHandle object, void \*newObjectID, size_t newObjectIDLen) | 更改对象标识符 | 
| [TEE_AllocatePersistentObjectEnumerator](#tee_allocatepersistentobjectenumerator) (TEE_ObjectEnumHandle \*obj_enumerator) | 分配未初始化对象枚举器的句柄 | 
| [TEE_FreePersistentObjectEnumerator](#tee_freepersistentobjectenumerator) (TEE_ObjectEnumHandle obj_enumerator) | 释放已分配的对象枚举器句柄。 | 
| [TEE_ResetPersistentObjectEnumerator](#tee_resetpersistentobjectenumerator) (TEE_ObjectEnumHandle obj_enumerator) | 将临时对象枚举器重置为其初始状态，即分配后的状态 | 
| [TEE_StartPersistentObjectEnumerator](#tee_startpersistentobjectenumerator) (TEE_ObjectEnumHandle obj_enumerator, uint32_t storage_id) | 开始枚举给定存储空间中的所有对象 | 
| [TEE_GetNextPersistentObject](#tee_getnextpersistentobject) (TEE_ObjectEnumHandle obj_enumerator, TEE_ObjectInfo \*object_info, void \*object_id, size_t \*object_id_len) | 获取对象枚举器中的下一个对象 | 
| [TEE_CloseAndDeletePersistentObject1](#tee_closeanddeletepersistentobject1) (TEE_ObjectHandle object) | 关闭打开的TEE_ObjectHandle并删除对象 | 


## 枚举类型说明


### __TEE_Whence

```
enum __TEE_Whence
```

**描述：**

数据流定位起始位置选项，用于TEE_SeekObjectData函数

| 枚举值 | 描述 | 
| -------- | -------- |
| TEE_DATA_SEEK_SET | 将起始位置定位为数据流的起始位置 | 
| TEE_DATA_SEEK_CUR | 将起始位置定位为当前数据流位置 | 
| TEE_DATA_SEEK_END | 将起始位置定位在数据流的末尾 | 

**起始版本：**

1


### Data_Flag_Constants

```
enum Data_Flag_Constants
```

**描述：**

TEE_ObjectHandle的handleFlags决定了TEE_ObjectHandle对对象数据流的访问权限

| 枚举值 | 描述 | 
| -------- | -------- |
| TEE_DATA_FLAG_ACCESS_READ | 对数据流具有读权限，可以读 | 
| TEE_DATA_FLAG_ACCESS_WRITE | 对数据流具有写权限，可以写和截断 | 
| TEE_DATA_FLAG_ACCESS_WRITE_META | 对数据流具有写入_META权限，可以删除和重命名操作 | 
| TEE_DATA_FLAG_SHARE_READ | 对数据流具有共享读权限，您可以打开多个TEE_ObjectHandles进行并发读 | 
| TEE_DATA_FLAG_SHARE_WRITE | 对数据流具有共享写入权限，可以打开多个TEE_ObjectHandles并发写入 | 
| TEE_DATA_FLAG_CREATE | 未使用 | 
| TEE_DATA_FLAG_EXCLUSIVE | 保护同名的现有文件。如果同名文件不存在，则创建新的数据文件；如果同名文件存在，则报错 | 
| TEE_DATA_FLAG_OVERWRITE | 保护同名的现有文件。如果同名文件不存在，则创建新的数据文件；如果同名文件存在，则报错 | 
| TEE_DATA_FLAG_OPEN_AESC | 如果bit29设置为1，则表示先打开低版本 | 

**起始版本：**

1


### Handle_Flag_Constants

```
enum Handle_Flag_Constants
```

**描述：**

TEE_ObjectHandle的句柄标志指示对象的一些信息，是否为永久对象，是否已初始化等。

| 枚举值 | 描述 | 
| -------- | -------- |
| TEE_HANDLE_FLAG_PERSISTENT | 持久化对象 | 
| TEE_HANDLE_FLAG_INITIALIZED | 对象已初始化 | 
| TEE_HANDLE_FLAG_KEY_SET | 未使用 | 
| TEE_HANDLE_FLAG_EXPECT_TWO_KEYS | 未使用 | 

**起始版本：**

1


### Miscellaneous_Constants

```
enum Miscellaneous_Constants
```

**描述：**

系统资源约束，如数据流位置指示可以采取的最大值

| 枚举值 | 描述 | 
| -------- | -------- |
| TEE_DATA_MAX_POSITION | 数据流的位置指示符可以占用的最大长度 | 
| TEE_OBJECT_ID_MAX_LEN | objectID的最大长度，实际扩展到64字节 | 

**起始版本：**

1


### Object_Storage_Constants

```
enum Object_Storage_Constants
```

**描述：**

存储ID，定义对应应用的存储空间

| 枚举值 | 描述 | 
| -------- | -------- |
| TEE_OBJECT_STORAGE_PRIVATE | 为每个应用程序单独使用私有存储空间 | 
| TEE_OBJECT_STORAGE_PERSO | 用于应用程序的单独个人存储空间 | 
| TEE_OBJECT_SEC_FLASH | 添加以实现安全闪存存储 | 
| TEE_OBJECT_STORAGE_RPMB | 添加用于rpmb存储 | 
| TEE_OBJECT_STORAGE_CE | 添加用于存储ce | 

**起始版本：**

1


### TEE_DATA_Size

```
enum TEE_DATA_Size
```

**描述：**

数据流可存储的最大数据字节数

| 枚举值 | 描述 | 
| -------- | -------- |
| TEE_DATA_OBJECT_MAX_SIZE | 对象数据流可存储的最大数据字节数 | 

**起始版本：**

1


### Usage_Constants

```
enum Usage_Constants
```

**描述：**

TEE_ObjectHandle的密钥使用方式，决定了对象密钥的使用情况

| 枚举值 | 描述 | 
| -------- | -------- |
| TEE_USAGE_EXTRACTABLE | 可以提取对象的密钥 | 
| TEE_USAGE_ENCRYPT | 对象的密钥可以用于加密 | 
| TEE_USAGE_DECRYPT | 对象的密钥可以用于解密 | 
| TEE_USAGE_MAC | 对象的密钥可以用于哈希计算 | 
| TEE_USAGE_SIGN | 对象的密钥可以用于签名 | 
| TEE_USAGE_VERIFY | 对象的密钥可以用来验签 | 
| TEE_USAGE_DERIVE | 对象的密钥可用于派生 | 
| TEE_USAGE_DEFAULT | 对象初始化，默认分配所有权限 | 

**起始版本：**

1


## 函数说明


### TEE_AllocatePersistentObjectEnumerator()

```
TEE_Result TEE_AllocatePersistentObjectEnumerator (TEE_ObjectEnumHandle * obj_enumerator)
```

**描述：**

分配未初始化对象枚举器的句柄

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| obj_enumerator | [OUT]指向新创建的对象枚举器句柄的指针 | 

**返回：**

TEE_SUCCESS 指示函数已成功执行

TEE_ERROR_OUT_OF_MEMORY 没有足够的内存来分配

**起始版本：**

1


### TEE_AllocateTransientObject()

```
TEE_Result TEE_AllocateTransientObject (uint32_t objectType, uint32_t maxObjectSize, TEE_ObjectHandle * object )
```

**描述：**

分配一个未初始化的对象来存储密钥或密钥对

objectType和maxObjectSize需要指定以预分配

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| objectType | [IN]待创建对象的类型，取值为TEE_ObjectType | 
| maxObjectSize | [IN]对象的最大字节数 | 
| object | [OUT]指向新创建对象句柄的指针 | 

**返回：**

TEE_SUCCESS 指示函数已成功执行

TEE_ERROR_OUT_OF_MEMORY 内存不足，无法分配

TEE_ERROR_NOT_SUPPORTED 不支持对象提供的字节

**起始版本：**

1


### TEE_CloseAndDeletePersistentObject1()

```
TEE_Result TEE_CloseAndDeletePersistentObject1 (TEE_ObjectHandle object)
```

**描述：**

关闭打开的TEE_ObjectHandle并删除对象

该对象是持久对象，并且需要使用TEE_DATA_FLAG_ACCESS_WRITE_META权限打开

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| object | [IN]需要关闭和删除的TEE_ObjectHandle | 

**返回：**

TEE_SUCCESS 指示函数已成功执行

TEE_ERROR_STORAGE_NOT_AVAILABLE 无法访问文件所在的存储区域

**起始版本：**

1


### TEE_CloseObject()

```
void TEE_CloseObject (TEE_ObjectHandle object)
```

**描述：**

关闭打开的TEE_ObjectHandle对象

对象可以是持久对象，也可以是临时对象

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| object | [IN]待关闭的TEE_ObjectHandle对象 | 

**起始版本：**

1


### TEE_CopyObjectAttributes1()

```
TEE_Result TEE_CopyObjectAttributes1 (TEE_ObjectHandle destObject, TEE_ObjectHandle srcObject )
```

**描述：**

使用初始化对象将TEE_Attribute赋值给未初始化的对象

该函数使用初始化对象将TEE_Attribute赋值给未初始化的对象，相当于将srcobject的TEE_Attribute复制到destobject中

两个对象的TEE_Attribute类型和编号必须匹配

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| destObject | [IN]要分配的未初始化的TEE_ObjectHandle | 
| srcObject | [IN]初始化的TEE_ObjectHandle用于给另一个对象赋值 | 

**返回：**

TEE_SUCCESS 指示函数已成功执行

TEE_ERROR_CORRUPT_OBJECT 文件损坏，文件句柄将被关闭

TEE_ERROR_STORAGE_NOT_AVAILABLE 无法访问文件所在的存储区域

**起始版本：**

1


### TEE_CreatePersistentObject()

```
TEE_Result TEE_CreatePersistentObject (uint32_t storageID, constvoid * ojbectID, size_t objectIDLen, uint32_t flags, TEE_ObjectHandle attributes, constvoid * initialData, size_t initialDataLen, TEE_ObjectHandle * object )
```

**描述：**

创建一个新的持久化对象

创建一个新的持久化对象，可以直接初始化数据流和TEE_Attribute，用户可以使用返回的句柄访问对象的TEE_Attribute和数据流

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| storageID | [IN]对应于每个应用程序的单独存储空间，值为Object_Storage_Constants | 
| ojbectID | [IN]对象标识符，要创建的对象的名称 | 
| objectIDLen | [IN]对象标识符的长度（按字节），不超过128字节 | 
| flags | [IN]对象创建后的标志，值可以是Data_Flag_Constants或Handle_Flag_Constants中的一个或多个 | 
| attributes | [IN]临时对象的TEE_ObjectHandle用于初始化对象的TEE_Attribute，可以是TEE_HANDLE_NULL | 
| initialData | [IN]初始化数据，用于初始化数据流数据 | 
| initialDataLen | [IN]初始数据长度（以字节为单位） | 
| object | [OUT]函数执行成功后返回的TEE_ObjectHandle | 

**返回：**

TEE_SUCCESS 指示函数已成功执行

TEE_ERROR_ITEM_NOT_FOUND storageID不存在

TEE_ERROR_ACCESS_CONFLICT 访问冲突

TEE_ERROR_OUT_OF_MEMORY 内存不足，无法完成操作

TEE_ERROR_STORAGE_NO_SPACE 没有足够的空间来创建对象

**起始版本：**

1


### TEE_FreePersistentObjectEnumerator()

```
void TEE_FreePersistentObjectEnumerator (TEE_ObjectEnumHandle obj_enumerator)
```

**描述：**

释放已分配的对象枚举器句柄。

函数调用后句柄失效，所有分配的句柄都被释放，与TEE_AllocatePersistentObjectEnumerator配对使用

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| obj_enumerator | [IN]待发布的TEE_ObjectEnumHandle | 

**起始版本：**

1


### TEE_FreeTransientObject()

```
void TEE_FreeTransientObject (TEE_ObjectHandle object)
```

**描述：**

释放已分配的临时对象

函数调用后，句柄失效，所有分配的都被释放。与TEE_AllocateTransientObject配对

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| object[IN] | 需要释放的TEE_ObjectHandle | 

**起始版本：**

1


### TEE_GenerateKey()

```
TEE_Result TEE_GenerateKey (TEE_ObjectHandle object, uint32_t keySize, TEE_Attribute * params, uint32_t paramCount )
```

**描述：**

此函数生成随机密钥或密钥对，并将其分配给临时对象

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| object | [IN]瞬态对象，用于存储生成的密钥 | 
| keySize | [IN]所需密钥的字节数 | 
| params | [IN]密钥生成参数说明 | 
| paramCount | [IN]生成密钥所需的参数数 | 

**返回：**

TEE_SUCCESS 指示函数已成功执行

TEE_ERROR_BAD_PARAMETERS 生成的密钥与临时对象可以存储的密钥类型不一致

**起始版本：**

1


### TEE_GetNextPersistentObject()

```
TEE_Result TEE_GetNextPersistentObject (TEE_ObjectEnumHandle obj_enumerator, TEE_ObjectInfo * object_info, void * object_id, size_t * object_id_len )
```

**描述：**

获取对象枚举器中的下一个对象

返回对象的TEE_ObjectInfo、objectID、objectIDLen信息

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| obj_enumerator | [IN]初始化对象枚举器TEE_ObjectEnumHandle | 
| object_info | [IN]存储获取到的TEE_ObjectInfo结构体指针 | 
| object_id | [IN]缓冲区指针，用于存储获取的objectID | 
| object_id_len | [IN]用于存储获取到的对象IDLen | 

**返回：**

TEE_SUCCESS 指示函数已成功执行

TEE_ITEM_NOT_FOUND 枚举器没有对象或枚举器尚未初始化

**起始版本：**

1


### TEE_GetObjectBufferAttribute()

```
TEE_Result TEE_GetObjectBufferAttribute (TEE_ObjectHandle object, uint32_t attributeID, void * buffer, size_t * size )
```

**描述：**

在TEE_ObjectHandle指向的对象的TEE_Attribute结构中获取联合的缓冲区内容

TEE_Attribute结构中的联合成员需要是ref。如果TEE_Attribute是私有的，则对象的使用常数必须包括TEE_USAGE_EXTRACTABLE

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| object | [IN]源TEE_ObjectHandle | 
| attributeID | [IN]要获取的属性ID，如TEE_ObjectAttribute，也可以自定义 | 
| buffer | [OUT]指针，指向的缓冲区用于存储获取的缓冲区的内容 | 
| size | [IN/OUT]指针，存储内容字节长度 | 

**返回：**

TEE_SUCCESS 指示函数已成功执行

TEE_ERROR_ITEM_NOT_FOUND 在对象中找不到要查找的TEE_Attribute，或者对象未初始化

TEE_ERROR_SHORT_BUFFER提供的缓冲区太小，无法存储获取的内容

**起始版本：**

1


### TEE_GetObjectInfo1()

```
TEE_Result TEE_GetObjectInfo1 (TEE_ObjectHandle object, TEE_ObjectInfo * objectInfo )
```

**描述：**

获取对象的TEE_ObjectInfo

获取对象的TEE_ObjectInfo，并将其复制到参数objectInfo指向的空间中，该空间由用户预分配

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| object | [IN]源TEE_ObjectHandle | 
| objectInfo | [OUT]用于存储TEE_ObjectInfo的结构体指针 | 

**返回：**

TEE_SUCCESS 指示函数已成功执行

TEE_ERROR_CORRUPT_OBJECT 文件损坏，文件句柄将被关闭

TEE_ERROR_STORAGE_NOT_AVAILABLE 无法访问文件所在的存储区域

**起始版本：**

1


### TEE_GetObjectValueAttribute()

```
TEE_Result TEE_GetObjectValueAttribute (TEE_ObjectHandle object, uint32_t attributeID, uint32_t * a, uint32_t * b )
```

**描述：**

在对象中的TEE_Attribute中获取联合的值

TEE_Attribute结构中联合的成员必须为value。如果TEE_Attribute是私有的，则对象的Usage_Constants需要包括TEE_USAGE_EXTRACTABLE

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| object | [IN]源TEE_ObjectHandle | 
| attributeID | [IN]需要获取的属性ID，如TEE_ObjectAttribute，也可以自定义 | 
| a | [OUT]指针，指向的空间用于存储a | 
| b | [OUT]指针，指向的空间用于存储b | 

**返回：**

TEE_SUCCESS 指示函数已成功执行

TEE_ERROR_ITEM_NOT_FOUND 在对象中找不到要查找的TEE_Attribute，或者对象未初始化

TEE_ERROR_ACCESS_DENIED 尝试获取私有TEE_Attribute，但未设置TEE_USAGE_EXTRACTABLE

**起始版本：**

1


### TEE_InitRefAttribute()

```
void TEE_InitRefAttribute (TEE_Attribute * attr, uint32_t attributeID, void * buffer, size_t length )
```

**描述：**

初始化缓冲区类型TEE_Attribute

TEE_Attribute结构中的联合成员需要是ref

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| attr | [OUT]要初始化的TEE_Attribute | 
| attributeID | [IN]分配给TEE_Attribute的ID | 
| buffer | [IN]缓冲区存储要分配的内容 | 
| length | [IN]赋值内容的字节长度 | 

**起始版本：**

1


### TEE_InitValueAttribute()

```
void TEE_InitValueAttribute (TEE_Attribute * attr, uint32_t attributeID, uint32_t a, uint32_t b )
```

**描述：**

初始化TEE_Attribute

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| attr | [OUT]要初始化的TEE_Attribute | 
| attributeID | [IN]分配给TEE_Attribute的ID | 
| a | [IN]将值赋值给TEE_Attribute中的联合的成员值a | 
| b | [IN]将值赋值给TEE_Attribute中的联合的成员值b | 

**起始版本：**

1


### TEE_OpenPersistentObject()

```
TEE_Result TEE_OpenPersistentObject (uint32_t storageID, constvoid * ojbectID, size_t objectIDLen, uint32_t flags, TEE_ObjectHandle * object )
```

**描述：**

打开现有的永久对象

打开现有的永久对象，用户可以使用返回的句柄访问对象的TEE_Attribute和数据流

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| storageID | [IN]对应于每个应用程序的单独存储空间，值为Object_Storage_Constants | 
| ojbectID | [IN]对象标识符，要打开的对象的名称 | 
| objectIDLen | [IN]对象标识符的长度（按字节），不超过128字节 | 
| flags | [IN]对象打开后的标志，值可以是Data_Flag_Constants或Handle_Flag_Constants中的一个或多个 | 
| object | [OUT]函数执行成功后返回的TEE_ObjectHandle | 

**返回：**

TEE_SUCCESS 指示函数已成功执行

TEE_ERROR_ITEM_NOT_FOUND storageID不存在或找不到对象标识符

TEE_ERROR_ACCESS_CONFLICT 访问冲突

TEE_ERROR_OUT_OF_MEMORY 内存不足，无法完成操作

**起始版本：**

1


### TEE_PopulateTransientObject()

```
TEE_Result TEE_PopulateTransientObject (TEE_ObjectHandle object, TEE_Attribute * attrs, uint32_t attrCount )
```

**描述：**

将参数attrs中的属性分配给未初始化的瞬态对象

确保对象仍未初始化

参数attrs由可信应用程序提供

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| object | [IN/OUT]TEE_ObjectHandle已创建但未初始化 | 
| attrs | [IN]对象属性数组，可以是一个或多个TEE_Attribute | 
| attrCount | [IN]数组成员数 | 

**返回：**

TEE_SUCCESS 指示函数已成功执行

TEE_ERROR_BAD_PARAMETERS 属性不正确或不一致

**起始版本：**

1


### TEE_ReadObjectData()

```
TEE_Result TEE_ReadObjectData (TEE_ObjectHandle ojbect, void * buffer, size_t size, uint32_t * count )
```

**描述：**

从对象的数据流读取数据的大小字节到缓冲区

从对象的数据流读取数据的大小字节到缓冲区，TEE_ObjectHandle需要使用TEE_DATA_FLAG_ACCESS_READ权限打开

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| object | [IN]要读取的TEE_ObjectHandle | 
| buffer | [OUT]存储读数据的缓冲区 | 
| size | [IN]按字节读取的数据大小 | 
| count | [OUT]按字节实际读取的数据大小 | 

**返回：**

TEE_SUCCESS 指示函数已成功执行

TEE_ERROR_OUT_OF_MEMORY 内存不足，无法完成操作

**起始版本：**

1


### TEE_RenamePersistentObject()

```
TEE_Result TEE_RenamePersistentObject (TEE_ObjectHandle object, void * newObjectID, size_t newObjectIDLen )
```

**描述：**

更改对象标识符

需要使用TEE_DATA_FLAG_ACCESS_WRITE_META权限打开TEE_ObjectHandle

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| object | [IN/OUT]要修改的对象句柄 | 
| newObjectID | [IN]新对象标识符 | 
| newObjectIDLen | [IN]新对象标识符长度 | 

**返回：**

TEE_SUCCESS 指示函数已成功执行

**起始版本：**

1


### TEE_ResetPersistentObjectEnumerator()

```
void TEE_ResetPersistentObjectEnumerator (TEE_ObjectEnumHandle obj_enumerator)
```

**描述：**

将临时对象枚举器重置为其初始状态，即分配后的状态

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| obj_enumerator | [IN]需要重置的对象枚举器的TEE_ObjectEnumHandle | 

**起始版本：**

1


### TEE_ResetTransientObject()

```
void TEE_ResetTransientObject (TEE_ObjectHandle object)
```

**描述：**

将瞬态对象重置为初始状态，即分配后的状态

可以重用已分配但未存储密钥的未初始化对象来存储密钥

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| object | [IN]需要重置的TEE_ObjectHandle | 

**起始版本：**

1


### TEE_RestrictObjectUsage1()

```
TEE_Result TEE_RestrictObjectUsage1 (TEE_ObjectHandle object, uint32_t objectUsage )
```

**描述：**

限制对象的objectUse位

此位决定对象中密钥的使用情况。取值范围为“使用量_常量”。对于参数objectUse的标志位：

如果此位设置为1，则对象的使用标志不会改变

当该参数设置为0时，清除该对象对应的对象使用标志。

新创建的对象将包含所有的使用量_常量，并且使用量标志只能清除，不能设置

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| object | [IN]需要限制的TEE_ObjectHandle | 
| objectUsage | [IN]用户希望更改的objectUsage | 

**起始版本：**

1

**返回：**

TEE_SUCCESS 指示函数已成功执行

TEE_ERROR_CORRUPT_OBJECT 文件损坏，文件句柄将被关闭

TEE_ERROR_STORAGE_NOT_AVAILABLE 无法访问文件所在的存储区域


### TEE_SeekObjectData()

```
TEE_Result TEE_SeekObjectData (TEE_ObjectHandle object, int32_t offset, TEE_Whence whence )
```

**描述：**

设置TEE_ObjectHandle指向的数据流位置

设置TEE_ObjectHandle指向的数据流位置，将数据流位置设置为：起始位置+偏移量参数whence控制偏移量的起始位置，

该值可以在TEE_Whence中选择，含义如下：

TEE_DATA_SEEK_SET，数据流偏移量的起始位置为文件头，为0

TEE_DATA_SEEK_CUR，数据流偏移的起始位置为当前位置

TEE_DATA_SEEK_END，数据流偏移量的起始位置是文件的末尾当参数偏移量为正数时，它向后偏移，当参数偏移量为负数时，它向前偏移。

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| object | [IN]需要设置的TEE_ObjectHandle | 
| offset | [IN]数据流位置移动的大小，大小不超过4096字节 | 
| whence | [IN]数据流偏移量的初始位置 | 

**起始版本：**

1

**返回：**

TEE_SUCCESS 指示函数已成功执行

TEE_ERROR_OVERFLOW 该操作导致位置指示器的值超过其系统限制TEE_DATA_MAX_POSIT


### TEE_StartPersistentObjectEnumerator()

```
TEE_Result TEE_StartPersistentObjectEnumerator (TEE_ObjectEnumHandle obj_enumerator, uint32_t storage_id )
```

**描述：**

开始枚举给定存储空间中的所有对象

对象的信息可以通过TEE_GetNextPersistentObject函数获取

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| obj_enumerator | [IN]分配的对象枚举器TEE_ObjectEnumHandle | 
| storage_id | [IN]对应于每个应用程序的单独存储空间，值为Object_Storage_Constants，<br/>目前仅支持TEE_STORAGE_PRIVATE | 

**返回：**

TEE_SUCCESS 指示函数已成功执行

TEE_ITEM_NOT_FOUND storageID不是TEE_STORAGE_PRIVATE或者存储空间中没有对象

**起始版本：**

1


### TEE_SyncPersistentObject()

```
TEE_Result TEE_SyncPersistentObject (TEE_ObjectHandle object)
```

**描述：**

同步打开的TEE_ObjectHandle并同步相应的安全属性文件到磁盘

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| object | [IN]需要同步的TEE_ObjectHandle | 

**返回：**

TEE_SUCCESS 指示函数已成功执行

**起始版本：**

1


### TEE_TruncateObjectData()

```
TEE_Result TEE_TruncateObjectData (TEE_ObjectHandle object, size_t size )
```

**描述：**

更改数据流的大小

如果大小小于当前数据流的大小，则删除所有多余的字节。如果大小大于当前数据流的大小，则使用“0”扩展

TEE_ObjectHandle需要具有TEE_DATA_FLAG_ACCESS_WRITE权限打开

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| object | [IN]要截断的TEE_ObjectHandle | 
| size | [IN]数据流的新长度，大小不超过4096字节 | 

**返回：**

TEE_SUCCESS 指示函数已成功执行

TEE_ERROR_STORAGE_NO_SPACE 没有足够的空间来执行操作

**起始版本：**

1


### TEE_WriteObjectData()

```
TEE_Result TEE_WriteObjectData (TEE_ObjectHandle ojbect, constvoid * buffer, size_t size )
```

**描述：**

将数据从缓冲区写入对象的数据流的大小字节

将数据从缓冲区写入对象的数据流的大小字节,TEE_ObjectHandle需要使用TEE_DATA_FLAG_ACCESS_WRITE权限打开

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| ojbect | [IN]要写入的TEE_ObjectHandle | 
| buffer | [IN]存储要写入的数据 | 
| size | [IN]要写入的数据长度，大小不超过4096字节 | 

**返回：**

TEE_SUCCESS 指示函数已成功执行

TEE_ERROR_OUT_OF_MEMORY 内存不足，无法完成操作

TEE_ERROR_STORAGE_NO_SPACE 没有足够的空间来执行操作

**起始版本：**

1
