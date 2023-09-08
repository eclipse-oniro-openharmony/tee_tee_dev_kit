# tee_client_constants.h


## 概述

公共数据及常量定义。

**起始版本：**
11
**相关模块：**

[TeeClient](_tee_client.md)


## 汇总


### 宏定义

| 名称 | 描述 | 
| -------- | -------- |
| [TEEC_PARAM_NUM](_tee_client.md#teec_param_num)   4 | 定义TEEC_Operation中TEEC_Parameter个数。  | 


### 枚举

| 名称 | 描述 | 
| -------- | -------- |
| [TEEC_ReturnCode](_tee_client.md#teec_returncode) {<br/>[TEEC_SUCCESS](_tee_client.md) = 0x0, [TEEC_ERROR_INVALID_CMD](_tee_client.md), [TEEC_ERROR_SERVICE_NOT_EXIST](_tee_client.md), [TEEC_ERROR_SESSION_NOT_EXIST](_tee_client.md),<br/>[TEEC_ERROR_SESSION_MAXIMUM](_tee_client.md), [TEEC_ERROR_REGISTER_EXIST_SERVICE](_tee_client.md), [TEEC_ERROR_TAGET_DEAD_FATAL](_tee_client.md), [TEEC_ERROR_READ_DATA](_tee_client.md),<br/>[TEEC_ERROR_WRITE_DATA](_tee_client.md), [TEEC_ERROR_TRUNCATE_OBJECT](_tee_client.md), [TEEC_ERROR_SEEK_DATA](_tee_client.md), [TEEC_ERROR_FSYNC_DATA](_tee_client.md),<br/>[TEEC_ERROR_RENAME_OBJECT](_tee_client.md), [TEEC_ERROR_TRUSTED_APP_LOAD_ERROR](_tee_client.md), [TEEC_ERROR_GENERIC](_tee_client.md) = 0xFFFF0000, [TEEC_ERROR_ACCESS_DENIED](_tee_client.md) = 0xFFFF0001,<br/>[TEEC_ERROR_CANCEL](_tee_client.md) = 0xFFFF0002, [TEEC_ERROR_ACCESS_CONFLICT](_tee_client.md) = 0xFFFF0003, [TEEC_ERROR_EXCESS_DATA](_tee_client.md) = 0xFFFF0004, [TEEC_ERROR_BAD_FORMAT](_tee_client.md) = 0xFFFF0005,<br/>[TEEC_ERROR_BAD_PARAMETERS](_tee_client.md) = 0xFFFF0006, [TEEC_ERROR_BAD_STATE](_tee_client.md) = 0xFFFF0007, [TEEC_ERROR_ITEM_NOT_FOUND](_tee_client.md) = 0xFFFF0008, [TEEC_ERROR_NOT_IMPLEMENTED](_tee_client.md) = 0xFFFF0009,<br/>[TEEC_ERROR_NOT_SUPPORTED](_tee_client.md) = 0xFFFF000A, [TEEC_ERROR_NO_DATA](_tee_client.md) = 0xFFFF000B, [TEEC_ERROR_OUT_OF_MEMORY](_tee_client.md) = 0xFFFF000C, [TEEC_ERROR_BUSY](_tee_client.md) = 0xFFFF000D,<br/>[TEEC_ERROR_COMMUNICATION](_tee_client.md) = 0xFFFF000E, [TEEC_ERROR_SECURITY](_tee_client.md) = 0xFFFF000F, [TEEC_ERROR_SHORT_BUFFER](_tee_client.md) = 0xFFFF0010, [TEEC_ERROR_MAC_INVALID](_tee_client.md) = 0xFFFF3071,<br/>[TEEC_ERROR_TARGET_DEAD](_tee_client.md) = 0xFFFF3024, [TEEC_FAIL](_tee_client.md) = 0xFFFF5002<br/>} | 定义函数返回的错误码。  | 
| [TEEC_ReturnCodeOrigin](_tee_client.md#teec_returncodeorigin) { [TEEC_ORIGIN_API](_tee_client.md) = 0x1, [TEEC_ORIGIN_COMMS](_tee_client.md) = 0x2, [TEEC_ORIGIN_TEE](_tee_client.md) = 0x3, [TEEC_ORIGIN_TRUSTED_APP](_tee_client.md) = 0x4 } | 定义函数返回错误码的来源。  | 
| [TEEC_SharedMemCtl](_tee_client.md#teec_sharedmemctl) { [TEEC_MEM_INPUT](_tee_client.md) = 0x1, [TEEC_MEM_OUTPUT](_tee_client.md) = 0x2, [TEEC_MEM_INOUT](_tee_client.md) = 0x3 } | 定义共享内存标识。  | 
| [TEEC_ParamType](_tee_client.md#teec_paramtype) {<br/>[TEEC_NONE](_tee_client.md) = 0x0, [TEEC_VALUE_INPUT](_tee_client.md) = 0x01, [TEEC_VALUE_OUTPUT](_tee_client.md) = 0x02, [TEEC_VALUE_INOUT](_tee_client.md) = 0x03,<br/>[TEEC_MEMREF_TEMP_INPUT](_tee_client.md) = 0x05, [TEEC_MEMREF_TEMP_OUTPUT](_tee_client.md) = 0x06, [TEEC_MEMREF_TEMP_INOUT](_tee_client.md) = 0x07, [TEEC_MEMREF_WHOLE](_tee_client.md) = 0xc,<br/>[TEEC_MEMREF_PARTIAL_INPUT](_tee_client.md) = 0xd, [TEEC_MEMREF_PARTIAL_OUTPUT](_tee_client.md) = 0xe, [TEEC_MEMREF_PARTIAL_INOUT](_tee_client.md) = 0xf<br/>} | 定义参数类型。  | 
| [TEEC_LoginMethod](_tee_client.md#teec_loginmethod) {<br/>[TEEC_LOGIN_PUBLIC](_tee_client.md) = 0x0, [TEEC_LOGIN_USER](_tee_client.md), [TEEC_LOGIN_GROUP](_tee_client.md), [TEEC_LOGIN_APPLICATION](_tee_client.md) = 0x4,<br/>[TEEC_LOGIN_USER_APPLICATION](_tee_client.md) = 0x5, [TEEC_LOGIN_GROUP_APPLICATION](_tee_client.md) = 0x6, [TEEC_LOGIN_IDENTIFY](_tee_client.md) = 0x7<br/>} | 定义Login方式。  | 
