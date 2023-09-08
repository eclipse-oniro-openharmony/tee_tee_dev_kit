# TEEC_Session


## 概述

描述客户端应用与安全世界之间建立的会话。

**起始版本：**

11

**相关模块：**

[TeeClient](_tee_client.md)


## 汇总


### 成员变量

| 名称 | 描述 | 
| -------- | -------- |
| **session_id** |  | 
| **service_id** |  | 
| **ops_cnt** |  | 
|  | union { | 
|  | struct [ListNode](_list_node.md)**head** | 
|  | uint64_t   **imp** | 
|  | }; | 
| **context** |  | 
