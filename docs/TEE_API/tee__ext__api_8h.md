# tee_ext_api.h


## 概述

扩展接口

**起始版本：**

1

**相关模块：**

[TeeExt](_tee_ext.md)


## 汇总


### 函数

| 名称 | 描述 | 
| -------- | -------- |
| [AddCaller_CA_exec](_tee_ext.md#addcaller_ca_exec) (constchar \*ca_name, uint32_t ca_uid) | TA可以调用此API添加调用者信息，允许调用此TA。此API用于CA，以二进制可执行文件的形式 | 
| [TEE_GetSessionType](_tee_ext.md#tee_getsessiontype) (void) | 获取当前会话类型 | 
