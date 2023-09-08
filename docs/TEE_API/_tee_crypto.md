# TeeCrypto


## 概述

TeeCrypto提供了一组加解密接口。

开发者可以使用这些接口实现加解密的相关功能。

**起始版本：**

1


## 汇总


### 文件

| 名称 | 描述 | 
| -------- | -------- |
| [tee_crypto_api.h](tee__crypto__api_8h.md) | 加解密接口 | 
| [tee_crypto_hal.h](tee__crypto__hal_8h.md) | 加解密接口 | 


### 结构体

| 名称 | 描述 | 
| -------- | -------- |
| [__TEE_OperationInfo](_____t_e_e___operation_info.md) | Operation信息 | 
| [TEE_OperationInfoKey](_t_e_e___operation_info_key.md) | Operation中存放的密钥信息 | 
| [TEE_OperationInfoMultiple](_t_e_e___operation_info_multiple.md) | 包含了Operation中的密钥信息 | 
| [__TEE_OperationHandle](_____t_e_e___operation_handle.md) | 加解密操作需要的句柄 | 
| [crypto_uint2uint](crypto__uint2uint.md) | 整数类型转换 | 
| [operation_src_dest](operation__src__dest.md) | 存放输入输出数据 | 
| [operation_ae_init](operation__ae__init.md) | 存放ae算法初始化相关数据 | 


### 宏定义

| 名称 | 描述 | 
| -------- | -------- |
| [NULL](#null)   ((void \*)0) | NULL定义 | 
| TEE_MAX_KEY_SIZE_IN_BITS   (1024 \* 8) | 密钥最大长度（以bits为单位） | 
| SW_RSA_KEYLEN   1024 | SW_RSA密钥长度 | 
| TEE_DH_MAX_SIZE_OF_OTHER_INFO   64 /\* bytes \*/ | DH其它信息的最大长度 | 
| TEE_OPTIONAL_ELEMENT_NONE   0x00000000 | 用于给alg_config_t结构体中element成员赋值，表示不需要曲线参数 | 
| RSA_PUBKEY_MAXSIZE   sizeof(CRYS_RSAUserPubKey_t) | RSA公钥最大长度 | 
| RSA_PRIVKEY_MAXSIZE   sizeof(CRYS_RSAUserPrivKey_t) | RES私钥最大长度 | 


### 类型定义

| 名称 | 描述 | 
| -------- | -------- |
| [tee_crypto_algorithm_id](#tee_crypto_algorithm_id) |  | 
| [TEE_OperationMode](#tee_operationmode) |  | 
| [TEE_OperationInfo](#tee_operationinfo) | 用于定义__TEE_OperationInfo结构体类型 | 
| [TEE_OperationHandle](#tee_operationhandle) | 用于定义__TEE_OperationHandle指针类型 | 
| [TEE_OperationHandleVar](#tee_operationhandlevar) | 用于定义__TEE_OperationHandle结构体类型 | 
| TEE_ObjectHandleVar | 用于定义__TEE_ObjectHandle结构体类型 | 


### 枚举

| 名称 | 描述 | 
| -------- | -------- |
| [__TEE_Operation_Constants](#__tee_operation_constants) {<br/>TEE_OPERATION_CIPHER = 0x1, TEE_OPERATION_MAC = 3, TEE_OPERATION_AE = 4, TEE_OPERATION_DIGEST = 5,<br/>TEE_OPERATION_ASYMMETRIC_CIPHER = 6, TEE_OPERATION_ASYMMETRIC_SIGNATURE = 7, TEE_OPERATION_KEY_DERIVATION = 8<br/>} | 加解密Operation操作句柄 | 
| [__tee_crypto_algorithm_id](#__tee_crypto_algorithm_id) {<br/>TEE_ALG_INVALID = 0x0, TEE_ALG_AES_ECB_NOPAD = 0x10000010, TEE_ALG_AES_CBC_NOPAD = 0x10000110, TEE_ALG_AES_CTR = 0x10000210,<br/>TEE_ALG_AES_CTS = 0x10000310, TEE_ALG_AES_XTS = 0x10000410, TEE_ALG_AES_CBC_MAC_NOPAD = 0x30000110, TEE_ALG_AES_CBC_MAC_PKCS5 = 0x30000510,<br/>TEE_ALG_AES_CMAC = 0x30000610, TEE_ALG_AES_GMAC = 0x30000810, TEE_ALG_AES_CCM = 0x40000710, TEE_ALG_AES_GCM = 0x40000810,<br/>TEE_ALG_DES_ECB_NOPAD = 0x10000011, TEE_ALG_DES_CBC_NOPAD = 0x10000111, TEE_ALG_DES_CBC_MAC_NOPAD = 0x30000111, TEE_ALG_DES_CBC_MAC_PKCS5 = 0x30000511,<br/>TEE_ALG_DES3_ECB_NOPAD = 0x10000013, TEE_ALG_DES3_CBC_NOPAD = 0x10000113, TEE_ALG_DES3_CBC_MAC_NOPAD = 0x30000113, TEE_ALG_DES3_CBC_MAC_PKCS5 = 0x30000513,<br/>TEE_ALG_RSASSA_PKCS1_V1_5_MD5 = 0x70001830, TEE_ALG_RSASSA_PKCS1_V1_5_SHA1 = 0x70002830, TEE_ALG_RSASSA_PKCS1_V1_5_SHA224 = 0x70003830, TEE_ALG_RSASSA_PKCS1_V1_5_SHA256 = 0x70004830,<br/>TEE_ALG_RSASSA_PKCS1_V1_5_SHA384 = 0x70005830, TEE_ALG_RSASSA_PKCS1_V1_5_SHA512 = 0x70006830, TEE_ALG_RSASSA_PKCS1_PSS_MGF1_MD5 = 0x70111930, TEE_ALG_RSASSA_PKCS1_PSS_MGF1_SHA1 = 0x70212930,<br/>TEE_ALG_RSASSA_PKCS1_PSS_MGF1_SHA224 = 0x70313930, TEE_ALG_RSASSA_PKCS1_PSS_MGF1_SHA256 = 0x70414930, TEE_ALG_RSASSA_PKCS1_PSS_MGF1_SHA384 = 0x70515930, TEE_ALG_RSASSA_PKCS1_PSS_MGF1_SHA512 = 0x70616930,<br/>TEE_ALG_RSAES_PKCS1_V1_5 = 0x60000130, TEE_ALG_RSAES_PKCS1_OAEP_MGF1_SHA1 = 0x60210230, TEE_ALG_RSAES_PKCS1_OAEP_MGF1_SHA224 = 0x60211230, TEE_ALG_RSAES_PKCS1_OAEP_MGF1_SHA256 = 0x60212230,<br/>TEE_ALG_RSAES_PKCS1_OAEP_MGF1_SHA384 = 0x60213230, TEE_ALG_RSAES_PKCS1_OAEP_MGF1_SHA512 = 0x60214230, TEE_ALG_RSA_NOPAD = 0x60000030, TEE_ALG_DSA_SHA1 = 0x70002131,<br/>TEE_ALG_DSA_SHA224 = 0x70003131, TEE_ALG_DSA_SHA256 = 0x70004131, TEE_ALG_DH_DERIVE_SHARED_SECRET = 0x80000032, TEE_ALG_MD5 = 0x50000001,<br/>TEE_ALG_SHA1 = 0x50000002, TEE_ALG_SHA224 = 0x50000003, TEE_ALG_SHA256 = 0x50000004, TEE_ALG_SHA384 = 0x50000005,<br/>TEE_ALG_SHA512 = 0x50000006, TEE_ALG_HMAC_MD5 = 0x30000001, TEE_ALG_HMAC_SHA1 = 0x30000002, TEE_ALG_HMAC_SHA224 = 0x30000003,<br/>TEE_ALG_HMAC_SHA256 = 0x30000004, TEE_ALG_HMAC_SHA384 = 0x30000005, TEE_ALG_HMAC_SHA512 = 0x30000006, TEE_ALG_HMAC_SM3 = 0x30000007,<br/>TEE_ALG_AES_ECB_PKCS5 = 0x10000020, TEE_ALG_AES_CBC_PKCS5 = 0x10000220, TEE_ALG_ECDSA_SHA1 = 0x70001042, TEE_ALG_ECDSA_SHA224 = 0x70002042,<br/>TEE_ALG_ECDSA_SHA256 = 0x70003042, TEE_ALG_ECDSA_SHA384 = 0x70004042, TEE_ALG_ECDSA_SHA512 = 0x70005042, TEE_ALG_ED25519 = 0x70005043,<br/>TEE_ALG_ECDH_DERIVE_SHARED_SECRET = 0x80000042, TEE_ALG_X25519 = 0x80000044, TEE_ALG_ECC = 0x80000001, TEE_ALG_ECDSA_P192 = 0x70001042,<br/>TEE_ALG_ECDSA_P224 = 0x70002042, TEE_ALG_ECDSA_P256 = 0x70003042, TEE_ALG_ECDSA_P384 = 0x70004042, TEE_ALG_ECDSA_P521 = 0x70005042,<br/>TEE_ALG_ECDH_P192 = 0x80001042, TEE_ALG_ECDH_P224 = 0x80002042, TEE_ALG_ECDH_P256 = 0x80003042, TEE_ALG_ECDH_P384 = 0x80004042,<br/>TEE_ALG_ECDH_P521 = 0x80005042, TEE_ALG_SM2_DSA_SM3 = 0x70006045, TEE_ALG_SM2_PKE = 0x80000045, TEE_ALG_SM3 = 0x50000007,<br/>TEE_ALG_SM4_ECB_NOPAD = 0x10000014, TEE_ALG_SM4_CBC_NOPAD = 0x10000114, TEE_ALG_SM4_CBC_PKCS7 = 0xF0000003, TEE_ALG_SM4_CTR = 0x10000214,<br/>TEE_ALG_SM4_CFB128 = 0xF0000000, TEE_ALG_SM4_XTS = 0x10000414, TEE_ALG_SM4_OFB = 0x10000514, TEE_ALG_AES_OFB = 0x10000510,<br/>TEE_ALG_SM4_GCM = 0xF0000005<br/>} | 加解密算法标识 | 
| [TEE_ECC_CURVE](#tee_ecc_curve) {<br/>TEE_ECC_CURVE_NIST_P192 = 0x00000001, TEE_ECC_CURVE_NIST_P224 = 0x00000002, TEE_ECC_CURVE_NIST_P256 = 0x00000003, TEE_ECC_CURVE_NIST_P384 = 0x00000004,<br/>TEE_ECC_CURVE_NIST_P521 = 0x00000005, TEE_ECC_CURVE_SM2 = 0x00000300, TEE_ECC_CURVE_25519 = 0x00000200<br/>} | 支持的ECC曲线 | 
| TEE_DH_HASH_Mode {<br/>**TEE_DH_HASH_SHA1_mode** = 0, **TEE_DH_HASH_SHA224_mode** = 1, **TEE_DH_HASH_SHA256_mode** = 2, **TEE_DH_HASH_SHA384_mode** = 3,<br/>**TEE_DH_HASH_SHA512_mode** = 4, **TEE_DH_HASH_NumOfModes**<br/>} | MGF1掩码函数类型 | 
| [__TEE_OperationMode](#__tee_operationmode) {<br/>TEE_MODE_ENCRYPT = 0x0, TEE_MODE_DECRYPT, TEE_MODE_SIGN, TEE_MODE_VERIFY,<br/>TEE_MODE_MAC, TEE_MODE_DIGEST, TEE_MODE_DERIVE<br/>} | 加解密算法模式 | 
| [tee_operation_state](#tee_operation_state) { TEE_OPERATION_STATE_INITIAL = 0x00000000, TEE_OPERATION_STATE_ACTIVE = 0x00000001 } | 加解密operation状态 | 
| CRYPTO_ENGINE { **SOFT_CRYPTO** = 2, **CRYPTO_ENGINE_MAX** = 1024 } | 加解密引擎类型 | 


### 函数

| 名称 | 描述 | 
| -------- | -------- |
| [TEE_AllocateOperation](#tee_allocateoperation) ([TEE_OperationHandle](#tee_operationhandle) \*operation, uint32_t algorithm, uint32_t mode, uint32_tmaxKeySize) | 申请操作句柄 | 
| [TEE_FreeOperation](#tee_freeoperation) ([TEE_OperationHandle](#tee_operationhandle) operation) | 释放操作句柄 | 
| [TEE_GetOperationInfo](#tee_getoperationinfo) (const [TEE_OperationHandle](#tee_operationhandle) operation, [TEE_OperationInfo](#tee_operationinfo) \*operationInfo) | 获取操作信息 | 
| [TEE_ResetOperation](#tee_resetoperation) ([TEE_OperationHandle](#tee_operationhandle) operation) | 复位操作句柄 | 
| [TEE_SetOperationKey](#tee_setoperationkey) ([TEE_OperationHandle](#tee_operationhandle) operation, const TEE_ObjectHandle key) | 设置操作密钥 | 
| [TEE_SetOperationKey2](#tee_setoperationkey2) ([TEE_OperationHandle](#tee_operationhandle) operation, const TEE_ObjectHandle key1, const TEE_ObjectHandle key2) | 设置操作密钥2 | 
| [TEE_CopyOperation](#tee_copyoperation) ([TEE_OperationHandle](#tee_operationhandle) dstOperation, const [TEE_OperationHandle](#tee_operationhandle) srcOperation) | 复制操作句柄 | 
| [TEE_CipherInit](#tee_cipherinit) ([TEE_OperationHandle](#tee_operationhandle) operation, constvoid \*IV, size_t IVLen) | 初始化密码上下文 | 
| [TEE_CipherUpdate](#tee_cipherupdate) ([TEE_OperationHandle](#tee_operationhandle) operation, constvoid \*srcData, size_t srcLen, void \*destData, size_t \*destLen) | 执行密码更新 | 
| [TEE_CipherDoFinal](#tee_cipherdofinal) ([TEE_OperationHandle](#tee_operationhandle) operation, constvoid \*srcData, size_t srcLen, void \*destData, size_t \*destLen) | 执行密码完成 | 
| [TEE_DigestUpdate](#tee_digestupdate) ([TEE_OperationHandle](#tee_operationhandle) operation, constvoid \*chunk, size_t chunkSize) | 摘要更新 | 
| [TEE_DigestDoFinal](#tee_digestdofinal) ([TEE_OperationHandle](#tee_operationhandle) operation, constvoid \*chunk, size_t chunkLen, void \*hash, size_t \*hashLen) | 执行摘要结束 | 
| [TEE_MACInit](#tee_macinit) ([TEE_OperationHandle](#tee_operationhandle) operation, void \*IV, size_t IVLen) | 执行mac初始化 | 
| [TEE_MACUpdate](#tee_macupdate) ([TEE_OperationHandle](#tee_operationhandle) operation, constvoid \*chunk, size_t chunkSize) | 执行mac更新 | 
| [TEE_MACComputeFinal](#tee_maccomputefinal) ([TEE_OperationHandle](#tee_operationhandle) operation, constvoid \*message, size_t messageLen, void \*mac, size_t \*macLen) | mac计算完成 | 
| [TEE_MACCompareFinal](#tee_maccomparefinal) ([TEE_OperationHandle](#tee_operationhandle) operation, constvoid \*message, size_t messageLen, constvoid \*mac, constsize_t macLen) | mac比较完成 | 
| [voidTEE_DeriveKey](#voidtee_derivekey) ([TEE_OperationHandle](#tee_operationhandle) operation, const TEE_Attribute \*params, uint32_t paramCount, TEE_ObjectHandle derivedKey) | 派生密钥 | 
| [TEE_GenerateRandom](#tee_generaterandom) (void \*randomBuffer, size_t randomBufferLen) | 生成随机数据 | 
| [TEE_AEInit](#tee_aeinit) ([TEE_OperationHandle](#tee_operationhandle) operation, void \*nonce, size_t nonceLen, uint32_t tagLen, size_t AADLen, size_t payloadLen) | ae初始化 | 
| [TEE_AEUpdateAAD](#tee_aeupdateaad) ([TEE_OperationHandle](#tee_operationhandle) operation, constvoid \*AADdata, size_t AADdataLen) | 更新ae aad | 
| [TEE_AEUpdate](#tee_aeupdate) ([TEE_OperationHandle](#tee_operationhandle) operation, void \*srcData, size_t srcLen, void \*destData, size_t \*destLen) | 更新ae | 
| [TEE_AEEncryptFinal](#tee_aeencryptfinal) ([TEE_OperationHandle](#tee_operationhandle) operation, void \*srcData, size_t srcLen, void \*destData, size_t \*destLen, void \*tag, size_t \*tagLen) | ae加密 | 
| [TEE_AEDecryptFinal](#tee_aedecryptfinal) ([TEE_OperationHandle](#tee_operationhandle) operation, void \*srcData, size_t srcLen, void \*destData, size_t \*destLen, void \*tag, size_t tagLen) | ae解密 | 
| [TEE_AsymmetricEncrypt](#tee_asymmetricencrypt) ([TEE_OperationHandle](#tee_operationhandle) operation, const TEE_Attribute \*params, uint32_t paramCount, void \*srcData, size_t srcLen, void \*destData, size_t \*destLen) | 非对称加密 | 
| [TEE_AsymmetricDecrypt](#tee_asymmetricdecrypt) ([TEE_OperationHandle](#tee_operationhandle) operation, const TEE_Attribute \*params, uint32_t paramCount, void \*srcData, size_t srcLen, void \*destData, size_t \*destLen) | 非对称解密 | 
| [TEE_AsymmetricSignDigest](#tee_asymmetricsigndigest) ([TEE_OperationHandle](#tee_operationhandle) operation, const TEE_Attribute \*params, uint32_t paramCount, void \*digest, size_t digestLen, void \*signature, size_t \*signatureLen) | 非对称签名 | 
| [TEE_AsymmetricVerifyDigest](#tee_asymmetricverifydigest) ([TEE_OperationHandle](#tee_operationhandle) operation, const TEE_Attribute \*params, uint32_t paramCount, void \*digest, size_t digestLen, void \*signature, size_t signatureLen) | 非对称验证 | 
| [TEE_GetOperationInfoMultiple](#tee_getoperationinfomultiple) ([TEE_OperationHandle](#tee_operationhandle) operation, [TEE_OperationInfoMultiple](_t_e_e___operation_info_multiple.md) \*operationInfoMultiple, constsize_t \*operationSize) | 获取操作信息 | 
| [TEE_IsAlgorithmSupported](#tee_isalgorithmsupported) (uint32_t algId, uint32_t element) | 检查算法是否被支持 | 
| [TEE_SetCryptoFlag](#tee_setcryptoflag) ([TEE_OperationHandle](#tee_operationhandle) operation, uint32_t crypto) | 将加密和解密引擎设置为运行 | 
| [TEE_SetObjectFlag](#tee_setobjectflag) (TEE_ObjectHandle object, uint32_t crypto) | 设置加解密引擎为object | 


## 宏定义说明


### NULL

```
#define NULL   ((void *)0)
```

**描述：**

NULL定义

**起始版本：**

1


## 类型定义说明


### tee_crypto_algorithm_id

```
typedef enum __tee_crypto_algorithm_idtee_crypto_algorithm_id
```

**参见:**

[__tee_crypto_algorithm_id](#__tee_crypto_algorithm_id)

**起始版本：**

1


### TEE_OperationHandle

```
typedef struct__TEE_OperationHandle* TEE_OperationHandle
```

**描述：**

用于定义__TEE_OperationHandle指针类型

**参见:**

[__TEE_OperationHandle](_____t_e_e___operation_handle.md)

**起始版本：**

1


### TEE_OperationHandleVar

```
typedef struct __TEE_OperationHandleTEE_OperationHandleVar
```

**描述：**

用于定义__TEE_OperationHandle结构体类型

**参见:**

[__TEE_OperationHandle](_____t_e_e___operation_handle.md)

**起始版本：**

1


### TEE_OperationInfo

```
typedef struct __TEE_OperationInfoTEE_OperationInfo
```

**描述：**

用于定义__TEE_OperationInfo结构体类型

**参见:**

[__TEE_OperationInfo](_____t_e_e___operation_info.md)

**起始版本：**

1


### TEE_OperationMode

```
typedef uint32_t TEE_OperationMode
```

**参见:**

[__TEE_OperationMode](#__tee_operationmode)

**起始版本：**

1


## 枚举类型说明


### __tee_crypto_algorithm_id

```
enum __tee_crypto_algorithm_id
```

**描述：**

加解密算法标识

| 枚举值 | 描述 | 
| -------- | -------- |
| TEE_ALG_INVALID | 无效算法 | 
| TEE_ALG_AES_ECB_NOPAD | AES_ECB_NOPAD | 
| TEE_ALG_AES_CBC_NOPAD | AES_CBC_NOPAD | 
| TEE_ALG_AES_CTR | AES_CTR | 
| TEE_ALG_AES_CTS | AES_CTS | 
| TEE_ALG_AES_XTS | AES_XTS | 
| TEE_ALG_AES_CBC_MAC_NOPAD | AES_CBC_MAC_NOPAD | 
| TEE_ALG_AES_CBC_MAC_PKCS5 | AES_CBC_MAC_PKCS5 | 
| TEE_ALG_AES_CMAC | AES_CMAC | 
| TEE_ALG_AES_GMAC | AES_GMAC | 
| TEE_ALG_AES_CCM | AES_CCM | 
| TEE_ALG_AES_GCM | AES_GCM | 
| TEE_ALG_DES_ECB_NOPAD | DES_ECB_NOPAD | 
| TEE_ALG_DES_CBC_NOPAD | DES_CBC_NOPAD | 
| TEE_ALG_DES_CBC_MAC_NOPAD | DES_CBC_MAC_NOPAD | 
| TEE_ALG_DES_CBC_MAC_PKCS5 | DES_CBC_MAC_PKCS5 | 
| TEE_ALG_DES3_ECB_NOPAD | DES3_ECB_NOPAD | 
| TEE_ALG_DES3_CBC_NOPAD | DES3_CBC_NOPAD | 
| TEE_ALG_DES3_CBC_MAC_NOPAD | DES3_CBC_MAC_NOPAD | 
| TEE_ALG_DES3_CBC_MAC_PKCS5 | DES3_CBC_MAC_PKCS5 | 
| TEE_ALG_RSASSA_PKCS1_V1_5_MD5 | RSASSA_PKCS1_V1_5_MD5 | 
| TEE_ALG_RSASSA_PKCS1_V1_5_SHA1 | RSASSA_PKCS1_V1_5_SHA1 | 
| TEE_ALG_RSASSA_PKCS1_V1_5_SHA224 | RSASSA_PKCS1_V1_5_SHA224 | 
| TEE_ALG_RSASSA_PKCS1_V1_5_SHA256 | RSASSA_PKCS1_V1_5_SHA256 | 
| TEE_ALG_RSASSA_PKCS1_V1_5_SHA384 | RSASSA_PKCS1_V1_5_SHA384 | 
| TEE_ALG_RSASSA_PKCS1_V1_5_SHA512 | RSASSA_PKCS1_V1_5_SHA512 | 
| TEE_ALG_RSASSA_PKCS1_PSS_MGF1_MD5 | RSASSA_PKCS1_PSS_MGF1_MD5 | 
| TEE_ALG_RSASSA_PKCS1_PSS_MGF1_SHA1 | RSASSA_PKCS1_PSS_MGF1_SHA1 | 
| TEE_ALG_RSASSA_PKCS1_PSS_MGF1_SHA224 | RSASSA_PKCS1_PSS_MGF1_SHA224 | 
| TEE_ALG_RSASSA_PKCS1_PSS_MGF1_SHA256 | RSASSA_PKCS1_PSS_MGF1_SHA256 | 
| TEE_ALG_RSASSA_PKCS1_PSS_MGF1_SHA384 | RSASSA_PKCS1_PSS_MGF1_SHA384 | 
| TEE_ALG_RSASSA_PKCS1_PSS_MGF1_SHA512 | RSASSA_PKCS1_PSS_MGF1_SHA512 | 
| TEE_ALG_RSAES_PKCS1_V1_5 | RSAES_PKCS1_V1_5 | 
| TEE_ALG_RSAES_PKCS1_OAEP_MGF1_SHA1 | RSAES_PKCS1_OAEP_MGF1_SHA1 | 
| TEE_ALG_RSAES_PKCS1_OAEP_MGF1_SHA224 | RSAES_PKCS1_OAEP_MGF1_SHA224 | 
| TEE_ALG_RSAES_PKCS1_OAEP_MGF1_SHA256 | RSAES_PKCS1_OAEP_MGF1_SHA256 | 
| TEE_ALG_RSAES_PKCS1_OAEP_MGF1_SHA384 | RSAES_PKCS1_OAEP_MGF1_SHA384 | 
| TEE_ALG_RSAES_PKCS1_OAEP_MGF1_SHA512 | RSAES_PKCS1_OAEP_MGF1_SHA512 | 
| TEE_ALG_RSA_NOPAD | RSA_NOPAD | 
| TEE_ALG_DSA_SHA1 | DSA_SHA1 | 
| TEE_ALG_DSA_SHA224 | DSA_SHA224 | 
| TEE_ALG_DSA_SHA256 | DSA_SHA256 | 
| TEE_ALG_DH_DERIVE_SHARED_SECRET | DH_DERIVE_SHARED_SECRET | 
| TEE_ALG_MD5 | MD5 | 
| TEE_ALG_SHA1 | SHA1 | 
| TEE_ALG_SHA224 | SHA224 | 
| TEE_ALG_SHA256 | SHA256 | 
| TEE_ALG_SHA384 | SHA384 | 
| TEE_ALG_SHA512 | SHA512 | 
| TEE_ALG_HMAC_MD5 | HMAC_MD5 | 
| TEE_ALG_HMAC_SHA1 | HMAC_SHA1 | 
| TEE_ALG_HMAC_SHA224 | HMAC_SHA1 | 
| TEE_ALG_HMAC_SHA256 | HMAC_SHA224 | 
| TEE_ALG_HMAC_SHA384 | HMAC_SHA256 | 
| TEE_ALG_HMAC_SHA512 | HMAC_SHA384 | 
| TEE_ALG_HMAC_SM3 | HMAC_SHA512 | 
| TEE_ALG_AES_ECB_PKCS5 | HMAC_SM3 | 
| TEE_ALG_AES_CBC_PKCS5 | AES_ECB_PKCS5 | 
| TEE_ALG_ECDSA_SHA1 | AES_CBC_PKCS5 | 
| TEE_ALG_ECDSA_SHA224 | ECDSA_SHA1 | 
| TEE_ALG_ECDSA_SHA256 | ECDSA_SHA224 | 
| TEE_ALG_ECDSA_SHA384 | ECDSA_SHA256 | 
| TEE_ALG_ECDSA_SHA512 | ECDSA_SHA384 | 
| TEE_ALG_ED25519 | ECDSA_SHA512 | 
| TEE_ALG_ECDH_DERIVE_SHARED_SECRET | ED25519 | 
| TEE_ALG_X25519 | ECDH_DERIVE_SHARED_SECRET | 
| TEE_ALG_ECC | X25519 | 
| TEE_ALG_ECDSA_P192 | ECC | 
| TEE_ALG_ECDSA_P224 | ECDSA_P192 | 
| TEE_ALG_ECDSA_P256 | ECDSA_P224 | 
| TEE_ALG_ECDSA_P384 | ECDSA_P256 | 
| TEE_ALG_ECDSA_P521 | ECDSA_P521 | 
| TEE_ALG_ECDH_P192 | ECDH_P192 | 
| TEE_ALG_ECDH_P224 | ECDH_P224 | 
| TEE_ALG_ECDH_P256 | ECDH_P256 | 
| TEE_ALG_ECDH_P384 | ECDH_P384 | 
| TEE_ALG_ECDH_P521 | ECDH_P521 | 
| TEE_ALG_SM2_DSA_SM3 | SM2_DSA_SM3 | 
| TEE_ALG_SM2_PKE | SM2_PKE | 
| TEE_ALG_SM3 | SM3 | 
| TEE_ALG_SM4_ECB_NOPAD | SM4_ECB_NOPAD | 
| TEE_ALG_SM4_CBC_NOPAD | SM4_CBC_NOPAD | 
| TEE_ALG_SM4_CBC_PKCS7 | SM4_CBC_PKCS7 | 
| TEE_ALG_SM4_CTR | SM4_CTR | 
| TEE_ALG_SM4_CFB128 | SM4_CFB128 | 
| TEE_ALG_SM4_XTS | SM4_XTS | 
| TEE_ALG_SM4_OFB | SM4_OFB | 
| TEE_ALG_AES_OFB | AES_OFB | 
| TEE_ALG_SM4_GCM | SM4_GCM | 

**起始版本：**

1


### __TEE_Operation_Constants

```
enum __TEE_Operation_Constants
```

**描述：**

加解密Operation操作句柄

| 枚举值 | 描述 | 
| -------- | -------- |
| TEE_OPERATION_CIPHER | Cipher | 
| TEE_OPERATION_MAC | Mac | 
| TEE_OPERATION_AE | AE | 
| TEE_OPERATION_DIGEST | Digest | 
| TEE_OPERATION_ASYMMETRIC_CIPHER | Asymmetric Cipher | 
| TEE_OPERATION_ASYMMETRIC_SIGNATURE | Asymmetric Signature | 
| TEE_OPERATION_KEY_DERIVATION | Key Derication | 

**起始版本：**

1


### __TEE_OperationMode

```
enum __TEE_OperationMode
```

**描述：**

加解密算法模式

| 枚举值 | 描述 | 
| -------- | -------- |
| TEE_MODE_ENCRYPT | 加密 | 
| TEE_MODE_DECRYPT | 解密 | 
| TEE_MODE_SIGN | 签名 | 
| TEE_MODE_VERIFY | 验签 | 
| TEE_MODE_MAC | mac | 
| TEE_MODE_DIGEST | 摘要 | 
| TEE_MODE_DERIVE | 衍生 | 

**起始版本：**

1


### TEE_ECC_CURVE

```
enum TEE_ECC_CURVE
```

**描述：**

支持的ECC曲线

| 枚举值 | 描述 | 
| -------- | -------- |
| TEE_ECC_CURVE_NIST_P192 | CURVE_NIST_P192 | 
| TEE_ECC_CURVE_NIST_P224 | CURVE_NIST_P224 | 
| TEE_ECC_CURVE_NIST_P256 | CURVE_NIST_P256 | 
| TEE_ECC_CURVE_NIST_P384 | CURVE_NIST_P384 | 
| TEE_ECC_CURVE_NIST_P521 | CURVE_NIST_P521 | 
| TEE_ECC_CURVE_SM2 | CURVE_SM2 256 bits | 
| TEE_ECC_CURVE_25519 | CURVE_25519 256 bits | 

**起始版本：**

1


### tee_operation_state

```
enum tee_operation_state
```

**描述：**

加解密operation状态

| 枚举值 | 描述 | 
| -------- | -------- |
| TEE_OPERATION_STATE_INITIAL | 初始状态 | 
| TEE_OPERATION_STATE_ACTIVE | 激活状态 | 

**起始版本：**

1


## 函数说明


### TEE_AEDecryptFinal()

```
TEE_Result TEE_AEDecryptFinal (TEE_OperationHandle operation, void * srcData, size_t srcLen, void * destData, size_t * destLen, void * tag, size_t tagLen )
```

**描述：**

ae解密

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| operation | [IN/OUT]操作句柄 | 
| srcData | [IN]源数据 | 
| srcLen | [IN]源数据长度 | 
| destData | [OUT]目标数据 | 
| destLen | [OUT]目标数据长度 | 
| tag | [OUT]tag缓冲区 | 
| tagLen | [OUT]tag缓冲区大小 | 

**返回：**

TEE_SUCCESS 成功

TEE_ERROR_MAC_INVALID tag是非法的

**起始版本：**

1


### TEE_AEEncryptFinal()

```
TEE_Result TEE_AEEncryptFinal (TEE_OperationHandle operation, void * srcData, size_t srcLen, void * destData, size_t * destLen, void * tag, size_t * tagLen )
```

**描述：**

ae加密

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| operation | [IN/OUT]操作句柄 | 
| srcData | [IN]源数据 | 
| srcLen | [IN]源数据长度 | 
| destData | [OUT]目标数据 | 
| destLen | [OUT]目标数据长度 | 
| tag | [OUT]tag缓冲区 | 
| tagLen | [OUT]tag缓冲区大小 | 

**返回：**

TEE_SUCCESS 成功

TEE_ERROR_GENERIC 其它错误

**起始版本：**

1


### TEE_AEInit()

```
TEE_Result TEE_AEInit (TEE_OperationHandle operation, void * nonce, size_t nonceLen, uint32_t tagLen, size_t AADLen, size_t payloadLen )
```

**描述：**

ae初始化

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| operation | [IN/OUT]操作句柄 | 
| nonce | [IN]nonce缓冲区 | 
| nonceLen | [IN]nonce缓冲区大小 | 
| tagLen | [IN]tag的大小 | 
| AADLen | [IN]aad的大小 | 
| payloadLen | [IN]payload的大小 | 

**返回：**

TEE_SUCCESS 成功

TEE_ERROR_GENERIC 其它错误

**起始版本：**

1


### TEE_AEUpdate()

```
TEE_Result TEE_AEUpdate (TEE_OperationHandle operation, void * srcData, size_t srcLen, void * destData, size_t * destLen )
```

**描述：**

更新ae

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| operation | [IN/OUT]操作句柄 | 
| srcData | [IN]源数据 | 
| srcLen | [IN]源数据大小 | 
| destData | [OUT]目标数据 | 
| destLen | [OUT]目标数据大小 | 

**返回：**

TEE_SUCCESS 成功

TEE_ERROR_GENERIC 其它错误

**起始版本：**

1


### TEE_AEUpdateAAD()

```
void TEE_AEUpdateAAD (TEE_OperationHandle operation, constvoid * AADdata, size_t AADdataLen )
```

**描述：**

更新ae aad

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| operation | [IN/OUT]操作句柄 | 
| AADdata | [IN]aad缓冲区 | 
| AADdataLen | [IN]aad缓冲区大小 | 

**起始版本：**

1


### TEE_AllocateOperation()

```
TEE_Result TEE_AllocateOperation (TEE_OperationHandle * operation, uint32_t algorithm, uint32_t mode, uint32_tmaxKeySize  )
```

**描述：**

申请操作句柄

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| operation | [IN/OUT]操作句柄 | 
| algorithm | [IN]加密算法ID | 
| mode | [IN]操作模式 | 
| maxKeySize | [IN]最大密钥大小 | 

**返回：**

TEE_SUCCESS 成功

TEE_ERROR_OUT_OF_MEMORY 操作句柄申请失败

TEE_ERROR_NOT_SUPPORTE 加密算法ID不支持

TEE_ERROR_GENERIC 其它错误

**起始版本：**

1


### TEE_AsymmetricDecrypt()

```
TEE_Result TEE_AsymmetricDecrypt (TEE_OperationHandle operation, const TEE_Attribute * params, uint32_t paramCount, void * srcData, size_t srcLen, void * destData, size_t * destLen )
```

**描述：**

非对称解密

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| operation | [IN/OUT]操作句柄 | 
| params | [IN]属性 | 
| paramCount | [IN]属性数量 | 
| srcData | [IN]源数据 | 
| srcLen | [IN]源数据长度 | 
| destData | [OUT]目标数据 | 
| destLen | [OUT]目标数据长度 | 

**返回：**

TEE_SUCCESS 成功

TEE_ERROR_BAD_PARAMETERS 非法参数

TEE_ERROR_GENERIC 其它错误

**起始版本：**

1


### TEE_AsymmetricEncrypt()

```
TEE_Result TEE_AsymmetricEncrypt (TEE_OperationHandle operation, const TEE_Attribute * params, uint32_t paramCount, void * srcData, size_t srcLen, void * destData, size_t * destLen )
```

**描述：**

非对称加密

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| operation | [IN/OUT]操作句柄 | 
| params | [IN]属性 | 
| paramCount | [IN]属性数量 | 
| srcData | [IN]源数据 | 
| srcLen | [IN]源数据长度 | 
| destData | [OUT]目标数据 | 
| destLen | [OUT]目标数据长度 | 

**返回：**

TEE_SUCCESS 成功

TEE_ERROR_BAD_PARAMETERS 非法参数

TEE_ERROR_GENERIC 其它错误

**起始版本：**

1


### TEE_AsymmetricSignDigest()

```
TEE_Result TEE_AsymmetricSignDigest (TEE_OperationHandle operation, const TEE_Attribute * params, uint32_t paramCount, void * digest, size_t digestLen, void * signature, size_t * signatureLen )
```

**描述：**

非对称签名

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| operation | [IN/OUT]操作句柄 | 
| params | [IN]属性 | 
| paramCount | [IN]属性数量 | 
| digest | [IN]摘要 | 
| digestLen | [IN]摘要长度 | 
| signature | [OUT]签名 | 
| signatureLen | [OUT]签名长度 | 

**返回：**

TEE_SUCCESS 成功

TEE_ERROR_BAD_PARAMETERS 非法参数

TEE_ERROR_GENERIC 其它错误

**起始版本：**

1


### TEE_AsymmetricVerifyDigest()

```
TEE_Result TEE_AsymmetricVerifyDigest (TEE_OperationHandle operation, const TEE_Attribute * params, uint32_t paramCount, void * digest, size_t digestLen, void * signature, size_t signatureLen )
```

**描述：**

非对称验证

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| operation | [IN/OUT]操作句柄 | 
| params | [IN]属性 | 
| paramCount | [IN]属性数量 | 
| digest | [IN]摘要 | 
| digestLen | [IN]摘要长度 | 
| signature | [OUT]签名 | 
| signatureLen | [OUT]签名长度 | 

**返回：**

TEE_SUCCESS 成功

TEE_ERROR_BAD_PARAMETERS 非法参数

TEE_ERROR_GENERIC 其它错误

**起始版本：**

1


### TEE_CipherDoFinal()

```
TEE_Result TEE_CipherDoFinal (TEE_OperationHandle operation, constvoid * srcData, size_t srcLen, void * destData, size_t * destLen )
```

**描述：**

执行密码完成

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| operation | [IN/OUT]操作句柄 | 
| srcData | [IN]源数据 | 
| srcLen | [IN]源数据长度 | 
| destData | [OUT]目标数据 | 
| destLen | [OUT]目标数据长度 | 

**返回：**

TEE_SUCCESS 成功

TEE_ERROR_BAD_PARAMETERS 非法参数

TEE_ERROR_GENERIC 其它错误

**起始版本：**

1


### TEE_CipherInit()

```
void TEE_CipherInit (TEE_OperationHandle operation, constvoid * IV, size_t IVLen )
```

**描述：**

初始化密码上下文

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| operation | [IN/OUT]操作句柄 | 
| IV | [IN]iv缓冲区，如果不使用设置为NULL | 
| IVLen | [IN]iv缓冲区的长度 | 

**起始版本：**

1


### TEE_CipherUpdate()

```
TEE_Result TEE_CipherUpdate (TEE_OperationHandle operation, constvoid * srcData, size_t srcLen, void * destData, size_t * destLen )
```

**描述：**

执行密码更新

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| operation | [IN/OUT]操作句柄 | 
| srcData | [IN]源数据 | 
| srcLen | [IN]源数据长度 | 
| destData | [OUT]目标数据 | 
| destLen | [OUT]目标数据长度 | 

**返回：**

TEE_SUCCESS 成功

TEE_ERROR_BAD_PARAMETERS 非法参数

TEE_ERROR_GENERIC 其它错误

**起始版本：**

1


### TEE_CopyOperation()

```
void TEE_CopyOperation (TEE_OperationHandle dstOperation, const TEE_OperationHandle srcOperation )
```

**描述：**

复制操作句柄

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| dstOperation | [IN/OUT]目标操作句柄 | 
| srcOperation | [IN/OUT]源操作句柄 | 

**起始版本：**

1


### TEE_DigestDoFinal()

```
TEE_Result TEE_DigestDoFinal (TEE_OperationHandle operation, constvoid * chunk, size_t chunkLen, void * hash, size_t * hashLen )
```

**描述：**

执行摘要结束

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| operation | [IN/OUT]操作句柄 | 
| chunk | [IN]块缓冲区 | 
| chunkLen | [IN]块缓冲区大小 | 
| hash | [out]哈希缓冲区 | 
| hashLen |  | 

**起始版本：**

1


### TEE_DigestUpdate()

```
void TEE_DigestUpdate (TEE_OperationHandle operation, constvoid * chunk, size_t chunkSize )
```

**描述：**

摘要更新

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| operation | [IN/OUT]操作句柄 | 
| chunk | [IN]块缓冲区 | 
| chunkSize | [IN]块缓冲区长度 | 

**起始版本：**

1


### TEE_FreeOperation()

```
void TEE_FreeOperation (TEE_OperationHandle operation)
```

**描述：**

释放操作句柄

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| operation | [IN/OUT]操作句柄 | 

**起始版本：**

1


### TEE_GenerateRandom()

```
void TEE_GenerateRandom (void * randomBuffer, size_t randomBufferLen )
```

**描述：**

生成随机数据

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| randomBuffer | [IN/OUT]随机缓冲区 | 
| randomBufferLen | [IN]随机缓冲区大小 | 

**起始版本：**

1


### TEE_GetOperationInfo()

```
void TEE_GetOperationInfo (const TEE_OperationHandle operation, TEE_OperationInfo * operationInfo )
```

**描述：**

获取操作信息

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| operation | [IN/OUT]操作句柄 | 
| operationInfo | [IN/OUT]操作信息 | 

**起始版本：**

1


### TEE_GetOperationInfoMultiple()

```
TEE_Result TEE_GetOperationInfoMultiple (TEE_OperationHandle operation, TEE_OperationInfoMultiple * operationInfoMultiple, constsize_t * operationSize )
```

**描述：**

获取操作信息

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| operation | [IN/OUT]操作句柄 | 
| operationInfoMultiple | [IN/OUT]操作信息 | 
| operationSize | [IN/OUT]操作信息大小 | 

**返回：**

TEE_SUCCESS 成功

TEE_ERROR_BAD_PARAMETERS 非法参数

TEE_ERROR_SHORT_BUFFER 缓冲区不足

**起始版本：**

1


### TEE_IsAlgorithmSupported()

```
TEE_Result TEE_IsAlgorithmSupported (uint32_t algId, uint32_t element )
```

**描述：**

检查算法是否被支持

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| algId | [IN]算法ID | 
| element | [IN]元素 | 

**返回：**

TEE_SUCCESS 支持

TEE_ERROR_NOT_SUPPORTED 不支持

**起始版本：**

1


### TEE_MACCompareFinal()

```
TEE_Result TEE_MACCompareFinal (TEE_OperationHandle operation, constvoid * message, size_t messageLen, constvoid * mac, constsize_t macLen )
```

**描述：**

mac比较完成

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| operation | [IN/OUT]操作句柄 | 
| message | [IN]message缓冲区 | 
| messageLen | [IN]message缓冲区大小 | 
| mac | [IN]mac缓冲区 | 
| macLen | [IN]mac缓冲区大小 | 

**返回：**

TEE_SUCCESS 成功

TEE_ERROR_GENERIC 其它错误

TEE_ERROR_MAC_INVALID 比较失败

**起始版本：**

1


### TEE_MACComputeFinal()

```
TEE_Result TEE_MACComputeFinal (TEE_OperationHandle operation, constvoid * message, size_t messageLen, void * mac, size_t * macLen )
```

**描述：**

mac计算完成

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| operation | [IN/OUT]操作句柄 | 
| message | [IN]message缓冲区 | 
| messageLen | [IN]message缓冲区的大小 | 
| mac | [OUT]mac缓冲区 | 
| macLen | [OUT]mac缓冲区大小 | 

**返回：**

TEE_SUCCESS 成功

TEE_ERROR_GENERIC 其它错误

**起始版本：**

1


### TEE_MACInit()

```
void TEE_MACInit (TEE_OperationHandle operation, void * IV, size_t IVLen )
```

**描述：**

执行mac初始化

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| operation | [IN/OUT]操作句柄 | 
| IV | [IN]iv缓冲区，如果不使用设置为NULL | 
| IVLen | [IN]iv缓冲区长度 | 

**起始版本：**

1


### TEE_MACUpdate()

```
void TEE_MACUpdate (TEE_OperationHandle operation, constvoid * chunk, size_t chunkSize )
```

**描述：**

执行mac更新

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| operation | [IN/OUT]操作句柄 | 
| chunk | [IN]块缓冲区 | 
| chunkSize | [IN]块缓冲区大小 | 

**起始版本：**

1


### TEE_ResetOperation()

```
void TEE_ResetOperation (TEE_OperationHandle operation)
```

**描述：**

复位操作句柄

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| operation | [IN/OUT]操作句柄 | 

**起始版本：**

1


### TEE_SetCryptoFlag()

```
TEE_Result TEE_SetCryptoFlag (TEE_OperationHandle operation, uint32_t crypto )
```

**描述：**

将加密和解密引擎设置为运行

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| operation | [IN/OUT]操作的句柄 | 
| crypto | [IN]要设置的engine | 

**返回：**

TEE_SUCCESS 设置加密引擎成功

TEE_ERROR_BAD_PARAMETERS 操作为NULLi或加密无效

**起始版本：**

1


### TEE_SetObjectFlag()

```
TEE_Result TEE_SetObjectFlag (TEE_ObjectHandle object, uint32_t crypto )
```

**描述：**

设置加解密引擎为object

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| operation | [IN/OUT]对象的句柄 | 
| crypto | [IN]要设置的engine | 

**返回：**

TEE_SUCCESS 设置加密引擎成功

TEE_ERROR_BAD_PARAMETERS 操作为NULLi或加密无效

**起始版本：**

1


### TEE_SetOperationKey()

```
TEE_Result TEE_SetOperationKey (TEE_OperationHandle operation, const TEE_ObjectHandle key )
```

**描述：**

设置操作密钥

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| operation | [IN/OUT]操作句柄 | 
| key | [IN/OUT]密钥 | 

**返回：**

TEE_SUCCESS 成功

TEE_ERROR_BAD_PARAMETERS 非法参数

TEE_ERROR_OUT_OF_MEMORY 密钥缓冲区申请失败

**起始版本：**

1


### TEE_SetOperationKey2()

```
TEE_Result TEE_SetOperationKey2 (TEE_OperationHandle operation, const TEE_ObjectHandle key1, const TEE_ObjectHandle key2 )
```

**描述：**

设置操作密钥2

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| operation | [IN/OUT]操作句柄 | 
| key1 | [IN/OUT]密钥1 | 
| key2 | [IN/OUT]密钥2 | 

**返回：**

TEE_SUCCESS 成功

TEE_ERROR_BAD_PARAMETERS 非法参数

**起始版本：**

1


### voidTEE_DeriveKey()

```
voidTEE_DeriveKey (TEE_OperationHandle operation, const TEE_Attribute * params, uint32_t paramCount, TEE_ObjectHandle derivedKey )
```

**描述：**

派生密钥

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| operation | [IN/OUT]操作句柄 | 
| params | [IN]属性 | 
| paramCount | [IN]属性的数量 | 
| derivedKey | [OUT]派生密钥 | 

**起始版本：**

1
