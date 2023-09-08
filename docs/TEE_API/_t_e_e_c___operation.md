# TEEC_Operation


## 概述

打开会话或发送命令时的参数。

**起始版本：**

11

**相关模块：**

[TeeClient](_tee_client.md)


## 汇总


### 成员变量

| 名称 | 描述 | 
| -------- | -------- |
| [started](#started) | 0 代表取消该命令，其他表示执行该命令 | 
| [paramTypes](#paramtypes) | 使用 | 
| **params** [[TEEC_PARAM_NUM](_tee_client.md#teec_param_num)] |  | 
| **session** |  | 
| **cancel_flag** |  | 


## 结构体成员变量说明


### paramTypes

```
uint32_t TEEC_Operation::paramTypes
```

**Value:**

```
TEEC_PARAM_TYPES 
```

**描述：**

使用

来创建该参数


### started

```
uint32_t TEEC_Operation::started
```

**描述：**

0 代表取消该命令，其他表示执行该命令
