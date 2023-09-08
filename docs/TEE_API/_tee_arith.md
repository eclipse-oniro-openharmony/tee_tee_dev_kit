# TeeArith


## 概述

TeeArith提供了一组安全可信执行环境（TEE）的大数操作接口。

开发者可以通过使用这些接口实现在TEE环境中完成大数运算，实现加解密操作。

**起始版本：**

1


## 汇总


### 文件

| 名称 | 描述 | 
| -------- | -------- |
| [tee_arith_api.h](tee__arith__api_8h.md) | 大数操作接口 | 


### 宏定义

| 名称 | 描述 | 
| -------- | -------- |
| [TEE_BigIntSizeInU32](#tee_bigintsizeinu32)(n) | 返回u32中的BigInt的大小 | 


### 函数

| 名称 | 描述 | 
| -------- | -------- |
| [TEE_BigIntFMMSizeInU32](#tee_bigintfmmsizeinu32) (size_t modulusSizeInBits) | 返回uint32_t值数组的大小 | 
| [TEE_BigIntFMMContextSizeInU32](#tee_bigintfmmcontextsizeinu32) (size_t modulusSizeInBits) | 返回表示快速模块化上下文所需的uint32_t值数组的大小 | 
| [TEE_BigIntInit](#tee_bigintinit) ([TEE_BigInt](#tee_bigint) \*bigInt, size_t len) | 初始化bigInt | 
| [TEE_BigIntInitFMMContext](#tee_bigintinitfmmcontext) (TEE_BigIntFMMContext \*context, size_t len, constTEE_BigInt \*modulus) | 计算快速模乘法的必要先决条件，并将它们存储在上下文中 | 
| [TEE_BigIntInitFMMContext1](#tee_bigintinitfmmcontext1) (TEE_BigIntFMMContext \*context, size_t len, constTEE_BigInt \*modulus) | 计算快速模乘法的必要先决条件，并将它们存储在上下文中 | 
| [TEE_BigIntInitFMM](#tee_bigintinitfmm) (TEE_BigIntFMM \*bigIntFMM, size_t len) | 初始化bigIntFMM并将其表示的值设置为零 | 
| [TEE_BigIntConvertFromOctetString](#tee_bigintconvertfromoctetstring) ([TEE_BigInt](#tee_bigint) \*dest, constuint8_t \*buffer, size_t bufferLen, int32_t sign) | 将bufferLen字节八位字节字符串缓冲区转换为TEE_BigInt格式 | 
| [TEE_BigIntConvertToOctetString](#tee_bigintconverttooctetstring) (void \*buffer, size_t \*bufferLen, const [TEE_BigInt](#tee_bigint) \*bigInt) | 将TEE_BigInt格式的整数的绝对值转换为八位字节字符串 | 
| [TEE_BigIntConvertFromS32](#tee_bigintconvertfroms32) ([TEE_BigInt](#tee_bigint) \*dest, int32_t shortVal) | 将\*dest设置为值shortVal | 
| [TEE_BigIntConvertToS32](#tee_bigintconverttos32) (int32_t \*dest, const [TEE_BigInt](#tee_bigint) \*src) | 将\*dest设置为src的值，包括src的符号 | 
| [TEE_BigIntCmp](#tee_bigintcmp) (const [TEE_BigInt](#tee_bigint) \*op1, const [TEE_BigInt](#tee_bigint) \*op2) | 检查op1&gt;op2、op1==op2或op1&lt;op2 | 
| [TEE_BigIntCmpS32](#tee_bigintcmps32) (const [TEE_BigInt](#tee_bigint) \*op, int32_t shortVal) | 检查是op&gt;shortVal、op==shortVal或op&lt;shortVal | 
| [TEE_BigIntShiftRight](#tee_bigintshiftright) ([TEE_BigInt](#tee_bigint) \*dest, const [TEE_BigInt](#tee_bigint) \*op, size_t bits) | 计算 \|dest\| = \|op\| &gt;&gt; bits | 
| [TEE_BigIntGetBit](#tee_bigintgetbit) (const [TEE_BigInt](#tee_bigint) \*src, uint32_t bitIndex) | 返回\|src\|的自然二进制表示的bitIndex位 | 
| [TEE_BigIntGetBitCount](#tee_bigintgetbitcount) (const [TEE_BigInt](#tee_bigint) \*src) | 返回\|src\|的自然二进制表示中的位数；即src的大小 | 
| [TEE_BigIntAdd](#tee_bigintadd) ([TEE_BigInt](#tee_bigint) \*dest, const [TEE_BigInt](#tee_bigint) \*op1, const [TEE_BigInt](#tee_bigint) \*op2) | 计算 dest= op1 + op2 | 
| [TEE_BigIntSub](#tee_bigintsub) ([TEE_BigInt](#tee_bigint) \*dest, const [TEE_BigInt](#tee_bigint) \*op1, const [TEE_BigInt](#tee_bigint) \*op2) | 计算 dest= op1 - op2 | 
| [TEE_BigIntNeg](#tee_bigintneg) ([TEE_BigInt](#tee_bigint) \*dest, const [TEE_BigInt](#tee_bigint) \*op) | 取反操作数：dest = -op | 
| [TEE_BigIntMul](#tee_bigintmul) ([TEE_BigInt](#tee_bigint) \*dest, const [TEE_BigInt](#tee_bigint) \*op1, const [TEE_BigInt](#tee_bigint) \*op2) | 计算 dest = op1 \* op2 | 
| [TEE_BigIntSquare](#tee_bigintsquare) ([TEE_BigInt](#tee_bigint) \*dest, const [TEE_BigInt](#tee_bigint) \*op) | 计算 dest = op \* op | 
| [TEE_BigIntDiv](#tee_bigintdiv) ([TEE_BigInt](#tee_bigint) \*dest_q, [TEE_BigInt](#tee_bigint) \*dest_r, const [TEE_BigInt](#tee_bigint) \*op1, const [TEE_BigInt](#tee_bigint) \*op2) | 计算dest_r和dest_q，使得op1 = dest_q \* op2 + dest_r | 
| [TEE_BigIntMod](#tee_bigintmod) ([TEE_BigInt](#tee_bigint) \*dest, const [TEE_BigInt](#tee_bigint) \*op, const [TEE_BigInt](#tee_bigint) \*n) | 计算dest = op (mod n)，使得0 &lt;= dest &lt; n | 
| [TEE_BigIntAddMod](#tee_bigintaddmod) ([TEE_BigInt](#tee_bigint) \*dest, const [TEE_BigInt](#tee_bigint) \*op1, const [TEE_BigInt](#tee_bigint) \*op2, const [TEE_BigInt](#tee_bigint) \*n) | 计算dest= (op1 + op2) (mod n) | 
| [TEE_BigIntSubMod](#tee_bigintsubmod) ([TEE_BigInt](#tee_bigint) \*dest, const [TEE_BigInt](#tee_bigint) \*op1, const [TEE_BigInt](#tee_bigint) \*op2, const [TEE_BigInt](#tee_bigint) \*n) | 计算dest = (op1 - op2) (mod n) | 
| [TEE_BigIntMulMod](#tee_bigintmulmod) ([TEE_BigInt](#tee_bigint) \*dest, const [TEE_BigInt](#tee_bigint) \*op1, const [TEE_BigInt](#tee_bigint) \*op2, const [TEE_BigInt](#tee_bigint) \*n) | 计算dest = (op1 \* op2) (mod n) | 
| [TEE_BigIntSquareMod](#tee_bigintsquaremod) ([TEE_BigInt](#tee_bigint) \*dest, const [TEE_BigInt](#tee_bigint) \*op, const [TEE_BigInt](#tee_bigint) \*n) | 计算dest = (op \* op) (mod n) | 
| [TEE_BigIntInvMod](#tee_bigintinvmod) ([TEE_BigInt](#tee_bigint) \*dest, const [TEE_BigInt](#tee_bigint) \*op, const [TEE_BigInt](#tee_bigint) \*n) | 计算dest，使dest \* op = 1 (mod n) | 
| [TEE_BigIntRelativePrime](#tee_bigintrelativeprime) (const [TEE_BigInt](#tee_bigint) \*op1, const [TEE_BigInt](#tee_bigint) \*op2) | 确定是否gcd(op1, op2) == 1 | 
| [TEE_BigIntComputeExtendedGcd](#tee_bigintcomputeextendedgcd) ([TEE_BigInt](#tee_bigint) \*gcd, [TEE_BigInt](#tee_bigint) \*u, [TEE_BigInt](#tee_bigint) \*v, const [TEE_BigInt](#tee_bigint) \*op1, const [TEE_BigInt](#tee_bigint) \*op2) | 计算输入参数op1和op2的最大公约数 | 
| [TEE_BigIntIsProbablePrime](#tee_bigintisprobableprime) (const [TEE_BigInt](#tee_bigint) \*op, uint32_t confidenceLevel) | 对op执行概率原始性检验 | 
| [TEE_BigIntConvertToFMM](#tee_bigintconverttofmm) (TEE_BigIntFMM \*dest, const [TEE_BigInt](#tee_bigint) \*src, const [TEE_BigInt](#tee_bigint) \*n, const TEE_BigIntFMMContext \*context) | 将src转换为适合进行快速模乘的表示 | 
| [TEE_BigIntConvertFromFMM](#tee_bigintconvertfromfmm) ([TEE_BigInt](#tee_bigint) \*dest, const TEE_BigIntFMM \*src, const [TEE_BigInt](#tee_bigint) \*n, const TEE_BigIntFMMContext \*context) | 将快速模乘表示中的src转换回TEE_BigInt表示 | 
| [TEE_BigIntComputeFMM](#tee_bigintcomputefmm) (TEE_BigIntFMM \*dest, const TEE_BigIntFMM \*op1, const TEE_BigIntFMM \*op2, const [TEE_BigInt](#tee_bigint) \*n, const TEE_BigIntFMMContext \*context) | 计算快速模乘法表示中的dest = op1 \* op2 | 
| [TEE_BigIntExpMod](#tee_bigintexpmod) ([TEE_BigInt](#tee_bigint) \*des, [TEE_BigInt](#tee_bigint) \*op1, const [TEE_BigInt](#tee_bigint) \*op2, const [TEE_BigInt](#tee_bigint) \*n, TEE_BigIntFMMContext \*context) | 计算des = (op1 ^ op2) (mod n) | 


### 变量

| 名称 | 描述 | 
| -------- | -------- |
| [TEE_BigInt](#tee_bigint) | below definitions are defined by Global Platform for compatibility: don't make any change to the content below | 
| **TEE_BigIntFMM** |  | 
| **TEE_BigIntFMMContext** |  | 


## 宏定义说明


### TEE_BigIntSizeInU32

```
#define TEE_BigIntSizeInU32( n)   ((((n) + 31) / 32) + 2)
```

**描述：**

返回u32中的BigInt的大小

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| n | [IN]TEE_BigInt类型 | 

**返回：**

32中的BigInt的大小

**起始版本：**

1


## 函数说明


### TEE_BigIntAdd()

```
void TEE_BigIntAdd (TEE_BigInt * dest, const TEE_BigInt * op1, const TEE_BigInt * op2 )
```

**描述：**

计算 dest= op1 + op2

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| dest | [OUT]指向TEE_BigInt的指针，用于存储结果op1 + op2 | 
| op1 | [IN]指向第一个操作数的指针 | 
| op2 | [IN]指向第二个操作数的指针 | 

**起始版本：**

1


### TEE_BigIntAddMod()

```
void TEE_BigIntAddMod (TEE_BigInt * dest, const TEE_BigInt * op1, const TEE_BigInt * op2, const TEE_BigInt * n )
```

**描述：**

计算dest= (op1 + op2) (mod n)

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| dest | [OUT]指向TEE_BigInt的指针，以保存结果(op1 + op2)(mod n) | 
| op1 | [IN]指向第一个操作数的指针 | 
| op2 | [IN]指向第二个操作数的指针 | 
| n | [IN]指向模数的指针，模数应大于1 | 

**起始版本：**

1


### TEE_BigIntCmp()

```
int32_t TEE_BigIntCmp (const TEE_BigInt * op1, const TEE_BigInt * op2 )
```

**描述：**

检查op1&gt;op2、op1==op2或op1&lt;op2

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| op1 | [IN]指向第一个操作数的指针 | 
| op2 | [IN]指向第二个操作数的指针 | 

**返回：**

0 op1==op2

正数 op1&gt;op2

负数 op1&lt;op2

**起始版本：**

1


### TEE_BigIntCmpS32()

```
int32_t TEE_BigIntCmpS32 (const TEE_BigInt * op, int32_t shortVal )
```

**描述：**

检查是op&gt;shortVal、op==shortVal或op&lt;shortVal

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| op | [IN]指向第一个操作数的指针 | 
| shortVal | [IN]指向第二个操作数的指针 | 

**返回：**

0 op==shortVal

正数 op&gt;shortVal

负数 op&lt;shortVal

**起始版本：**

1


### TEE_BigIntComputeExtendedGcd()

```
void TEE_BigIntComputeExtendedGcd (TEE_BigInt * gcd, TEE_BigInt * u, TEE_BigInt * v, const TEE_BigInt * op1, const TEE_BigInt * op2 )
```

**描述：**

计算输入参数op1和op2的最大公约数

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| gcd | [OUT]指向TEE_BigInt的指针，用于保存op1和op2的最大公约数 | 
| u | [OUT]指向TEE_BigInt的指针，用于保存第一个系数 | 
| v | [OUT]指向TEE_BigInt的指针，用于保存第二个系数 | 
| op1 | [IN]指向第一个操作数的指针 | 
| op2 | [IN]指向第二个操作数的指针 | 

**起始版本：**

1


### TEE_BigIntComputeFMM()

```
void TEE_BigIntComputeFMM (TEE_BigIntFMM * dest, const TEE_BigIntFMM * op1, const TEE_BigIntFMM * op2, const TEE_BigInt * n, const TEE_BigIntFMMContext * context )
```

**描述：**

计算快速模乘法表示中的dest = op1 \* op2

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| dest | [OUT]指向TEE_BigIntFMM的指针，用于保存结果op1 \* op2 | 
| op1 | [IN]指向第一个操作数的指针 | 
| op2 | [IN]指向第二个操作数的指针 | 
| n | [IN]模数指针 | 
| context | [IN]指向先前使用TEE_BigIntInitFMMContext1初始化的上下文的指针 | 

**起始版本：**

1


### TEE_BigIntConvertFromFMM()

```
void TEE_BigIntConvertFromFMM (TEE_BigInt * dest, const TEE_BigIntFMM * src, const TEE_BigInt * n, const TEE_BigIntFMMContext * context )
```

**描述：**

将快速模乘表示中的src转换回TEE_BigInt表示

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| dest | [OUT]指向用于保存转换结果的初始化TEE_BigIntFMM内存区域的指针 | 
| src | [IN]指向保存快速模乘表示中值的TEE_BigIntFMM的指针 | 
| n | [IN]模数指针 | 
| context | [IN]指向先前使用TEE_BigIntInitFMMContext1初始化的上下文的指针 | 

**起始版本：**

1


### TEE_BigIntConvertFromOctetString()

```
TEE_Result TEE_BigIntConvertFromOctetString (TEE_BigInt * dest, constuint8_t * buffer, size_t bufferLen, int32_t sign )
```

**描述：**

将bufferLen字节八位字节字符串缓冲区转换为TEE_BigInt格式

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| dest | [OUT]指向用于保存结果的TEE_BigInt的指针 | 
| buffer | [IN]指向包含整数的八位字节字符串表示形式的缓冲区的指针 | 
| bufferLen | [IN]buffer的长度（以字节为单位） | 
| sign | [IN]dest的标志被设置为标志的标志 | 

**返回：**

TEE_SUCCESS 支持

TEE_ERROR_OVERFLOW 为dest分配的内存太小

**起始版本：**

1


### TEE_BigIntConvertFromS32()

```
void TEE_BigIntConvertFromS32 (TEE_BigInt * dest, int32_t shortVal )
```

**描述：**

将\*dest设置为值shortVal

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| dest | [OUT]指向用于存储结果的TEE_BigInt的指针 | 
| shortVal | [IN]输入值 | 

**起始版本：**

1


### TEE_BigIntConvertToFMM()

```
void TEE_BigIntConvertToFMM (TEE_BigIntFMM * dest, const TEE_BigInt * src, const TEE_BigInt * n, const TEE_BigIntFMMContext * context )
```

**描述：**

将src转换为适合进行快速模乘的表示

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| dest | [OUT]TEE_BigIntFMM初始化内存区域指针 | 
| src | [IN]指向要转换的TEE_BigInt的指针 | 
| n | [IN]模数指针 | 
| context | [IN]指向先前使用TEE_BigIntInitFMMContext1初始化的上下文的指针 | 

**起始版本：**

1


### TEE_BigIntConvertToOctetString()

```
TEE_Result TEE_BigIntConvertToOctetString (void * buffer, size_t * bufferLen, const TEE_BigInt * bigInt )
```

**描述：**

将TEE_BigInt格式的整数的绝对值转换为八位字节字符串

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| buffer | [OUT]写入整数的转换八位字节字符串表示的输出缓冲区 | 
| bufferLen | [IN]buffer的长度（以字节为单位） | 
| bigInt | [IN]指向将转换为八位字节字符串的整数的指针 | 

**返回：**

TEE_SUCCESS 支持

TEE_ERROR_SHORT_BUFFER 输出缓冲区太小，无法包含八位字节字符串

**起始版本：**

1


### TEE_BigIntConvertToS32()

```
TEE_Result TEE_BigIntConvertToS32 (int32_t * dest, const TEE_BigInt * src )
```

**描述：**

将\*dest设置为src的值，包括src的符号

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| dest | [OUT]指向用于存储结果的int32_t的指针 | 
| src | [IN]输入值指针 | 

**返回：**

TEE_SUCCESS 支持

TEE_ERROR_OVERFLOW src不适合int32_t

**起始版本：**

1


### TEE_BigIntDiv()

```
void TEE_BigIntDiv (TEE_BigInt * dest_q, TEE_BigInt * dest_r, const TEE_BigInt * op1, const TEE_BigInt * op2 )
```

**描述：**

计算dest_r和dest_q，使得op1 = dest_q \* op2 + dest_r

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| dest_q | [OUT]指向TEE_BigInt的指针，用于存储商 | 
| dest_r | [IN]指向TEE_BigInt的指针，用于存储余数 | 
| op1 | [OUT]指向第一个操作数的指针，被除数 | 
| op2 | [IN]指向第二个操作数的指针，除数 | 

**返回：**

TEE_SUCCESS 操作成功

TEE_ERROR_BAD_PARAMETERS 其中存在至少一个参数为NULL

**起始版本：**

1


### TEE_BigIntExpMod()

```
TEE_Result TEE_BigIntExpMod (TEE_BigInt * des, TEE_BigInt * op1, const TEE_BigInt * op2, const TEE_BigInt * n, TEE_BigIntFMMContext * context )
```

**描述：**

计算des = (op1 ^ op2) (mod n)

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| des | [OUT]指向TEE_BigInt的指针，以保存结果(op1 ^ op2)(mod n) | 
| op1 | [IN]指向第一个操作数的指针 | 
| op2 | [IN]指向第二个操作数的指针 | 
| n | [IN]模数指针 | 
| context | [IN]指向以前使用TEE_BigIntInitFMMContext1或NULL初始化的上下文的指针 | 

**返回：**

TEE_SUCCESS 成功

TEE_ERROR_NOT_SUPPORTED 不支持n的值

**起始版本：**

1


### TEE_BigIntFMMContextSizeInU32()

```
size_t TEE_BigIntFMMContextSizeInU32 (size_t modulusSizeInBits)
```

**描述：**

返回表示快速模块化上下文所需的uint32_t值数组的大小

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| modulusSizeInBits | [IN]模数的大小（以bits为单位） | 

**返回：**

存储TEE_BigIntFMMContext所需的字节数，给定长度模数modSizeInBits

**起始版本：**

1


### TEE_BigIntFMMSizeInU32()

```
size_t TEE_BigIntFMMSizeInU32 (size_t modulusSizeInBits)
```

**描述：**

返回uint32_t值数组的大小

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| modulusSizeInBits | [IN]模数的大小（以bits为单位） | 

**返回：**

给定长度模数modSizeInBits，存储TEE_BigIntFMM所需的字节数

**起始版本：**

1


### TEE_BigIntGetBit()

```
bool TEE_BigIntGetBit (const TEE_BigInt * src, uint32_t bitIndex )
```

**描述：**

返回|src|的自然二进制表示的bitIndex位

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| src | [IN]整数指针 | 
| bitIndex | [IN]要读取的位的偏移量，从最低有效位的偏移量0开始 | 

**返回：**

true |src|中bitIndexth位的布尔值为“1”

false |src|中bitIndexth位的布尔值为“0”

**起始版本：**

1


### TEE_BigIntGetBitCount()

```
uint32_t TEE_BigIntGetBitCount (const TEE_BigInt * src)
```

**描述：**

返回|src|的自然二进制表示中的位数；即src的大小

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| src | [IN]整数指针 | 

**返回：**

0 src=0

src的自然二进制表示中的位数。

**起始版本：**

1


### TEE_BigIntInit()

```
void TEE_BigIntInit (TEE_BigInt * bigInt, size_t len )
```

**描述：**

初始化bigInt

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| bigInt | [OUT]指向要初始化的TEE_BigInt的指针 | 
| len | [IN]bigInt指向的内存的大小，单位为uint32_t | 

**起始版本：**

1


### TEE_BigIntInitFMM()

```
void TEE_BigIntInitFMM (TEE_BigIntFMM * bigIntFMM, size_t len )
```

**描述：**

初始化bigIntFMM并将其表示的值设置为零

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| bigIntFMM | [IN]要初始化的TEE_BigIntFMM指针 | 
| len | [IN]bigIntFMM指向的内存大小，单位为uint32_t | 

**起始版本：**

1


### TEE_BigIntInitFMMContext()

```
void TEE_BigIntInitFMMContext (TEE_BigIntFMMContext * context, size_t len, constTEE_BigInt * modulus )
```

**描述：**

计算快速模乘法的必要先决条件，并将它们存储在上下文中

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| context | [OUT]要初始化的TEE_BigIntFMMContext指针 | 
| len | [IN]上下文指向的内存的大小（以uint32_t为单位） | 
| modulus | [IN]模数 | 

**起始版本：**

1


### TEE_BigIntInitFMMContext1()

```
TEE_Result TEE_BigIntInitFMMContext1 (TEE_BigIntFMMContext * context, size_t len, constTEE_BigInt * modulus )
```

**描述：**

计算快速模乘法的必要先决条件，并将它们存储在上下文中

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| context | [OUT]要初始化的TEE_BigIntFMMContext指针 | 
| len | [IN]上下文指向的内存的大小（以uint32_t为单位） | 
| modulus | [IN]模数 | 

**返回：**

TEE_SUCCESS 成功

其它返回值 失败

**起始版本：**

1


### TEE_BigIntInvMod()

```
void TEE_BigIntInvMod (TEE_BigInt * dest, const TEE_BigInt * op, const TEE_BigInt * n )
```

**描述：**

计算dest，使dest \* op = 1 (mod n)

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| dest | [OUT]指向TEE_BigInt的指针，以保存结果（op^-1）(mod n) | 
| op | [IN]操作数指针 | 
| n | [IN]指向模数的指针。模数应大于1 | 

**起始版本：**

1


### TEE_BigIntIsProbablePrime()

```
int32_t TEE_BigIntIsProbablePrime (const TEE_BigInt * op, uint32_t confidenceLevel )
```

**描述：**

对op执行概率原始性检验

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| op | [IN]经过原始测试的候选号码 | 
| confidenceLevel | [IN]非结论性测试的期望置信水平 | 

**返回：**

0 op是复合数

1 op是素数

-1 测试是非结论性的，但op是复合的概率小于2^（-confidenceLevel）

**起始版本：**

1


### TEE_BigIntMod()

```
void TEE_BigIntMod (TEE_BigInt * dest, const TEE_BigInt * op, const TEE_BigInt * n )
```

**描述：**

计算dest = op (mod n)，使得0 &lt;= dest &lt; n

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| dest | [OUT]指向TEE_BigInt的指针，以保存结果op (mod n) | 
| op | [IN]指向要缩减mod n的操作数的指针 | 
| n | [IN]指向模数的指针。模数应大于1 | 

**起始版本：**

1


### TEE_BigIntMul()

```
void TEE_BigIntMul (TEE_BigInt * dest, const TEE_BigInt * op1, const TEE_BigInt * op2 )
```

**描述：**

计算 dest = op1 \* op2

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| dest | [OUT]指向TEE_BigInt的指针，用于存储结果op1 \* op2 | 
| op1 | [IN]指向第一个操作数的指针 | 
| op2 | [IN]指向第二个操作数的指针 | 

**起始版本：**

1


### TEE_BigIntMulMod()

```
void TEE_BigIntMulMod (TEE_BigInt * dest, const TEE_BigInt * op1, const TEE_BigInt * op2, const TEE_BigInt * n )
```

**描述：**

计算dest = (op1 \* op2) (mod n)

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| dest | [OUT]指向TEE_BigInt的指针，以保存结果(op1 \* op2)(mod n) | 
| op1 | [IN]指向第一个操作数的指针 | 
| op2 | [IN]指向第二个操作数的指针 | 
| n | [IN]指向模数的指针，模数应大于1 | 

**起始版本：**

1


### TEE_BigIntNeg()

```
void TEE_BigIntNeg (TEE_BigInt * dest, const TEE_BigInt * op )
```

**描述：**

取反操作数：dest = -op

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| dest | [OUT]P指向TEE_BigInt的指针，用于存储结果-op | 
| op | [IN]指向要取反的操作数的指针 | 

**起始版本：**

1


### TEE_BigIntRelativePrime()

```
bool TEE_BigIntRelativePrime (const TEE_BigInt * op1, const TEE_BigInt * op2 )
```

**描述：**

确定是否gcd(op1, op2) == 1

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| op1 | [IN]指向第一个操作数的指针 | 
| op2 | [IN]指向第二个操作数的指针 | 

**返回：**

true gcd(op1, op2) == 1

false gcd(op1, op2) != 1

**起始版本：**

1


### TEE_BigIntShiftRight()

```
void TEE_BigIntShiftRight (TEE_BigInt * dest, const TEE_BigInt * op, size_t bits )
```

**描述：**

计算 |dest| = |op| &gt;&gt; bits

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| dest | [OUT]指向TEE_BigInt的指针，用于保存移位的结果 | 
| op | [IN]指向要移位的操作数的指针 | 
| bits | [IN]要移位的位数 | 

**起始版本：**

1


### TEE_BigIntSquare()

```
void TEE_BigIntSquare (TEE_BigInt * dest, const TEE_BigInt * op )
```

**描述：**

计算 dest = op \* op

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| dest | [OUT]指向TEE_BigInt的指针，用于存储结果op \* op | 
| op | [IN]指向要平方的操作数的指针 | 

**起始版本：**

1


### TEE_BigIntSquareMod()

```
void TEE_BigIntSquareMod (TEE_BigInt * dest, const TEE_BigInt * op, const TEE_BigInt * n )
```

**描述：**

计算dest = (op \* op) (mod n)

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| dest | [OUT]指向TEE_BigInt的指针，以保存结果(op \* op)(mod n) | 
| op | [IN]操作数指针 | 
| n | [IN]指向模数的指针。模数应大于1 | 

**起始版本：**

1


### TEE_BigIntSub()

```
void TEE_BigIntSub (TEE_BigInt * dest, const TEE_BigInt * op1, const TEE_BigInt * op2 )
```

**描述：**

计算 dest= op1 - op2

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| dest | [OUT]指向TEE_BigInt的指针，用于存储结果op1 - op2 | 
| op1 | [IN]指向第一个操作数的指针 | 
| op2 | [IN]指向第二个操作数的指针 | 

**起始版本：**

1


### TEE_BigIntSubMod()

```
void TEE_BigIntSubMod (TEE_BigInt * dest, const TEE_BigInt * op1, const TEE_BigInt * op2, const TEE_BigInt * n )
```

**描述：**

计算dest = (op1 - op2) (mod n)

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| dest | [OUT]指向TEE_BigInt的指针，以保存结果(op1 - op2)(mod n) | 
| op1 | [IN]指向第一个操作数的指针 | 
| op2 | [IN]指向第二个操作数的指针 | 
| n | [IN]指向模数的指针，模数应大于1 | 

**起始版本：**

1


## 变量说明


### TEE_BigInt

```
typedefuint32_t TEE_BigInt
```

**描述：**

below definitions are defined by Global Platform for compatibility: don't make any change to the content below

**起始版本：**

1
