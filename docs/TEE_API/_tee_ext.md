# TeeExt


## 概述

TeeExt提供了一组扩展接口。

开发者可以使用这些接口实现获取当前会话类型、添加信息等扩展功能。

**起始版本：**

1


## 汇总


### 文件

| 名称 | 描述 | 
| -------- | -------- |
| [tee_ext_api.h](tee__ext__api_8h.md) | 扩展接口 | 
| [tee_hw_ext_api.h](tee__hw__ext__api_8h.md) | 扩展接口 | 


### 函数

| 名称 | 描述 | 
| -------- | -------- |
| [AddCaller_CA_exec](#addcaller_ca_exec) (constchar \*ca_name, uint32_t ca_uid) | TA可以调用此API添加调用者信息，允许调用此TA。此API用于CA，以二进制可执行文件的形式 | 
| [TEE_GetSessionType](#tee_getsessiontype) (void) | 获取当前会话类型 | 
| [TEE_EXT_GetDeviceUniqueId](#tee_ext_getdeviceuniqueid) (uint8_t \*device_unique_id, uint32_t \*length) | 在TEE中获取设备唯一ID | 


## 函数说明


### AddCaller_CA_exec()

```
TEE_Result AddCaller_CA_exec (constchar * ca_name, uint32_t ca_uid )
```

**描述：**

TA可以调用此API添加调用者信息，允许调用此TA。此API用于CA，以二进制可执行文件的形式

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| ca_name | [IN]CA调用方的进程名称 | 
| ca_uid | [IN]CA调用方的uid | 

**返回：**

TEE_SUCCESS 操作成功

others 无法为目标CA添加主叫方信息

**起始版本：**

1


### TEE_EXT_GetDeviceUniqueId()

```
TEE_Result TEE_EXT_GetDeviceUniqueId (uint8_t * device_unique_id, uint32_t * length )
```

**描述：**

在TEE中获取设备唯一ID

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| device_unique_id | [IN]用于存储结果的缓冲区 | 
| length | [IN/OUT]设备ID缓冲区长度 | 

**返回：**

TEE_SUCCESS 操作成功

others 操作失败

**起始版本：**

1


### TEE_GetSessionType()

```
uint32_t TEE_GetSessionType (void )
```

**描述：**

获取当前会话类型

**返回：**

当前会话的会话类型

**起始版本：**

1
