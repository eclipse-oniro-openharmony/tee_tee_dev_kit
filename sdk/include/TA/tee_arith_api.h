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
 * @file tee_arith_api.h
 *
 * @brief ���������ӿ�
 *
 * @since 1
 */

#ifndef TEE_ARITH_API_H
#define TEE_ARITH_API_H

#include <tee_defines.h>

/**
 * below definitions are defined by Global Platform
 * for compatibility:
 * don't make any change to the content below
 */
typedef uint32_t TEE_BigInt;
typedef uint32_t TEE_BigIntFMM;
typedef uint32_t TEE_BigIntFMMContext;

/**
 * @brief ����u32�е�BigInt�Ĵ�С
 *
 * @param n [IN]TEE_BigInt����
 *
 * @return 32�е�BigInt�Ĵ�С
 *
 */
#define TEE_BigIntSizeInU32(n) ((((n) + 31) / 32) + 2)

/**
 * @brief ����uint32_tֵ����Ĵ�С
 *
 * @param modulusSizeInBits [IN]ģ���Ĵ�С����bitsΪ��λ��
 *
 * @return ��������ģ��modSizeInBits���洢TEE_BigIntFMM������ֽ���
 *
 */
size_t TEE_BigIntFMMSizeInU32(size_t modulusSizeInBits);

/**
 * @brief ���ر�ʾ����ģ�黯�����������uint32_tֵ����Ĵ�С
 *
 * @param modulusSizeInBits [IN]ģ���Ĵ�С����bitsΪ��λ��
 *
 * @return �洢TEE_BigIntFMMContext������ֽ�������������ģ��modSizeInBits
 *
 */
size_t TEE_BigIntFMMContextSizeInU32(size_t modulusSizeInBits);

/**
 * @brief ��ʼ��bigInt
 *
 * @param bigInt [OUT]ָ��Ҫ��ʼ����TEE_BigInt��ָ��
 * @param len [IN]bigIntָ����ڴ�Ĵ�С����λΪuint32_t
 *
 */
void TEE_BigIntInit(TEE_BigInt *bigInt, size_t len);

/**
 * @brief �������ģ�˷��ı�Ҫ�Ⱦ��������������Ǵ洢����������
 *
 * @param context [OUT]Ҫ��ʼ����TEE_BigIntFMMContextָ��
 * @param len [IN]������ָ����ڴ�Ĵ�С����uint32_tΪ��λ��
 * @param modulus [IN]ģ��
 *
 */
void TEE_BigIntInitFMMContext(TEE_BigIntFMMContext *context, size_t len, const TEE_BigInt *modulus);


/**
 * @brief �������ģ�˷��ı�Ҫ�Ⱦ��������������Ǵ洢����������
 *
 * @param context [OUT]Ҫ��ʼ����TEE_BigIntFMMContextָ��
 * @param len [IN]������ָ����ڴ�Ĵ�С����uint32_tΪ��λ��
 * @param modulus [IN]ģ��
 *
 * @return TEE_SUCCESS �ɹ�
 * @return ��������ֵ ʧ��
 *
 */
TEE_Result TEE_BigIntInitFMMContext1(TEE_BigIntFMMContext *context, size_t len, const TEE_BigInt *modulus);

/**
 * @brief ��ʼ��bigIntFMM�������ʾ��ֵ����Ϊ��
 *
 * @param bigIntFMM [IN]Ҫ��ʼ����TEE_BigIntFMMָ��
 * @param len [IN]bigIntFMMָ����ڴ��С����λΪuint32_t
 *
 */
void TEE_BigIntInitFMM(TEE_BigIntFMM *bigIntFMM, size_t len);

/**
 * @brief ��bufferLen�ֽڰ�λ�ֽ��ַ���������ת��ΪTEE_BigInt��ʽ
 *
 * @param dest [OUT]ָ�����ڱ�������TEE_BigInt��ָ��
 * @param buffer [IN]ָ����������İ�λ�ֽ��ַ�����ʾ��ʽ�Ļ�������ָ��
 * @param bufferLen [IN]buffer�ĳ��ȣ����ֽ�Ϊ��λ��
 * @param sign [IN]dest�ı�־������Ϊ��־�ı�־
 *
 * @return TEE_SUCCESS ֧��
 * @return TEE_ERROR_OVERFLOW Ϊdest������ڴ�̫С
 *
 */
TEE_Result TEE_BigIntConvertFromOctetString(TEE_BigInt *dest, const uint8_t *buffer, size_t bufferLen, int32_t sign);

/**
 * @brief ��TEE_BigInt��ʽ�������ľ���ֵת��Ϊ��λ�ֽ��ַ���
 *
 * @param buffer [OUT]д��������ת����λ�ֽ��ַ�����ʾ�����������
 * @param bufferLen [IN]buffer�ĳ��ȣ����ֽ�Ϊ��λ��
 * @param bigInt [IN]ָ��ת��Ϊ��λ�ֽ��ַ�����������ָ��
 *
 * @return TEE_SUCCESS ֧��
 * @return TEE_ERROR_SHORT_BUFFER ���������̫С���޷�������λ�ֽ��ַ���
 *
 */
TEE_Result TEE_BigIntConvertToOctetString(void *buffer, size_t *bufferLen, const TEE_BigInt *bigInt);

/**
 * @brief ��*dest����ΪֵshortVal
 *
 * @param dest [OUT]ָ�����ڴ洢�����TEE_BigInt��ָ��
 * @param shortVal [IN]����ֵ
 *
 */
void TEE_BigIntConvertFromS32(TEE_BigInt *dest, int32_t shortVal);

/**
 * @brief ��*dest����Ϊsrc��ֵ������src�ķ���
 *
 * @param dest [OUT]ָ�����ڴ洢�����int32_t��ָ��
 * @param src [IN]����ֵָ��
 *
 * @return TEE_SUCCESS ֧��
 * @return TEE_ERROR_OVERFLOW src���ʺ�int32_t
 *
 */
TEE_Result TEE_BigIntConvertToS32(int32_t *dest, const TEE_BigInt *src);

/**
 * @brief ���op1>op2��op1==op2��op1<op2
 *
 * @param op1 [IN]ָ���һ����������ָ��
 * @param op2 [IN]ָ��ڶ�����������ָ��
 *
 * @return 0 op1==op2
 * @return ���� op1>op2
 *
 */
int32_t TEE_BigIntCmp(const TEE_BigInt *op1, const TEE_BigInt *op2);

/**
 * @brief �����op>shortVal��op==shortVal��op<shortVal
 *
 * @param op [IN]ָ���һ����������ָ��
 * @param shortVal [IN]ָ��ڶ�����������ָ��
 *
 * @return 0 op1==shortVal
 * @return ���� op1>shortVal
 *
 */
int32_t TEE_BigIntCmpS32(const TEE_BigInt *op, int32_t shortVal);

/**
 * @brief ���� |dest| = |op| >> bits
 *
 * @param dest [OUT]ָ��TEE_BigInt��ָ�룬���ڱ�����λ�Ľ��
 * @param op [IN]ָ��Ҫ��λ�Ĳ�������ָ��
 * @param bits [IN]Ҫ��λ��λ��
 *
 */
void TEE_BigIntShiftRight(TEE_BigInt *dest, const TEE_BigInt *op, size_t bits);

/**
 * @brief ����|src|����Ȼ�����Ʊ�ʾ��bitIndexλ
 *
 * @param src [IN]����ָ��
 * @param bitIndex [IN]Ҫ��ȡ��λ��ƫ�������������Чλ��ƫ����0��ʼ
 *
 * @return true |src|��bitIndexthλ�Ĳ���ֵΪ��1��
 * @return false |src|��bitIndexthλ�Ĳ���ֵΪ��0��
 *
 */
bool TEE_BigIntGetBit(const TEE_BigInt *src, uint32_t bitIndex);

/**
 * @brief ����|src|����Ȼ�����Ʊ�ʾ�е�λ������src�Ĵ�С
 *
 * @param src [IN]����ָ��
 *
 * @return 0 src=0
 * @return src����Ȼ�����Ʊ�ʾ�е�λ����
 *
 */
uint32_t TEE_BigIntGetBitCount(const TEE_BigInt *src);

#if defined(API_LEVEL) && (API_LEVEL >= API_LEVEL1_2)
/**
 * @brief ��op����Ȼ�����Ʊ�ʾ��bitIndex��1λ����Ϊ1��0
 *
 * @param op [IN/OUT]����ָ��
 * @param bitIndex [IN]Ҫ���õ�λ��ƫ�������������Чλ��ƫ����0��ʼ
 * @param value [IN]Ҫ���õ�λֵ������true��ʾ��1����false��ʾ��0��
 *
 * @return TEE_SUCCESS ֧��
 * @return TEE_ERROR_OVERFLOW bitIndexthλ����op�ķ���λ����
 *
 */
TEE_Result TEE_BigIntSetBit(TEE_BigInt *op, uint32_t bitIndex, bool value);

/**
 * @brief ��src��ֵ��ֵ��dest
 *
 * @param dest [OUT]Ҫ�����TEE_BigIntָ��
 * @param src [IN]ָ��Դ��������ָ��
 *
 * @return TEE_SUCCESS ֧��
 * @return TEE_ERROR_OVERFLOW ���dest��������������src��ֵ
 *
 */
TEE_Result TEE_BigIntAssign(TEE_BigInt *dest, const TEE_BigInt *src);

/**
 * @brief ��src��ֵ����dest
 *
 * @param dest [OUT]Ҫ�����TEE_BigIntָ��
 * @param src [IN]ָ��Դ��������ָ��
 *
 * @return TEE_SUCCESS ֧��
 * @return TEE_ERROR_OVERFLOW ���dest��������������src��ֵ
 *
 */
TEE_Result TEE_BigIntAbs(TEE_BigInt *dest, const TEE_BigInt *src);
#endif /* API_LEVEL */

/**
 * @brief ���� dest= op1 + op2
 *
 * @param dest [OUT]ָ��TEE_BigInt��ָ�룬���ڴ洢���op1 + op2
 * @param op1 [IN]ָ���һ����������ָ��
 * @param op2 [IN]ָ��ڶ�����������ָ��
 *
 */
void TEE_BigIntAdd(TEE_BigInt *dest, const TEE_BigInt *op1, const TEE_BigInt *op2);

/**
 * @brief ���� dest= op1 - op2
 *
 * @param dest [OUT]ָ��TEE_BigInt��ָ�룬���ڴ洢���op1 - op2
 * @param op1 [IN]ָ���һ����������ָ��
 * @param op2 [IN]ָ��ڶ�����������ָ��
 *
 */
void TEE_BigIntSub(TEE_BigInt *dest, const TEE_BigInt *op1, const TEE_BigInt *op2);

/**
 * @brief ȡ����������dest = -op
 *
 * @param dest [OUT]Pָ��TEE_BigInt��ָ�룬���ڴ洢���-op
 * @param op [IN]ָ��Ҫȡ���Ĳ�������ָ��
 *
 */
void TEE_BigIntNeg(TEE_BigInt *dest, const TEE_BigInt *op);

/**
 * @brief ���� dest = op1 * op2
 *
 * @param dest [OUT]ָ��TEE_BigInt��ָ�룬���ڴ洢���op1 * op2
 * @param op1 [IN]ָ���һ����������ָ��
 * @param op2 [IN]ָ��ڶ�����������ָ��
 *
 */
void TEE_BigIntMul(TEE_BigInt *dest, const TEE_BigInt *op1, const TEE_BigInt *op2);

/**
 * @brief ���� dest = op * op
 *
 * @param dest [OUT]ָ��TEE_BigInt��ָ�룬���ڴ洢���op * op
 * @param op [IN]ָ��Ҫƽ���Ĳ�������ָ��
 *
 */
void TEE_BigIntSquare(TEE_BigInt *dest, const TEE_BigInt *op);

/**
 * @brief ����dest_r��dest_q��ʹ��op1 = dest_q * op2 + dest_r
 *
 * @param dest_q [OUT]ָ��TEE_BigInt��ָ�룬���ڴ洢��
 * @param dest_r [IN]ָ��TEE_BigInt��ָ�룬���ڴ洢����
 * @param op1 [OUT]ָ���һ����������ָ�룬������
 * @param op2 [IN]ָ��ڶ�����������ָ�룬����
 *
 * @return TEE_SUCCESS �����ɹ�
 * @return TEE_ERROR_BAD_PARAMETERS ���д�������һ������ΪNULL
 *
 */
void TEE_BigIntDiv(TEE_BigInt *dest_q, TEE_BigInt *dest_r, const TEE_BigInt *op1, const TEE_BigInt *op2);

/**
 * @brief ����dest = op (mod n)��ʹ��0 <= dest < n
 *
 * @param dest [OUT]ָ��TEE_BigInt��ָ�룬�Ա�����op (mod n)
 * @param op [IN]ָ��Ҫ����mod n�Ĳ�������ָ��
 * @param n [IN]ָ��ģ����ָ�롣ģ��Ӧ����1
 *
 */
void TEE_BigIntMod(TEE_BigInt *dest, const TEE_BigInt *op, const TEE_BigInt *n);

/**
 * @brief ����dest= (op1 + op2) (mod n)
 *
 * @param dest [OUT]ָ��TEE_BigInt��ָ�룬�Ա�����(op1 + op2)(mod n)
 * @param op1 [IN]ָ���һ����������ָ��
 * @param op2 [IN]ָ��ڶ�����������ָ��
 * @param n [IN]ָ��ģ����ָ�룬ģ��Ӧ����1
 *
 */
void TEE_BigIntAddMod(TEE_BigInt *dest, const TEE_BigInt *op1, const TEE_BigInt *op2, const TEE_BigInt *n);

/**
 * @brief ����dest = (op1 - op2) (mod n)
 *
 * @param dest [OUT]ָ��TEE_BigInt��ָ�룬�Ա�����(op1 - op2)(mod n)
 * @param op1 [IN]ָ���һ����������ָ��
 * @param op2 [IN]ָ��ڶ�����������ָ��
 * @param n [IN]ָ��ģ����ָ�룬ģ��Ӧ����1
 *
 */
void TEE_BigIntSubMod(TEE_BigInt *dest, const TEE_BigInt *op1, const TEE_BigInt *op2, const TEE_BigInt *n);

/**
 * @brief ����dest = (op1 * op2) (mod n)
 *
 * @param dest [OUT]ָ��TEE_BigInt��ָ�룬�Ա�����(op1 * op2)(mod n)
 * @param op1 [IN]ָ���һ����������ָ��
 * @param op2 [IN]ָ��ڶ�����������ָ��
 * @param n [IN]ָ��ģ����ָ�룬ģ��Ӧ����1
 *
 */
void TEE_BigIntMulMod(TEE_BigInt *dest, const TEE_BigInt *op1, const TEE_BigInt *op2, const TEE_BigInt *n);

/**
 * @brief ����dest = (op * op) (mod n)
 *
 * @param dest [OUT]ָ��TEE_BigInt��ָ�룬�Ա�����(op * op)(mod n)
 * @param op [IN]������ָ��
 * @param n [IN]ָ��ģ����ָ�롣ģ��Ӧ����1
 *
 */
void TEE_BigIntSquareMod(TEE_BigInt *dest, const TEE_BigInt *op, const TEE_BigInt *n);

/**
 * @brief ����dest��ʹdest * op = 1 (mod n)
 *
 * @param dest [OUT]ָ��TEE_BigInt��ָ�룬�Ա�������op^-1��(mod n)
 * @param op [IN]������ָ��
 * @param n [IN]ָ��ģ����ָ�롣ģ��Ӧ����1
 *
 */
void TEE_BigIntInvMod(TEE_BigInt *dest, const TEE_BigInt *op, const TEE_BigInt *n);

/**
 * @brief ȷ���Ƿ�gcd(op1, op2) == 1
 *
 * @param op1 [IN]ָ���һ����������ָ��
 * @param op2 [IN]ָ��ڶ�����������ָ��
 *
 * @return true gcd(op1, op2) == 1
 * @return false gcd(op1, op2) != 1
 *
 */
bool TEE_BigIntRelativePrime(const TEE_BigInt *op1, const TEE_BigInt *op2);

/**
 * @brief �����������op1��op2�����Լ��
 *
 * @param gcd [OUT]ָ��TEE_BigInt��ָ�룬���ڱ���op1��op2�����Լ��
 * @param u [OUT]ָ��TEE_BigInt��ָ�룬���ڱ����һ��ϵ��
 * @param v [OUT]ָ��TEE_BigInt��ָ�룬���ڱ���ڶ���ϵ��
 * @param op1 [IN]ָ���һ����������ָ��
 * @param op2 [IN]ָ��ڶ�����������ָ��
 *
 */
void TEE_BigIntComputeExtendedGcd(TEE_BigInt *gcd, TEE_BigInt *u, TEE_BigInt *v, const TEE_BigInt *op1,
                                  const TEE_BigInt *op2);
/**
 * @brief ��opִ�и���ԭʼ�Լ���
 *
 * @param op [IN]����ԭʼ���Եĺ�ѡ����
 * @param confidenceLevel [IN]�ǽ����Բ��Ե���������ˮƽ
 *
 * @return 0 op�Ǹ�����
 * @return 1 op������
 * @return -1 �����Ƿǽ����Եģ���op�Ǹ��ϵĸ���С��2^��-confidenceLevel��
 *
 */
int32_t TEE_BigIntIsProbablePrime(const TEE_BigInt *op, uint32_t confidenceLevel);

/**
 * @brief ��srcת��Ϊ�ʺϽ��п���ģ�˵ı�ʾ
 *
 * @param dest [OUT]TEE_BigIntFMM��ʼ���ڴ�����ָ��
 * @param src [IN]ָ��Ҫת����TEE_BigInt��ָ��
 * @param n [IN]ģ��ָ��
 * @param context [IN]ָ����ǰʹ��TEE_BigIntInitFMMContext1��ʼ���������ĵ�ָ��
 *
 */
void TEE_BigIntConvertToFMM(TEE_BigIntFMM *dest, const TEE_BigInt *src, const TEE_BigInt *n,
                            const TEE_BigIntFMMContext *context);

/**
 * @brief ������ģ�˱�ʾ�е�srcת����TEE_BigInt��ʾ
 *
 * @param dest [OUT]ָ�����ڱ���ת������ĳ�ʼ��TEE_BigIntFMM�ڴ������ָ��
 * @param src [IN]ָ�򱣴����ģ�˱�ʾ��ֵ��TEE_BigIntFMM��ָ��
 * @param n [IN]ģ��ָ��
 * @param context [IN]ָ����ǰʹ��TEE_BigIntInitFMMContext1��ʼ���������ĵ�ָ��
 *
 */
void TEE_BigIntConvertFromFMM(TEE_BigInt *dest, const TEE_BigIntFMM *src, const TEE_BigInt *n,
                              const TEE_BigIntFMMContext *context);

/**
 * @brief �������ģ�˷���ʾ�е�dest = op1 * op2
 *
 * @param dest [OUT]ָ��TEE_BigIntFMM��ָ�룬���ڱ�����op1 * op2
 * @param op1 [IN]ָ���һ����������ָ��
 * @param op2 [IN]ָ��ڶ�����������ָ��
 * @param n [IN]ģ��ָ��
 * @param context[IN]ָ����ǰʹ��TEE_BigIntInitFMMContext1��ʼ���������ĵ�ָ��
 *
 */
void TEE_BigIntComputeFMM(TEE_BigIntFMM *dest, const TEE_BigIntFMM *op1, const TEE_BigIntFMM *op2, const TEE_BigInt *n,
                          const TEE_BigIntFMMContext *context);

/**
 * @brief ����dest = (op1 ^ op2) (mod n)
 *
 * @param des [OUT]ָ��TEE_BigInt��ָ�룬�Ա�����(op1 ^ op2)(mod n)
 * @param op1 [IN]ָ���һ����������ָ��
 * @param op2 [IN]ָ��ڶ�����������ָ��
 * @param n [IN]ģ��ָ��
 * @param context [IN]ָ����ǰʹ��TEE_BigIntInitFMMContext1��NULL��ʼ���������ĵ�ָ��
 *
 * @return TEE_SUCCESS �ɹ�
 * @return TEE_ERROR_NOT_SUPPORTED ��֧��n��ֵ
 *
 */
TEE_Result TEE_BigIntExpMod(TEE_BigInt *des, TEE_BigInt *op1, const TEE_BigInt *op2, const TEE_BigInt *n,
                            TEE_BigIntFMMContext *context);

#endif
