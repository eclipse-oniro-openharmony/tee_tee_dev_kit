# TeeMemMgmt


## 概述

TeeMemMgmt提供了一组内存操作接口。

开发者可以使用这些接口实现对内存操作相关的功能。

**起始版本：**

1


## 汇总


### 文件

| 名称 | 描述 | 
| -------- | -------- |
| [tee_mem_mgmt_api.h](tee__mem__mgmt__api_8h.md) | 内存操作接口 | 


### 宏定义

| 名称 | 描述 | 
| -------- | -------- |
| **ZERO_SIZE_PTR**   ((void \*)16) |  | 
| **zero_or_null_ptr**(x)   ((unsigned long)(x) &lt;= (unsigned long)ZERO_SIZE_PTR) |  | 
| **TEE_MALLOC_FILL_ZERO**   0x00000000 |  | 
| **TEE_MALLOC_NO_FILL**   0x00000001 |  | 
| **TEE_MALLOC_NO_SHARE**   0x00000002 |  | 
| **TEE_MEMORY_ACCESS_READ**   0x00000001 |  | 
| **TEE_MEMORY_ACCESS_WRITE**   0x00000002 |  | 
| **TEE_MEMORY_ACCESS_ANY_OWNER**   0x00000004 |  | 


### 枚举

| 名称 | 描述 | 
| -------- | -------- |
| **MALLOC_HINT** {<br/>**ZERO** = 0, **NOT_ZERO** = 1, **ALIGN_004** = 0x80000002, **ALIGN_008** = 0x80000003,<br/>**ALIGN_016** = 0x80000004, **ALIGN_032** = 0x80000005, **ALIGN_064** = 0x80000006, **ALIGN_128** = 0x80000007,<br/>**ALIGN_256** = 0x80000008, **ALIGN_004_ZERO** = 0x80000012, **ALIGN_008_ZERO** = 0x80000013, **ALIGN_016_ZERO** = 0x80000014,<br/>**ALIGN_032_ZERO** = 0x80000015, **ALIGN_064_ZERO** = 0x80000016, **ALIGN_128_ZERO** = 0x80000017, **ALIGN_256_ZERO** = 0x80000018<br/>} |  | 


### 函数

| 名称 | 描述 | 
| -------- | -------- |
| [TEE_MemFill](#tee_memfill) (void \*buffer, uint32_t x, size_t size) | 用x填充缓冲区的第一个大小字节 | 
| [TEE_MemMove](#tee_memmove) (void \*dest, constvoid \*src, size_t size) | 将大小字节从src复制到dest | 
| [TEE_Malloc](#tee_malloc) (size_t size, uint32_t hint) | 使用提示值分配大小字节的内存返回的指针将兼容任何C基本数据类型 | 
| [TEE_Free](#tee_free) (void \*buffer) | 释放TEE_Malloc分配的内存 | 
| [TEE_Realloc](#tee_realloc) (void \*buffer, size_t new_size) | 重新分配内存 | 
| [TEE_MemCompare](#tee_memcompare) (constvoid \*buffer1, constvoid \*buffer2, size_t size) | 内存内容比较 | 
| [TEE_CheckMemoryAccessRights](#tee_checkmemoryaccessrights) (uint32_t accessFlags, constvoid \*buffer, size_t size) | 检查缓冲区的访问权限 | 
| [TEE_SetInstanceData](#tee_setinstancedata) (void \*instanceData) | 用于在同一实例的不同会话中共享的全局变量 | 
| [TEE_GetInstanceData](#tee_getinstancedata) (void) | 获取TEE_SetInstanceData设置的指针 | 


## 函数说明


### TEE_CheckMemoryAccessRights()

```
TEE_Result TEE_CheckMemoryAccessRights (uint32_t accessFlags, constvoid * buffer, size_t size )
```

**描述：**

检查缓冲区的访问权限

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| accessFlags | [IN]待检查的访问权限 | 
| buffer | [IN]指向内存的指针 | 
| size | [IN]要检查的内存大小 | 

**返回：**

TEE_SUCCESS 具有访问权限

TEE_ERROR_ACCESS_DENIED 没有访问权限

**起始版本：**

1


### TEE_Free()

```
void TEE_Free (void * buffer)
```

**描述：**

释放TEE_Malloc分配的内存

如果缓冲区等于NULL，则TEE_Free将不执行任何操作

调用者应确保缓冲区是由TEE_Malloc或TEE_Realloc创建的，并且不应两次释放一个内存，操作结果不可预测

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| buffer | [IN]指向内存的指针 | 

**起始版本：**

1


### TEE_GetInstanceData()

```
void* TEE_GetInstanceData (void )
```

**描述：**

获取TEE_SetInstanceData设置的指针

**返回：**

指向TEE_SetInstanceData设置的变量的指针，指针不应为NULL

NULL 未设置InstanceData

**起始版本：**

1


### TEE_Malloc()

```
void* TEE_Malloc (size_t size, uint32_t hint )
```

**描述：**

使用提示值分配大小字节的内存返回的指针将兼容任何C基本数据类型

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| size | [IN]将分配的内存大小 | 
| hint | [IN]标志，0表示返回的内存将填充“\0” | 

**返回：**

指向新分配内存的指针

NULL 表示分配时失败

**起始版本：**

1


### TEE_MemCompare()

```
int32_t TEE_MemCompare (constvoid * buffer1, constvoid * buffer2, size_t size )
```

**描述：**

内存内容比较

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| buffer1 | [IN]第一个指针 | 
| buffer2 | [IN]第二个指针 | 
| size | [IN]要比较的字节大小 | 

**返回：**

-1 buffer1 &lt; buffer2

0 buffer1 == buffer2

1 buffer1 &gt; buffer2

**起始版本：**

1


### TEE_MemFill()

```
void TEE_MemFill (void * buffer, uint32_t x, size_t size )
```

**描述：**

用x填充缓冲区的第一个大小字节

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| buffer | [OUT]缓冲区指针 | 
| x | [IN]填充值 | 
| size | [IN]字节数 | 

**起始版本：**

1


### TEE_MemMove()

```
void TEE_MemMove (void * dest, constvoid * src, size_t size )
```

**描述：**

将大小字节从src复制到dest

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| dest | [OUT]dest缓冲区指针 | 
| src | [IN]src缓冲区指针 | 
| size | [IN]字节数 | 

**起始版本：**

1


### TEE_Realloc()

```
void* TEE_Realloc (void * buffer, size_t new_size )
```

**描述：**

重新分配内存

如果new_size大于旧size，则旧内存的内容不会更改，剩余内存是随机字节

修改内存大小时将有一个新的分配操作

如果分配失败，将返回旧内存，此函数将返回NULL

如果缓冲区等于NULL，则此函数与TEE_Malloc相同

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| buffer | [IN]指向内存的指针 | 
| new_size | [IN]重新分配的大小 | 

**返回：**

指向新内存的指针，不应为NULL

NULL表示失败

**起始版本：**

1


### TEE_SetInstanceData()

```
void TEE_SetInstanceData (void * instanceData)
```

**描述：**

用于在同一实例的不同会话中共享的全局变量

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| instanceData | [IN]全局变量地址 | 

**起始版本：**

1
