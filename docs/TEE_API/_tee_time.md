# TeeTime


## 概述

TeeTime提供了一组安全时间接口。

开发者可以使用这些接口实现安全时间相关的功能。

**起始版本：**

1


## 汇总


### 文件

| 名称 | 描述 | 
| -------- | -------- |
| [tee_time_api.h](tee__time__api_8h.md) | 安全时间接口 | 


### 函数

| 名称 | 描述 | 
| -------- | -------- |
| [TEE_GetSystemTime](#tee_getsystemtime) (TEE_Time \*time) | 获取当前TEE系统时间 | 
| [TEE_Wait](#tee_wait) (uint32_t timeout) | 等待指定的毫秒数 | 
| [TEE_GetTAPersistentTime](#tee_gettapersistenttime) (TEE_Time \*time) | 检索受信任应用程序的持久时间 | 
| [TEE_SetTAPersistentTime](#tee_settapersistenttime) (TEE_Time \*time) | 设置当前受信任应用程序的持久化时间 | 
| [TEE_GetREETime](#tee_getreetime) (TEE_Time \*time) | 获取当前REE系统时间 | 


## 函数说明


### TEE_GetREETime()

```
void TEE_GetREETime (TEE_Time * time)
```

**描述：**

获取当前REE系统时间

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| time | [OUT]当前REE系统时间 | 

**起始版本：**

1


### TEE_GetSystemTime()

```
void TEE_GetSystemTime (TEE_Time * time)
```

**描述：**

获取当前TEE系统时间

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| time | [OUT]当前系统时间 | 

**起始版本：**

1


### TEE_GetTAPersistentTime()

```
TEE_Result TEE_GetTAPersistentTime (TEE_Time * time)
```

**描述：**

检索受信任应用程序的持久时间

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| time | [IN]受信任应用程序的持久时间 | 

**返回：**

TEE_SUCCESS 成功

TEE_ERROR_TIME_NOT_SET 持久时间尚未设置

TEE_ERROR_TIME_NEEDS_RESET 永久时间已设置，但可能已损坏，不得再信任

TEE_ERROR_OVERFLOW TA持续时间中的秒数超过了uint32_t的范围

TEE_ERROR_OUT_OF_MEMORY 没有足够的内存来完成操作

**起始版本：**

1


### TEE_SetTAPersistentTime()

```
TEE_Result TEE_SetTAPersistentTime (TEE_Time * time)
```

**描述：**

设置当前受信任应用程序的持久化时间

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| time | [IN]受信任应用程序的持久时间 | 

**返回：**

TEE_SUCCESS 成功

TEE_ERROR_OUT_OF_MEMORY 没有足够的内存来完成操作

TEE_ERROR_STORAGE_NO_SPACE 没有足够的存储空间来完成操作

**起始版本：**

1


### TEE_Wait()

```
TEE_Result TEE_Wait (uint32_t timeout)
```

**描述：**

等待指定的毫秒数

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| timeout | [IN]指定的毫秒数 | 

**返回：**

TEE_SUCCESS 成功

TEE_ERROR_CANCEL 等待已取消

TEE_ERROR_OUT_OF_MEMORY 没有足够的内存来完成操作

**起始版本：**

1
