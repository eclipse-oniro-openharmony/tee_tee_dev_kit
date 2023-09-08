# tee_crypto_hal.h


## 概述

加解密接口

开发者可以使用这些接口实现加解密的相关功能。

**起始版本：**

1

**相关模块：**

[TeeCrypto](_tee_crypto.md)


## 汇总


### 枚举

| 名称 | 描述 | 
| -------- | -------- |
| CRYPTO_ENGINE { **SOFT_CRYPTO** = 2, **CRYPTO_ENGINE_MAX** = 1024 } | 加解密引擎类型 | 


### 函数

| 名称 | 描述 | 
| -------- | -------- |
| [TEE_SetCryptoFlag](_tee_crypto.md#tee_setcryptoflag) ([TEE_OperationHandle](_tee_crypto.md#tee_operationhandle) operation, uint32_t crypto) | 将加密和解密引擎设置为运行 | 
| [TEE_SetObjectFlag](_tee_crypto.md#tee_setobjectflag) (TEE_ObjectHandle object, uint32_t crypto) | 设置加解密引擎为object | 
