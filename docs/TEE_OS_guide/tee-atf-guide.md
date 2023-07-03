# TEE ATF适配指导<a name="ZH-CN_TOPIC_0000001259525365"></a>

## 概述<a name="section28401073463"></a>

### 功能简介<a name="section8714171624613"></a>

ATF提供了安全世界的参考实现软件\[ARMv8-A\],包括执行的\[Secure Monitor\] \[TEE-SMC\]异常级别 3\(EL3\)。它实现了各种 ARM 接口标准,如电源状态协调接口\(\[PSCI\]\),可信板启动要求\(TBBR,ARM DEN0006C-1\)和\[SMC 呼叫公约\] \[SMCCC\]。

### 约束与限制<a name="section1856813710473"></a>

-   芯片架构为ARMv7/ARMv8架构。
-   CPU需支持安全内存和非安全内存的划分，安全和非安全外设的划分。

## 开发指导<a name="section1890104185118"></a>

### 场景介绍<a name="section114981014165118"></a>

芯片使能TEE时，需要对ATF进行适配，以下将相关内容做介绍。

### 接口说明<a name="section125843344514"></a>

以下接口说明列表将介绍我们在实际使能TEE过程中对ATF适配将用到的接口。

**表 1**  ATF适配调用接口表

<a name="table1438413213012"></a>
<table><thead align="left"><tr id="row4385321203"><th class="cellrowborder" valign="top" width="50%" id="mcps1.2.3.1.1"><p id="p16707841711"><a name="p16707841711"></a><a name="p16707841711"></a>接口名</p>
</th>
<th class="cellrowborder" valign="top" width="50%" id="mcps1.2.3.1.2"><p id="p47071841714"><a name="p47071841714"></a><a name="p47071841714"></a>描述</p>
</th>
</tr>
</thead>
<tbody><tr id="row18385142118019"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p938320351251"><a name="p938320351251"></a><a name="p938320351251"></a>int32_t is_system_suspend(void)</p>
</td>
<td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p4397135112520"><a name="p4397135112520"></a><a name="p4397135112520"></a>判断系统是否进入休眠（根据teeos内部驱动是否需要休眠调用）。</p>
</td>
</tr>
</tbody>
</table>

### 开发步骤<a name="section26814557584"></a>

1.  ATF编译使能

    ARM从v6架构开始就引入了TrustZone技术，将AMR核的工作状态分为安全态和非安全态两种，在芯片级别对硬件资源提供保护和隔离。在实现了BL32（即安全OS）的平台，需要在ATF中添加SPD模块来实现安全世界和非安全世界的切换，对于teeos，我们提供了teed模块。按照ATF的编译框架，在ATF根目录下的Makefile里面添加如下选项可以使能teed：

    ```
    SPD := teed
    ```

2.  teed代码适配

    1.  系统休眠判定修改

        需要在teed\_pm.c文件里实现is\_system\_suspend函数，以便在向ATF中psci流程注册的钩子函数中判断系统是否进入休眠，依此决定cpu是否需要进tee处理安全OS中的休眠流程。

    2.  其他场景修改

        对于有些不是ATF直接拉起teeos的场景，由EL2或者EL1发SMC到EL3直接调用teed\_init函数拉起teeos时，需要在iteed\_init函数里设置scr\_el3屏蔽FIQ中断，可以在teed.mk里面添加BOOT\_BL32\_FROM\_OTHER\_EXCEPTION宏。

    **表 2**  teed smc id管理列表

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
    <td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.3 "><p id="p2791811142513"><a name="p2791811142513"></a><a name="p2791811142513"></a>SIO线程处理请求</p>
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


相关配置接口见[接口说明](#section125843344514)。

### 开发实例<a name="section3748141455319"></a>

在芯片使能TEE ATF时，is\_system\_suspend函数的开发实例。

```
int32_t is_system_suspend(void)
{
    /* idle状态返回0, suspend状态返回1*/
    return 1;
}
```

