# tee_client_type.h


## 概述

基本数据类型和数据结构定义。

**起始版本：**

11

**相关模块：**

[TeeClient](_tee_client.md)


## 汇总


### 结构体

| 名称 | 描述 | 
| -------- | -------- |
| [ListNode](_list_node.md) | 链表类型定义。 | 
| [TEEC_UUID](_t_e_e_c___u_u_i_d.md) | TEEC_UUID类型定义，遵循RFC4122 [2]，用于标识安全应用。 | 
| [TEEC_Context](_t_e_e_c___context.md) | 描述客户端应用与安全世界之间建立的连接环境。 | 
| [TEEC_Session](_t_e_e_c___session.md) | 描述客户端应用与安全世界之间建立的会话。 | 
| [TEEC_SharedMemory](_t_e_e_c___shared_memory.md) | 描述一块共享内存，可以注册，也可以分配。 | 
| [TEEC_TempMemoryReference](_t_e_e_c___temp_memory_reference.md) | 描述一块临时缓冲区指针。 | 
| [TEEC_RegisteredMemoryReference](_t_e_e_c___registered_memory_reference.md) | 描述共享内存指针，指向事先注册或分配好的共享内存。 | 
| [TEEC_Value](_t_e_e_c___value.md) | 描述少量数据。 | 
| [TEEC_Parameter](union_t_e_e_c___parameter.md) | 描述[TEEC_Operation](_t_e_e_c___operation.md)所对应的参数类型。 | 
| [TEEC_Operation](_t_e_e_c___operation.md) | 打开会话或发送命令时的参数。 | 


### 类型定义

| 名称 | 描述 | 
| -------- | -------- |
| [TEEC_Result](_tee_client.md#teec_result) | 函数返回值类型定义。 | 
