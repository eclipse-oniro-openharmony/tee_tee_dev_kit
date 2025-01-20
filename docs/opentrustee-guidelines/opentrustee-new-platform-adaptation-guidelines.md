# OpenTrustee 新平台适配指南
本章节介绍如何在新设备上使能OpenTrustee，目前OpenTrustee只支持运行在ARM TrustZone机制的芯片上，在新设备上使能OpenTrustee包含TEE CLient、Tzdriver、TEE Loader、ATF等部件的适配。
## Tee Client适配

### Tee Client使能

在一个新单板使能TEE Client，需要修改单板配置文件，增加TEE Client部件。

以RK3568芯片为例，在vendor/hihope/rk3568/config.json中增加以下内容：

```c
{
  "subsystem": "tee",
  "components": [
    {
      "component": "tee_client",
      "features": []
    }
  ]
}
```

### Tee Client编译命令

Tee Client代码位置：`base/tee/tee_client`

本模块支持单独编译调试，以RK3568芯片为例，运行以下命令编译TEE Client部件：

```shell
./build.sh --product-name rk3568 --ccache --build-target tee_client
```

编译产物路径：out/rk3568/tee/tee_client

可将编译产物自行推入设备中进行调试：

```shell
hdc file send cadaemon.json /system/profile/
hdc file send cadaemon.cfg /system/etc/init/
hdc file send libteec.z.so /system/lib/
hdc file send libcadaemon.z.so /system/lib/
hdc file send tlogcat /system/bin/
```

## Tzdriver适配

Tzdriver是部署在REE侧的内核驱动，支持REE和TEE之间通信。Tzdriver处理来自于Tee Client的命令，并发送smc指令从REE切换到TEE。

Linux内核tzdriver代码位置：`base/tee/tee_tzdriver/linux`

### Tzdriver使能

1、修改linux内核代码仓中设备的defconfig文件，增加tzdriver的配置选项：

```
#
# TEEOS
#
CONFIG_TZDRIVER=y
CONFIG_CPU_AFF_NR=1
CONFIG_KERNEL_CLIENT=y
CONFIG_TEELOG=y
CONFIG_PAGES_MEM=y
CONFIG_THIRDPARTY_COMPATIBLE=y
```

各选项其含义如下表所示：

**表 1**  配置选项说明

<table><thead align="left"><tr id="row1860844162318"><th class="cellrowborder" valign="top" width="44.2%" id="mcps1.2.3.1.1"><p id="p106081345231"><a name="p106081345231"></a><a name="p106081345231"></a>参数</p>
</th>
<th class="cellrowborder" valign="top" width="55.800000000000004%" id="mcps1.2.3.1.2"><p id="p560812472313"><a name="p560812472313"></a><a name="p560812472313"></a>说明</p>
</th>
</tr>
</thead>
<tbody><tr id="row186081645232"><td class="cellrowborder" valign="top" width="44.2%" headers="mcps1.2.3.1.1 "><p id="p1160813419237"><a name="p1160813419237"></a><a name="p1160813419237"></a>CONFIG_TZDRIVER</p>
</td>
<td class="cellrowborder" valign="top" width="55.800000000000004%" headers="mcps1.2.3.1.2 "><p id="p7608447236"><a name="p7608447236"></a><a name="p7608447236"></a>模块开关，使能tzdriver必须打开</p>
</td>
</tr>
<tr id="row560920412318"><td class="cellrowborder" valign="top" width="44.2%" headers="mcps1.2.3.1.1 "><p id="p1460916482311"><a name="p1460916482311"></a><a name="p1460916482311"></a>CONFIG_CPU_AFF_NR</p>
</td>
<td class="cellrowborder" valign="top" width="55.800000000000004%" headers="mcps1.2.3.1.2 "><p id="p1960913482311"><a name="p1960913482311"></a><a name="p1960913482311"></a>CA绑核功能，非零值代表限制仅cpuid小于CONFIG_CPU_AFF_NR的CPU可以进入TEE，0代表无限制，当前只支持在0核运行，所以值为1</p>
</td>
</tr>
<tr id="row10656158102412"><td class="cellrowborder" valign="top" width="44.2%" headers="mcps1.2.3.1.1 "><p id="p4656168192410"><a name="p4656168192410"></a><a name="p4656168192410"></a>CONFIG_KERNEL_CLIENT</p>
</td>
<td class="cellrowborder" valign="top" width="55.800000000000004%" headers="mcps1.2.3.1.2 "><p id="p1565616817245"><a name="p1565616817245"></a><a name="p1565616817245"></a>内核CA支持，默认建议开启</p>
</td>
</tr>
<tr id="row1051612110244"><td class="cellrowborder" valign="top" width="44.2%" headers="mcps1.2.3.1.1 "><p id="p351620118241"><a name="p351620118241"></a><a name="p351620118241"></a>CONFIG_TEELOG</p>
</td>
<td class="cellrowborder" valign="top" width="55.800000000000004%" headers="mcps1.2.3.1.2 "><p id="p15516161182413"><a name="p15516161182413"></a><a name="p15516161182413"></a>TEE日志开关，默认建议开启</p>
</td>
</tr>
<tr id="row9724616192417"><td class="cellrowborder" valign="top" width="44.2%" headers="mcps1.2.3.1.1 "><p id="p3725516102418"><a name="p3725516102418"></a><a name="p3725516102418"></a>CONFIG_PAGES_MEM</p>
</td>
<td class="cellrowborder" valign="top" width="55.800000000000004%" headers="mcps1.2.3.1.2 "><p id="p382054615243"><a name="p382054615243"></a><a name="p382054615243"></a>tlogger使用的内存类型，默认需要开启</p>
</td>
</tr>
<tr id="row9724616192417"><td class="cellrowborder" valign="top" width="44.2%" headers="mcps1.2.3.1.1 "><p id="p3725516102418"><a name="p3725516102418"></a><a name="p3725516102418"></a>CONFIG_THIRDPARTY_COMPATIBLE</p>
</td>
<td class="cellrowborder" valign="top" width="55.800000000000004%" headers="mcps1.2.3.1.2 "><p id="p382054615243"><a name="p382054615243"></a><a name="p382054615243"></a>兼容第三方opteed的适配，例如适配RK3568芯片需要开启此选项</p>
</td> 
</tr>
</tbody>
</table>

2、其他内核相关修改在kernel/linux/patches仓，以补丁方式提供。

RK3568芯片的patch在kernel/linux/patches/linux-5.10/rk3568_patch/kernel.patch，其他芯片平台也可以参考这个patch，每个芯片应该创建自己的patch文件。此patch补丁中应当包含以下内容：

- 对于内核的根Makefile的修改（在其中引用tzdriver仓的子Makefile，其中tzdriver path需要修改为实际的相对路径，注意Linux kernel的编译是会将kernel仓代码拷贝到out目录打patch，因此这个相对路径是相对于out下的临时kernel仓的路径）。

  ```
  obj-y += {tzdriver path}
  ```

- 对于内核的根Kconfig的需改（在其中引用tzdriver仓的子Kconfig，其中tzdriver path需要修改为实际的相对路径，同上需要注意这个相对路径应当是在out目录下的临时kernel仓路径）。

  ```
  source "{tzdriver path}/Kconfig"
  ```

- dtsi的修改：需要在相应芯片的disi文件中包含trusted\_core节点，对于RK3568芯片，需要修改patch中的/arch/arm64/boot/dts/rockchip/rk3568-toybrick-x0.dtsi文件，新增以下内容

  ```
  /{
      trusted_core {
          compatible = "trusted_core";
          interrupts = <0 73 4>;
      };
  };
  ```

  其中，Linux内核中tzdriver支持中断号的动态配置，上面的73为spi中断号

>![](public_sys-resources/icon-caution.gif) **注意：** 
>注意dtsi里面的spi中断号应该比实际的中断号小32，且需要保证不与其他组件的中断号冲突。

### Tzdriver编译命令

Tzdriver部件跟随kernel一起编译，以rk3568为例，可以单独编译boot_linux.img，编译命令如下
```Bash
./build.sh --product-name rk3568 --ccache --build-target kernel --gn-args linux_kernel_version=\"linux-5.10\"
```

## TEE Loader适配
TEE Loader主要负责加载安全镜像并将启动参数传递给TEEOS。启动参数包含TEEOS用到的安全内存地址和大小，串口的地址，gic寄存器配置，以及其它拓展参数。

**表 2**  启动参数列表

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
<td class="cellrowborder" valign="top" width="28.64%" headers="mcps1.2.4.1.2 "><p id="p294mcpsimp"><a name="p294mcpsimp"></a><a name="p294mcpsimp"></a>TEEOS内存大小</p>
</td>
<td class="cellrowborder" valign="top" width="51.05%" headers="mcps1.2.4.1.3 "><p id="p320093615420"><a name="p320093615420"></a><a name="p320093615420"></a>2MB的整数倍。</p>
</td>
</tr>
<tr id="row01861049151317"><td class="cellrowborder" valign="top" width="20.31%" headers="mcps1.2.4.1.1 "><p id="p297mcpsimp"><a name="p297mcpsimp"></a><a name="p297mcpsimp"></a>phys_region_start</p>
</td>
<td class="cellrowborder" valign="top" width="28.64%" headers="mcps1.2.4.1.2 "><p id="p299mcpsimp"><a name="p299mcpsimp"></a><a name="p299mcpsimp"></a>TEEOS内存起始物理地址</p>
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
<td class="cellrowborder" valign="top" width="51.05%" headers="mcps1.2.4.1.3 "><p id="p18872120461"><a name="p18872120461"></a><a name="p18872120461"></a>格式需要和TEEOS核对。</p>
</td>
</tr>
</tbody>
</table>

#### 开发实例

- 下面展示了在芯片使能TEE时，使用 TEE Loader配置启动参数的开发实例。详细示例代码位于 `base/tee/tee_os_framework/sample/teeloader` 目录。

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

/* 设置TEEOS的起始地址size的大小 */
int32_t set_teeos_mem(uintptr_t teeos_base_addr, uint64_t size)
{
    g_teeos_base_addr = teeos_base_addr;
    g_teeos_cfg.phys_region_size = size;
    if ((uint64_t)teeos_base_addr % ALIGN_SIZE_2M != 0)
        return -1;

    g_teeos_cfg.phys_region_start = (uint64_t)teeos_base_addr;

    return 0;
}

/* 设置TEEOS的串口地址和类型 */
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

/* 拷贝启动参数到目标位置，配置TEEOS属性后调用此接口 */
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

- 第二步，需要在 `base/tee/tee_os_kernel/kernel/arch/aarch64/plat` 目录下添加新平台的适配代码，可以参考`plat/rk3399`里的适配。同时，需要在 `base/tee/tee_os_kernel/config.mk` 中更新 CHCORE_PLAT 配置

  ```makefile
  CHCORE_PLAT=new_plat
  ```
>![](public_sys-resources/icon-caution.gif) **注意：** 
启动参数的适配方法可以按照产品的习惯要求采用不同的方法，例如结构体中直接填写相应的参数，或者增加配置文件的方法。启动参数也可以直接配置在TEEOS 中，以 RK3568 平台为例（详见 `base/tee/tee_os_kernel/kernel/arch/aarch64/plat/rk3568/machine.c`）。

## ATF适配

- ARM从v6架构开始就引入了TrustZone技术，将AMR核的工作状态分为安全态和非安全态两种，在芯片级别对硬件资源提供保护和隔离。在实现了BL32（即安全OS）的平台，需要在ATF中添加SPD模块来实现安全世界和非安全世界的切换，我们提供了teed模块示例代码，位于 `base/tee/tee_os_framework/sample/teed`。按照ATF的编译框架，在ATF根目录下的Makefile里面添加如下选项可以使能teed。
```makefile
    SPD := teed
```
- 同时，TEEOS 需要适配 ATF 中的 teed，适配代码位于`base/tee/tee_os_kernel/kernel/arch/aarch64/trustzone/spd/teed`目录中，此代码开发者适配新平台时无需修改。
- 另外，teed中支持的SMC交互指令如下。

**表 3**  teed smc id管理列表

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

#### 兼容opteed
有些单板的ATF中已经集成了opteed，对于这种情况，OpenTrustee提供了兼容opteed的方案，具体适配代码位于 `base/tee/tee_os_kernel/kernel/arch/aarch64/trustzone/spd/opteed` 目录中。
另外，若需要使能新的 TEED，可以在`spd`目录下添加相应 TEED 的适配代码，并且在 `base/tee/tee_os_kernel/config.mk` 中更新 CHCORE_SPD 配置.

  ```makefile
  CHCORE_SPD=new_teed
  ```

## TEEOS镜像构建指导

以RK3568芯片为例，TEEOS的二进制文件（bl32.bin）被打包在uboot.img中，以下是构建TEEOS镜像的指导。

### 编译TEEOS

TEEOS内核代码位置：`base/tee/tee_os_kernel`

TEEOS框架代码位置：`base/tee/tee_os_framework`

切换目录至OpenHarmony源码根目录, 输入以下指令编译TEEOS镜像

```Bash
./build.sh --product-name rk3568 --build-target tee --ccache
```
构建产物为TEEOS镜像，路径如下：`base/tee/tee_os_kernel/kernel/bl32.bin`

### 编译uboot.img
根据以下步骤编译uboot.img
-  克隆`https://gitee.com/hihope-rockchip/device_rockchip_third_party_rkbin`，其中包含bl31.elf
-  克隆`https://gitee.com/hihope-rockchip/device_rockchip_third_party_uboot`，其中包含u-boot
-  将rkbin和u-boot放在同一目录下, 修改rkbin/RKTRUST/RK3568TRUST.ini中BL32_OPTION下的PATH指向bl32.bin
-    其中，u-boot/make.sh中有以下内容，搜索RK3568TRUST.ini中包含_bl32_的文件名，如果BL32的文件名是bl32.bin的话，需要修改以下匹配规则
```Bash
BL32_BIN=`sed -n '/_bl32_/s/PATH=//p' ${INI} | tr -d '\r'`
```
-  修改u-boot/configs/rk3568_defconfig，关闭OPTEE驱动，增大镜像大小到6M。
```Bash
- CONFIG_OPTEE_CLIENT=y
+ CONFIG_SPL_FIT_IMAGE_KB=6144
+ CONFIG_SPL_FIT_IMAGE_MULTIPLE=1
```
>![](public_sys-resources/icon-caution.gif) **注意：** 
>烧录的时候需要修改分区表parameter.txt，和uboot.img的镜像大小一致
-  修改make.sh中编译工具链路径，使其指向正确的路径（可以使用openharmony工程prebuilts目录下的工具链）
```Bash
-CROSS_COMPILE_ARM32=../prebuilts/gcc/linux-x86/arm/gcc-linaro-6.3.1-2017.05-x86_64_arm-linux-gnueabihf/bin/arm-linux-gnueabihf-
-CROSS_COMPILE_ARM64=../prebuilts/gcc/linux-x86/aarch64/gcc-linaro-6.3.1-2017.05-x86_64_aarch64-linux-gnu/bin/aarch64-linux-gnu-
```
-  在u-boot目录下执行./make.sh rk3568，最终会在u-boot目录中生成uboot.img
