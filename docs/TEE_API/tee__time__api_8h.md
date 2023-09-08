# tee_time_api.h


## 概述

安全时间接口

开发者可以使用这些接口实现安全时间相关的功能。

**起始版本：**

1

**相关模块：**

[TeeTime](_tee_time.md)


## 汇总


### 函数

| 名称 | 描述 | 
| -------- | -------- |
| [TEE_GetSystemTime](_tee_time.md#tee_getsystemtime) (TEE_Time \*time) | 获取当前TEE系统时间 | 
| [TEE_Wait](_tee_time.md#tee_wait) (uint32_t timeout) | 等待指定的毫秒数 | 
| [TEE_GetTAPersistentTime](_tee_time.md#tee_gettapersistenttime) (TEE_Time \*time) | 检索受信任应用程序的持久时间 | 
| [TEE_SetTAPersistentTime](_tee_time.md#tee_settapersistenttime) (TEE_Time \*time) | 设置当前受信任应用程序的持久化时间 | 
| [TEE_GetREETime](_tee_time.md#tee_getreetime) (TEE_Time \*time) | 获取当前REE系统时间 | 
