# tee_client_api.h


## 概述

客户端应用访问安全应用相关接口定义。

使用示例：

1.初始化TEE环境：调用TEEC_InitializeContext初始化TEE环境；

2.打开会话：调用TEEC_OpenSession，参数为安全应用TA的UUID；

3.发送命令：调用TEEC_InvokeCommand向安全应用发送命令；

4.关闭会话：调用接口TEEC_CloseSession，关闭会话；

5.关闭TEE环境：调用接口TEEC_FinalizeContext，关闭TEE环境。

**起始版本：**

11

**相关模块：**

[TeeClient](_tee_client.md)


## 汇总


### 函数

| 名称 | 描述 | 
| -------- | -------- |
| [TEEC_InitializeContext](_tee_client.md#teec_initializecontext) (const char \*name, [TEEC_Context](_t_e_e_c___context.md) \*context) | 初始化TEE环境。 | 
| [TEEC_FinalizeContext](_tee_client.md#teec_finalizecontext) ([TEEC_Context](_t_e_e_c___context.md) \*context) | 关闭TEE环境。 | 
| [TEEC_OpenSession](_tee_client.md#teec_opensession) ([TEEC_Context](_t_e_e_c___context.md) \*context, [TEEC_Session](_t_e_e_c___session.md) \*session, const [TEEC_UUID](_t_e_e_c___u_u_i_d.md) \*destination, uint32_t connectionMethod, const void \*connectionData, [TEEC_Operation](_t_e_e_c___operation.md) \*operation, uint32_t \*returnOrigin) | 打开会话。 | 
| [TEEC_CloseSession](_tee_client.md#teec_closesession) ([TEEC_Session](_t_e_e_c___session.md) \*session) | 关闭会话。 | 
| [TEEC_InvokeCommand](_tee_client.md#teec_invokecommand) ([TEEC_Session](_t_e_e_c___session.md) \*session, uint32_t commandID, [TEEC_Operation](_t_e_e_c___operation.md) \*operation, uint32_t \*returnOrigin) | 发送命令。 | 
| [TEEC_RegisterSharedMemory](_tee_client.md#teec_registersharedmemory) ([TEEC_Context](_t_e_e_c___context.md) \*context, [TEEC_SharedMemory](_t_e_e_c___shared_memory.md) \*sharedMem) | 注册共享内存。 | 
| [TEEC_AllocateSharedMemory](_tee_client.md#teec_allocatesharedmemory) ([TEEC_Context](_t_e_e_c___context.md) \*context, [TEEC_SharedMemory](_t_e_e_c___shared_memory.md) \*sharedMem) | 申请共享内存。 | 
| [TEEC_ReleaseSharedMemory](_tee_client.md#teec_releasesharedmemory) ([TEEC_SharedMemory](_t_e_e_c___shared_memory.md) \*sharedMem) | 释放共享内存。 | 
| [TEEC_RequestCancellation](_tee_client.md#teec_requestcancellation) ([TEEC_Operation](_t_e_e_c___operation.md) \*operation) | 取消正在运行的操作。 | 
