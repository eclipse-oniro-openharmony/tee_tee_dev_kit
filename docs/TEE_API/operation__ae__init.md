# operation_ae_init


## 概述

存放ae算法初始化相关数据

**起始版本：**

1

**相关模块：**

[TeeCrypto](_tee_crypto.md)


## 汇总


### 成员变量

| 名称 | 描述 | 
| -------- | -------- |
| [nonce](#nonce) | nonce | 
| [nonce_len](#nonce_len) | nonce长度 | 
| [tag_len](#tag_len) | tag长度 | 
| [aad_len](#aad_len) | aad长度 | 
| [payload_len](#payload_len) | payload长度 | 


## 结构体成员变量说明


### aad_len

```
size_t operation_ae_init::aad_len
```

**描述：**

aad长度


### nonce

```
void* operation_ae_init::nonce
```

**描述：**

nonce


### nonce_len

```
size_t operation_ae_init::nonce_len
```

**描述：**

nonce长度


### payload_len

```
size_t operation_ae_init::payload_len
```

**描述：**

payload长度


### tag_len

```
uint32_t operation_ae_init::tag_len
```

**描述：**

tag长度
