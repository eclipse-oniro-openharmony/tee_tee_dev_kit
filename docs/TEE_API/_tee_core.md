# TeeCore


## 概述

TeeCore提供了一组TA会话操作接口。

开发者可以通过使用这些接口在TA中完成执行会话相关的命令、以及引起死机等。

**起始版本：**

1


## 汇总


### 文件

| 名称 | 描述 | 
| -------- | -------- |
| [tee_core_api.h](tee__core__api_8h.md) | TA会话操作接口 | 


### 函数

| 名称 | 描述 | 
| -------- | -------- |
| [TEE_Panic](#tee_panic) (TEE_Result panicCode) | 在受信任的应用程序实例中引发死机 | 
| [TEE_OpenTASession](#tee_opentasession) (const TEE_UUID \*destination, uint32_t cancellationRequestTimeout, uint32_t paramTypes, TEE_Param params[TEE_PARAMS_NUM], TEE_TASessionHandle \*session, uint32_t \*returnOrigin) | 使用受信任应用程序打开新会话 | 
| [TEE_CloseTASession](#tee_closetasession) (TEE_TASessionHandle session) | 关闭由TEE_OpenTASession打开的客户端会话 | 
| [TEE_InvokeTACommand](#tee_invoketacommand) (TEE_TASessionHandle session, uint32_t cancellationRequestTimeout, uint32_t commandID, uint32_t paramTypes, TEE_Param params[TEE_PARAMS_NUM], uint32_t \*returnOrigin) | 在客户端受信任应用程序实例和目标受信任应用程序实例之间打开的会话中调用命令 | 


### 变量

| 名称 | 描述 | 
| -------- | -------- |
| **TEE_TASessionHandle** |  | 


## 函数说明


### TEE_CloseTASession()

```
void TEE_CloseTASession (TEE_TASessionHandle session)
```

**描述：**

关闭由TEE_OpenTASession打开的客户端会话

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| session | [IN]TEE_OpenTASession打开的会话句柄 | 

**起始版本：**

1


### TEE_InvokeTACommand()

```
TEE_Result TEE_InvokeTACommand (TEE_TASessionHandle session, uint32_t cancellationRequestTimeout, uint32_t commandID, uint32_t paramTypes, TEE_Param params[TEE_PARAMS_NUM], uint32_t * returnOrigin )
```

**描述：**

在客户端受信任应用程序实例和目标受信任应用程序实例之间打开的会话中调用命令

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| session | [IN]打开的会话句柄 | 
| cancellationRequestTimeout | [IN]以毫秒为单位的超时或特殊值 | 
| commandID | [IN]要调用的命令的标识符 | 
| paramTypes | [IN]操作中传递的所有参数的类型 | 
| params | [IN]操作中传递的参数 | 
| returnOrigin | [IN]指向将包含返回原点的变量的指针 | 

**返回：**

TEE_SUCCESS 调用操作成功

TEE_ERROR_ACCESS_DENIED 向目标TA调用命令被拒绝

**起始版本：**

1


### TEE_OpenTASession()

```
TEE_Result TEE_OpenTASession (const TEE_UUID * destination, uint32_t cancellationRequestTimeout, uint32_t paramTypes, TEE_Param params[TEE_PARAMS_NUM], TEE_TASessionHandle * session, uint32_t * returnOrigin )
```

**描述：**

使用受信任应用程序打开新会话

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| destination | [IN]指向包含目标受信任应用程序的UUID的TEE_UUID结构的指针 | 
| cancellationRequestTimeout | [IN]以毫秒为单位的超时或特殊值 | 
| paramTypes | [IN]操作中传递的所有参数的类型 | 
| params | [IN]操作中传递的参数 | 
| session | [OUT]指向将接收客户端会话句柄的变量的指针 | 
| returnOrigin | [OUT]指向将包含返回原点的变量的指针 | 

**返回：**

TEE_SUCCESS 成功打开会话

TEE_ERROR_ITEM_NOT_FOUND 在TEE中找不到目标TA

TEE_ERROR_ACCESS_DENIED 对目标受信任应用程序的访问被拒绝

**起始版本：**

1


### TEE_Panic()

```
void TEE_Panic (TEE_Result panicCode)
```

**描述：**

在受信任的应用程序实例中引发死机

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| panicCode | [IN]TA定义的信息性恐慌代码 | 

**起始版本：**

1
