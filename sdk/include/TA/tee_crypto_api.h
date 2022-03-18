/*
 * Copyright (C) 2022 Huawei Technologies Co., Ltd.
 * Licensed under the Mulan PSL v2.
 * You can use this software according to the terms and conditions of the Mulan PSL v2.
 * You may obtain a copy of Mulan PSL v2 at:
 *     http://license.coscl.org.cn/MulanPSL2
 * THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND, EITHER EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR
 * PURPOSE.
 * See the Mulan PSL v2 for more details.
 */

/**
 * @file tee_crypto_api.h
 *
 * @brief 加解密接口
 *
 * 开发者可以使用这些接口实现加解密的相关功能。
 *
 * @since 1
 */

#ifndef TEE_CRYPTO_API_H
#define TEE_CRYPTO_API_H

#include <pthread.h> /* pthread_mutex_t */
#include <tee_defines.h>
#include <tee_mem_mgmt_api.h>

#ifndef NULL
/**
 * NULL定义
 */
#define NULL ((void *)0)
#endif
/**
 * @brief 密钥最大长度（以bits为单位）
 */
#define TEE_MAX_KEY_SIZE_IN_BITS      (1024 * 8)
/**
 * @brief SW_RSA密钥长度
 */
#define SW_RSA_KEYLEN                 1024
/**
 * @brief DH其它信息的最大长度
 */
#define TEE_DH_MAX_SIZE_OF_OTHER_INFO 64 /* bytes */

/**
 * @brief 加解密Operation操作句柄
 */
enum __TEE_Operation_Constants {
    /** Cipher */
    TEE_OPERATION_CIPHER               = 0x1,
    /** Mac */
    TEE_OPERATION_MAC                  = 3,
    /** AE */
    TEE_OPERATION_AE                   = 4,
    /** Digest */
    TEE_OPERATION_DIGEST               = 5,
    /** Asymmetric Cipher */
    TEE_OPERATION_ASYMMETRIC_CIPHER    = 6,
    /** Asymmetric Signature */
    TEE_OPERATION_ASYMMETRIC_SIGNATURE = 7,
    /** Key Derication */
    TEE_OPERATION_KEY_DERIVATION       = 8,
};

/**
 * @brief 加解密算法标识
 */
enum __tee_crypto_algorithm_id {
    /** 无效算法 */
    TEE_ALG_INVALID                      = 0x0,
    /** AES_ECB_NOPAD */
    TEE_ALG_AES_ECB_NOPAD                = 0x10000010,
    /** AES_CBC_NOPAD */
    TEE_ALG_AES_CBC_NOPAD                = 0x10000110,
    /** AES_CTR */
    TEE_ALG_AES_CTR                      = 0x10000210,
    /** AES_CTS */
    TEE_ALG_AES_CTS                      = 0x10000310,
    /** AES_XTS */
    TEE_ALG_AES_XTS                      = 0x10000410,
    /** AES_CBC_MAC_NOPAD */
    TEE_ALG_AES_CBC_MAC_NOPAD            = 0x30000110,
    /** AES_CBC_MAC_PKCS5 */
    TEE_ALG_AES_CBC_MAC_PKCS5            = 0x30000510,
    /** AES_CMAC */
    TEE_ALG_AES_CMAC                     = 0x30000610,
    /** AES_GMAC */
    TEE_ALG_AES_GMAC                     = 0x30000810,
    /** AES_CCM */
    TEE_ALG_AES_CCM                      = 0x40000710,
    /** AES_GCM */
    TEE_ALG_AES_GCM                      = 0x40000810,
    /** DES_ECB_NOPAD */
    TEE_ALG_DES_ECB_NOPAD                = 0x10000011,
    /** DES_CBC_NOPAD */
    TEE_ALG_DES_CBC_NOPAD                = 0x10000111,
    /** DES_CBC_MAC_NOPAD */
    TEE_ALG_DES_CBC_MAC_NOPAD            = 0x30000111,
    /** DES_CBC_MAC_PKCS5 */
    TEE_ALG_DES_CBC_MAC_PKCS5            = 0x30000511,
    /** DES3_ECB_NOPAD */
    TEE_ALG_DES3_ECB_NOPAD               = 0x10000013,
    /** DES3_CBC_NOPAD */
    TEE_ALG_DES3_CBC_NOPAD               = 0x10000113,
    /** DES3_CBC_MAC_NOPAD */
    TEE_ALG_DES3_CBC_MAC_NOPAD           = 0x30000113,
    /** DES3_CBC_MAC_PKCS5 */
    TEE_ALG_DES3_CBC_MAC_PKCS5           = 0x30000513,
    /** RSASSA_PKCS1_V1_5_MD5 */
    TEE_ALG_RSASSA_PKCS1_V1_5_MD5        = 0x70001830,
    /** RSASSA_PKCS1_V1_5_SHA1 */
    TEE_ALG_RSASSA_PKCS1_V1_5_SHA1       = 0x70002830,
    /** RSASSA_PKCS1_V1_5_SHA224 */
    TEE_ALG_RSASSA_PKCS1_V1_5_SHA224     = 0x70003830,
    /** RSASSA_PKCS1_V1_5_SHA256 */
    TEE_ALG_RSASSA_PKCS1_V1_5_SHA256     = 0x70004830,
    /** RSASSA_PKCS1_V1_5_SHA384 */
    TEE_ALG_RSASSA_PKCS1_V1_5_SHA384     = 0x70005830,
    /** RSASSA_PKCS1_V1_5_SHA512 */
    TEE_ALG_RSASSA_PKCS1_V1_5_SHA512     = 0x70006830,
    /** RSASSA_PKCS1_PSS_MGF1_MD5 */
    TEE_ALG_RSASSA_PKCS1_PSS_MGF1_MD5    = 0x70111930,
    /** RSASSA_PKCS1_PSS_MGF1_SHA1 */
    TEE_ALG_RSASSA_PKCS1_PSS_MGF1_SHA1   = 0x70212930,
    /** RSASSA_PKCS1_PSS_MGF1_SHA224 */
    TEE_ALG_RSASSA_PKCS1_PSS_MGF1_SHA224 = 0x70313930,
    /** RSASSA_PKCS1_PSS_MGF1_SHA256 */
    TEE_ALG_RSASSA_PKCS1_PSS_MGF1_SHA256 = 0x70414930,
    /** RSASSA_PKCS1_PSS_MGF1_SHA384 */
    TEE_ALG_RSASSA_PKCS1_PSS_MGF1_SHA384 = 0x70515930,
    /** RSASSA_PKCS1_PSS_MGF1_SHA512 */
    TEE_ALG_RSASSA_PKCS1_PSS_MGF1_SHA512 = 0x70616930,
    /** RSAES_PKCS1_V1_5 */
    TEE_ALG_RSAES_PKCS1_V1_5             = 0x60000130,
    /** RSAES_PKCS1_OAEP_MGF1_SHA1 */
    TEE_ALG_RSAES_PKCS1_OAEP_MGF1_SHA1   = 0x60210230,
    /** RSAES_PKCS1_OAEP_MGF1_SHA224 */
    TEE_ALG_RSAES_PKCS1_OAEP_MGF1_SHA224 = 0x60211230,
    /** RSAES_PKCS1_OAEP_MGF1_SHA256 */
    TEE_ALG_RSAES_PKCS1_OAEP_MGF1_SHA256 = 0x60212230,
    /** RSAES_PKCS1_OAEP_MGF1_SHA384 */
    TEE_ALG_RSAES_PKCS1_OAEP_MGF1_SHA384 = 0x60213230,
    /** RSAES_PKCS1_OAEP_MGF1_SHA512 */
    TEE_ALG_RSAES_PKCS1_OAEP_MGF1_SHA512 = 0x60214230,
    /** RSA_NOPAD */
    TEE_ALG_RSA_NOPAD                    = 0x60000030,
    /** DSA_SHA1 */
    TEE_ALG_DSA_SHA1                     = 0x70002131,
    /** DSA_SHA224 */
    TEE_ALG_DSA_SHA224                   = 0x70003131,
    /** DSA_SHA256 */
    TEE_ALG_DSA_SHA256                   = 0x70004131,
    /** DH_DERIVE_SHARED_SECRET */
    TEE_ALG_DH_DERIVE_SHARED_SECRET      = 0x80000032,
    /** MD5 */
    TEE_ALG_MD5                          = 0x50000001,
    /** SHA1 */
    TEE_ALG_SHA1                         = 0x50000002,
    /** SHA224 */
    TEE_ALG_SHA224                       = 0x50000003,
    /** SHA256 */
    TEE_ALG_SHA256                       = 0x50000004,
    /** SHA384 */
    TEE_ALG_SHA384                       = 0x50000005,
    /** SHA512 */
    TEE_ALG_SHA512                       = 0x50000006,
    /** HMAC_MD5 */
    TEE_ALG_HMAC_MD5                     = 0x30000001,
    /** HMAC_SHA1 */
    TEE_ALG_HMAC_SHA1                    = 0x30000002,
    /** HMAC_SHA1 */
    TEE_ALG_HMAC_SHA224                  = 0x30000003,
    /** HMAC_SHA224 */
    TEE_ALG_HMAC_SHA256                  = 0x30000004,
    /** HMAC_SHA256 */
    TEE_ALG_HMAC_SHA384                  = 0x30000005,
    /** HMAC_SHA384 */
    TEE_ALG_HMAC_SHA512                  = 0x30000006,
    /** HMAC_SHA512 */
    TEE_ALG_HMAC_SM3                     = 0x30000007,
    /** HMAC_SM3 */
    TEE_ALG_AES_ECB_PKCS5                = 0x10000020,
    /** AES_ECB_PKCS5 */
    TEE_ALG_AES_CBC_PKCS5                = 0x10000220,
    /** AES_CBC_PKCS5 */
    TEE_ALG_ECDSA_SHA1                   = 0x70001042,
    /** ECDSA_SHA1 */
    TEE_ALG_ECDSA_SHA224                 = 0x70002042,
    /** ECDSA_SHA224 */
    TEE_ALG_ECDSA_SHA256                 = 0x70003042,
    /** ECDSA_SHA256 */
    TEE_ALG_ECDSA_SHA384                 = 0x70004042,
    /** ECDSA_SHA384 */
    TEE_ALG_ECDSA_SHA512                 = 0x70005042,
    /** ECDSA_SHA512 */
    TEE_ALG_ED25519                      = 0x70005043,
    /** ED25519 */
    TEE_ALG_ECDH_DERIVE_SHARED_SECRET    = 0x80000042,
    /** ECDH_DERIVE_SHARED_SECRET */
    TEE_ALG_X25519                       = 0x80000044,
    /** X25519 */
    TEE_ALG_ECC                          = 0x80000001,
    /** ECC */
    TEE_ALG_ECDSA_P192                   = 0x70001042,
    /** ECDSA_P192 */
    TEE_ALG_ECDSA_P224                   = 0x70002042,
    /** ECDSA_P224 */
    TEE_ALG_ECDSA_P256                   = 0x70003042,
    /** ECDSA_P256 */
    TEE_ALG_ECDSA_P384                   = 0x70004042,
    /** ECDSA_P521 */
    TEE_ALG_ECDSA_P521                   = 0x70005042,
    /** ECDH_P192 */
    TEE_ALG_ECDH_P192                    = 0x80001042,
    /** ECDH_P224 */
    TEE_ALG_ECDH_P224                    = 0x80002042,
    /** ECDH_P256 */
    TEE_ALG_ECDH_P256                    = 0x80003042,
    /** ECDH_P384 */
    TEE_ALG_ECDH_P384                    = 0x80004042,
    /** ECDH_P521 */
    TEE_ALG_ECDH_P521                    = 0x80005042,
#ifdef CONFIG_CRYPTO_SUPPORT_SIPHASH
    /** SIP_HASH */
    TEE_ALG_SIP_HASH                     = 0xF0000002,
#endif
    /** SM2_DSA_SM3 */
    TEE_ALG_SM2_DSA_SM3                  = 0x70006045,
    /** SM2_PKE */
    TEE_ALG_SM2_PKE                      = 0x80000045,
    /** SM3 */
    TEE_ALG_SM3                          = 0x50000007,
    /** SM4_ECB_NOPAD */
    TEE_ALG_SM4_ECB_NOPAD                = 0x10000014,
    /** SM4_CBC_NOPAD */
    TEE_ALG_SM4_CBC_NOPAD                = 0x10000114,
    /** SM4_CBC_PKCS7 */
    TEE_ALG_SM4_CBC_PKCS7                = 0xF0000003,
    /** SM4_CTR */
    TEE_ALG_SM4_CTR                      = 0x10000214,
    /** SM4_CFB128 */
    TEE_ALG_SM4_CFB128                   = 0xF0000000,
    /** SM4_XTS */
    TEE_ALG_SM4_XTS                      = 0x10000414,
    /** SM4_OFB */
    TEE_ALG_SM4_OFB                      = 0x10000514,
    /** AES_OFB */
    TEE_ALG_AES_OFB                      = 0x10000510,
    /** SM4_GCM */
    TEE_ALG_SM4_GCM                      = 0xF0000005,
};

/**
 * @see __tee_crypto_algorithm_id
 */
typedef enum __tee_crypto_algorithm_id tee_crypto_algorithm_id;
/**
 * @brief 无可选元素
 */
#define TEE_OPTIONAL_ELEMENT_NONE 0x00000000

/**
 * @brief 支持的ECC曲线
 */
typedef enum {
    /** CURVE_NIST_P192 */
    TEE_ECC_CURVE_NIST_P192 = 0x00000001,
    /** CURVE_NIST_P224 */
    TEE_ECC_CURVE_NIST_P224 = 0x00000002,
    /** CURVE_NIST_P256 */
    TEE_ECC_CURVE_NIST_P256 = 0x00000003,
    /** CURVE_NIST_P384 */
    TEE_ECC_CURVE_NIST_P384 = 0x00000004,
    /** CURVE_NIST_P521 */
    TEE_ECC_CURVE_NIST_P521 = 0x00000005,
    /** CURVE_SM2 256 bits */
    TEE_ECC_CURVE_SM2       = 0x00000300,
    /** CURVE_25519 256 bits */
    TEE_ECC_CURVE_25519     = 0x00000200,
} TEE_ECC_CURVE;

/**
 * @brief MGF1掩码函数类型
 */
typedef enum {
    TEE_DH_HASH_SHA1_mode   = 0,
    TEE_DH_HASH_SHA224_mode = 1,
    TEE_DH_HASH_SHA256_mode = 2,
    TEE_DH_HASH_SHA384_mode = 3,
    TEE_DH_HASH_SHA512_mode = 4,
    TEE_DH_HASH_NumOfModes,
} TEE_DH_HASH_Mode;

/**
 * @brief 加解密算法模式
 */
enum __TEE_OperationMode {
    /** 加密 */
    TEE_MODE_ENCRYPT = 0x0,
    /** 解密 */
    TEE_MODE_DECRYPT,
    /** 签名 */
    TEE_MODE_SIGN,
    /** 验签 */
    TEE_MODE_VERIFY,
    /** mac */
    TEE_MODE_MAC,
    /** 摘要 */
    TEE_MODE_DIGEST,
    /** 衍生 */
    TEE_MODE_DERIVE
};

/**
 * @brief 加解密operation状态
 */
enum tee_operation_state {
    /** 初始状态 */
    TEE_OPERATION_STATE_INITIAL = 0x00000000,
    /** 激活状态 */
    TEE_OPERATION_STATE_ACTIVE  = 0x00000001,
};

/**
 * @see __TEE_OperationMode
 */
typedef uint32_t TEE_OperationMode;

/**
 * @brief Operation信息
 */
struct __TEE_OperationInfo {
    /** 算法ID */
    uint32_t algorithm;        /* #__TEE_CRYPTO_ALGORITHM_ID */
    /** operation类型 */
    uint32_t operationClass;   /* #__TEE_Operation_Constants */
    /** Operation模式 */
    uint32_t mode;             /* #__TEE_OperationMode */
    /** 摘要长度 */
    uint32_t digestLength;
    /** 最大密钥长度 */
    uint32_t maxKeySize;
    /** 密钥长度 */
    uint32_t keySize;
    /** 所需密钥用法 */
    uint32_t requiredKeyUsage;
    /** 句柄状态 */
    uint32_t handleState;
    /** 密钥 */
    void *keyValue;
};

/**
 * @brief 用于定义__TEE_OperationInfo结构体类型
 *
 * @see __TEE_OperationInfo
 */
typedef struct __TEE_OperationInfo TEE_OperationInfo;

/**
 * @brief Operation中存放的密钥信息
 */
typedef struct {
    /** 密钥长度 */
    uint32_t keySize;
    /** 所需密钥用法 */
    uint32_t requiredKeyUsage;
} TEE_OperationInfoKey;

/**
 * @brief 包含了Operation中的密钥信息
 */
typedef struct {
    /** 算法ID */
    uint32_t algorithm;
    /** operation类型 */
    uint32_t operationClass;
    /** Operation模式 */
    uint32_t mode;
    /** 摘要长度 */
    uint32_t digestLength;
    /** 最大密钥长度 */
    uint32_t maxKeySize;
    /** 句柄状态 */
    uint32_t handleState;
    /** operation状态 */
    uint32_t operationState;
    /** 密钥数量 */
    uint32_t numberOfKeys;
    /** 密钥信息 */
    TEE_OperationInfoKey keyInformation[];
} TEE_OperationInfoMultiple;

/**
 * @brief 加解密操作需要的句柄
 */
struct __TEE_OperationHandle {
    /** 算法ID */
    uint32_t algorithm;        /* #__TEE_CRYPTO_ALGORITHM_ID */
    /** operation类型 */
    uint32_t operationClass;   /* #__TEE_Operation_Constants */
    /** Operation模式 */
    uint32_t mode;             /* #__TEE_OperationMode */
    /** 摘要长度 */
    uint32_t digestLength;
    /** 最大密钥长度 */
    uint32_t maxKeySize;
    /** 密钥长度 */
    uint32_t keySize;
    /** 密钥长度 */
    uint32_t keySize2;
    /** 所需密钥用法 */
    uint32_t requiredKeyUsage;
    /** 句柄状态 */
    uint32_t handleState;
    /** 密钥 */
    void *keyValue;
    /** 密钥 */
    void *keyValue2;
    /**  */
    void *crypto_ctxt;
    /**  */
    void *hmac_rest_ctext;
    /** iv */
    void *IV;
    /** 公钥 */
    void *publicKey;
    /** 公钥长度 */
    uint32_t publicKeyLen;
    /** 私钥 */
    void *privateKey;
    /** 私钥长度 */
    uint32_t privateKeyLen;
    /** iv长度 */
    uint32_t IVLen;
    /** operation锁 */
    pthread_mutex_t operation_lock;
    /** hal信息 */
    void *hal_info;
};

/**
 * @brief 整数类型转换
 */
typedef struct {
    /** 源 */
    uint32_t src;
    /** 目标 */
    uint32_t dest;
} crypto_uint2uint;

/**
 * @brief RSA公钥最大长度
 */
#define RSA_PUBKEY_MAXSIZE sizeof(CRYS_RSAUserPubKey_t)
/**
 * @brief RES私钥最大长度
 */
#define RSA_PRIVKEY_MAXSIZE sizeof(CRYS_RSAUserPrivKey_t)

/**
 * @brief 存放输入输出数据
 */
typedef struct {
    /** 源数据 */
    void *src_data;
    /** 源数据长度 */
    size_t src_len;
    /** 目标数据 */
    void *dest_data;
    /** 目标数据长度 */
    size_t *dest_len;
} operation_src_dest;

/**
 * @brief 存放ae算法初始化相关数据
 */
typedef struct {
    /** nonce */
    void *nonce;
    /** nonce长度 */
    size_t nonce_len;
    /** tag长度 */
    uint32_t tag_len;
    /** aad长度 */
    size_t aad_len;
    /** payload长度 */
    size_t payload_len;
} operation_ae_init;

/**
 * @brief 用于定义__TEE_OperationHandle指针类型
 *
 * @see __TEE_OperationHandle
 */
typedef struct __TEE_OperationHandle *TEE_OperationHandle;

/**
 * @brief 用于定义__TEE_OperationHandle结构体类型
 *
 * @see __TEE_OperationHandle
 */
typedef struct __TEE_OperationHandle TEE_OperationHandleVar;

/**
 * @brief 用于定义__TEE_ObjectHandle结构体类型
 */
typedef struct __TEE_ObjectHandle TEE_ObjectHandleVar;

/**
 * @brief 申请操作句柄
 *
 * @param operation [IN/OUT]操作句柄
 * @param algorithm [IN]加密算法ID
 * @param mode [IN]操作模式
 * @param maxKeySize [IN]最大密钥大小
 *
 * @return TEE_SUCCESS 成功
 * @return TEE_ERROR_OUT_OF_MEMORY 操作句柄申请失败
 * @return TEE_ERROR_NOT_SUPPORTE 加密算法ID不支持
 * @return TEE_ERROR_GENERIC 其它错误
 *
 */
TEE_Result TEE_AllocateOperation(TEE_OperationHandle *operation, uint32_t algorithm, uint32_t mode,
                                 uint32_t maxKeySize);

/**
 * @brief 释放操作句柄
 *
 * @param operation [IN/OUT]操作句柄
 *
 */
void TEE_FreeOperation(TEE_OperationHandle operation);

/**
 * @brief 获取操作信息
 *
 * @param operation [IN/OUT]操作句柄
 * @param operationInfo [IN/OUT]操作信息
 *
 */
void TEE_GetOperationInfo(const TEE_OperationHandle operation, TEE_OperationInfo *operationInfo);

/**
 * @brief 复位操作句柄
 *
 * @param operation [IN/OUT]操作句柄
 *
 */
void TEE_ResetOperation(TEE_OperationHandle operation);

/**
 * @brief 设置操作密钥
 *
 * @param operation [IN/OUT]操作句柄
 * @param key [IN/OUT]密钥
 *
 * @return TEE_SUCCESS 成功
 * @return TEE_ERROR_BAD_PARAMETERS 非法参数
 * @return TEE_ERROR_OUT_OF_MEMORY 密钥缓冲区申请失败
 *
 */
TEE_Result TEE_SetOperationKey(TEE_OperationHandle operation, const TEE_ObjectHandle key);

/**
 * @brief 设置操作密钥2
 *
 * @param operation [IN/OUT]操作句柄
 * @param key1 [IN/OUT]密钥1
 * @param key2 [IN/OUT]密钥2
 *
 * @return TEE_SUCCESS 成功
 * @return TEE_ERROR_BAD_PARAMETERS 非法参数
 *
 */
TEE_Result TEE_SetOperationKey2(TEE_OperationHandle operation, const TEE_ObjectHandle key1,
                                const TEE_ObjectHandle key2);

/**
 * @brief 复制操作句柄
 *
 * @param dstOperation [IN/OUT]目标操作句柄
 * @param srcOperation [IN/OUT]源操作句柄
 *
 */
void TEE_CopyOperation(TEE_OperationHandle dstOperation, const TEE_OperationHandle srcOperation);

/**
 * @brief 初始化密码上下文
 *
 * @param operation [IN/OUT]操作句柄
 * @param IV [IN]iv缓冲区，如果不使用设置为NULL
 * @param IVLen [IN]iv缓冲区的长度
 *
 */
void TEE_CipherInit(TEE_OperationHandle operation, const void *IV, size_t IVLen);

/**
 * @brief 执行密码更新
 *
 * @param operation [IN/OUT]操作句柄
 * @param srcData [IN]源数据
 * @param srcLen [IN]源数据长度
 * @param destData [OUT]目标数据
 * @param destLen [OUT]目标数据长度
 *
 * @return TEE_SUCCESS 成功
 * @return TEE_ERROR_BAD_PARAMETERS 非法参数
 * @return TEE_ERROR_GENERIC 其它错误
 *
 */
TEE_Result TEE_CipherUpdate(TEE_OperationHandle operation, const void *srcData, size_t srcLen, void *destData,
                            size_t *destLen);

/**
 * @brief 执行密码完成
 *
 * @param operation [IN/OUT]操作句柄
 * @param srcData [IN]源数据
 * @param srcLen [IN]源数据长度
 * @param destData [OUT]目标数据
 * @param destLen [OUT]目标数据长度
 *
 * @return TEE_SUCCESS 成功
 * @return TEE_ERROR_BAD_PARAMETERS 非法参数
 * @return TEE_ERROR_GENERIC 其它错误
 *
 */
TEE_Result TEE_CipherDoFinal(TEE_OperationHandle operation, const void *srcData, size_t srcLen, void *destData,
                             size_t *destLen);

/**
 * @brief 摘要更新
 *
 * @param operation [IN/OUT]操作句柄
 * @param chunk [IN]块缓冲区
 * @param chunkSize [IN]块缓冲区长度
 *
 */
void TEE_DigestUpdate(TEE_OperationHandle operation, const void *chunk, size_t chunkSize);

/**
 * @brief 执行摘要结束
 *
 * @param operation [IN/OUT]操作句柄
 * @param chunk [IN]块缓冲区
 * @param chunkLen [IN]块缓冲区大小
 * @param hash [out]哈希缓冲区
 * @param hashLen
 *
 */
TEE_Result TEE_DigestDoFinal(TEE_OperationHandle operation, const void *chunk, size_t chunkLen, void *hash,
                             size_t *hashLen);

/**
 * @brief 执行mac初始化
 *
 * @param operation [IN/OUT]操作句柄
 * @param IV [IN]iv缓冲区，如果不使用设置为NULL
 * @param IVLen [IN]iv缓冲区长度
 *
 */
void TEE_MACInit(TEE_OperationHandle operation, void *IV, size_t IVLen);

/**
 * @brief 执行mac更新
 *
 * @param operation [IN/OUT]操作句柄
 * @param chunk [IN]块缓冲区
 * @param chunkSize [IN]块缓冲区大小
 *
 */
void TEE_MACUpdate(TEE_OperationHandle operation, const void *chunk, size_t chunkSize);

/**
 * @brief mac计算完成
 *
 * @param operation [IN/OUT]操作句柄
 * @param message [IN]message缓冲区
 * @param messageLen [IN]message缓冲区的大小
 * @param mac [OUT]mac缓冲区
 * @param macLen [OUT]mac缓冲区大小
 *
 * @return TEE_SUCCESS 成功
 * @return TEE_ERROR_GENERIC 其它错误
 *
 */
TEE_Result TEE_MACComputeFinal(TEE_OperationHandle operation, const void *message, size_t messageLen, void *mac,
                               size_t *macLen);

/**
 * @brief mac比较完成
 *
 * @param operation [IN/OUT]操作句柄
 * @param message [IN]message缓冲区
 * @param messageLen [IN]message缓冲区大小
 * @param mac [OUT]mac缓冲区
 * @param macLen [OUT]mac缓冲区大小
 *
 * @return TEE_SUCCESS 成功
 * @return TEE_ERROR_GENERIC 其它错误
 * @return TEE_ERROR_MAC_INVALID 比较失败
 *
 */
TEE_Result TEE_MACCompareFinal(TEE_OperationHandle operation, const void *message, size_t messageLen, const void *mac,
                               const size_t macLen);

/**
 * @brief 派生密钥
 *
 * @param operation [IN/OUT]操作句柄
 * @param params [IN]属性
 * @param paramCount [IN]属性的数量
 * @param derivedKey [OUT]派生密钥
 *
 */
void TEE_DeriveKey(TEE_OperationHandle operation, const TEE_Attribute *params, uint32_t paramCount,
                   TEE_ObjectHandle derivedKey);

/**
 * @brief 生成随机数据
 *
 * @param randomBuffer [IN/OUT]随机缓冲区
 * @param randomBufferLen [IN]随机缓冲区大小
 *
 */
void TEE_GenerateRandom(void *randomBuffer, size_t randomBufferLen);

/**
 * @brief ae初始化
 *
 * @param operation [IN/OUT]操作句柄
 * @param nonce [IN]nonce缓冲区
 * @param nonceLen [IN]nonce缓冲区大小
 * @param tagLen [IN]tag的大小
 * @param AADLen [IN]aad的大小
 * @param payloadLen [IN]payload的大小
 *
 * @return TEE_SUCCESS 成功
 * @return TEE_ERROR_GENERIC 其它错误
 *
 */
TEE_Result TEE_AEInit(TEE_OperationHandle operation, void *nonce, size_t nonceLen, uint32_t tagLen, size_t AADLen,
                      size_t payloadLen);

/**
 * @brief 更新ae aad
 *
 * @param operation [IN/OUT]操作句柄
 * @param AADdata [IN]aad缓冲区
 * @param AADdataLen [IN]aad缓冲区大小
 *
 */
void TEE_AEUpdateAAD(TEE_OperationHandle operation, const void *AADdata, size_t AADdataLen);

/**
 * @brief 更新ae
 *
 * @param operation [IN/OUT]操作句柄
 * @param srcData [IN]源数据
 * @param srcLen [IN]源数据大小
 * @param destData [OUT]目标数据
 * @param destLen [OUT]目标数据大小
 *
 * @return TEE_SUCCESS 成功
 * @return TEE_ERROR_GENERIC 其它错误
 *
 */
TEE_Result TEE_AEUpdate(TEE_OperationHandle operation, void *srcData, size_t srcLen, void *destData, size_t *destLen);

/**
 * @brief ae加密
 *
 * @param operation [IN/OUT]操作句柄
 * @param srcData [IN]源数据
 * @param srcLen [IN]源数据长度
 * @param destData [OUT]目标数据
 * @param destLen [OUT]目标数据长度
 * @param tag [OUT]tag缓冲区
 * @param tagLen [OUT]tag缓冲区大小
 *
 * @return TEE_SUCCESS 成功
 * @return TEE_ERROR_GENERIC 其它错误
 *
 */
TEE_Result TEE_AEEncryptFinal(TEE_OperationHandle operation, void *srcData, size_t srcLen, void *destData,
                              size_t *destLen, void *tag, size_t *tagLen);

/**
 * @brief ae解密
 *
 * @param operation [IN/OUT]操作句柄
 * @param srcData [IN]源数据
 * @param srcLen [IN]源数据长度
 * @param destData [OUT]目标数据
 * @param destLen [OUT]目标数据长度
 * @param tag [OUT]tag缓冲区
 * @param tagLen[OUT]tag缓冲区大小
 *
 * @return TEE_SUCCESS 成功
 * @return TEE_ERROR_MAC_INVALID tag是非法的
 *
 */
TEE_Result TEE_AEDecryptFinal(TEE_OperationHandle operation, void *srcData, size_t srcLen, void *destData,
                              size_t *destLen, void *tag, size_t tagLen);

/**
 * @brief 非对称加密
 *
 * @param operation [IN/OUT]操作句柄
 * @param params [IN]属性
 * @param paramCount [IN]属性数量
 * @param srcData [IN]源数据
 * @param srcLen [IN]源数据长度
 * @param destData [OUT]目标数据
 * @param destLen [OUT]目标数据长度
 *
 * @return TEE_SUCCESS 成功
 * @return TEE_ERROR_BAD_PARAMETERS 非法参数
 * @return TEE_ERROR_GENERIC 其它错误
 *
 */
TEE_Result TEE_AsymmetricEncrypt(TEE_OperationHandle operation, const TEE_Attribute *params, uint32_t paramCount,
                                 void *srcData, size_t srcLen, void *destData, size_t *destLen);

/**
 * @brief 非对称解密
 *
 * @param operation [IN/OUT]操作句柄
 * @param params [IN]属性
 * @param paramCount [IN]属性数量
 * @param srcData [IN]源数据
 * @param srcLen [IN]源数据长度
 * @param destData [OUT]目标数据
 * @param destLen [OUT]目标数据长度
 *
 * @return TEE_SUCCESS 成功
 * @return TEE_ERROR_BAD_PARAMETERS 非法参数
 * @return TEE_ERROR_GENERIC 其它错误
 *
 */
TEE_Result TEE_AsymmetricDecrypt(TEE_OperationHandle operation, const TEE_Attribute *params, uint32_t paramCount,
                                 void *srcData, size_t srcLen, void *destData, size_t *destLen);

/**
 * @brief 非对称签名
 *
 * @param operation [IN/OUT]操作句柄
 * @param params [IN]属性
 * @param paramCount [IN]属性数量
 * @param digest [IN]摘要
 * @param digestLen [IN]摘要长度
 * @param signature [OUT]签名
 * @param signatureLen [OUT]签名长度
 *
 * @return TEE_SUCCESS 成功
 * @return TEE_ERROR_BAD_PARAMETERS 非法参数
 * @return TEE_ERROR_GENERIC 其它错误
 *
 */
TEE_Result TEE_AsymmetricSignDigest(TEE_OperationHandle operation, const TEE_Attribute *params, uint32_t paramCount,
                                    void *digest, size_t digestLen, void *signature, size_t *signatureLen);

/**
 * @brief 非对称验证
 *
 * @param operation [IN/OUT]操作句柄
 * @param params [IN]属性
 * @param paramCount [IN]属性数量
 * @param digest [IN]摘要
 * @param digestLen [IN]摘要长度
 * @param signature [OUT]签名
 * @param signatureLen [OUT]签名长度
 *
 * @return TEE_SUCCESS 成功
 * @return TEE_ERROR_BAD_PARAMETERS 非法参数
 * @return TEE_ERROR_GENERIC 其它错误
 *
 */
TEE_Result TEE_AsymmetricVerifyDigest(TEE_OperationHandle operation, const TEE_Attribute *params, uint32_t paramCount,
                                      void *digest, size_t digestLen, void *signature, size_t signatureLen);


/**
 * @brief 批量获取操作信息
 *
 * @param operation [IN/OUT]操作句柄
 * @param operationInfoMultiple [IN/OUT]批量操作信息
 * @param operationSize [IN/OUT]操作信息数量
 *
 * @return TEE_SUCCESS 成功
 * @return TEE_ERROR_BAD_PARAMETERS 非法参数
 * @return TEE_ERROR_SHORT_BUFFER 缓冲区不足
 *
 */
TEE_Result TEE_GetOperationInfoMultiple(TEE_OperationHandle operation, TEE_OperationInfoMultiple *operationInfoMultiple,
                                        const size_t *operationSize);

/**
 * @brief 检查算法是否被支持
 *
 * @param algId [IN]算法ID
 * @param element [IN]元素
 *
 * @return TEE_SUCCESS 支持
 * @return TEE_ERROR_NOT_SUPPORTED 不支持
 *
 */
TEE_Result TEE_IsAlgorithmSupported(uint32_t algId, uint32_t element);

#endif
