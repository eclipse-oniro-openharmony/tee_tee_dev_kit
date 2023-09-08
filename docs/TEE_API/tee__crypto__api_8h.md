# tee_crypto_api.h


## 概述

加解密接口

开发者可以使用这些接口实现加解密的相关功能。

**起始版本：**

1

**相关模块：**

[TeeCrypto](_tee_crypto.md)


## 汇总


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
| [NULL](_tee_crypto.md#null)   ((void \*)0) | NULL定义 | 
| [TEE_MAX_KEY_SIZE_IN_BITS](_tee_crypto.md)   (1024 \* 8) | 密钥最大长度（以bits为单位） | 
| [SW_RSA_KEYLEN](_tee_crypto.md)   1024 | SW_RSA密钥长度 | 
| [TEE_DH_MAX_SIZE_OF_OTHER_INFO](_tee_crypto.md)   64 /\* bytes \*/ | DH其它信息的最大长度 | 
| [TEE_OPTIONAL_ELEMENT_NONE](_tee_crypto.md)   0x00000000 | 用于给alg_config_t结构体中element成员赋值，表示不需要曲线参数 | 
| [RSA_PUBKEY_MAXSIZE](_tee_crypto.md)   sizeof(CRYS_RSAUserPubKey_t) | RSA公钥最大长度 | 
| [RSA_PRIVKEY_MAXSIZE](_tee_crypto.md)   sizeof(CRYS_RSAUserPrivKey_t) | RES私钥最大长度 | 


### 类型定义

| 名称 | 描述 | 
| -------- | -------- |
| [tee_crypto_algorithm_id](_tee_crypto.md#tee_crypto_algorithm_id) |  | 
| [TEE_OperationMode](_tee_crypto.md#tee_operationmode) |  | 
| [TEE_OperationInfo](_tee_crypto.md#tee_operationinfo) | 用于定义__TEE_OperationInfo结构体类型 | 
| [TEE_OperationHandle](_tee_crypto.md#tee_operationhandle) | 用于定义__TEE_OperationHandle指针类型 | 
| [TEE_OperationHandleVar](_tee_crypto.md#tee_operationhandlevar) | 用于定义__TEE_OperationHandle结构体类型 | 
| [TEE_ObjectHandleVar](_tee_crypto.md) | 用于定义__TEE_ObjectHandle结构体类型 | 


### 枚举

| 名称 | 描述 | 
| -------- | -------- |
| [__TEE_Operation_Constants](_tee_crypto.md#__tee_operation_constants) {<br/>[TEE_OPERATION_CIPHER](_tee_crypto.md) = 0x1, [TEE_OPERATION_MAC](_tee_crypto.md) = 3, [TEE_OPERATION_AE](_tee_crypto.md) = 4, [TEE_OPERATION_DIGEST](_tee_crypto.md) = 5,<br/>[TEE_OPERATION_ASYMMETRIC_CIPHER](_tee_crypto.md) = 6, [TEE_OPERATION_ASYMMETRIC_SIGNATURE](_tee_crypto.md) = 7, [TEE_OPERATION_KEY_DERIVATION](_tee_crypto.md) = 8<br/>} | 加解密Operation操作句柄 | 
| [__tee_crypto_algorithm_id](_tee_crypto.md#__tee_crypto_algorithm_id) {<br/>[TEE_ALG_INVALID](_tee_crypto.md) = 0x0, [TEE_ALG_AES_ECB_NOPAD](_tee_crypto.md) = 0x10000010, [TEE_ALG_AES_CBC_NOPAD](_tee_crypto.md) = 0x10000110, [TEE_ALG_AES_CTR](_tee_crypto.md) = 0x10000210,<br/>[TEE_ALG_AES_CTS](_tee_crypto.md) = 0x10000310, [TEE_ALG_AES_XTS](_tee_crypto.md) = 0x10000410, [TEE_ALG_AES_CBC_MAC_NOPAD](_tee_crypto.md) = 0x30000110, [TEE_ALG_AES_CBC_MAC_PKCS5](_tee_crypto.md) = 0x30000510,<br/>[TEE_ALG_AES_CMAC](_tee_crypto.md) = 0x30000610, [TEE_ALG_AES_GMAC](_tee_crypto.md) = 0x30000810, [TEE_ALG_AES_CCM](_tee_crypto.md) = 0x40000710, [TEE_ALG_AES_GCM](_tee_crypto.md) = 0x40000810,<br/>[TEE_ALG_DES_ECB_NOPAD](_tee_crypto.md) = 0x10000011, [TEE_ALG_DES_CBC_NOPAD](_tee_crypto.md) = 0x10000111, [TEE_ALG_DES_CBC_MAC_NOPAD](_tee_crypto.md) = 0x30000111, [TEE_ALG_DES_CBC_MAC_PKCS5](_tee_crypto.md) = 0x30000511,<br/>[TEE_ALG_DES3_ECB_NOPAD](_tee_crypto.md) = 0x10000013, [TEE_ALG_DES3_CBC_NOPAD](_tee_crypto.md) = 0x10000113, [TEE_ALG_DES3_CBC_MAC_NOPAD](_tee_crypto.md) = 0x30000113, [TEE_ALG_DES3_CBC_MAC_PKCS5](_tee_crypto.md) = 0x30000513,<br/>[TEE_ALG_RSASSA_PKCS1_V1_5_MD5](_tee_crypto.md) = 0x70001830, [TEE_ALG_RSASSA_PKCS1_V1_5_SHA1](_tee_crypto.md) = 0x70002830, [TEE_ALG_RSASSA_PKCS1_V1_5_SHA224](_tee_crypto.md) = 0x70003830, [TEE_ALG_RSASSA_PKCS1_V1_5_SHA256](_tee_crypto.md) = 0x70004830,<br/>[TEE_ALG_RSASSA_PKCS1_V1_5_SHA384](_tee_crypto.md) = 0x70005830, [TEE_ALG_RSASSA_PKCS1_V1_5_SHA512](_tee_crypto.md) = 0x70006830, [TEE_ALG_RSASSA_PKCS1_PSS_MGF1_MD5](_tee_crypto.md) = 0x70111930, [TEE_ALG_RSASSA_PKCS1_PSS_MGF1_SHA1](_tee_crypto.md) = 0x70212930,<br/>[TEE_ALG_RSASSA_PKCS1_PSS_MGF1_SHA224](_tee_crypto.md) = 0x70313930, [TEE_ALG_RSASSA_PKCS1_PSS_MGF1_SHA256](_tee_crypto.md) = 0x70414930, [TEE_ALG_RSASSA_PKCS1_PSS_MGF1_SHA384](_tee_crypto.md) = 0x70515930, [TEE_ALG_RSASSA_PKCS1_PSS_MGF1_SHA512](_tee_crypto.md) = 0x70616930,<br/>[TEE_ALG_RSAES_PKCS1_V1_5](_tee_crypto.md) = 0x60000130, [TEE_ALG_RSAES_PKCS1_OAEP_MGF1_SHA1](_tee_crypto.md) = 0x60210230, [TEE_ALG_RSAES_PKCS1_OAEP_MGF1_SHA224](_tee_crypto.md) = 0x60211230, [TEE_ALG_RSAES_PKCS1_OAEP_MGF1_SHA256](_tee_crypto.md) = 0x60212230,<br/>[TEE_ALG_RSAES_PKCS1_OAEP_MGF1_SHA384](_tee_crypto.md) = 0x60213230, [TEE_ALG_RSAES_PKCS1_OAEP_MGF1_SHA512](_tee_crypto.md) = 0x60214230, [TEE_ALG_RSA_NOPAD](_tee_crypto.md) = 0x60000030, [TEE_ALG_DSA_SHA1](_tee_crypto.md) = 0x70002131,<br/>[TEE_ALG_DSA_SHA224](_tee_crypto.md) = 0x70003131, [TEE_ALG_DSA_SHA256](_tee_crypto.md) = 0x70004131, [TEE_ALG_DH_DERIVE_SHARED_SECRET](_tee_crypto.md) = 0x80000032, [TEE_ALG_MD5](_tee_crypto.md) = 0x50000001,<br/>[TEE_ALG_SHA1](_tee_crypto.md) = 0x50000002, [TEE_ALG_SHA224](_tee_crypto.md) = 0x50000003, [TEE_ALG_SHA256](_tee_crypto.md) = 0x50000004, [TEE_ALG_SHA384](_tee_crypto.md) = 0x50000005,<br/>[TEE_ALG_SHA512](_tee_crypto.md) = 0x50000006, [TEE_ALG_HMAC_MD5](_tee_crypto.md) = 0x30000001, [TEE_ALG_HMAC_SHA1](_tee_crypto.md) = 0x30000002, [TEE_ALG_HMAC_SHA224](_tee_crypto.md) = 0x30000003,<br/>[TEE_ALG_HMAC_SHA256](_tee_crypto.md) = 0x30000004, [TEE_ALG_HMAC_SHA384](_tee_crypto.md) = 0x30000005, [TEE_ALG_HMAC_SHA512](_tee_crypto.md) = 0x30000006, [TEE_ALG_HMAC_SM3](_tee_crypto.md) = 0x30000007,<br/>[TEE_ALG_AES_ECB_PKCS5](_tee_crypto.md) = 0x10000020, [TEE_ALG_AES_CBC_PKCS5](_tee_crypto.md) = 0x10000220, [TEE_ALG_ECDSA_SHA1](_tee_crypto.md) = 0x70001042, [TEE_ALG_ECDSA_SHA224](_tee_crypto.md) = 0x70002042,<br/>[TEE_ALG_ECDSA_SHA256](_tee_crypto.md) = 0x70003042, [TEE_ALG_ECDSA_SHA384](_tee_crypto.md) = 0x70004042, [TEE_ALG_ECDSA_SHA512](_tee_crypto.md) = 0x70005042, [TEE_ALG_ED25519](_tee_crypto.md) = 0x70005043,<br/>[TEE_ALG_ECDH_DERIVE_SHARED_SECRET](_tee_crypto.md) = 0x80000042, [TEE_ALG_X25519](_tee_crypto.md) = 0x80000044, [TEE_ALG_ECC](_tee_crypto.md) = 0x80000001, [TEE_ALG_ECDSA_P192](_tee_crypto.md) = 0x70001042,<br/>[TEE_ALG_ECDSA_P224](_tee_crypto.md) = 0x70002042, [TEE_ALG_ECDSA_P256](_tee_crypto.md) = 0x70003042, [TEE_ALG_ECDSA_P384](_tee_crypto.md) = 0x70004042, [TEE_ALG_ECDSA_P521](_tee_crypto.md) = 0x70005042,<br/>[TEE_ALG_ECDH_P192](_tee_crypto.md) = 0x80001042, [TEE_ALG_ECDH_P224](_tee_crypto.md) = 0x80002042, [TEE_ALG_ECDH_P256](_tee_crypto.md) = 0x80003042, [TEE_ALG_ECDH_P384](_tee_crypto.md) = 0x80004042,<br/>[TEE_ALG_ECDH_P521](_tee_crypto.md) = 0x80005042, [TEE_ALG_SM2_DSA_SM3](_tee_crypto.md) = 0x70006045, [TEE_ALG_SM2_PKE](_tee_crypto.md) = 0x80000045, [TEE_ALG_SM3](_tee_crypto.md) = 0x50000007,<br/>[TEE_ALG_SM4_ECB_NOPAD](_tee_crypto.md) = 0x10000014, [TEE_ALG_SM4_CBC_NOPAD](_tee_crypto.md) = 0x10000114, [TEE_ALG_SM4_CBC_PKCS7](_tee_crypto.md) = 0xF0000003, [TEE_ALG_SM4_CTR](_tee_crypto.md) = 0x10000214,<br/>[TEE_ALG_SM4_CFB128](_tee_crypto.md) = 0xF0000000, [TEE_ALG_SM4_XTS](_tee_crypto.md) = 0x10000414, [TEE_ALG_SM4_OFB](_tee_crypto.md) = 0x10000514, [TEE_ALG_AES_OFB](_tee_crypto.md) = 0x10000510,<br/>[TEE_ALG_SM4_GCM](_tee_crypto.md) = 0xF0000005<br/>} | 加解密算法标识 | 
| [TEE_ECC_CURVE](_tee_crypto.md#tee_ecc_curve) {<br/>[TEE_ECC_CURVE_NIST_P192](_tee_crypto.md) = 0x00000001, [TEE_ECC_CURVE_NIST_P224](_tee_crypto.md) = 0x00000002, [TEE_ECC_CURVE_NIST_P256](_tee_crypto.md) = 0x00000003, [TEE_ECC_CURVE_NIST_P384](_tee_crypto.md) = 0x00000004,<br/>[TEE_ECC_CURVE_NIST_P521](_tee_crypto.md) = 0x00000005, [TEE_ECC_CURVE_SM2](_tee_crypto.md) = 0x00000300, [TEE_ECC_CURVE_25519](_tee_crypto.md) = 0x00000200<br/>} | 支持的ECC曲线 | 
| TEE_DH_HASH_Mode {<br/>**TEE_DH_HASH_SHA1_mode** = 0, **TEE_DH_HASH_SHA224_mode** = 1, **TEE_DH_HASH_SHA256_mode** = 2, **TEE_DH_HASH_SHA384_mode** = 3,<br/>**TEE_DH_HASH_SHA512_mode** = 4, **TEE_DH_HASH_NumOfModes**<br/>} | MGF1掩码函数类型 | 
| [__TEE_OperationMode](_tee_crypto.md#__tee_operationmode) {<br/>[TEE_MODE_ENCRYPT](_tee_crypto.md) = 0x0, [TEE_MODE_DECRYPT](_tee_crypto.md), [TEE_MODE_SIGN](_tee_crypto.md), [TEE_MODE_VERIFY](_tee_crypto.md),<br/>[TEE_MODE_MAC](_tee_crypto.md), [TEE_MODE_DIGEST](_tee_crypto.md), [TEE_MODE_DERIVE](_tee_crypto.md)<br/>} | 加解密算法模式 | 
| [tee_operation_state](_tee_crypto.md#tee_operation_state) { [TEE_OPERATION_STATE_INITIAL](_tee_crypto.md) = 0x00000000, [TEE_OPERATION_STATE_ACTIVE](_tee_crypto.md) = 0x00000001 } | 加解密operation状态 | 


### 函数

| 名称 | 描述 | 
| -------- | -------- |
| [TEE_AllocateOperation](_tee_crypto.md#tee_allocateoperation) ([TEE_OperationHandle](_tee_crypto.md#tee_operationhandle) \*operation, uint32_t algorithm, uint32_t mode, uint32_tmaxKeySize) | 申请操作句柄 | 
| [TEE_FreeOperation](_tee_crypto.md#tee_freeoperation) ([TEE_OperationHandle](_tee_crypto.md#tee_operationhandle) operation) | 释放操作句柄 | 
| [TEE_GetOperationInfo](_tee_crypto.md#tee_getoperationinfo) (const [TEE_OperationHandle](_tee_crypto.md#tee_operationhandle) operation, [TEE_OperationInfo](_tee_crypto.md#tee_operationinfo) \*operationInfo) | 获取操作信息 | 
| [TEE_ResetOperation](_tee_crypto.md#tee_resetoperation) ([TEE_OperationHandle](_tee_crypto.md#tee_operationhandle) operation) | 复位操作句柄 | 
| [TEE_SetOperationKey](_tee_crypto.md#tee_setoperationkey) ([TEE_OperationHandle](_tee_crypto.md#tee_operationhandle) operation, const TEE_ObjectHandle key) | 设置操作密钥 | 
| [TEE_SetOperationKey2](_tee_crypto.md#tee_setoperationkey2) ([TEE_OperationHandle](_tee_crypto.md#tee_operationhandle) operation, const TEE_ObjectHandle key1, const TEE_ObjectHandle key2) | 设置操作密钥2 | 
| [TEE_CopyOperation](_tee_crypto.md#tee_copyoperation) ([TEE_OperationHandle](_tee_crypto.md#tee_operationhandle) dstOperation, const [TEE_OperationHandle](_tee_crypto.md#tee_operationhandle) srcOperation) | 复制操作句柄 | 
| [TEE_CipherInit](_tee_crypto.md#tee_cipherinit) ([TEE_OperationHandle](_tee_crypto.md#tee_operationhandle) operation, constvoid \*IV, size_t IVLen) | 初始化密码上下文 | 
| [TEE_CipherUpdate](_tee_crypto.md#tee_cipherupdate) ([TEE_OperationHandle](_tee_crypto.md#tee_operationhandle) operation, constvoid \*srcData, size_t srcLen, void \*destData, size_t \*destLen) | 执行密码更新 | 
| [TEE_CipherDoFinal](_tee_crypto.md#tee_cipherdofinal) ([TEE_OperationHandle](_tee_crypto.md#tee_operationhandle) operation, constvoid \*srcData, size_t srcLen, void \*destData, size_t \*destLen) | 执行密码完成 | 
| [TEE_DigestUpdate](_tee_crypto.md#tee_digestupdate) ([TEE_OperationHandle](_tee_crypto.md#tee_operationhandle) operation, constvoid \*chunk, size_t chunkSize) | 摘要更新 | 
| [TEE_DigestDoFinal](_tee_crypto.md#tee_digestdofinal) ([TEE_OperationHandle](_tee_crypto.md#tee_operationhandle) operation, constvoid \*chunk, size_t chunkLen, void \*hash, size_t \*hashLen) | 执行摘要结束 | 
| [TEE_MACInit](_tee_crypto.md#tee_macinit) ([TEE_OperationHandle](_tee_crypto.md#tee_operationhandle) operation, void \*IV, size_t IVLen) | 执行mac初始化 | 
| [TEE_MACUpdate](_tee_crypto.md#tee_macupdate) ([TEE_OperationHandle](_tee_crypto.md#tee_operationhandle) operation, constvoid \*chunk, size_t chunkSize) | 执行mac更新 | 
| [TEE_MACComputeFinal](_tee_crypto.md#tee_maccomputefinal) ([TEE_OperationHandle](_tee_crypto.md#tee_operationhandle) operation, constvoid \*message, size_t messageLen, void \*mac, size_t \*macLen) | mac计算完成 | 
| [TEE_MACCompareFinal](_tee_crypto.md#tee_maccomparefinal) ([TEE_OperationHandle](_tee_crypto.md#tee_operationhandle) operation, constvoid \*message, size_t messageLen, constvoid \*mac, constsize_t macLen) | mac比较完成 | 
| [voidTEE_DeriveKey](_tee_crypto.md#voidtee_derivekey) ([TEE_OperationHandle](_tee_crypto.md#tee_operationhandle) operation, const TEE_Attribute \*params, uint32_t paramCount, TEE_ObjectHandle derivedKey) | 派生密钥 | 
| [TEE_GenerateRandom](_tee_crypto.md#tee_generaterandom) (void \*randomBuffer, size_t randomBufferLen) | 生成随机数据 | 
| [TEE_AEInit](_tee_crypto.md#tee_aeinit) ([TEE_OperationHandle](_tee_crypto.md#tee_operationhandle) operation, void \*nonce, size_t nonceLen, uint32_t tagLen, size_t AADLen, size_t payloadLen) | ae初始化 | 
| [TEE_AEUpdateAAD](_tee_crypto.md#tee_aeupdateaad) ([TEE_OperationHandle](_tee_crypto.md#tee_operationhandle) operation, constvoid \*AADdata, size_t AADdataLen) | 更新ae aad | 
| [TEE_AEUpdate](_tee_crypto.md#tee_aeupdate) ([TEE_OperationHandle](_tee_crypto.md#tee_operationhandle) operation, void \*srcData, size_t srcLen, void \*destData, size_t \*destLen) | 更新ae | 
| [TEE_AEEncryptFinal](_tee_crypto.md#tee_aeencryptfinal) ([TEE_OperationHandle](_tee_crypto.md#tee_operationhandle) operation, void \*srcData, size_t srcLen, void \*destData, size_t \*destLen, void \*tag, size_t \*tagLen) | ae加密 | 
| [TEE_AEDecryptFinal](_tee_crypto.md#tee_aedecryptfinal) ([TEE_OperationHandle](_tee_crypto.md#tee_operationhandle) operation, void \*srcData, size_t srcLen, void \*destData, size_t \*destLen, void \*tag, size_t tagLen) | ae解密 | 
| [TEE_AsymmetricEncrypt](_tee_crypto.md#tee_asymmetricencrypt) ([TEE_OperationHandle](_tee_crypto.md#tee_operationhandle) operation, const TEE_Attribute \*params, uint32_t paramCount, void \*srcData, size_t srcLen, void \*destData, size_t \*destLen) | 非对称加密 | 
| [TEE_AsymmetricDecrypt](_tee_crypto.md#tee_asymmetricdecrypt) ([TEE_OperationHandle](_tee_crypto.md#tee_operationhandle) operation, const TEE_Attribute \*params, uint32_t paramCount, void \*srcData, size_t srcLen, void \*destData, size_t \*destLen) | 非对称解密 | 
| [TEE_AsymmetricSignDigest](_tee_crypto.md#tee_asymmetricsigndigest) ([TEE_OperationHandle](_tee_crypto.md#tee_operationhandle) operation, const TEE_Attribute \*params, uint32_t paramCount, void \*digest, size_t digestLen, void \*signature, size_t \*signatureLen) | 非对称签名 | 
| [TEE_AsymmetricVerifyDigest](_tee_crypto.md#tee_asymmetricverifydigest) ([TEE_OperationHandle](_tee_crypto.md#tee_operationhandle) operation, const TEE_Attribute \*params, uint32_t paramCount, void \*digest, size_t digestLen, void \*signature, size_t signatureLen) | 非对称验证 | 
| [TEE_GetOperationInfoMultiple](_tee_crypto.md#tee_getoperationinfomultiple) ([TEE_OperationHandle](_tee_crypto.md#tee_operationhandle) operation, [TEE_OperationInfoMultiple](_t_e_e___operation_info_multiple.md) \*operationInfoMultiple, constsize_t \*operationSize) | 获取操作信息 | 
| [TEE_IsAlgorithmSupported](_tee_crypto.md#tee_isalgorithmsupported) (uint32_t algId, uint32_t element) | 检查算法是否被支持 | 
