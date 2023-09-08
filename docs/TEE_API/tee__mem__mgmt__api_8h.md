# tee_mem_mgmt_api.h


## 概述

内存操作接口

开发者可以使用这些接口实现对内存操作相关的功能。

**起始版本：**

1

**相关模块：**

[TeeMemMgmt](_tee_mem_mgmt.md)


## 汇总


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
| [TEE_MemFill](_tee_mem_mgmt.md#tee_memfill) (void \*buffer, uint32_t x, size_t size) | 用x填充缓冲区的第一个大小字节 | 
| [TEE_MemMove](_tee_mem_mgmt.md#tee_memmove) (void \*dest, constvoid \*src, size_t size) | 将大小字节从src复制到dest | 
| [TEE_Malloc](_tee_mem_mgmt.md#tee_malloc) (size_t size, uint32_t hint) | 使用提示值分配大小字节的内存返回的指针将兼容任何C基本数据类型 | 
| [TEE_Free](_tee_mem_mgmt.md#tee_free) (void \*buffer) | 释放TEE_Malloc分配的内存 | 
| [TEE_Realloc](_tee_mem_mgmt.md#tee_realloc) (void \*buffer, size_t new_size) | 重新分配内存 | 
| [TEE_MemCompare](_tee_mem_mgmt.md#tee_memcompare) (constvoid \*buffer1, constvoid \*buffer2, size_t size) | 内存内容比较 | 
| [TEE_CheckMemoryAccessRights](_tee_mem_mgmt.md#tee_checkmemoryaccessrights) (uint32_t accessFlags, constvoid \*buffer, size_t size) | 检查缓冲区的访问权限 | 
| [TEE_SetInstanceData](_tee_mem_mgmt.md#tee_setinstancedata) (void \*instanceData) | 用于在同一实例的不同会话中共享的全局变量 | 
| [TEE_GetInstanceData](_tee_mem_mgmt.md#tee_getinstancedata) (void) | 获取TEE_SetInstanceData设置的指针 | 
