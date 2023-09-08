# TEE_OperationInfoMultiple


## 概述

包含了Operation中的密钥信息

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
| [handleState](#handlestate) | 句柄状态 | 
| [operationState](#operationstate) | operation状态 | 
| [numberOfKeys](#numberofkeys) | 密钥数量 | 
| [keyInformation](#keyinformation) [] | 密钥信息 | 


## 结构体成员变量说明


### algorithm

```
uint32_t TEE_OperationInfoMultiple::algorithm
```

**描述：**

算法ID


### digestLength

```
uint32_t TEE_OperationInfoMultiple::digestLength
```

**描述：**

摘要长度


### handleState

```
uint32_t TEE_OperationInfoMultiple::handleState
```

**描述：**

句柄状态


### keyInformation

```
TEE_OperationInfoKey TEE_OperationInfoMultiple::keyInformation[]
```

**描述：**

密钥信息


### maxKeySize

```
uint32_t TEE_OperationInfoMultiple::maxKeySize
```

**描述：**

最大密钥长度


### mode

```
uint32_t TEE_OperationInfoMultiple::mode
```

**描述：**

Operation模式


### numberOfKeys

```
uint32_t TEE_OperationInfoMultiple::numberOfKeys
```

**描述：**

密钥数量


### operationClass

```
uint32_t TEE_OperationInfoMultiple::operationClass
```

**描述：**

operation类型


### operationState

```
uint32_t TEE_OperationInfoMultiple::operationState
```

**描述：**

operation状态
