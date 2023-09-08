# __TEE_OperationInfo


## 概述

Operation信息

**起始版本：**

1

**相关模块：**

[TeeCrypto](_tee_crypto.md)


## 汇总


### 成员变量

| 名称 | 描述 | 
| -------- | -------- |
| [algorithm](#algorithm) | 算法ID | 
| [operationClass](#operationclass) | operation类型 | 
| [mode](#mode) | Operation模式 | 
| [digestLength](#digestlength) | 摘要长度 | 
| [maxKeySize](#maxkeysize) | 最大密钥长度 | 
| [keySize](#keysize) | 密钥长度 | 
| [requiredKeyUsage](#requiredkeyusage) | 所需密钥用法 | 
| [handleState](#handlestate) | 句柄状态 | 
| [keyValue](#keyvalue) | 密钥 | 


## 结构体成员变量说明


### algorithm

```
uint32_t __TEE_OperationInfo::algorithm
```

**描述：**

算法ID


### digestLength

```
uint32_t __TEE_OperationInfo::digestLength
```

**描述：**

摘要长度


### handleState

```
uint32_t __TEE_OperationInfo::handleState
```

**描述：**

句柄状态


### keySize

```
uint32_t __TEE_OperationInfo::keySize
```

**描述：**

密钥长度


### keyValue

```
void* __TEE_OperationInfo::keyValue
```

**描述：**

密钥


### maxKeySize

```
uint32_t __TEE_OperationInfo::maxKeySize
```

**描述：**

最大密钥长度


### mode

```
uint32_t __TEE_OperationInfo::mode
```

**描述：**

Operation模式


### operationClass

```
uint32_t __TEE_OperationInfo::operationClass
```

**描述：**

operation类型


### requiredKeyUsage

```
uint32_t __TEE_OperationInfo::requiredKeyUsage
```

**描述：**

所需密钥用法
