# 使能新平台的指南
## TEE 安全镜像Loader适配指导

### 概述

#### 功能简介

TEE Loader主要负责加载安全镜像并将启动参数传递给TEE OS的功能。

#### 约束与限制

-   芯片架构为ARMv7/ARMv8架构。
-   CPU需支持安全内存和非安全内存的划分，安全和非安全外设的划分。

### 开发指导

#### 场景介绍

由于安全镜像在flash中是以加密形式存储的，因此需要对镜像进行解密处理，随后拷贝到目标执行地址。在TEE OS启动时，需要对其传递启动参数。以上这些都是在Loader中进行的。因此在芯片使能TEE时，需要对Loader进行开发适配。

#### 接口说明<a name="section125843344514"></a>

以下接口说明列表将介绍我们在实际使能TEE过程中对Loader适配将用到的接口。包括：

-   启动参数配置
-   镜像加载

**表 1**  启动参数配置调用接口表

<a name="table1496711447522"></a>

<table><thead align="left"><tr id="row596815449522"><th class="cellrowborder" valign="top" width="47.67%" id="mcps1.2.4.1.1"><p id="p196816442521"><a name="p196816442521"></a><a name="p196816442521"></a>接口名</p>
</th>
<th class="cellrowborder" valign="top" width="32.129999999999995%" id="mcps1.2.4.1.2"><p id="p09681444165214"><a name="p09681444165214"></a><a name="p09681444165214"></a>描述</p>
</th>
<th class="cellrowborder" valign="top" width="20.200000000000003%" id="mcps1.2.4.1.3"><p id="p19431194717249"><a name="p19431194717249"></a><a name="p19431194717249"></a>必选/可选</p>
</th>
</tr>
</thead>
<tbody><tr id="row51478313471"><td class="cellrowborder" valign="top" width="47.67%" headers="mcps1.2.4.1.1 "><p id="p497mcpsimp"><a name="p497mcpsimp"></a><a name="p497mcpsimp"></a>void set_teeos_mem(uintptr_t teeos_base_addr, uint64_t size)</p>
</td>
<td class="cellrowborder" valign="top" width="32.129999999999995%" headers="mcps1.2.4.1.2 "><p id="p498mcpsimp"><a name="p498mcpsimp"></a><a name="p498mcpsimp"></a>设置TEE OS的起始地址size大小。</p>
</td>
<td class="cellrowborder" valign="top" width="20.200000000000003%" headers="mcps1.2.4.1.3 "><p id="p12431947122419"><a name="p12431947122419"></a><a name="p12431947122419"></a><strong id="b1532325172812"><a name="b1532325172812"></a><a name="b1532325172812"></a>必选，OS需要。</strong></p>
</td>
</tr>
<tr id="row214793104714"><td class="cellrowborder" valign="top" width="47.67%" headers="mcps1.2.4.1.1 "><p id="p66112184581"><a name="p66112184581"></a><a name="p66112184581"></a>void set_teeos_uart(uint64_t uart_addr)</p>
</td>
<td class="cellrowborder" valign="top" width="32.129999999999995%" headers="mcps1.2.4.1.2 "><p id="p196103189585"><a name="p196103189585"></a><a name="p196103189585"></a>设置串口地址。</p>
</td>
<td class="cellrowborder" valign="top" width="20.200000000000003%" headers="mcps1.2.4.1.3 "><p id="p164311747112411"><a name="p164311747112411"></a><a name="p164311747112411"></a>可选，建议配置，启动调试用。</p>
</td>
</tr>
<tr id="row3147173114720"><td class="cellrowborder" valign="top" width="47.67%" headers="mcps1.2.4.1.1 "><p id="p512mcpsimp"><a name="p512mcpsimp"></a><a name="p512mcpsimp"></a>void set_gic(struct gic_config_t gic_config)</p>
</td>
<td class="cellrowborder" valign="top" width="32.129999999999995%" headers="mcps1.2.4.1.2 "><p id="p513mcpsimp"><a name="p513mcpsimp"></a><a name="p513mcpsimp"></a>配置gic寄存器。结构体定义见。</p>
</td>
<td class="cellrowborder" valign="top" width="20.200000000000003%" headers="mcps1.2.4.1.3 "><p id="p1043194716242"><a name="p1043194716242"></a><a name="p1043194716242"></a><strong id="b0722234132818"><a name="b0722234132818"></a><a name="b0722234132818"></a>必选，OS需要。</strong></p>
</td>
</tr>
<tr id="row914714313476"><td class="cellrowborder" valign="top" width="47.67%" headers="mcps1.2.4.1.1 "><p id="p2744165514582"><a name="p2744165514582"></a><a name="p2744165514582"></a>bool copy_extend_datas(void * extend_datas, uint64_t extend_length)</p>
</td>
<td class="cellrowborder" valign="top" width="32.129999999999995%" headers="mcps1.2.4.1.2 "><p id="p519mcpsimp"><a name="p519mcpsimp"></a><a name="p519mcpsimp"></a>保留字段拷贝。</p>
</td>
<td class="cellrowborder" valign="top" width="20.200000000000003%" headers="mcps1.2.4.1.3 "><p id="p1431194762415"><a name="p1431194762415"></a><a name="p1431194762415"></a>可选，根据实际情况。</p>
</td>
</tr>
<tr id="row514715384711"><td class="cellrowborder" valign="top" width="47.67%" headers="mcps1.2.4.1.1 "><p id="p520mcpsimp"><a name="p520mcpsimp"></a><a name="p520mcpsimp"></a>bool copy_teeos_cfg(void)</p>
</td>
<td class="cellrowborder" valign="top" width="32.129999999999995%" headers="mcps1.2.4.1.2 "><p id="p521mcpsimp"><a name="p521mcpsimp"></a><a name="p521mcpsimp"></a>拷贝启动参数到目标位置，<strong id="b163513559278"><a name="b163513559278"></a><a name="b163513559278"></a>配置TEE OS属性后再调用此接口</strong>。</p>
</td>
<td class="cellrowborder" valign="top" width="20.200000000000003%" headers="mcps1.2.4.1.3 "><p id="p3431747192413"><a name="p3431747192413"></a><a name="p3431747192413"></a><strong id="b625310456284"><a name="b625310456284"></a><a name="b625310456284"></a>必选，OS需要。</strong></p>
</td>
</tr>
</tbody>
</table>


**表 2**  镜像加载调用接口表

<a name="table15516113715563"></a>
<table><thead align="left"><tr id="row5516113795615"><th class="cellrowborder" valign="top" width="50%" id="mcps1.2.3.1.1"><p id="p10101114295614"><a name="p10101114295614"></a><a name="p10101114295614"></a>接口名</p>
</th>
<th class="cellrowborder" valign="top" width="50%" id="mcps1.2.3.1.2"><p id="p161017421562"><a name="p161017421562"></a><a name="p161017421562"></a>描述</p>
</th>
</tr>
</thead>
<tbody><tr id="row14516237105613"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p1336862755413"><a name="p1336862755413"></a><a name="p1336862755413"></a>uintptr_t read_teeos(const char *part_name, uint32_t part_size)</p>
</td>
<td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p1351643714569"><a name="p1351643714569"></a><a name="p1351643714569"></a>将TEE OS从flash分区读镜像到RAM中。</p>
</td>
</tr>
<tr id="row051633710563"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p1463764075419"><a name="p1463764075419"></a><a name="p1463764075419"></a>int32_t verify_teeos(uintptr_t buf_addr)</p>
</td>
<td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p105166372562"><a name="p105166372562"></a><a name="p105166372562"></a>TEE OS镜像验签。</p>
</td>
</tr>
<tr id="row15516143712565"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p155165377567"><a name="p155165377567"></a><a name="p155165377567"></a>int32_t decrypt_teeos(uintptr_t buf_addr)</p>
</td>
<td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p851619377562"><a name="p851619377562"></a><a name="p851619377562"></a>TEE OS镜像解密。</p>
</td>
</tr>
<tr id="row13516153714567"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p351653735618"><a name="p351653735618"></a><a name="p351653735618"></a>int32_t copy_teeos(uintptr_t buf_addr)</p>
</td>
<td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p152771458171114"><a name="p152771458171114"></a><a name="p152771458171114"></a>TEE OS镜像拷贝到目标执行地址。</p>
</td>
</tr>
<tr id="row15835727135513"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p158111828105519"><a name="p158111828105519"></a><a name="p158111828105519"></a>uint64_t get_teeos_start(void)</p>
</td>
<td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p281114283558"><a name="p281114283558"></a><a name="p281114283558"></a>获取TEE OS启动地址。</p>
</td>
</tr>
</tbody>
</table>


#### 接口参数结构体定义<a name="section15511163311570"></a>

```
struct gic_config_t {
    char version;
    union {
    struct v2_t {
        p_region_t dist;
        p_region_t contr;
    } v2;
    struct v3_t {
        p_region_t dist;
        uint32_t redist_num;
        uint32_t redist_stride;
        p_region_t redist[GICR_MAX_NUM];
        } v3;
   };
};
```

#### 开发步骤

1.  启动参数配置

    启动参数包含TEE OS用到的安全内存地址和大小，串口的地址，gic寄存器配置，其它拓展参数。

    loader中启动参数的适配方法可以按照产品的习惯要求采用不同的方法，例如结构体中直接填写相应的参数，或者增加配置文件的方法。

    **表 3**  启动参数列表

    <a name="table31859499130"></a>
    <table><thead align="left"><tr id="row3185174941311"><th class="cellrowborder" valign="top" width="20.31%" id="mcps1.2.4.1.1"><p id="p9186154916134"><a name="p9186154916134"></a><a name="p9186154916134"></a>参数</p>
    </th>
    <th class="cellrowborder" valign="top" width="28.64%" id="mcps1.2.4.1.2"><p id="p16186104911131"><a name="p16186104911131"></a><a name="p16186104911131"></a>含义</p>
    </th>
    <th class="cellrowborder" valign="top" width="51.05%" id="mcps1.2.4.1.3"><p id="p12200103644211"><a name="p12200103644211"></a><a name="p12200103644211"></a>备注</p>
    </th>
    </tr>
    </thead>
    <tbody><tr id="row20186849151316"><td class="cellrowborder" valign="top" width="20.31%" headers="mcps1.2.4.1.1 "><p id="p5186194911131"><a name="p5186194911131"></a><a name="p5186194911131"></a>plat_cfg_size</p>
    </td>
    <td class="cellrowborder" valign="top" width="28.64%" headers="mcps1.2.4.1.2 "><p id="p618617496136"><a name="p618617496136"></a><a name="p618617496136"></a>启动参数大小，包含extend_paras部分</p>
    </td>
    <td class="cellrowborder" valign="top" width="51.05%" headers="mcps1.2.4.1.3 "><p id="p1520073664218"><a name="p1520073664218"></a><a name="p1520073664218"></a>整个启动参数的buffer大小，值为sizeof（struct platform_info） + extend _length。</p>
    </td>
    </tr>
    <tr id="row1618618496132"><td class="cellrowborder" valign="top" width="20.31%" headers="mcps1.2.4.1.1 "><p id="p292mcpsimp"><a name="p292mcpsimp"></a><a name="p292mcpsimp"></a>phys_region_size</p>
    </td>
    <td class="cellrowborder" valign="top" width="28.64%" headers="mcps1.2.4.1.2 "><p id="p294mcpsimp"><a name="p294mcpsimp"></a><a name="p294mcpsimp"></a>TEE OS内存大小</p>
    </td>
    <td class="cellrowborder" valign="top" width="51.05%" headers="mcps1.2.4.1.3 "><p id="p320093615420"><a name="p320093615420"></a><a name="p320093615420"></a>2MB的整数倍。</p>
    </td>
    </tr>
    <tr id="row01861049151317"><td class="cellrowborder" valign="top" width="20.31%" headers="mcps1.2.4.1.1 "><p id="p297mcpsimp"><a name="p297mcpsimp"></a><a name="p297mcpsimp"></a>phys_region_start</p>
    </td>
    <td class="cellrowborder" valign="top" width="28.64%" headers="mcps1.2.4.1.2 "><p id="p299mcpsimp"><a name="p299mcpsimp"></a><a name="p299mcpsimp"></a>TEE OS内存起始物理地址</p>
    </td>
    <td class="cellrowborder" valign="top" width="51.05%" headers="mcps1.2.4.1.3 "><p id="p1020063614220"><a name="p1020063614220"></a><a name="p1020063614220"></a>2MB的整数倍。</p>
    </td>
    </tr>
    <tr id="row18186649111319"><td class="cellrowborder" valign="top" width="20.31%" headers="mcps1.2.4.1.1 "><p id="p302mcpsimp"><a name="p302mcpsimp"></a><a name="p302mcpsimp"></a>uart_addr</p>
    </td>
    <td class="cellrowborder" valign="top" width="28.64%" headers="mcps1.2.4.1.2 "><p id="p304mcpsimp"><a name="p304mcpsimp"></a><a name="p304mcpsimp"></a>串口寄存器的基地址</p>
    </td>
    <td class="cellrowborder" valign="top" width="51.05%" headers="mcps1.2.4.1.3 "><p id="p1920015362427"><a name="p1920015362427"></a><a name="p1920015362427"></a>-</p>
    </td>
    </tr>
    <tr id="row59731131174610"><td class="cellrowborder" valign="top" width="20.31%" headers="mcps1.2.4.1.1 "><p id="p9973113119460"><a name="p9973113119460"></a><a name="p9973113119460"></a>struct gic_config_t</p>
    </td>
    <td class="cellrowborder" valign="top" width="28.64%" headers="mcps1.2.4.1.2 "><p id="p11973103114611"><a name="p11973103114611"></a><a name="p11973103114611"></a>gic寄存器参数</p>
    </td>
    <td class="cellrowborder" valign="top" width="51.05%" headers="mcps1.2.4.1.3 "><p id="p797319312466"><a name="p797319312466"></a><a name="p797319312466"></a>-</p>
    </td>
    </tr>
    <tr id="row1188221104618"><td class="cellrowborder" valign="top" width="20.31%" headers="mcps1.2.4.1.1 "><p id="p19884213469"><a name="p19884213469"></a><a name="p19884213469"></a>extend_datas</p>
    </td>
    <td class="cellrowborder" valign="top" width="28.64%" headers="mcps1.2.4.1.2 "><p id="p98811215461"><a name="p98811215461"></a><a name="p98811215461"></a>其它参数</p>
    </td>
    <td class="cellrowborder" valign="top" width="51.05%" headers="mcps1.2.4.1.3 "><p id="p18872120461"><a name="p18872120461"></a><a name="p18872120461"></a>格式需要和TEE OS核对。</p>
    </td>
    </tr>
    </tbody>
    </table>
    
2.  镜像加载

    tee loader适配在芯片平台的bios或者fastboot、preloader中，主要完成的功能有配置并传递TEE OS的启动参数、将TEE OS镜像加载到目标内存位置、传递共享内存信息给TEE OS。loader中启动参数的适配方法可以按照产品的习惯和要求采用不同的方法，例如结构体中直接填写相应的参数，或者增加配置文件。以下为镜像加载步骤：

    1.  从flash分区读镜像到RAM中。
    2.  TEE OS镜像验签。
    3.  TEE OS镜像解密。
    4.  TEE OS镜像拷贝到目标执行地址。
    5.  TEE OS启动地址传递。获取TEE OS的启动地址，一般情况下需要将这个值传给atf, 作为ATF初始化TEE OS的入口地址。

相关配置接口见[接口说明](#section125843344514)。

#### 开发实例<a name="section3748141455319"></a>

- 使用 TEE Loader，在芯片使能TEE时，配置启动参数的开发实例。详细示例代码位于 `base/tee/tee_os_framework/sample/teeloader` 目录。

```C
#define TEEOS_TEXT_OFFSET (0x8000)
#define ALIGN_SIZE_2M    (0x200000)
struct platform_info {
    uint64_t plat_cfg_size;
    uint64_t phys_region_size;
    paddr_t phys_region_start;
    paddr_t uart_addr;
    struct gic_config_t gic_config;
    struct extend_datas_t {
        uint64_t extend_length;
        char extend_paras[0];
    } extend_datas;
};
static struct platform_info g_teeos_cfg;

static uintptr_t g_teeos_base_addr = 0;

/* 设置TEE OS的起始地址size的大小 */
int32_t set_teeos_mem(uintptr_t teeos_base_addr, uint64_t size)
{
    g_teeos_base_addr = teeos_base_addr;
    g_teeos_cfg.phys_region_size = size;
    if ((uint64_t)teeos_base_addr % ALIGN_SIZE_2M != 0)
        return -1;

    g_teeos_cfg.phys_region_start = (uint64_t)teeos_base_addr;

    return 0;
}

/* 设置TEE OS的串口地址和类型 */
void set_teeos_uart(uint64_t uart_addr)
{
    g_teeos_cfg.uart_addr = uart_addr;
}

/* 配置gic寄存器 */
void set_gic(struct gic_config_t gic_config)
{
    g_teeos_cfg.gic_config = gic_config;
}

bool copy_extend_datas(void *extend_datas, uint64_t extend_length)
{
    if (extend_datas == NULL)
        return false;

    if (sizeof(struct platform_info) + extend_length > MAX_CONFIG_LENGTH)
        return false;
 
    g_teeos_cfg.extend_datas.extend_length = extend_length;
    char *dst = (char *)(uintptr_t)(g_teeos_cfg.phys_region_start + sizeof(g_teeos_cfg));

    if (memcpy_s(dst, MAX_CONFIG_LENGTH - sizeof(g_teeos_cfg),
                 extend_datas, extend_length) != EOK)
        return false;
    return true;
}

/* 拷贝启动参数到目标位置，配置TEE OS属性后调用此接口 */
bool copy_teeos_cfg(void)
{
    if (g_teeos_cfg.phys_region_start == 0)
        return false;

    g_teeos_cfg.plat_cfg_size = sizeof(struct platform_info) + g_teeos_cfg.extend_datas.extend_length;
    char *dst = (void *)(uintptr_t)g_teeos_cfg.phys_region_start;

    if (memcpy_s(dst, sizeof(g_teeos_cfg),
                 (char *)&g_teeos_cfg, g_teeos_cfg.plat_cfg_size - sizeof(uint64_t)) != EOK)
        return false;

    return true;
}

uint64_t get_teeos_start(void)
{
    return g_teeos_cfg.phys_region_start;
}

uint64_t get_teeos_code_start(void)
{
    return g_teeos_cfg.phys_region_start + TEEOS_TEXT_OFFSET;
}

uint64_t get_teeos_size(void)
{
    return g_teeos_cfg.phys_region_size;
}
```

- 不使用 TEE Loader（如 ATF 闭源等原因），需要在 TEE OS 中配置平台相关参数，以 RK3568 平台为例（详见 `base/tee/tee_os_kernel/kernel/arch/aarch64/plat/rk3568/machine.c`）。

- 若需要使能新平台，需要在 `base/tee/tee_os_kernel/kernel/arch/aarch64/plat` 目录下添加新平台的适配代码，并且在 `base/tee/tee_os_kernel/config.mk` 中更新 CHCORE_PLAT 配置

  ```makefile
  CHCORE_PLAT=new_plat
  ```


## TEE ATF适配指导

### 概述

#### 功能简介

ATF提供了安全世界的参考实现软件\[ARMv8-A\],包括执行的\[Secure Monitor\] \[TEE-SMC\]异常级别 3\(EL3\)。它实现了各种 ARM 接口标准,如电源状态协调接口\(\[PSCI\]\),可信板启动要求\(TBBR,ARM DEN0006C-1\)和\[SMC 呼叫公约\] \[SMCCC\]。

#### 约束与限制

-   芯片架构为ARMv7/ARMv8架构。
-   CPU需支持安全内存和非安全内存的划分，安全和非安全外设的划分。

### 开发指导

#### 场景介绍

芯片使能TEE时，需要对ATF进行适配，以下将相关内容做介绍。

#### 接口说明

**表 4**  teed smc id管理列表

<a name="table7787117252"></a>

<table><thead align="left"><tr id="row4781911102519"><th class="cellrowborder" valign="top" width="25%" id="mcps1.2.5.1.1"><p id="p47816119253"><a name="p47816119253"></a><a name="p47816119253"></a>smc</p>
</th>
<th class="cellrowborder" valign="top" width="25%" id="mcps1.2.5.1.2"><p id="p87881112256"><a name="p87881112256"></a><a name="p87881112256"></a>smc id</p>
</th>
<th class="cellrowborder" valign="top" width="25%" id="mcps1.2.5.1.3"><p id="p57814117258"><a name="p57814117258"></a><a name="p57814117258"></a>处理的内容</p>
</th>
<th class="cellrowborder" valign="top" width="25%" id="mcps1.2.5.1.4"><p id="p13782011162511"><a name="p13782011162511"></a><a name="p13782011162511"></a>来自安全/非安全侧</p>
</th>
</tr>
</thead>
<tbody><tr id="row10781211152514"><td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.1 "><p id="p1978151112256"><a name="p1978151112256"></a><a name="p1978151112256"></a>TEE_HANDLED_S_EL1_INTR</p>
</td>
<td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.2 "><p id="p07861113251"><a name="p07861113251"></a><a name="p07861113251"></a>0xf2000006</p>
</td>
<td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.3 "><p id="p279101182513"><a name="p279101182513"></a><a name="p279101182513"></a>FIQ中断</p>
</td>
<td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.4 "><p id="p1679611172510"><a name="p1679611172510"></a><a name="p1679611172510"></a>安全侧</p>
</td>
</tr>
<tr id="row07901112519"><td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.1 "><p id="p1279181162517"><a name="p1279181162517"></a><a name="p1279181162517"></a>TEE_ENTRY_DONE</p>
</td>
<td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.2 "><p id="p279311122513"><a name="p279311122513"></a><a name="p279311122513"></a>0xf2000000</p>
</td>
<td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.3 "><p id="p1579111182510"><a name="p1579111182510"></a><a name="p1579111182510"></a>TEE完成smc命令返回</p>
</td>
<td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.4 "><p id="p10791411112516"><a name="p10791411112516"></a><a name="p10791411112516"></a>安全侧</p>
</td>
</tr>
<tr id="row167911118256"><td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.1 "><p id="p97971192516"><a name="p97971192516"></a><a name="p97971192516"></a>TEE_ON_DONE</p>
<p id="p8791911192516"><a name="p8791911192516"></a><a name="p8791911192516"></a>TEE_RESUME_DONE)</p>
<p id="p15791311172515"><a name="p15791311172515"></a><a name="p15791311172515"></a>TEE_SUSPEND_DONE</p>
</td>
<td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.2 "><p id="p18793113256"><a name="p18793113256"></a><a name="p18793113256"></a>0xf2000001</p>
<p id="p147914114255"><a name="p147914114255"></a><a name="p147914114255"></a>0xf2000004</p>
<p id="p14797113257"><a name="p14797113257"></a><a name="p14797113257"></a>0xf2000003</p>
</td>
<td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.3 "><p id="p1779111172511"><a name="p1779111172511"></a><a name="p1779111172511"></a>TEE启动完成，resume, suspend完成</p>
</td>
<td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.4 "><p id="p157914117254"><a name="p157914117254"></a><a name="p157914117254"></a>安全侧</p>
</td>
</tr>
<tr id="row4791011122513"><td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.1 "><p id="p187961192516"><a name="p187961192516"></a><a name="p187961192516"></a>TEE_STD_REQUEST</p>
</td>
<td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.2 "><p id="p679411142518"><a name="p679411142518"></a><a name="p679411142518"></a>0xb2000008</p>
</td>
<td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.3 "><p id="p187941111255"><a name="p187941111255"></a><a name="p187941111255"></a>REE(tzdriver)侧的标准命令请求</p>
</td>
<td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.4 "><p id="p67912115257"><a name="p67912115257"></a><a name="p67912115257"></a>非安全侧</p>
</td>
</tr>
<tr id="row14796115256"><td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.1 "><p id="p37911117254"><a name="p37911117254"></a><a name="p37911117254"></a>TEE_STD_REE_SIQ</p>
</td>
<td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.2 "><p id="p187921142511"><a name="p187921142511"></a><a name="p187921142511"></a>0xb200000a</p>
</td>
<td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.3 "><p id="p2791811142513"><a name="p2791811142513"></a><a name="p2791811142513"></a>SIQ线程处理请求</p>
</td>
<td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.4 "><p id="p779121113252"><a name="p779121113252"></a><a name="p779121113252"></a>非安全侧</p>
</td>
</tr>
<tr id="row279511162517"><td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.1 "><p id="p117961102511"><a name="p117961102511"></a><a name="p117961102511"></a>TEE_STD_RESPONSE</p>
</td>
<td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.2 "><p id="p127911172513"><a name="p127911172513"></a><a name="p127911172513"></a>0xb2000009</p>
</td>
<td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.3 "><p id="p979111132517"><a name="p979111132517"></a><a name="p979111132517"></a>安全侧完成smc命令返回</p>
</td>
<td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.4 "><p id="p87916115255"><a name="p87916115255"></a><a name="p87916115255"></a>安全侧</p>
</td>
</tr>
<tr id="row157919117258"><td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.1 "><p id="p187961116259"><a name="p187961116259"></a><a name="p187961116259"></a>TEE_STD_CRASH</p>
</td>
<td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.2 "><p id="p1779211102520"><a name="p1779211102520"></a><a name="p1779211102520"></a>0xb200000b</p>
</td>
<td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.3 "><p id="p1279111152516"><a name="p1279111152516"></a><a name="p1279111152516"></a>安全侧出现crash，告知ATF，此时ATF不会再处理任何非安全侧的请求</p>
</td>
<td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.4 "><p id="p38011115254"><a name="p38011115254"></a><a name="p38011115254"></a>安全侧</p>
</td>
</tr>
</tbody>
</table>

#### 开发实例<a name="section3748141455319"></a>

- 在芯片使能TEE ATF时，需要修改teed，适配示例代码位于 `base/tee/tee_os_framework/sample/teed`
- TEE OS 需要适配 ATF 中的 TEED，目前 TEE OS 支持 teed 和 opteed 两种，分别位于 `base/tee/tee_os_kernel/kernel/arch/aarch64/trustzone/spd` 下的 teed 和 opteed 两个目录中。
- 若需要使能新的 TEED，需要在该目录下添加相应 TEED 的适配代码，并且在 `base/tee/tee_os_kernel/tee_tee_os_kernel/config.mk` 中更新 CHCORE_SPD 配置

  ```makefile
  CHCORE_SPD=new_teed
  ```