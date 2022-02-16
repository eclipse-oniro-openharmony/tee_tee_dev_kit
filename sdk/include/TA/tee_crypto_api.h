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
 * @brief �ӽ��ܽӿ�
 *
 * �����߿���ʹ����Щ�ӿ�ʵ�ּӽ��ܵ���ع��ܡ�
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
 * NULL����
 */
#define NULL ((void *)0)
#endif
/**
 * @brief ��Կ��󳤶ȣ���bitsΪ��λ��
 */
#define TEE_MAX_KEY_SIZE_IN_BITS      (1024 * 8)
/**
 * @brief SW_RSA��Կ����
 */
#define SW_RSA_KEYLEN                 1024
/**
 * @brief DH������Ϣ����󳤶�
 */
#define TEE_DH_MAX_SIZE_OF_OTHER_INFO 64 /* bytes */

/**
 * @brief �ӽ���Operation�������
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
 * @brief �ӽ����㷨��ʶ
 */
enum __tee_crypto_algorithm_id {
    /** ��Ч�㷨 */
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
 * @brief �޿�ѡԪ��
 */
#define TEE_OPTIONAL_ELEMENT_NONE 0x00000000

/**
 * @brief ֧�ֵ�ECC����
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
 * @brief MGF1���뺯������
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
 * @brief �ӽ����㷨ģʽ
 */
enum __TEE_OperationMode {
    /** ���� */
    TEE_MODE_ENCRYPT = 0x0,
    /** ���� */
    TEE_MODE_DECRYPT,
    /** ǩ�� */
    TEE_MODE_SIGN,
    /** ��ǩ */
    TEE_MODE_VERIFY,
    /** mac */
    TEE_MODE_MAC,
    /** ժҪ */
    TEE_MODE_DIGEST,
    /** ���� */
    TEE_MODE_DERIVE
};

/**
 * @brief �ӽ���operation״̬
 */
enum tee_operation_state {
    /** ��ʼ״̬ */
    TEE_OPERATION_STATE_INITIAL = 0x00000000,
    /** ����״̬ */
    TEE_OPERATION_STATE_ACTIVE  = 0x00000001,
};

/**
 * @see __TEE_OperationMode
 */
typedef uint32_t TEE_OperationMode;

/**
 * @brief Operation��Ϣ
 */
struct __TEE_OperationInfo {
    /** �㷨ID */
    uint32_t algorithm;        /* #__TEE_CRYPTO_ALGORITHM_ID */
    /** operation���� */
    uint32_t operationClass;   /* #__TEE_Operation_Constants */
    /** Operationģʽ */
    uint32_t mode;             /* #__TEE_OperationMode */
    /** ժҪ���� */
    uint32_t digestLength;
    /** �����Կ���� */
    uint32_t maxKeySize;
    /** ��Կ���� */
    uint32_t keySize;
    /** ������Կ�÷� */
    uint32_t requiredKeyUsage;
    /** ���״̬ */
    uint32_t handleState;
    /** ��Կ */
    void *keyValue;
};

/**
 * @brief ���ڶ���__TEE_OperationInfo�ṹ������
 *
 * @see __TEE_OperationInfo
 */
typedef struct __TEE_OperationInfo TEE_OperationInfo;

/**
 * @brief Operation�д�ŵ���Կ��Ϣ
 */
typedef struct {
    /** ��Կ���� */
    uint32_t keySize;
    /** ������Կ�÷� */
    uint32_t requiredKeyUsage;
} TEE_OperationInfoKey;

/**
 * @brief ������Operation�е���Կ��Ϣ
 */
typedef struct {
    /** �㷨ID */
    uint32_t algorithm;
    /** operation���� */
    uint32_t operationClass;
    /** Operationģʽ */
    uint32_t mode;
    /** ժҪ���� */
    uint32_t digestLength;
    /** �����Կ���� */
    uint32_t maxKeySize;
    /** ���״̬ */
    uint32_t handleState;
    /** operation״̬ */
    uint32_t operationState;
    /** ��Կ���� */
    uint32_t numberOfKeys;
    /** ��Կ��Ϣ */
    TEE_OperationInfoKey keyInformation[];
} TEE_OperationInfoMultiple;

/**
 * @brief �ӽ��ܲ�����Ҫ�ľ��
 */
struct __TEE_OperationHandle {
    /** �㷨ID */
    uint32_t algorithm;        /* #__TEE_CRYPTO_ALGORITHM_ID */
    /** operation���� */
    uint32_t operationClass;   /* #__TEE_Operation_Constants */
    /** Operationģʽ */
    uint32_t mode;             /* #__TEE_OperationMode */
    /** ժҪ���� */
    uint32_t digestLength;
    /** �����Կ���� */
    uint32_t maxKeySize;
    /** ��Կ���� */
    uint32_t keySize;
    /** ��Կ���� */
    uint32_t keySize2;
    /** ������Կ�÷� */
    uint32_t requiredKeyUsage;
    /** ���״̬ */
    uint32_t handleState;
    /** ��Կ */
    void *keyValue;
    /** ��Կ */
    void *keyValue2;
    /**  */
    void *crypto_ctxt;
    /**  */
    void *hmac_rest_ctext;
    /** iv */
    void *IV;
    /** ��Կ */
    void *publicKey;
    /** ��Կ���� */
    uint32_t publicKeyLen;
    /** ˽Կ */
    void *privateKey;
    /** ˽Կ���� */
    uint32_t privateKeyLen;
    /** iv���� */
    uint32_t IVLen;
    /** operation�� */
    pthread_mutex_t operation_lock;
    /** hal��Ϣ */
    void *hal_info;
};

/**
 * @brief ��������ת��
 */
typedef struct {
    /** Դ */
    uint32_t src;
    /** Ŀ�� */
    uint32_t dest;
} crypto_uint2uint;

/**
 * @brief RSA��Կ��󳤶�
 */
#define RSA_PUBKEY_MAXSIZE sizeof(CRYS_RSAUserPubKey_t)
/**
 * @brief RES˽Կ��󳤶�
 */
#define RSA_PRIVKEY_MAXSIZE sizeof(CRYS_RSAUserPrivKey_t)

/**
 * @brief ��������������
 */
typedef struct {
    /** Դ���� */
    void *src_data;
    /** Դ���ݳ��� */
    size_t src_len;
    /** Ŀ������ */
    void *dest_data;
    /** Ŀ�����ݳ��� */
    size_t *dest_len;
} operation_src_dest;

/**
 * @brief ���ae�㷨��ʼ���������
 */
typedef struct {
    /** nonce */
    void *nonce;
    /** nonce���� */
    size_t nonce_len;
    /** tag���� */
    uint32_t tag_len;
    /** aad���� */
    size_t aad_len;
    /** payload���� */
    size_t payload_len;
} operation_ae_init;

/**
 * @brief ���ڶ���__TEE_OperationHandleָ������
 *
 * @see __TEE_OperationHandle
 */
typedef struct __TEE_OperationHandle *TEE_OperationHandle;

/**
 * @brief ���ڶ���__TEE_OperationHandle�ṹ������
 *
 * @see __TEE_OperationHandle
 */
typedef struct __TEE_OperationHandle TEE_OperationHandleVar;

/**
 * @brief ���ڶ���__TEE_ObjectHandle�ṹ������
 */
typedef struct __TEE_ObjectHandle TEE_ObjectHandleVar;

/**
 * @brief ����������
 *
 * @param operation [IN/OUT]�������
 * @param algorithm [IN]�����㷨ID
 * @param mode [IN]����ģʽ
 * @param maxKeySize [IN]�����Կ��С
 *
 * @return TEE_SUCCESS �ɹ�
 * @return TEE_ERROR_OUT_OF_MEMORY �����������ʧ��
 * @return TEE_ERROR_NOT_SUPPORTE �����㷨ID��֧��
 * @return TEE_ERROR_GENERIC ��������
 *
 */
TEE_Result TEE_AllocateOperation(TEE_OperationHandle *operation, uint32_t algorithm, uint32_t mode,
                                 uint32_t maxKeySize);

/**
 * @brief �ͷŲ������
 *
 * @param operation [IN/OUT]�������
 *
 */
void TEE_FreeOperation(TEE_OperationHandle operation);

/**
 * @brief ��ȡ������Ϣ
 *
 * @param operation [IN/OUT]�������
 * @param operationInfo [IN/OUT]������Ϣ
 *
 */
void TEE_GetOperationInfo(const TEE_OperationHandle operation, TEE_OperationInfo *operationInfo);

/**
 * @brief ��λ�������
 *
 * @param operation [IN/OUT]�������
 *
 */
void TEE_ResetOperation(TEE_OperationHandle operation);

/**
 * @brief ���ò�����Կ
 *
 * @param operation [IN/OUT]�������
 * @param key [IN/OUT]��Կ
 *
 * @return TEE_SUCCESS �ɹ�
 * @return TEE_ERROR_BAD_PARAMETERS �Ƿ�����
 * @return TEE_ERROR_OUT_OF_MEMORY ��Կ����������ʧ��
 *
 */
TEE_Result TEE_SetOperationKey(TEE_OperationHandle operation, const TEE_ObjectHandle key);

/**
 * @brief ���ò�����Կ2
 *
 * @param operation [IN/OUT]�������
 * @param key1 [IN/OUT]��Կ1
 * @param key2 [IN/OUT]��Կ2
 *
 * @return TEE_SUCCESS �ɹ�
 * @return TEE_ERROR_BAD_PARAMETERS �Ƿ�����
 *
 */
TEE_Result TEE_SetOperationKey2(TEE_OperationHandle operation, const TEE_ObjectHandle key1,
                                const TEE_ObjectHandle key2);

/**
 * @brief ���Ʋ������
 *
 * @param dstOperation [IN/OUT]Ŀ��������
 * @param srcOperation [IN/OUT]Դ�������
 *
 */
void TEE_CopyOperation(TEE_OperationHandle dstOperation, const TEE_OperationHandle srcOperation);

/**
 * @brief ��ʼ������������
 *
 * @param operation [IN/OUT]�������
 * @param IV [IN]iv�������������ʹ������ΪNULL
 * @param IVLen [IN]iv�������ĳ���
 *
 */
void TEE_CipherInit(TEE_OperationHandle operation, const void *IV, size_t IVLen);

/**
 * @brief ִ���������
 *
 * @param operation [IN/OUT]�������
 * @param srcData [IN]Դ����
 * @param srcLen [IN]Դ���ݳ���
 * @param destData [OUT]Ŀ������
 * @param destLen [OUT]Ŀ�����ݳ���
 *
 * @return TEE_SUCCESS �ɹ�
 * @return TEE_ERROR_BAD_PARAMETERS �Ƿ�����
 * @return TEE_ERROR_GENERIC ��������
 *
 */
TEE_Result TEE_CipherUpdate(TEE_OperationHandle operation, const void *srcData, size_t srcLen, void *destData,
                            size_t *destLen);

/**
 * @brief ִ���������
 *
 * @param operation [IN/OUT]�������
 * @param srcData [IN]Դ����
 * @param srcLen [IN]Դ���ݳ���
 * @param destData [OUT]Ŀ������
 * @param destLen [OUT]Ŀ�����ݳ���
 *
 * @return TEE_SUCCESS �ɹ�
 * @return TEE_ERROR_BAD_PARAMETERS �Ƿ�����
 * @return TEE_ERROR_GENERIC ��������
 *
 */
TEE_Result TEE_CipherDoFinal(TEE_OperationHandle operation, const void *srcData, size_t srcLen, void *destData,
                             size_t *destLen);

/**
 * @brief ժҪ����
 *
 * @param operation [IN/OUT]�������
 * @param chunk [IN]�黺����
 * @param chunkSize [IN]�黺��������
 *
 */
void TEE_DigestUpdate(TEE_OperationHandle operation, const void *chunk, size_t chunkSize);

/**
 * @brief ִ��ժҪ����
 *
 * @param operation [IN/OUT]�������
 * @param chunk [IN]�黺����
 * @param chunkLen [IN]�黺������С
 * @param hash [out]��ϣ������
 * @param hashLen
 *
 */
TEE_Result TEE_DigestDoFinal(TEE_OperationHandle operation, const void *chunk, size_t chunkLen, void *hash,
                             size_t *hashLen);

/**
 * @brief ִ��mac��ʼ��
 *
 * @param operation [IN/OUT]�������
 * @param IV [IN]iv�������������ʹ������ΪNULL
 * @param IVLen [IN]iv����������
 *
 */
void TEE_MACInit(TEE_OperationHandle operation, void *IV, size_t IVLen);

/**
 * @brief ִ��mac����
 *
 * @param operation [IN/OUT]�������
 * @param chunk [IN]�黺����
 * @param chunkSize [IN]�黺������С
 *
 */
void TEE_MACUpdate(TEE_OperationHandle operation, const void *chunk, size_t chunkSize);

/**
 * @brief mac�������
 *
 * @param operation [IN/OUT]�������
 * @param message [IN]message������
 * @param messageLen [IN]message�������Ĵ�С
 * @param mac [OUT]mac������
 * @param macLen [OUT]mac��������С
 *
 * @return TEE_SUCCESS �ɹ�
 * @return TEE_ERROR_GENERIC ��������
 *
 */
TEE_Result TEE_MACComputeFinal(TEE_OperationHandle operation, const void *message, size_t messageLen, void *mac,
                               size_t *macLen);

/**
 * @brief mac�Ƚ����
 *
 * @param operation [IN/OUT]�������
 * @param message [IN]message������
 * @param messageLen [IN]message��������С
 * @param mac [OUT]mac������
 * @param macLen [OUT]mac��������С
 *
 * @return TEE_SUCCESS �ɹ�
 * @return TEE_ERROR_GENERIC ��������
 * @return TEE_ERROR_MAC_INVALID �Ƚ�ʧ��
 *
 */
TEE_Result TEE_MACCompareFinal(TEE_OperationHandle operation, const void *message, size_t messageLen, const void *mac,
                               const size_t macLen);

/**
 * @brief ������Կ
 *
 * @param operation [IN/OUT]�������
 * @param params [IN]����
 * @param paramCount [IN]���Ե�����
 * @param derivedKey [OUT]������Կ
 *
 */
void TEE_DeriveKey(TEE_OperationHandle operation, const TEE_Attribute *params, uint32_t paramCount,
                   TEE_ObjectHandle derivedKey);

/**
 * @brief �����������
 *
 * @param randomBuffer [IN/OUT]���������
 * @param randomBufferLen [IN]�����������С
 *
 */
void TEE_GenerateRandom(void *randomBuffer, size_t randomBufferLen);

/**
 * @brief ae��ʼ��
 *
 * @param operation [IN/OUT]�������
 * @param nonce [IN]nonce������
 * @param nonceLen [IN]nonce��������С
 * @param tagLen [IN]tag�Ĵ�С
 * @param AADLen [IN]aad�Ĵ�С
 * @param payloadLen [IN]payload�Ĵ�С
 *
 * @return TEE_SUCCESS �ɹ�
 * @return TEE_ERROR_GENERIC ��������
 *
 */
TEE_Result TEE_AEInit(TEE_OperationHandle operation, void *nonce, size_t nonceLen, uint32_t tagLen, size_t AADLen,
                      size_t payloadLen);

/**
 * @brief ����ae aad
 *
 * @param operation [IN/OUT]�������
 * @param AADdata [IN]aad������
 * @param AADdataLen [IN]aad��������С
 *
 */
void TEE_AEUpdateAAD(TEE_OperationHandle operation, const void *AADdata, size_t AADdataLen);

/**
 * @brief ����ae
 *
 * @param operation [IN/OUT]�������
 * @param srcData [IN]Դ����
 * @param srcLen [IN]Դ���ݴ�С
 * @param destData [OUT]Ŀ������
 * @param destLen [OUT]Ŀ�����ݴ�С
 *
 * @return TEE_SUCCESS �ɹ�
 * @return TEE_ERROR_GENERIC ��������
 *
 */
TEE_Result TEE_AEUpdate(TEE_OperationHandle operation, void *srcData, size_t srcLen, void *destData, size_t *destLen);

/**
 * @brief ae����
 *
 * @param operation [IN/OUT]�������
 * @param srcData [IN]Դ����
 * @param srcLen [IN]Դ���ݳ���
 * @param destData [OUT]Ŀ������
 * @param destLen [OUT]Ŀ�����ݳ���
 * @param tag [OUT]tag������
 * @param tagLen [OUT]tag��������С
 *
 * @return TEE_SUCCESS �ɹ�
 * @return TEE_ERROR_GENERIC ��������
 *
 */
TEE_Result TEE_AEEncryptFinal(TEE_OperationHandle operation, void *srcData, size_t srcLen, void *destData,
                              size_t *destLen, void *tag, size_t *tagLen);

/**
 * @brief ae����
 *
 * @param operation [IN/OUT]�������
 * @param srcData [IN]Դ����
 * @param srcLen [IN]Դ���ݳ���
 * @param destData [OUT]Ŀ������
 * @param destLen [OUT]Ŀ�����ݳ���
 * @param tag [OUT]tag������
 * @param tagLen[OUT]tag��������С
 *
 * @return TEE_SUCCESS �ɹ�
 * @return TEE_ERROR_MAC_INVALID tag�ǷǷ���
 *
 */
TEE_Result TEE_AEDecryptFinal(TEE_OperationHandle operation, void *srcData, size_t srcLen, void *destData,
                              size_t *destLen, void *tag, size_t tagLen);

/**
 * @brief �ǶԳƼ���
 *
 * @param operation [IN/OUT]�������
 * @param params [IN]����
 * @param paramCount [IN]��������
 * @param srcData [IN]Դ����
 * @param srcLen [IN]Դ���ݳ���
 * @param destData [OUT]Ŀ������
 * @param destLen [OUT]Ŀ�����ݳ���
 *
 * @return TEE_SUCCESS �ɹ�
 * @return TEE_ERROR_BAD_PARAMETERS �Ƿ�����
 * @return TEE_ERROR_GENERIC ��������
 *
 */
TEE_Result TEE_AsymmetricEncrypt(TEE_OperationHandle operation, const TEE_Attribute *params, uint32_t paramCount,
                                 void *srcData, size_t srcLen, void *destData, size_t *destLen);

/**
 * @brief �ǶԳƽ���
 *
 * @param operation [IN/OUT]�������
 * @param params [IN]����
 * @param paramCount [IN]��������
 * @param srcData [IN]Դ����
 * @param srcLen [IN]Դ���ݳ���
 * @param destData [OUT]Ŀ������
 * @param destLen [OUT]Ŀ�����ݳ���
 *
 * @return TEE_SUCCESS �ɹ�
 * @return TEE_ERROR_BAD_PARAMETERS �Ƿ�����
 * @return TEE_ERROR_GENERIC ��������
 *
 */
TEE_Result TEE_AsymmetricDecrypt(TEE_OperationHandle operation, const TEE_Attribute *params, uint32_t paramCount,
                                 void *srcData, size_t srcLen, void *destData, size_t *destLen);

/**
 * @brief �ǶԳ�ǩ��
 *
 * @param operation [IN/OUT]�������
 * @param params [IN]����
 * @param paramCount [IN]��������
 * @param digest [IN]ժҪ
 * @param digestLen [IN]ժҪ����
 * @param signature [OUT]ǩ��
 * @param signatureLen [OUT]ǩ������
 *
 * @return TEE_SUCCESS �ɹ�
 * @return TEE_ERROR_BAD_PARAMETERS �Ƿ�����
 * @return TEE_ERROR_GENERIC ��������
 *
 */
TEE_Result TEE_AsymmetricSignDigest(TEE_OperationHandle operation, const TEE_Attribute *params, uint32_t paramCount,
                                    void *digest, size_t digestLen, void *signature, size_t *signatureLen);

/**
 * @brief �ǶԳ���֤
 *
 * @param operation [IN/OUT]�������
 * @param params [IN]����
 * @param paramCount [IN]��������
 * @param digest [IN]ժҪ
 * @param digestLen [IN]ժҪ����
 * @param signature [OUT]ǩ��
 * @param signatureLen [OUT]ǩ������
 *
 * @return TEE_SUCCESS �ɹ�
 * @return TEE_ERROR_BAD_PARAMETERS �Ƿ�����
 * @return TEE_ERROR_GENERIC ��������
 *
 */
TEE_Result TEE_AsymmetricVerifyDigest(TEE_OperationHandle operation, const TEE_Attribute *params, uint32_t paramCount,
                                      void *digest, size_t digestLen, void *signature, size_t signatureLen);


/**
 * @brief ������ȡ������Ϣ
 *
 * @param operation [IN/OUT]�������
 * @param operationInfoMultiple [IN/OUT]����������Ϣ
 * @param operationSize [IN/OUT]������Ϣ����
 *
 * @return TEE_SUCCESS �ɹ�
 * @return TEE_ERROR_BAD_PARAMETERS �Ƿ�����
 * @return TEE_ERROR_SHORT_BUFFER ����������
 *
 */
TEE_Result TEE_GetOperationInfoMultiple(TEE_OperationHandle operation, TEE_OperationInfoMultiple *operationInfoMultiple,
                                        const size_t *operationSize);

/**
 * @brief ����㷨�Ƿ�֧��
 *
 * @param algId [IN]�㷨ID
 * @param element [IN]Ԫ��
 *
 * @return TEE_SUCCESS ֧��
 * @return TEE_ERROR_NOT_SUPPORTED ��֧��
 *
 */
TEE_Result TEE_IsAlgorithmSupported(uint32_t algId, uint32_t element);

#endif
