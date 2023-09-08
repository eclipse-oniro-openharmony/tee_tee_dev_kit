# TEEC_Context


## 概述

描述客户端应用与安全世界之间建立的连接环境。

**起始版本：**

11

**相关模块：**

[TeeClient](_tee_client.md)


## 汇总


### 成员变量

| 名称 | 描述 | 
| -------- | -------- |
| **fd** |  | 
| **ta_path** |  | 
| **session_list** |  | 
| **shrd_mem_list** |  | 
|  | union { | 
|  | struct { | 
|  | void \*   **buffer** | 
|  | sem_t   **buffer_barrier** | 
|  | }   **share_buffer** | 
|  | uint64_t   **imp** | 
|  | }; | 
