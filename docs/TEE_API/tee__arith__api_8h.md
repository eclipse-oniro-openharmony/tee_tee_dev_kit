# tee_arith_api.h


## 概述

大数操作接口

**起始版本：**

1

**相关模块：**

[TeeArith](_tee_arith.md)


## 汇总


### 宏定义

| 名称 | 描述 | 
| -------- | -------- |
| [TEE_BigIntSizeInU32](_tee_arith.md#tee_bigintsizeinu32)(n) | 返回u32中的BigInt的大小 | 


### 函数

| 名称 | 描述 | 
| -------- | -------- |
| [TEE_BigIntFMMSizeInU32](_tee_arith.md#tee_bigintfmmsizeinu32) (size_t modulusSizeInBits) | 返回uint32_t值数组的大小 | 
| [TEE_BigIntFMMContextSizeInU32](_tee_arith.md#tee_bigintfmmcontextsizeinu32) (size_t modulusSizeInBits) | 返回表示快速模块化上下文所需的uint32_t值数组的大小 | 
| [TEE_BigIntInit](_tee_arith.md#tee_bigintinit) ([TEE_BigInt](_tee_arith.md#tee_bigint) \*bigInt, size_t len) | 初始化bigInt | 
| [TEE_BigIntInitFMMContext](_tee_arith.md#tee_bigintinitfmmcontext) (TEE_BigIntFMMContext \*context, size_t len, constTEE_BigInt \*modulus) | 计算快速模乘法的必要先决条件，并将它们存储在上下文中 | 
| [TEE_BigIntInitFMMContext1](_tee_arith.md#tee_bigintinitfmmcontext1) (TEE_BigIntFMMContext \*context, size_t len, constTEE_BigInt \*modulus) | 计算快速模乘法的必要先决条件，并将它们存储在上下文中 | 
| [TEE_BigIntInitFMM](_tee_arith.md#tee_bigintinitfmm) (TEE_BigIntFMM \*bigIntFMM, size_t len) | 初始化bigIntFMM并将其表示的值设置为零 | 
| [TEE_BigIntConvertFromOctetString](_tee_arith.md#tee_bigintconvertfromoctetstring) ([TEE_BigInt](_tee_arith.md#tee_bigint) \*dest, constuint8_t \*buffer, size_t bufferLen, int32_t sign) | 将bufferLen字节八位字节字符串缓冲区转换为TEE_BigInt格式 | 
| [TEE_BigIntConvertToOctetString](_tee_arith.md#tee_bigintconverttooctetstring) (void \*buffer, size_t \*bufferLen, const [TEE_BigInt](_tee_arith.md#tee_bigint) \*bigInt) | 将TEE_BigInt格式的整数的绝对值转换为八位字节字符串 | 
| [TEE_BigIntConvertFromS32](_tee_arith.md#tee_bigintconvertfroms32) ([TEE_BigInt](_tee_arith.md#tee_bigint) \*dest, int32_t shortVal) | 将\*dest设置为值shortVal | 
| [TEE_BigIntConvertToS32](_tee_arith.md#tee_bigintconverttos32) (int32_t \*dest, const [TEE_BigInt](_tee_arith.md#tee_bigint) \*src) | 将\*dest设置为src的值，包括src的符号 | 
| [TEE_BigIntCmp](_tee_arith.md#tee_bigintcmp) (const [TEE_BigInt](_tee_arith.md#tee_bigint) \*op1, const [TEE_BigInt](_tee_arith.md#tee_bigint) \*op2) | 检查op1&gt;op2、op1==op2或op1&lt;op2 | 
| [TEE_BigIntCmpS32](_tee_arith.md#tee_bigintcmps32) (const [TEE_BigInt](_tee_arith.md#tee_bigint) \*op, int32_t shortVal) | 检查是op&gt;shortVal、op==shortVal或op&lt;shortVal | 
| [TEE_BigIntShiftRight](_tee_arith.md#tee_bigintshiftright) ([TEE_BigInt](_tee_arith.md#tee_bigint) \*dest, const [TEE_BigInt](_tee_arith.md#tee_bigint) \*op, size_t bits) | 计算 \|dest\| = \|op\| &gt;&gt; bits | 
| [TEE_BigIntGetBit](_tee_arith.md#tee_bigintgetbit) (const [TEE_BigInt](_tee_arith.md#tee_bigint) \*src, uint32_t bitIndex) | 返回\|src\|的自然二进制表示的bitIndex位 | 
| [TEE_BigIntGetBitCount](_tee_arith.md#tee_bigintgetbitcount) (const [TEE_BigInt](_tee_arith.md#tee_bigint) \*src) | 返回\|src\|的自然二进制表示中的位数；即src的大小 | 
| [TEE_BigIntAdd](_tee_arith.md#tee_bigintadd) ([TEE_BigInt](_tee_arith.md#tee_bigint) \*dest, const [TEE_BigInt](_tee_arith.md#tee_bigint) \*op1, const [TEE_BigInt](_tee_arith.md#tee_bigint) \*op2) | 计算 dest= op1 + op2 | 
| [TEE_BigIntSub](_tee_arith.md#tee_bigintsub) ([TEE_BigInt](_tee_arith.md#tee_bigint) \*dest, const [TEE_BigInt](_tee_arith.md#tee_bigint) \*op1, const [TEE_BigInt](_tee_arith.md#tee_bigint) \*op2) | 计算 dest= op1 - op2 | 
| [TEE_BigIntNeg](_tee_arith.md#tee_bigintneg) ([TEE_BigInt](_tee_arith.md#tee_bigint) \*dest, const [TEE_BigInt](_tee_arith.md#tee_bigint) \*op) | 取反操作数：dest = -op | 
| [TEE_BigIntMul](_tee_arith.md#tee_bigintmul) ([TEE_BigInt](_tee_arith.md#tee_bigint) \*dest, const [TEE_BigInt](_tee_arith.md#tee_bigint) \*op1, const [TEE_BigInt](_tee_arith.md#tee_bigint) \*op2) | 计算 dest = op1 \* op2 | 
| [TEE_BigIntSquare](_tee_arith.md#tee_bigintsquare) ([TEE_BigInt](_tee_arith.md#tee_bigint) \*dest, const [TEE_BigInt](_tee_arith.md#tee_bigint) \*op) | 计算 dest = op \* op | 
| [TEE_BigIntDiv](_tee_arith.md#tee_bigintdiv) ([TEE_BigInt](_tee_arith.md#tee_bigint) \*dest_q, [TEE_BigInt](_tee_arith.md#tee_bigint) \*dest_r, const [TEE_BigInt](_tee_arith.md#tee_bigint) \*op1, const [TEE_BigInt](_tee_arith.md#tee_bigint) \*op2) | 计算dest_r和dest_q，使得op1 = dest_q \* op2 + dest_r | 
| [TEE_BigIntMod](_tee_arith.md#tee_bigintmod) ([TEE_BigInt](_tee_arith.md#tee_bigint) \*dest, const [TEE_BigInt](_tee_arith.md#tee_bigint) \*op, const [TEE_BigInt](_tee_arith.md#tee_bigint) \*n) | 计算dest = op (mod n)，使得0 &lt;= dest &lt; n | 
| [TEE_BigIntAddMod](_tee_arith.md#tee_bigintaddmod) ([TEE_BigInt](_tee_arith.md#tee_bigint) \*dest, const [TEE_BigInt](_tee_arith.md#tee_bigint) \*op1, const [TEE_BigInt](_tee_arith.md#tee_bigint) \*op2, const [TEE_BigInt](_tee_arith.md#tee_bigint) \*n) | 计算dest= (op1 + op2) (mod n) | 
| [TEE_BigIntSubMod](_tee_arith.md#tee_bigintsubmod) ([TEE_BigInt](_tee_arith.md#tee_bigint) \*dest, const [TEE_BigInt](_tee_arith.md#tee_bigint) \*op1, const [TEE_BigInt](_tee_arith.md#tee_bigint) \*op2, const [TEE_BigInt](_tee_arith.md#tee_bigint) \*n) | 计算dest = (op1 - op2) (mod n) | 
| [TEE_BigIntMulMod](_tee_arith.md#tee_bigintmulmod) ([TEE_BigInt](_tee_arith.md#tee_bigint) \*dest, const [TEE_BigInt](_tee_arith.md#tee_bigint) \*op1, const [TEE_BigInt](_tee_arith.md#tee_bigint) \*op2, const [TEE_BigInt](_tee_arith.md#tee_bigint) \*n) | 计算dest = (op1 \* op2) (mod n) | 
| [TEE_BigIntSquareMod](_tee_arith.md#tee_bigintsquaremod) ([TEE_BigInt](_tee_arith.md#tee_bigint) \*dest, const [TEE_BigInt](_tee_arith.md#tee_bigint) \*op, const [TEE_BigInt](_tee_arith.md#tee_bigint) \*n) | 计算dest = (op \* op) (mod n) | 
| [TEE_BigIntInvMod](_tee_arith.md#tee_bigintinvmod) ([TEE_BigInt](_tee_arith.md#tee_bigint) \*dest, const [TEE_BigInt](_tee_arith.md#tee_bigint) \*op, const [TEE_BigInt](_tee_arith.md#tee_bigint) \*n) | 计算dest，使dest \* op = 1 (mod n) | 
| [TEE_BigIntRelativePrime](_tee_arith.md#tee_bigintrelativeprime) (const [TEE_BigInt](_tee_arith.md#tee_bigint) \*op1, const [TEE_BigInt](_tee_arith.md#tee_bigint) \*op2) | 确定是否gcd(op1, op2) == 1 | 
| [TEE_BigIntComputeExtendedGcd](_tee_arith.md#tee_bigintcomputeextendedgcd) ([TEE_BigInt](_tee_arith.md#tee_bigint) \*gcd, [TEE_BigInt](_tee_arith.md#tee_bigint) \*u, [TEE_BigInt](_tee_arith.md#tee_bigint) \*v, const [TEE_BigInt](_tee_arith.md#tee_bigint) \*op1, const [TEE_BigInt](_tee_arith.md#tee_bigint) \*op2) | 计算输入参数op1和op2的最大公约数 | 
| [TEE_BigIntIsProbablePrime](_tee_arith.md#tee_bigintisprobableprime) (const [TEE_BigInt](_tee_arith.md#tee_bigint) \*op, uint32_t confidenceLevel) | 对op执行概率原始性检验 | 
| [TEE_BigIntConvertToFMM](_tee_arith.md#tee_bigintconverttofmm) (TEE_BigIntFMM \*dest, const [TEE_BigInt](_tee_arith.md#tee_bigint) \*src, const [TEE_BigInt](_tee_arith.md#tee_bigint) \*n, const TEE_BigIntFMMContext \*context) | 将src转换为适合进行快速模乘的表示 | 
| [TEE_BigIntConvertFromFMM](_tee_arith.md#tee_bigintconvertfromfmm) ([TEE_BigInt](_tee_arith.md#tee_bigint) \*dest, const TEE_BigIntFMM \*src, const [TEE_BigInt](_tee_arith.md#tee_bigint) \*n, const TEE_BigIntFMMContext \*context) | 将快速模乘表示中的src转换回TEE_BigInt表示 | 
| [TEE_BigIntComputeFMM](_tee_arith.md#tee_bigintcomputefmm) (TEE_BigIntFMM \*dest, const TEE_BigIntFMM \*op1, const TEE_BigIntFMM \*op2, const [TEE_BigInt](_tee_arith.md#tee_bigint) \*n, const TEE_BigIntFMMContext \*context) | 计算快速模乘法表示中的dest = op1 \* op2 | 
| [TEE_BigIntExpMod](_tee_arith.md#tee_bigintexpmod) ([TEE_BigInt](_tee_arith.md#tee_bigint) \*des, [TEE_BigInt](_tee_arith.md#tee_bigint) \*op1, const [TEE_BigInt](_tee_arith.md#tee_bigint) \*op2, const [TEE_BigInt](_tee_arith.md#tee_bigint) \*n, TEE_BigIntFMMContext \*context) | 计算des = (op1 ^ op2) (mod n) | 


### 变量

| 名称 | 描述 | 
| -------- | -------- |
| [TEE_BigInt](_tee_arith.md#tee_bigint) | below definitions are defined by Global Platform for compatibility: don't make any change to the content below | 
| **TEE_BigIntFMM** |  | 
| **TEE_BigIntFMMContext** |  | 
