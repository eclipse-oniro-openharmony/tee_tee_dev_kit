# TEE客户端应用开发指导<a name="ZH-CN_TOPIC_0000001202869620"></a>

## 接口说明<a name="section373320286518"></a>

TEE为CA提供了一组标准API来访问TA，这些API声明在“tee\_client\_api.h”中，符合GP规范，详细接口定义及使用可参考GP文档：TEE Client API  [Specification](https://globalplatform.org/specs-library/?filter-committee=tee)  v1.0 \(GPD\_SPE\_007\)。CA必须按照合理的顺序调用这些API，才能正确访问TA，用户态CA接口列表如下：

<a name="table12163622913"></a>
<table><thead align="left"><tr id="row111891027912"><th class="cellrowborder" valign="top" width="36.05%" id="mcps1.1.3.1.1"><p id="p16189172593"><a name="p16189172593"></a><a name="p16189172593"></a>接口名</p>
</th>
<th class="cellrowborder" valign="top" width="63.949999999999996%" id="mcps1.1.3.1.2"><p id="p191899219916"><a name="p191899219916"></a><a name="p191899219916"></a>描述</p>
</th>
</tr>
</thead>
<tbody><tr id="row23075814814"><td class="cellrowborder" valign="top" width="36.05%" headers="mcps1.1.3.1.1 "><p id="p1886146163917"><a name="p1886146163917"></a><a name="p1886146163917"></a>TEEC_InitializeContext</p>
</td>
<td class="cellrowborder" valign="top" width="63.949999999999996%" headers="mcps1.1.3.1.2 "><p id="p1746563812810"><a name="p1746563812810"></a><a name="p1746563812810"></a>初始化路径为name的TEE环境，参数name可以为空，成功后，CA与TEE建立一条连接。</p>
</td>
</tr>
<tr id="row124255515487"><td class="cellrowborder" valign="top" width="36.05%" headers="mcps1.1.3.1.1 "><p id="p6312103974015"><a name="p6312103974015"></a><a name="p6312103974015"></a>TEEC_FinalizeContext</p>
</td>
<td class="cellrowborder" valign="top" width="63.949999999999996%" headers="mcps1.1.3.1.2 "><p id="p1593083764118"><a name="p1593083764118"></a><a name="p1593083764118"></a>关闭context指向的TEE环境，断开CA与TEE环境的链接.</p>
</td>
</tr>
<tr id="row284812823715"><td class="cellrowborder" valign="top" width="36.05%" headers="mcps1.1.3.1.1 "><p id="p1584858183710"><a name="p1584858183710"></a><a name="p1584858183710"></a>TEEC_OpenSession</p>
</td>
<td class="cellrowborder" valign="top" width="63.949999999999996%" headers="mcps1.1.3.1.2 "><p id="p10911328435"><a name="p10911328435"></a><a name="p10911328435"></a>在指定的TEE环境context下，为CA与UUID为destination的安全服务建立一条链接，链接方式为connectionMethod，链接数据是connectionData，传递的数据包含在opetation里。</p>
<a name="ul148701034144319"></a><a name="ul148701034144319"></a><ul id="ul148701034144319"><li>打开会话成功后，输出参数session是对该连接的一个描述；</li></ul>
<a name="ul11446340184312"></a><a name="ul11446340184312"></a><ul id="ul11446340184312"><li>如果打开会话失败，输出参数returnOrigin为错误来源。</li></ul>
</td>
</tr>
<tr id="row2118181123720"><td class="cellrowborder" valign="top" width="36.05%" headers="mcps1.1.3.1.1 "><p id="p18118811123719"><a name="p18118811123719"></a><a name="p18118811123719"></a>TEEC_CloseSession</p>
</td>
<td class="cellrowborder" valign="top" width="63.949999999999996%" headers="mcps1.1.3.1.2 "><p id="p811971114373"><a name="p811971114373"></a><a name="p811971114373"></a>关闭session指向的会话，断开CA与安全服务的链接。</p>
</td>
</tr>
<tr id="row55817223714"><td class="cellrowborder" valign="top" width="36.05%" headers="mcps1.1.3.1.1 "><p id="p14586243720"><a name="p14586243720"></a><a name="p14586243720"></a>TEEC_InvokeCommand</p>
</td>
<td class="cellrowborder" valign="top" width="63.949999999999996%" headers="mcps1.1.3.1.2 "><p id="p14144320467"><a name="p14144320467"></a><a name="p14144320467"></a>在指定的会话session里，由CA向安全服务发送命令commandID，发送的数据为operation，如果发送命令失败，输出参数returnOrigin为错误来源</p>
</td>
</tr>
<tr id="row138741157103619"><td class="cellrowborder" valign="top" width="36.05%" headers="mcps1.1.3.1.1 "><p id="p887415716369"><a name="p887415716369"></a><a name="p887415716369"></a>TEEC_RegisterSharedMemory</p>
</td>
<td class="cellrowborder" valign="top" width="63.949999999999996%" headers="mcps1.1.3.1.2 "><p id="p2879162344611"><a name="p2879162344611"></a><a name="p2879162344611"></a>在指定的TEE环境context内注册共享内存sharedMem。</p>
</td>
</tr>
<tr id="row12546560371"><td class="cellrowborder" valign="top" width="36.05%" headers="mcps1.1.3.1.1 "><p id="p17546106183715"><a name="p17546106183715"></a><a name="p17546106183715"></a>TEEC_AllocateSharedMemory</p>
</td>
<td class="cellrowborder" valign="top" width="63.949999999999996%" headers="mcps1.1.3.1.2 "><p id="p974925519462"><a name="p974925519462"></a><a name="p974925519462"></a>在指定的TEE环境context内申请共享内存sharedMem。</p>
</td>
</tr>
<tr id="row9414165419368"><td class="cellrowborder" valign="top" width="36.05%" headers="mcps1.1.3.1.1 "><p id="p134143547363"><a name="p134143547363"></a><a name="p134143547363"></a>TEEC_ReleaseSharedMemory</p>
</td>
<td class="cellrowborder" valign="top" width="63.949999999999996%" headers="mcps1.1.3.1.2 "><p id="p34146543368"><a name="p34146543368"></a><a name="p34146543368"></a>释放已注册成功的的或已申请成功的共享内存sharedMem。</p>
<a name="ul62661041114819"></a><a name="ul62661041114819"></a><ul id="ul62661041114819"><li>如果是通过TEEC_AllocateSharedMemory方式获取的共享内存，释放时会回收这块内存；</li><li>如果是通过TEEC_RegisterSharedMemory方式获取的共享内存，释放时不会回收共享内存所指向的本地内存。</li></ul>
</td>
</tr>
<tr id="row52456519361"><td class="cellrowborder" valign="top" width="36.05%" headers="mcps1.1.3.1.1 "><p id="p162467514361"><a name="p162467514361"></a><a name="p162467514361"></a>TEEC_RequestCancellation</p>
</td>
<td class="cellrowborder" valign="top" width="63.949999999999996%" headers="mcps1.1.3.1.2 "><p id="p769203616493"><a name="p769203616493"></a><a name="p769203616493"></a>取消掉一个正在运行的TEEC_OpenSession或者是一个TEEC_InvokeCommand命令。</p>
<a name="ul58371120185019"></a><a name="ul58371120185019"></a><ul id="ul58371120185019"><li>此操作仅仅是发送一个cancel的消息，现版本TEEOS不支持该特性。</li></ul>
</td>
</tr>
</tbody>
</table>

内核态CA的接口如下：
<table><thead align="left"><tr><th class="cellrowborder" valign="top" width="36.05%"><p>接口名</p>
</th>
<th class="cellrowborder" valign="top" width="64%"><p>描述</p>
</th>
</tr>
</thead>
<tbody>
<tr><td class="cellrowborder" valign="top" width="36.05%" headers="mcps1.1.3.1.1 "><p>teek_initialize_context</p>
</td>
<td class="cellrowborder" valign="top" width="64%" headers="mcps1.1.3.1.2 "><p>初始化路径为name的TEE环境，参数name可以为空，成功后，CA与TEE建立一条连接。</p>
</td>
</tr>
<tr><td class="cellrowborder" valign="top" width="36.05%" headers="mcps1.1.3.1.1 "><p>teek_finalize_context</p>
</td>
<td class="cellrowborder" valign="top" width="64%" headers="mcps1.1.3.1.2 "><p>关闭context指向的TEE环境，断开CA与TEE环境的链接.</p>
</td>
</tr>
<tr><td class="cellrowborder" valign="top" width="36.05%" headers="mcps1.1.3.1.1 "><p>teek_open_session</p>
</td>
<td class="cellrowborder" valign="top" width="64%" headers="mcps1.1.3.1.2 "><p>在指定的TEE环境context下，为CA与UUID为destination的安全服务建立一条链接，链接方式为connectionMethod，链接数据是connectionData，传递的数据包含在opetation里。</p>
<a name="ul148701034144319"></a><a name="ul148701034144319"></a><ul id="ul148701034144319"><li>打开会话成功后，输出参数session是对该连接的一个描述；</li></ul>
<a name="ul11446340184312"></a><a name="ul11446340184312"></a><ul id="ul11446340184312"><li>如果打开会话失败，输出参数returnOrigin为错误来源。</li></ul>
</td>
</tr>
<tr><td class="cellrowborder" valign="top" width="36.05%" headers="mcps1.1.3.1.1 "><p>teek_close_session</p>
</td>
<td class="cellrowborder" valign="top" width="64%" headers="mcps1.1.3.1.2 "><p>关闭session指向的会话，断开CA与安全服务的链接。</p>
</td>
</tr>
<tr><td class="cellrowborder" valign="top" width="36.05%" headers="mcps1.1.3.1.1 "><p>teek_invoke_command</p>
</td>
<td class="cellrowborder" valign="top" width="64%" headers="mcps1.1.3.1.2 "><p>在指定的会话session里，由CA向安全服务发送命令commandID，发送的数据为operation，如果发送命令失败，输出参数returnOrigin为错误来源.</p>
</td>
</tr>
</tbody>
</table>

## 开发说明<a name="section1249761325219"></a>

TEE对CA的形式不做限定，在openHarmony上可以是native程序或SA服务，开发样例可参考GP文档：TEE Client API  [Specification](https://globalplatform.org/specs-library/?filter-committee=tee)  v1.0 \(GPD\_SPE\_007\)。

### 特性与约束<a name="section139361816527"></a>

TEE实现的接口与GP规范相同，但是结构体及使用限制与GP规范有差异，下面介绍差异点。

1.  TEEC\_Context定义支持指定TA路径加载功能

    TEEC\_Context中定义了ta文件路径，在调用TEEC\_OpenSession前，将TEEC\_Context中ta\_path赋值到具体TA路径（限制/data目录下），可将该路径的TA加载到TEEOS中，如果不指定TA路径，会从默认路径（系统组件：system/bin，芯片组件：vendor/bin）下根据UUID查找对应UUID.sec文件加载到TEEOS中。

    ```
    context.ta_path = (uint8_t *)"/data/58dbb3b9-4a0c-42d2-a84d-7c7ab17539fc.sec";
    ```

2.  调用TEEC\_OpenSession中connectionMethod只支持TEEC\_LOGIN\_IDENTIFY

    TEEC\_OpenSession函数中第四个参数为connectionMethod，GP规范定义了六种Login Method，TEEOS在以上基础拓展了TEEC\_LOGIN\_IDENTIFY的类型，且只支持该种connectionMethod。

    ```
    enum TEEC_LoginMethod {
        TEEC_LOGIN_PUBLIC = 0x0,            /* 不需要Login数据 */
        TEEC_LOGIN_USER,                    /* 提供用户运行客户端应用的Login数据 */
        TEEC_LOGIN_GROUP,                   /* 提供组用户运行客户端应用的Login数据 */
        TEEC_LOGIN_APPLICATION = 0x4,       /* 提供客户端应用自己的Login数据 */
        TEEC_LOGIN_USER_APPLICATION = 0x5,  /* 提供用户运行客户端应用的Login数据，以及客户端应用自己的Login数据*/
        TEEC_LOGIN_GROUP_APPLICATION = 0x6, /* 提供组用户运行客户端应用的Login数据，以及客户端应用自己的Login数据*/
        TEEC_LOGIN_IDENTIFY = 0x7,          /* TEEOS预留LoginMethod */
    };
    ```

3.  调用TEEC\_OpenSession时，TEEC\_Operation参数有限制

    在调用TEEC\_OpenSession接口时，TEEC\_Operation中params\[2\]和params\[3\]是预留给系统的，不允许CA使用，CA仅可以使用params\[0\]和params\[1\]。在调用TEEC\_InvokeCommand接口时，则没有限制。


## 客户端应用错误码分析<a name="section950012975911"></a>

以TEEC\_OpenSession为例，客户端应用需要关注函数返回值TEEC\_Result和函数的出参returnOrigin。

```
TEEC_Result TEEC_OpenSession(
    TEEC_Context *context,
    TEEC_Session *session,
    const TEEC_UUID *destination,
    uint32_t connectionMethod,
    const void *connectionData,
    TEEC_Operation *operation,
    uint32_t *returnOrigin);
```

returnOrigin表示错误来源，其取值参考枚举TEEC\_ReturnCodeOrigin；TEEC\_Result表示具体错误码，其取值参考枚举TEEC\_ReturnCode。

TEEC\_ReturnCodeOrigin和TEEC\_ReturnCode均定义在头文件“tee\_client\_constants.h”中。

**举例**：

日志中可能会发现类似的打印“\[error\]  TEEC\_OpenSession: open session failed, code=0x2, origin=3”，

origin=3表示错误来源，查询TEEC\_ReturnCodeOrigin可知错误来自TEE OS；code=0x2表示具体错误码，查询TEEC\_ReturnCode可知该错误码表示安全服务不存在，此时需要分析安全服务是否加载成功。

