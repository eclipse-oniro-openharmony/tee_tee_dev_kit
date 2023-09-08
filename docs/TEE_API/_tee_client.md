# TeeClient


## 概述

TEEC_API 客户端(非安全侧)接口。

提供非安全侧(正常模式)下客户端程序访问安全模式下安全应用相关接口。

**起始版本：**

11


## 汇总


### 文件

| 名称 | 描述 | 
| -------- | -------- |
| [tee_client_api.h](tee__client__api_8h.md) | 客户端应用访问安全应用相关接口定义。 | 
| [tee_client_constants.h](tee__client__constants_8h.md) | 公共数据及常量定义。 | 
| [tee_client_type.h](tee__client__type_8h.md) | 基本数据类型和数据结构定义。 | 


### 结构体

| 名称 | 描述 | 
| -------- | -------- |
| [ListNode](_list_node.md) | 链表类型定义。 | 
| [TEEC_UUID](_t_e_e_c___u_u_i_d.md) | TEEC_UUID类型定义，遵循RFC4122 [2]，用于标识安全应用。 | 
| [TEEC_Context](_t_e_e_c___context.md) | 描述客户端应用与安全世界之间建立的连接环境。 | 
| [TEEC_Session](_t_e_e_c___session.md) | 描述客户端应用与安全世界之间建立的会话。 | 
| [TEEC_SharedMemory](_t_e_e_c___shared_memory.md) | 描述一块共享内存，可以注册，也可以分配。 | 
| [TEEC_TempMemoryReference](_t_e_e_c___temp_memory_reference.md) | 描述一块临时缓冲区指针。 | 
| [TEEC_RegisteredMemoryReference](_t_e_e_c___registered_memory_reference.md) | 描述共享内存指针，指向事先注册或分配好的共享内存。 | 
| [TEEC_Value](_t_e_e_c___value.md) | 描述少量数据。 | 
| [TEEC_Parameter](union_t_e_e_c___parameter.md) | 描述[TEEC_Operation](_t_e_e_c___operation.md)所对应的参数类型。 | 
| [TEEC_Operation](_t_e_e_c___operation.md) | 打开会话或发送命令时的参数。 | 


### 宏定义

| 名称 | 描述 | 
| -------- | -------- |
| [TEEC_PARAM_NUM](#teec_param_num)   4 | 定义TEEC_Operation中TEEC_Parameter个数。 | 


### 类型定义

| 名称 | 描述 | 
| -------- | -------- |
| [TEEC_Result](#teec_result) | 函数返回值类型定义。 | 


### 枚举

| 名称 | 描述 | 
| -------- | -------- |
| [TEEC_ReturnCode](#teec_returncode) {<br/>TEEC_SUCCESS = 0x0, TEEC_ERROR_INVALID_CMD, TEEC_ERROR_SERVICE_NOT_EXIST, TEEC_ERROR_SESSION_NOT_EXIST,<br/>TEEC_ERROR_SESSION_MAXIMUM, TEEC_ERROR_REGISTER_EXIST_SERVICE, TEEC_ERROR_TAGET_DEAD_FATAL, TEEC_ERROR_READ_DATA,<br/>TEEC_ERROR_WRITE_DATA, TEEC_ERROR_TRUNCATE_OBJECT, TEEC_ERROR_SEEK_DATA, TEEC_ERROR_FSYNC_DATA,<br/>TEEC_ERROR_RENAME_OBJECT, TEEC_ERROR_TRUSTED_APP_LOAD_ERROR, TEEC_ERROR_GENERIC = 0xFFFF0000, TEEC_ERROR_ACCESS_DENIED = 0xFFFF0001,<br/>TEEC_ERROR_CANCEL = 0xFFFF0002, TEEC_ERROR_ACCESS_CONFLICT = 0xFFFF0003, TEEC_ERROR_EXCESS_DATA = 0xFFFF0004, TEEC_ERROR_BAD_FORMAT = 0xFFFF0005,<br/>TEEC_ERROR_BAD_PARAMETERS = 0xFFFF0006, TEEC_ERROR_BAD_STATE = 0xFFFF0007, TEEC_ERROR_ITEM_NOT_FOUND = 0xFFFF0008, TEEC_ERROR_NOT_IMPLEMENTED = 0xFFFF0009,<br/>TEEC_ERROR_NOT_SUPPORTED = 0xFFFF000A, TEEC_ERROR_NO_DATA = 0xFFFF000B, TEEC_ERROR_OUT_OF_MEMORY = 0xFFFF000C, TEEC_ERROR_BUSY = 0xFFFF000D,<br/>TEEC_ERROR_COMMUNICATION = 0xFFFF000E, TEEC_ERROR_SECURITY = 0xFFFF000F, TEEC_ERROR_SHORT_BUFFER = 0xFFFF0010, TEEC_ERROR_MAC_INVALID = 0xFFFF3071,<br/>TEEC_ERROR_TARGET_DEAD = 0xFFFF3024, TEEC_FAIL = 0xFFFF5002<br/>} | 定义函数返回的错误码。 | 
| [TEEC_ReturnCodeOrigin](#teec_returncodeorigin) { TEEC_ORIGIN_API = 0x1, TEEC_ORIGIN_COMMS = 0x2, TEEC_ORIGIN_TEE = 0x3, TEEC_ORIGIN_TRUSTED_APP = 0x4 } | 定义函数返回错误码的来源。 | 
| [TEEC_SharedMemCtl](#teec_sharedmemctl) { TEEC_MEM_INPUT = 0x1, TEEC_MEM_OUTPUT = 0x2, TEEC_MEM_INOUT = 0x3 } | 定义共享内存标识。 | 
| [TEEC_ParamType](#teec_paramtype) {<br/>TEEC_NONE = 0x0, TEEC_VALUE_INPUT = 0x01, TEEC_VALUE_OUTPUT = 0x02, TEEC_VALUE_INOUT = 0x03,<br/>TEEC_MEMREF_TEMP_INPUT = 0x05, TEEC_MEMREF_TEMP_OUTPUT = 0x06, TEEC_MEMREF_TEMP_INOUT = 0x07, TEEC_MEMREF_WHOLE = 0xc,<br/>TEEC_MEMREF_PARTIAL_INPUT = 0xd, TEEC_MEMREF_PARTIAL_OUTPUT = 0xe, TEEC_MEMREF_PARTIAL_INOUT = 0xf<br/>} | 定义参数类型。 | 
| [TEEC_LoginMethod](#teec_loginmethod) {<br/>TEEC_LOGIN_PUBLIC = 0x0, TEEC_LOGIN_USER, TEEC_LOGIN_GROUP, TEEC_LOGIN_APPLICATION = 0x4,<br/>TEEC_LOGIN_USER_APPLICATION = 0x5, TEEC_LOGIN_GROUP_APPLICATION = 0x6, TEEC_LOGIN_IDENTIFY = 0x7<br/>} | 定义Login方式。 | 


### 函数

| 名称 | 描述 | 
| -------- | -------- |
| [TEEC_InitializeContext](#teec_initializecontext) (const char \*name, [TEEC_Context](_t_e_e_c___context.md) \*context) | 初始化TEE环境。 | 
| [TEEC_FinalizeContext](#teec_finalizecontext) ([TEEC_Context](_t_e_e_c___context.md) \*context) | 关闭TEE环境。 | 
| [TEEC_OpenSession](#teec_opensession) ([TEEC_Context](_t_e_e_c___context.md) \*context, [TEEC_Session](_t_e_e_c___session.md) \*session, const [TEEC_UUID](_t_e_e_c___u_u_i_d.md) \*destination, uint32_t connectionMethod, const void \*connectionData, [TEEC_Operation](_t_e_e_c___operation.md) \*operation, uint32_t \*returnOrigin) | 打开会话。 | 
| [TEEC_CloseSession](#teec_closesession) ([TEEC_Session](_t_e_e_c___session.md) \*session) | 关闭会话。 | 
| [TEEC_InvokeCommand](#teec_invokecommand) ([TEEC_Session](_t_e_e_c___session.md) \*session, uint32_t commandID, [TEEC_Operation](_t_e_e_c___operation.md) \*operation, uint32_t \*returnOrigin) | 发送命令。 | 
| [TEEC_RegisterSharedMemory](#teec_registersharedmemory) ([TEEC_Context](_t_e_e_c___context.md) \*context, [TEEC_SharedMemory](_t_e_e_c___shared_memory.md) \*sharedMem) | 注册共享内存。 | 
| [TEEC_AllocateSharedMemory](#teec_allocatesharedmemory) ([TEEC_Context](_t_e_e_c___context.md) \*context, [TEEC_SharedMemory](_t_e_e_c___shared_memory.md) \*sharedMem) | 申请共享内存。 | 
| [TEEC_ReleaseSharedMemory](#teec_releasesharedmemory) ([TEEC_SharedMemory](_t_e_e_c___shared_memory.md) \*sharedMem) | 释放共享内存。 | 
| [TEEC_RequestCancellation](#teec_requestcancellation) ([TEEC_Operation](_t_e_e_c___operation.md) \*operation) | 取消正在运行的操作。 | 


## 宏定义说明


### TEEC_PARAM_NUM

```
#define TEEC_PARAM_NUM   4
```

**描述：**

定义TEEC_Operation中TEEC_Parameter个数。

**起始版本：**

11


## 类型定义说明


### TEEC_Result

```
typedef enum TEEC_ReturnCodeTEEC_Result
```

**描述：**

函数返回值类型定义。

**起始版本：**

11


## 枚举类型说明


### TEEC_LoginMethod

```
enum TEEC_LoginMethod
```

**描述：**

定义Login方式。

| 枚举值 | 描述 | 
| -------- | -------- |
| TEEC_LOGIN_PUBLIC | 不需要Login数据 | 
| TEEC_LOGIN_USER | 提供用户运行客户端应用的Login数据 | 
| TEEC_LOGIN_GROUP | 提供组用户运行客户端应用的Login数据 | 
| TEEC_LOGIN_APPLICATION | 提供客户端应用自己的Login数据 | 
| TEEC_LOGIN_USER_APPLICATION | 提供用户运行客户端应用的Login数据，以及客户端应用自己的Login数据 | 
| TEEC_LOGIN_GROUP_APPLICATION | 提供组用户运行客户端应用的Login数据，以及客户端应用自己的Login数据 | 
| TEEC_LOGIN_IDENTIFY | TEEOS预留LoginMethod | 

**起始版本：**

11


### TEEC_ParamType

```
enum TEEC_ParamType
```

**描述：**

定义参数类型。

| 枚举值 | 描述 | 
| -------- | -------- |
| TEEC_NONE | 参数没有使用 | 
| TEEC_VALUE_INPUT | 与TEEC_Value对应，只能作为输入，数据流是从客户端应用到安全应用 | 
| TEEC_VALUE_OUTPUT | 与TEEC_Value对应，只能作为输出，数据流是从安全应用到客户端应用 | 
| TEEC_VALUE_INOUT | 与TEEC_Value对应，既可输入也可输出 | 
| TEEC_MEMREF_TEMP_INPUT | 与TEEC_TempMemoryReference对应，只能作为输入，数据流是从客户端应用到安全应用 | 
| TEEC_MEMREF_TEMP_OUTPUT | 与TEEC_TempMemoryReference对应，只能作为输出，数据流是从安全应用到客户端应用 | 
| TEEC_MEMREF_TEMP_INOUT | 与TEEC_TempMemoryReference对应，既可输入也可输出，可在客户端应用与安全应用之间双向传递 | 
| TEEC_MEMREF_WHOLE | 与TEEC_RegisteredMemoryReference对应，引用整块内存，数据流与所指向的共享内存的标识TEEC_SharedMemCtl一致 | 
| TEEC_MEMREF_PARTIAL_INPUT | 与TEEC_RegisteredMemoryReference对应，只能作为输入，数据流是从客户端应用到安全应用 | 
| TEEC_MEMREF_PARTIAL_OUTPUT | 与TEEC_RegisteredMemoryReference对应，只能作为输出，数据流是从安全应用到客户端应用 | 
| TEEC_MEMREF_PARTIAL_INOUT | 与TEEC_RegisteredMemoryReference对应，既可输入也可输出，可在客户端应用与安全应用之间双向传递 | 

**起始版本：**

11


### TEEC_ReturnCode

```
enum TEEC_ReturnCode
```

**描述：**

定义函数返回的错误码。

| 枚举值 | 描述 | 
| -------- | -------- |
| TEEC_SUCCESS | 函数返回成功 | 
| TEEC_ERROR_INVALID_CMD | 非法命令，安全应用不支持的命令 | 
| TEEC_ERROR_SERVICE_NOT_EXIST | 安全应用不存在 | 
| TEEC_ERROR_SESSION_NOT_EXIST | 客户端应用与安全应用的连接不存在 | 
| TEEC_ERROR_SESSION_MAXIMUM | 安全应用的连接数已满 | 
| TEEC_ERROR_REGISTER_EXIST_SERVICE | 注册已经存在的安全应用 | 
| TEEC_ERROR_TAGET_DEAD_FATAL | 安全OS框架错误 | 
| TEEC_ERROR_READ_DATA | 读取文件错误 | 
| TEEC_ERROR_WRITE_DATA | 写入文件错误 | 
| TEEC_ERROR_TRUNCATE_OBJECT | 截断文件错误 | 
| TEEC_ERROR_SEEK_DATA | 查找文件错误 | 
| TEEC_ERROR_FSYNC_DATA | 同步文件错误 | 
| TEEC_ERROR_RENAME_OBJECT | 重命名文件错误 | 
| TEEC_ERROR_TRUSTED_APP_LOAD_ERROR | 打开会话时，加载安全应用失败 | 
| TEEC_ERROR_GENERIC | 通用错误，初始化安全应用失败 | 
| TEEC_ERROR_ACCESS_DENIED | 权限校验失败，打开TEE环境、打开会话和发送命令都会进行权限的校验，权限校验不通过会返回此类错误 | 
| TEEC_ERROR_CANCEL | 操作已取消，如果参数的取消标志位已置位，再对此参数进行操作时返回此类错误 | 
| TEEC_ERROR_ACCESS_CONFLICT | 并发访问导致权限冲突，安全存储服务中对文件的并发访问可能会产生此类错误 | 
| TEEC_ERROR_EXCESS_DATA | 操作包含的数据太多 ，安全应用无法解析 | 
| TEEC_ERROR_BAD_FORMAT | 数据格式不正确，客户端应用填充的参数格式安全应用无法解析 | 
| TEEC_ERROR_BAD_PARAMETERS | 参数无效，入参为null或非法等错误 | 
| TEEC_ERROR_BAD_STATE | 当前状态下的操作无效，请求安全存储服务操作时，如果没有初始化安全存储服务，会返回此类错误 | 
| TEEC_ERROR_ITEM_NOT_FOUND | 请求的数据未找到 | 
| TEEC_ERROR_NOT_IMPLEMENTED | 请求的操作存在但暂未实现，请求取消操作时返回此类错误 | 
| TEEC_ERROR_NOT_SUPPORTED | 请求的操作有效但未支持，请求安全加解密服务的一些算法，如DSA等时返回此类错误 | 
| TEEC_ERROR_NO_DATA | 数据错误 ，请求的操作找不到对应的数据 | 
| TEEC_ERROR_OUT_OF_MEMORY | 系统可用资源不足，内存申请失败会返回此类错误 | 
| TEEC_ERROR_BUSY | 系统繁忙，系统可能正在独占一些资源 | 
| TEEC_ERROR_COMMUNICATION | 非安全世界应用程序与安全应用通信时发生错误 | 
| TEEC_ERROR_SECURITY | 检测到安全错误，安全世界发生错误 | 
| TEEC_ERROR_SHORT_BUFFER | 输入长度小于输出长度，使用类型为TEEC_MEMREF_TEMP_OUTPUT时需要注意此类错误 | 
| TEEC_ERROR_MAC_INVALID | MAC值校验错误 | 
| TEEC_ERROR_TARGET_DEAD | 安全应用崩溃 | 
| TEEC_FAIL | 通用错误 | 

**起始版本：**

11


### TEEC_ReturnCodeOrigin

```
enum TEEC_ReturnCodeOrigin
```

**描述：**

定义函数返回错误码的来源。

| 枚举值 | 描述 | 
| -------- | -------- |
| TEEC_ORIGIN_API | 错误码来自客户端API | 
| TEEC_ORIGIN_COMMS | 错误码来自非安全世界与安全世界的通信 | 
| TEEC_ORIGIN_TEE | 错误码来自安全世界 | 
| TEEC_ORIGIN_TRUSTED_APP | 错误码来自安全应用 | 

**起始版本：**

11


### TEEC_SharedMemCtl

```
enum TEEC_SharedMemCtl
```

**描述：**

定义共享内存标识。

| 枚举值 | 描述 | 
| -------- | -------- |
| TEEC_MEM_INPUT | 共享内存的数据流是从客户端应用到安全应用 | 
| TEEC_MEM_OUTPUT | 共享内存的数据流是从安全应用到客户端应用 | 
| TEEC_MEM_INOUT | 共享内存可在客户端应用与安全应用之间双向传递 | 

**起始版本：**

11


## 函数说明


### TEEC_AllocateSharedMemory()

```
TEEC_Result TEEC_AllocateSharedMemory (TEEC_Context * context, TEEC_SharedMemory * sharedMem )
```

**描述：**

申请共享内存。

在指定的TEE环境context内申请共享内存sharedMem， 通过共享内存可以实现非安全世界与安全世界传递数据时的零拷贝，需要操作系统的支持， 目前的实现中，该方式不能实现零拷贝。

注意，如果入参sharedMem的size域设置为0，函数会返回成功，但无法使用这块共享内存，因为这块内存既没有地址也没有大小。

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| context | [IN/OUT] 已初始化成功的TEE环境。 | 
| sharedMem | [IN/OUT] 共享内存指针，共享内存的大小不能为零。 | 

**返回：**

TEEC_SUCCESS  发送命令成功； TEEC_ERROR_BAD_PARAMETERS  参数不正确，参数context为null或sharedMem为null； TEEC_ERROR_OUT_OF_MEMORY  系统可用资源不足，分配失败。

**起始版本：**

11


### TEEC_CloseSession()

```
void TEEC_CloseSession (TEEC_Session * session)
```

**描述：**

关闭会话。

关闭session指向的会话，断开客户端应用与安全应用的连接。

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| session | [IN/OUT] 指向已成功打开的会话。 | 

**起始版本：**

11


### TEEC_FinalizeContext()

```
void TEEC_FinalizeContext (TEEC_Context * context)
```

**描述：**

关闭TEE环境。

关闭context指向的TEE环境，断开客户端应用与TEE环境的连接。

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| context | [IN/OUT] 指向已初始化成功的TEE环境。 | 

**起始版本：**

11


### TEEC_InitializeContext()

```
TEEC_Result TEEC_InitializeContext (const char * name, TEEC_Context * context )
```

**描述：**

初始化TEE环境。

初始化路径为name的TEE环境，参数name可以为null， 初始化TEE环境是打开会话、发送命令的基础， 初始化成功后，客户端应用与TEE建立一条连接。

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| name | [IN] TEE环境路径。 | 
| context | [IN/OUT] context指针，安全世界环境句柄。 | 

**返回：**

TEEC_SUCCESS  初始化TEE环境成功； TEEC_ERROR_BAD_PARAMETERS  参数不正确，name不正确或context为null； TEEC_ERROR_GENERIC  系统可用资源不足等原因。

**起始版本：**

11


### TEEC_InvokeCommand()

```
TEEC_Result TEEC_InvokeCommand (TEEC_Session * session, uint32_t commandID, TEEC_Operation * operation, uint32_t * returnOrigin )
```

**描述：**

发送命令。

在指定的会话session里，由客户端应用向安全应用发送命令commandID， 发送的数据为operation，如果发送命令失败，输出参数returnOrigin为错误来源。

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| session | [IN/OUT] 指向已打开成功的会话。 | 
| commandID | [IN] 安全应用支持的命令ID，由安全应用定义。 | 
| operation | [IN/OUT] 包含了客户端应用向安全应用发送的数据内容。 | 
| returnOrigin | [IN/OUT] 错误来源，取值范围为[TEEC_ReturnCodeOrigin](#teec_returncodeorigin)  。 | 

**返回：**

TEEC_SUCCESS  发送命令成功； TEEC_ERROR_BAD_PARAMETERS  参数不正确，参数session为null或参数operation格式不正确； TEEC_ERROR_ACCESS_DENIED  系统调用权限访问失败； TEEC_ERROR_OUT_OF_MEMORY  系统可用资源不足； 其它返回值参考[TEEC_ReturnCode](#teec_returncode)  。

**起始版本：**

11


### TEEC_OpenSession()

```
TEEC_Result TEEC_OpenSession (TEEC_Context * context, TEEC_Session * session, const TEEC_UUID * destination, uint32_t connectionMethod, const void * connectionData, TEEC_Operation * operation, uint32_t * returnOrigin )
```

**描述：**

打开会话。

在指定的TEE环境context下，为客户端应用与UUID为destination的安全应用建立一条连接， 连接方式是connectionMethod，连接数据是connectionData，传递的数据包含在operation里。 打开会话成功后，输出参数session是对该连接的一个描述； 如果打开会话失败，输出参数returnOrigin为错误来源。

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| context | [IN/OUT] 指向已初始化成功的TEE环境。 | 
| session | [OUT] 指向会话，取值不能为null。 | 
| destination | [IN] 安全应用的UUID，一个安全应用拥有唯一的UUID。 | 
| connectionMethod | [IN] 连接方式，取值范围为[TEEC_LoginMethod](#teec_loginmethod)  。 | 
| connectionData | [IN] 与连接方式相对应的连接数据， 如果连接方式为TEEC_LOGIN_PUBLIC  、TEEC_LOGIN_USER  、 TEEC_LOGIN_USER_APPLICATION  、TEEC_LOGIN_GROUP_APPLICATION  ，连接数据取值必须为null， 如果连接方式为TEEC_LOGIN_GROUP  、TEEC_LOGIN_GROUP_APPLICATION  ， 连接数据必须指向类型为uint32_t的数据，此数据表示客户端应用期望连接的组用户。 | 
| operation | [IN/OUT] 客户端应用与安全应用传递的数据。 | 
| returnOrigin | [IN/OUT] 错误来源，取值范围为[TEEC_ReturnCodeOrigin](#teec_returncodeorigin)  。 | 

**返回：**

TEEC_SUCCESS  打开会话成功； TEEC_ERROR_BAD_PARAMETERS  参数不正确，context、session或destination为null； TEEC_ERROR_ACCESS_DENIED  系统调用权限访问失败； TEEC_ERROR_OUT_OF_MEMORY  系统可用资源不足； TEEC_ERROR_TRUSTED_APP_LOAD_ERROR  加载安全应用失败； 其它返回值参考[TEEC_ReturnCode](#teec_returncode)  。

**起始版本：**

11


### TEEC_RegisterSharedMemory()

```
TEEC_Result TEEC_RegisterSharedMemory (TEEC_Context * context, TEEC_SharedMemory * sharedMem )
```

**描述：**

注册共享内存。

在指定的TEE环境context内注册共享内存sharedMem， 通过注册的方式获取共享内存来实现零拷贝，需要操作系统的支持， 目前的实现中，该方式不能实现零拷贝。

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| context | [IN/OUT] 已初始化成功的TEE环境。 | 
| sharedMem | [IN/OUT] 共享内存指针，共享内存所指向的内存不能为null、大小不能为零。 | 

**返回：**

TEEC_SUCCESS  发送命令成功； TEEC_ERROR_BAD_PARAMETERS  参数不正确，context或sharedMem为null，或共享内存所指向的内存为空。

**起始版本：**

11


### TEEC_ReleaseSharedMemory()

```
void TEEC_ReleaseSharedMemory (TEEC_SharedMemory * sharedMem)
```

**描述：**

释放共享内存。

释放已注册成功的的或已申请成功的共享内存sharedMem。

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| sharedMem | [IN/OUT] 指向已注册成功或申请成功的共享内存。 | 

**注意:**

如果是通过[TEEC_AllocateSharedMemory](#teec_allocatesharedmemory)  方式获取的共享内存， 释放时会回收这块内存；如果是通过[TEEC_RegisterSharedMemory](#teec_registersharedmemory)  方式 获取的共享内存，释放时不会回收共享内存所指向的本地内存。

**起始版本：**

11


### TEEC_RequestCancellation()

```
void TEEC_RequestCancellation (TEEC_Operation * operation)
```

**描述：**

取消正在运行的操作。

取消掉一个正在运行的open Session或者是一个invoke command 发送一个cancel的signal后立即返回。

注意，此操作仅仅是发送一个cancel的消息，是否进行cancel操作由TEE或TA决定，目前这个取消操作不生效。

**参数：**

| 名称 | 描述 | 
| -------- | -------- |
| operation | [IN/OUT] 包含了客户端应用向安全应用发送的数据内容。 | 

**起始版本：**

11
