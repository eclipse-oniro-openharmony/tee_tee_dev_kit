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
 * @brief 大数操作接口
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
 * @brief 返回u32中的BigInt的大小
 *
 * @param n [IN]TEE_BigInt类型
 *
 * @return 32中的BigInt的大小
 *
 */
#define TEE_BigIntSizeInU32(n) ((((n) + 31) / 32) + 2)

/**
 * @brief 返回uint32_t值数组的大小
 *
 * @param modulusSizeInBits [IN]模数的大小（以bits为单位）
 *
 * @return 给定长度模数modSizeInBits，存储TEE_BigIntFMM所需的字节数
 *
 */
size_t TEE_BigIntFMMSizeInU32(size_t modulusSizeInBits);

/**
 * @brief 返回表示快速模块化上下文所需的uint32_t值数组的大小
 *
 * @param modulusSizeInBits [IN]模数的大小（以bits为单位）
 *
 * @return 存储TEE_BigIntFMMContext所需的字节数，给定长度模数modSizeInBits
 *
 */
size_t TEE_BigIntFMMContextSizeInU32(size_t modulusSizeInBits);

/**
 * @brief 初始化bigInt
 *
 * @param bigInt [OUT]指向要初始化的TEE_BigInt的指针
 * @param len [IN]bigInt指向的内存的大小，单位为uint32_t
 *
 */
void TEE_BigIntInit(TEE_BigInt *bigInt, size_t len);

/**
 * @brief 计算快速模乘法的必要先决条件，并将它们存储在上下文中
 *
 * @param context [OUT]要初始化的TEE_BigIntFMMContext指针
 * @param len [IN]上下文指向的内存的大小（以uint32_t为单位）
 * @param modulus [IN]模数
 *
 */
void TEE_BigIntInitFMMContext(TEE_BigIntFMMContext *context, size_t len, const TEE_BigInt *modulus);


/**
 * @brief 计算快速模乘法的必要先决条件，并将它们存储在上下文中
 *
 * @param context [OUT]要初始化的TEE_BigIntFMMContext指针
 * @param len [IN]上下文指向的内存的大小（以uint32_t为单位）
 * @param modulus [IN]模数
 *
 * @return TEE_SUCCESS 成功
 * @return 其它返回值 失败
 *
 */
TEE_Result TEE_BigIntInitFMMContext1(TEE_BigIntFMMContext *context, size_t len, const TEE_BigInt *modulus);

/**
 * @brief 初始化bigIntFMM并将其表示的值设置为零
 *
 * @param bigIntFMM [IN]要初始化的TEE_BigIntFMM指针
 * @param len [IN]bigIntFMM指向的内存大小，单位为uint32_t
 *
 */
void TEE_BigIntInitFMM(TEE_BigIntFMM *bigIntFMM, size_t len);

/**
 * @brief 将bufferLen字节八位字节字符串缓冲区转换为TEE_BigInt格式
 *
 * @param dest [OUT]指向用于保存结果的TEE_BigInt的指针
 * @param buffer [IN]指向包含整数的八位字节字符串表示形式的缓冲区的指针
 * @param bufferLen [IN]buffer的长度（以字节为单位）
 * @param sign [IN]dest的标志被设置为标志的标志
 *
 * @return TEE_SUCCESS 支持
 * @return TEE_ERROR_OVERFLOW 为dest分配的内存太小
 *
 */
TEE_Result TEE_BigIntConvertFromOctetString(TEE_BigInt *dest, const uint8_t *buffer, size_t bufferLen, int32_t sign);

/**
 * @brief 将TEE_BigInt格式的整数的绝对值转换为八位字节字符串
 *
 * @param buffer [OUT]写入整数的转换八位字节字符串表示的输出缓冲区
 * @param bufferLen [IN]buffer的长度（以字节为单位）
 * @param bigInt [IN]指向将转换为八位字节字符串的整数的指针
 *
 * @return TEE_SUCCESS 支持
 * @return TEE_ERROR_SHORT_BUFFER 输出缓冲区太小，无法包含八位字节字符串
 *
 */
TEE_Result TEE_BigIntConvertToOctetString(void *buffer, size_t *bufferLen, const TEE_BigInt *bigInt);

/**
 * @brief 将*dest设置为值shortVal
 *
 * @param dest [OUT]指向用于存储结果的TEE_BigInt的指针
 * @param shortVal [IN]输入值
 *
 */
void TEE_BigIntConvertFromS32(TEE_BigInt *dest, int32_t shortVal);

/**
 * @brief 将*dest设置为src的值，包括src的符号
 *
 * @param dest [OUT]指向用于存储结果的int32_t的指针
 * @param src [IN]输入值指针
 *
 * @return TEE_SUCCESS 支持
 * @return TEE_ERROR_OVERFLOW src不适合int32_t
 *
 */
TEE_Result TEE_BigIntConvertToS32(int32_t *dest, const TEE_BigInt *src);

/**
 * @brief 检查op1>op2、op1==op2或op1<op2
 *
 * @param op1 [IN]指向第一个操作数的指针
 * @param op2 [IN]指向第二个操作数的指针
 *
 * @return 0 op1==op2
 * @return 正数 op1>op2
 *
 */
int32_t TEE_BigIntCmp(const TEE_BigInt *op1, const TEE_BigInt *op2);

/**
 * @brief 检查是op>shortVal、op==shortVal或op<shortVal
 *
 * @param op [IN]指向第一个操作数的指针
 * @param shortVal [IN]指向第二个操作数的指针
 *
 * @return 0 op1==shortVal
 * @return 正数 op1>shortVal
 *
 */
int32_t TEE_BigIntCmpS32(const TEE_BigInt *op, int32_t shortVal);

/**
 * @brief 计算 |dest| = |op| >> bits
 *
 * @param dest [OUT]指向TEE_BigInt的指针，用于保存移位的结果
 * @param op [IN]指向要移位的操作数的指针
 * @param bits [IN]要移位的位数
 *
 */
void TEE_BigIntShiftRight(TEE_BigInt *dest, const TEE_BigInt *op, size_t bits);

/**
 * @brief 返回|src|的自然二进制表示的bitIndex位
 *
 * @param src [IN]整数指针
 * @param bitIndex [IN]要读取的位的偏移量，从最低有效位的偏移量0开始
 *
 * @return true |src|中bitIndexth位的布尔值为“1”
 * @return false |src|中bitIndexth位的布尔值为“0”
 *
 */
bool TEE_BigIntGetBit(const TEE_BigInt *src, uint32_t bitIndex);

/**
 * @brief 返回|src|的自然二进制表示中的位数；即src的大小
 *
 * @param src [IN]整数指针
 *
 * @return 0 src=0
 * @return src的自然二进制表示中的位数。
 *
 */
uint32_t TEE_BigIntGetBitCount(const TEE_BigInt *src);

#if defined(API_LEVEL) && (API_LEVEL >= API_LEVEL1_2)
/**
 * @brief 将op的自然二进制表示的bitIndex第1位设置为1或0
 *
 * @param op [IN/OUT]整数指针
 * @param bitIndex [IN]要设置的位的偏移量，从最低有效位的偏移量0开始
 * @param value [IN]要设置的位值，其中true表示“1”，false表示“0”
 *
 * @return TEE_SUCCESS 支持
 * @return TEE_ERROR_OVERFLOW bitIndexth位大于op的分配位长度
 *
 */
TEE_Result TEE_BigIntSetBit(TEE_BigInt *op, uint32_t bitIndex, bool value);

/**
 * @brief 将src的值赋值给dest
 *
 * @param dest [OUT]要分配的TEE_BigInt指针
 * @param src [IN]指向源操作数的指针
 *
 * @return TEE_SUCCESS 支持
 * @return TEE_ERROR_OVERFLOW 如果dest操作数不能容纳src的值
 *
 */
TEE_Result TEE_BigIntAssign(TEE_BigInt *dest, const TEE_BigInt *src);

/**
 * @brief 将src的值赋给dest
 *
 * @param dest [OUT]要分配的TEE_BigInt指针
 * @param src [IN]指向源操作数的指针
 *
 * @return TEE_SUCCESS 支持
 * @return TEE_ERROR_OVERFLOW 如果dest操作数不能容纳src的值
 *
 */
TEE_Result TEE_BigIntAbs(TEE_BigInt *dest, const TEE_BigInt *src);
#endif /* API_LEVEL */

/**
 * @brief 计算 dest= op1 + op2
 *
 * @param dest [OUT]指向TEE_BigInt的指针，用于存储结果op1 + op2
 * @param op1 [IN]指向第一个操作数的指针
 * @param op2 [IN]指向第二个操作数的指针
 *
 */
void TEE_BigIntAdd(TEE_BigInt *dest, const TEE_BigInt *op1, const TEE_BigInt *op2);

/**
 * @brief 计算 dest= op1 - op2
 *
 * @param dest [OUT]指向TEE_BigInt的指针，用于存储结果op1 - op2
 * @param op1 [IN]指向第一个操作数的指针
 * @param op2 [IN]指向第二个操作数的指针
 *
 */
void TEE_BigIntSub(TEE_BigInt *dest, const TEE_BigInt *op1, const TEE_BigInt *op2);

/**
 * @brief 取反操作数：dest = -op
 *
 * @param dest [OUT]P指向TEE_BigInt的指针，用于存储结果-op
 * @param op [IN]指向要取反的操作数的指针
 *
 */
void TEE_BigIntNeg(TEE_BigInt *dest, const TEE_BigInt *op);

/**
 * @brief 计算 dest = op1 * op2
 *
 * @param dest [OUT]指向TEE_BigInt的指针，用于存储结果op1 * op2
 * @param op1 [IN]指向第一个操作数的指针
 * @param op2 [IN]指向第二个操作数的指针
 *
 */
void TEE_BigIntMul(TEE_BigInt *dest, const TEE_BigInt *op1, const TEE_BigInt *op2);

/**
 * @brief 计算 dest = op * op
 *
 * @param dest [OUT]指向TEE_BigInt的指针，用于存储结果op * op
 * @param op [IN]指向要平方的操作数的指针
 *
 */
void TEE_BigIntSquare(TEE_BigInt *dest, const TEE_BigInt *op);

/**
 * @brief 计算dest_r和dest_q，使得op1 = dest_q * op2 + dest_r
 *
 * @param dest_q [OUT]指向TEE_BigInt的指针，用于存储商
 * @param dest_r [IN]指向TEE_BigInt的指针，用于存储余数
 * @param op1 [OUT]指向第一个操作数的指针，被除数
 * @param op2 [IN]指向第二个操作数的指针，除数
 *
 * @return TEE_SUCCESS 操作成功
 * @return TEE_ERROR_BAD_PARAMETERS 其中存在至少一个参数为NULL
 *
 */
void TEE_BigIntDiv(TEE_BigInt *dest_q, TEE_BigInt *dest_r, const TEE_BigInt *op1, const TEE_BigInt *op2);

/**
 * @brief 计算dest = op (mod n)，使得0 <= dest < n
 *
 * @param dest [OUT]指向TEE_BigInt的指针，以保存结果op (mod n)
 * @param op [IN]指向要缩减mod n的操作数的指针
 * @param n [IN]指向模数的指针。模数应大于1
 *
 */
void TEE_BigIntMod(TEE_BigInt *dest, const TEE_BigInt *op, const TEE_BigInt *n);

/**
 * @brief 计算dest= (op1 + op2) (mod n)
 *
 * @param dest [OUT]指向TEE_BigInt的指针，以保存结果(op1 + op2)(mod n)
 * @param op1 [IN]指向第一个操作数的指针
 * @param op2 [IN]指向第二个操作数的指针
 * @param n [IN]指向模数的指针，模数应大于1
 *
 */
void TEE_BigIntAddMod(TEE_BigInt *dest, const TEE_BigInt *op1, const TEE_BigInt *op2, const TEE_BigInt *n);

/**
 * @brief 计算dest = (op1 - op2) (mod n)
 *
 * @param dest [OUT]指向TEE_BigInt的指针，以保存结果(op1 - op2)(mod n)
 * @param op1 [IN]指向第一个操作数的指针
 * @param op2 [IN]指向第二个操作数的指针
 * @param n [IN]指向模数的指针，模数应大于1
 *
 */
void TEE_BigIntSubMod(TEE_BigInt *dest, const TEE_BigInt *op1, const TEE_BigInt *op2, const TEE_BigInt *n);

/**
 * @brief 计算dest = (op1 * op2) (mod n)
 *
 * @param dest [OUT]指向TEE_BigInt的指针，以保存结果(op1 * op2)(mod n)
 * @param op1 [IN]指向第一个操作数的指针
 * @param op2 [IN]指向第二个操作数的指针
 * @param n [IN]指向模数的指针，模数应大于1
 *
 */
void TEE_BigIntMulMod(TEE_BigInt *dest, const TEE_BigInt *op1, const TEE_BigInt *op2, const TEE_BigInt *n);

/**
 * @brief 计算dest = (op * op) (mod n)
 *
 * @param dest [OUT]指向TEE_BigInt的指针，以保存结果(op * op)(mod n)
 * @param op [IN]操作数指针
 * @param n [IN]指向模数的指针。模数应大于1
 *
 */
void TEE_BigIntSquareMod(TEE_BigInt *dest, const TEE_BigInt *op, const TEE_BigInt *n);

/**
 * @brief 计算dest，使dest * op = 1 (mod n)
 *
 * @param dest [OUT]指向TEE_BigInt的指针，以保存结果（op^-1）(mod n)
 * @param op [IN]操作数指针
 * @param n [IN]指向模数的指针。模数应大于1
 *
 */
void TEE_BigIntInvMod(TEE_BigInt *dest, const TEE_BigInt *op, const TEE_BigInt *n);

/**
 * @brief 确定是否gcd(op1, op2) == 1
 *
 * @param op1 [IN]指向第一个操作数的指针
 * @param op2 [IN]指向第二个操作数的指针
 *
 * @return true gcd(op1, op2) == 1
 * @return false gcd(op1, op2) != 1
 *
 */
bool TEE_BigIntRelativePrime(const TEE_BigInt *op1, const TEE_BigInt *op2);

/**
 * @brief 计算输入参数op1和op2的最大公约数
 *
 * @param gcd [OUT]指向TEE_BigInt的指针，用于保存op1和op2的最大公约数
 * @param u [OUT]指向TEE_BigInt的指针，用于保存第一个系数
 * @param v [OUT]指向TEE_BigInt的指针，用于保存第二个系数
 * @param op1 [IN]指向第一个操作数的指针
 * @param op2 [IN]指向第二个操作数的指针
 *
 */
void TEE_BigIntComputeExtendedGcd(TEE_BigInt *gcd, TEE_BigInt *u, TEE_BigInt *v, const TEE_BigInt *op1,
                                  const TEE_BigInt *op2);
/**
 * @brief 对op执行概率原始性检验
 *
 * @param op [IN]经过原始测试的候选号码
 * @param confidenceLevel [IN]非结论性测试的期望置信水平
 *
 * @return 0 op是复合数
 * @return 1 op是素数
 * @return -1 测试是非结论性的，但op是复合的概率小于2^（-confidenceLevel）
 *
 */
int32_t TEE_BigIntIsProbablePrime(const TEE_BigInt *op, uint32_t confidenceLevel);

/**
 * @brief 将src转换为适合进行快速模乘的表示
 *
 * @param dest [OUT]TEE_BigIntFMM初始化内存区域指针
 * @param src [IN]指向要转换的TEE_BigInt的指针
 * @param n [IN]模数指针
 * @param context [IN]指向先前使用TEE_BigIntInitFMMContext1初始化的上下文的指针
 *
 */
void TEE_BigIntConvertToFMM(TEE_BigIntFMM *dest, const TEE_BigInt *src, const TEE_BigInt *n,
                            const TEE_BigIntFMMContext *context);

/**
 * @brief 将快速模乘表示中的src转换回TEE_BigInt表示
 *
 * @param dest [OUT]指向用于保存转换结果的初始化TEE_BigIntFMM内存区域的指针
 * @param src [IN]指向保存快速模乘表示中值的TEE_BigIntFMM的指针
 * @param n [IN]模数指针
 * @param context [IN]指向先前使用TEE_BigIntInitFMMContext1初始化的上下文的指针
 *
 */
void TEE_BigIntConvertFromFMM(TEE_BigInt *dest, const TEE_BigIntFMM *src, const TEE_BigInt *n,
                              const TEE_BigIntFMMContext *context);

/**
 * @brief 计算快速模乘法表示中的dest = op1 * op2
 *
 * @param dest [OUT]指向TEE_BigIntFMM的指针，用于保存结果op1 * op2
 * @param op1 [IN]指向第一个操作数的指针
 * @param op2 [IN]指向第二个操作数的指针
 * @param n [IN]模数指针
 * @param context[IN]指向先前使用TEE_BigIntInitFMMContext1初始化的上下文的指针
 *
 */
void TEE_BigIntComputeFMM(TEE_BigIntFMM *dest, const TEE_BigIntFMM *op1, const TEE_BigIntFMM *op2, const TEE_BigInt *n,
                          const TEE_BigIntFMMContext *context);

/**
 * @brief 计算dest = (op1 ^ op2) (mod n)
 *
 * @param des [OUT]指向TEE_BigInt的指针，以保存结果(op1 ^ op2)(mod n)
 * @param op1 [IN]指向第一个操作数的指针
 * @param op2 [IN]指向第二个操作数的指针
 * @param n [IN]模数指针
 * @param context [IN]指向以前使用TEE_BigIntInitFMMContext1或NULL初始化的上下文的指针
 *
 * @return TEE_SUCCESS 成功
 * @return TEE_ERROR_NOT_SUPPORTED 不支持n的值
 *
 */
TEE_Result TEE_BigIntExpMod(TEE_BigInt *des, TEE_BigInt *op1, const TEE_BigInt *op2, const TEE_BigInt *n,
                            TEE_BigIntFMMContext *context);

#endif
