# TEE_OperationInfoKey


## 概述

Operation中存放的密钥信息

**起始版本：**

1

**相关模块：**

[TeeCrypto](_tee_crypto.md)


## 汇总


### 成员变量

| 名称 | 描述 | 
| -------- | -------- |
| [keySize](#keysize) | 密钥长度 | 
| [requiredKeyUsage](#requiredkeyusage) | 所需密钥用法 | 


## 结构体成员变量说明


### keySize

```
uint32_t TEE_OperationInfoKey::keySize
```

**描述：**

密钥长度


### requiredKeyUsage

```
uint32_t TEE_OperationInfoKey::requiredKeyUsage
```

**描述：**

所需密钥用法
