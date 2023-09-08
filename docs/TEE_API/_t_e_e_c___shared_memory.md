# TEEC_SharedMemory


## 概述

描述一块共享内存，可以注册，也可以分配。

**起始版本：**

11

**相关模块：**

[TeeClient](_tee_client.md)


## 汇总


### 成员变量

| 名称 | 描述 | 
| -------- | -------- |
| **buffer** |  | 
| **size** |  | 
| **flags** |  | 
| **ops_cnt** |  | 
| **is_allocated** |  | 
|  | union { | 
|  | struct [ListNode](_list_node.md)**head** | 
|  | void \*   **imp** | 
|  | }; | 
| **context** |  | 
