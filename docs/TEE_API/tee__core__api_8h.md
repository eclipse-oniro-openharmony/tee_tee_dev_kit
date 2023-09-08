# tee_core_api.h


## 概述

TA会话操作接口

**起始版本：**

1

**相关模块：**

[TeeCore](_tee_core.md)


## 汇总


### 函数

| 名称 | 描述 | 
| -------- | -------- |
| [TEE_Panic](_tee_core.md#tee_panic) (TEE_Result panicCode) | 在受信任的应用程序实例中引发死机 | 
| [TEE_OpenTASession](_tee_core.md#tee_opentasession) (const TEE_UUID \*destination, uint32_t cancellationRequestTimeout, uint32_t paramTypes, TEE_Param params[TEE_PARAMS_NUM], TEE_TASessionHandle \*session, uint32_t \*returnOrigin) | 使用受信任应用程序打开新会话 | 
| [TEE_CloseTASession](_tee_core.md#tee_closetasession) (TEE_TASessionHandle session) | 关闭由TEE_OpenTASession打开的客户端会话 | 
| [TEE_InvokeTACommand](_tee_core.md#tee_invoketacommand) (TEE_TASessionHandle session, uint32_t cancellationRequestTimeout, uint32_t commandID, uint32_t paramTypes, TEE_Param params[TEE_PARAMS_NUM], uint32_t \*returnOrigin) | 在客户端受信任应用程序实例和目标受信任应用程序实例之间打开的会话中调用命令 | 


### 变量

| 名称 | 描述 | 
| -------- | -------- |
| **TEE_TASessionHandle** |  | 
