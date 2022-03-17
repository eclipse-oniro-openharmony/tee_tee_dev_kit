# TEE Tzdriver适配指导<a name="ZH-CN_TOPIC_0000001257014487"></a>

-   [概述](#section896014101054)
-   [适配指导以及适配实例](#section3853521853)
    -   [tzdriver入口](#section24427168613)
    -   [工程编译适配](#section17202641776)

-   [驱动初始化](#section837564171719)
-   [spi中断号配置](#section495111567195)
-   [配置选项](#section3247165292119)
    -   [内核配置选项](#section19684124015119)
    -   [TEE日志配置](#section95881957215)

-   [常见问题](#section4549193932515)
    -   [ioctl失败，未执行到tzdriver](#section105004622512)
    -   [ioctl时tzdriver报错invalid cmd](#section137051444142912)


## 概述<a name="section896014101054"></a>

tzdriver是TEE的内核驱动，主要功能是在整个TEE子系统中起连接作用，是使用TEE OS服务的桥梁，tzdriver处理来自于tee\_client的ioctl命令，并通过smc指令从REE切换到TEE。

>![](public_sys-resources/icon-caution.gif) **注意：** 
>单独适配tzdriver，系统并不能正常启动，必须同时适配TEE OS。

## 适配指导以及适配实例<a name="section3853521853"></a>

本章节中会讲述如何针对一款芯片适配tzdriver，此章节中以3516DV300芯片为例。

第一小节介绍tzdriver入口和tzdriver代码位置，本章下面几个小节，每小节都是一个适配步骤。（这几个适配步骤顺序无关，但推荐将配置选项放到最后）

### tzdriver入口<a name="section24427168613"></a>

-   LiteOs内核tzdriver代码位置：base/tee/tee\_tee\_tzdriver/liteos。

-   Linux内核tzdriver代码位置：base/tee/tee\_tee\_tzdriver/linux。

tzdriver是内核中的一个字符设备驱动。

tzdriver初始化时会创建一个字符设备文件，一般为/dev/tc\_ns\_client，用户态进程可以打开此节点，以及通过ioctl接口调用tzdriver相关功能。

tzdriver总入口在core/tc\_client\_driver.c中的tc\_client\_ioctl函数。

设备节点函数接口：

```
static const struct file_operations_vfs g_tc_ns_client_fops = {
    .open = tc_client_open,
    .close = tc_client_close,
    .ioctl = tc_client_ioctl,
    .mmap = tc_client_mmap,
};
```

### 工程编译适配<a name="section17202641776"></a>

tzdriver需要被编译到内核中作为内核驱动，两种内核下tzdriver的编译使能方法也有差异。

-   LiteOs内核tzdriver编译适配

    hb工具的--tee选项（完整命令为hb build --tee）即可编译tee所有组件，包含tzdriver。

    tzdriver的编译涉及到kernel/liteos\_a仓，tzdriver是LiteOs kernel仓的组件，同时tzdriver需要依赖libmbedtls组件（或其他等效的sha256实现）。

    涉及以下编译配置文件（如果以下的修改已经存在，且不存在编译问题，则无需适配）：

    1.  drivers/Kconfig

        ```
        source "../../base/tee/tee_tee_tzdriver/liteos/Kconfig"
        ```

    2.  lib/libmbedtls/BUILD.gn

        >![](public_sys-resources/icon-note.gif) **说明：** 
        >--tee选项会自动打开这个库的编译，如果不遇到编译错误，无需修改此文件，如若遇到某个源文件无法成功编译，可以将其去掉。

    3.  tools/build/mk/los\_config.mk

        --tee选项会打开此编译。

        ```
        ifeq ($(LOSCFG_DRIVERS_TZDRIVER), y)
        LITEOS_BASELIB   += -ltzdriver -lmbedtls
        LIB_SUBDIRS         += $(LITEOS_DRIVERS_BASE_PATH)/../../base/tee/tee_tee_tzdriver/liteos/ $(LITEOSTOPDIR)/lib/libmbedtls
        LITEOS_TZDRIVER_INCLUDE += -I $(LITEOS_DRIVERS_BASE_PATH)/../../base/tee/tee_tee_tzdriver/liteos/include/
        endif
        ```

    4.  BUILD.gn

        tzdriver的BUILD.gn是被driver/liteos/BUILD.gn所引用。

        ```
        import("//kernel/liteos_a/liteos.gni")
        group("liteos") {
        deps = [
        "hievent",
        "../../base/tee/tee_tee_tzdriver/liteos",
        ]
        }
        config("public") {
        configs = [
        "hievent:public",
        "../../base/tee/tee_tee_tzdriver/liteos:public",
        ]
        }
        ```



-   Linux内核tzdriver编译适配

    在Linux内核可以通过defconfig文件中的CONFIG\_TZDRIVER选项控制tzdriver的编译使能。

    1.  defconfig文件修改

        defconfig文件在kernel/linux/config仓，每个芯片应当创建自己的defconfig文件，后面会介绍tzdriver中的所有defconfig配置项。

    2.  kernel补丁

        其他内核相关修改在kernel/linux/patches仓，以补丁方式提供。

        hi3516dv300平台的patch在kernel/linux/patches/linux-5.10/hi3516dv300\_patch/hi3516dv300.patch，其他芯片平台也可以参考这个patch，每个芯片应该创建自己的patch文件。此patch补丁中应当包含以下内容：

        -   对于内核的根Makefile的修改（在其中引用tzdriver仓的子Makefile，其中tzdriver path需要修改为实际的相对路径，注意Linux kernel的编译是会将kernel仓代码拷贝到out目录打patch，因此这个相对路径是相对于out下的临时kernel仓的路径）。

            ```
            obj-y += {tzdriver path}
            ```

        -   对于内核的根Kconfig的需改（在其中引用tzdriver仓的子Kconfig，其中tzdriver path需要修改为实际的相对路径，同上需要注意这个相对路径应当是在out目录下的临时kernel仓路径）。

            ```
            source "{tzdriver path}/Kconfig"
            ```

        -   dtsi的修改：新增芯片的trusted\_core.dtsi（后文中会给出dtsi文件内容，建议命名为"芯片平台\_trusted\_core.dtsi"），并在产品的总dts文件中引用trusted\_core.dtsi。

            ```
            #include "hi3516dv300_trusted_core.dtsi"
            ```




## 驱动初始化<a name="section837564171719"></a>

tzdriver作为内核驱动，其初始化方式在两种内核中并不一致。

-   Linux内核中tzdriver驱动初始化方式

    自动初始化，无需适配修改。


-   LiteOs内核中tzdriver驱动初始化方式
    1.  修改device/hisilicon/hispark\_taurus/sdk\_liteos仓：

        mpp/module\_init/BUILD.gn中需要包含头文件路径："//base/tee/tee\_tee\_tzdriver/liteos/include"

    2.  在mpp/module\_init/src/system\_init.c中SystemInit函数需要手动调用tc\_init函数（参考下面两个代码片段）。

        示例代码1：

        ```
        void SystemInit_tzdriverInit(void)
        {
        #ifdef LOSCFG_DRIVERS_TZDRIVER
            tc_init();
            dprintf("tzdriver init end\n");
        #endif
        }
        ```

        示例代码2：

        ```
        SystemInit_NetInit();  /* need to check later */
        SystemInit_MountRootfs();
        SystemInit_ConsoleInit();
        SystemInit_tzdriverInit(); 
        ```



## spi中断号配置<a name="section495111567195"></a>

LiteOs内核和Linux内核中tzdriver均支持中断号的动态配置。

-   Linux内核tzdriver的spi中断号配置。在dtsi文件中包含trusted\_core节点。（下面73就是中断号）

    ```
    /{
        trusted_core {
            compatible = "trusted_core";
            interrupts = <0 73 4>;
        };
    };
    ```

    >![](public_sys-resources/icon-caution.gif) **注意：** 
    >注意dtsi里面的spi中断号应该比实际的中断号小32，且需要保证不与其他组件的中断号冲突。


-   Liteos内核tzdriver的spi中断号配置。直接定义在代码中（下面NUM\_HAL\_INTERRUPT\_TEE\_SPI\_NOTIFY就是中断号）。

    位置在core/tz\_spi\_notify.c

    ```
      unsigned int irq;
      int ret;
    
      irq = NUM_HAL_INTERRUPT_TEE_SPI_NOTIFY;
      ret = devm_request_irq(irq, tc_secure_notify,
        IRQF_NO_SUSPEND, TC_NS_CLIENT_DEV, NULL);
      if (ret < 0) {
        tloge("device irq %u request failed %u", irq, ret);
        return ret;
      }
    ```


## 配置选项<a name="section3247165292119"></a>

### 内核配置选项<a name="section19684124015119"></a>

tzdriver有一些特性或者选项，可以选择配置，控制这些选项的地方如下：

-   Linux内核tzdriver配置选项

    tzdriver选项应该写在kernel/linux/config仓，修改芯片的defconfig文件：

    ```
    #
    # TEE OS
    #
    CONFIG_TZDRIVER=y
    # CONFIG_ASAN_DEBUG is not set
    # CONFIG_CLIENT_AUTH is not set
    CONFIG_CPU_AFF_NR=0
    CONFIG_TA_AFFINITY=y
    CONFIG_TA_AFFINITY_CPU_NUMS=8
    CONFIG_KERNEL_CLIENT=y
    CONFIG_TEELOG=y
    CONFIG_TEE_LOG_ACHIVE_PATH="/data/log/tee/last_teemsg"
    CONFIG_PAGES_MEM=y
    
    ```

    各选项其含义如下表所示：

    **表 1**  配置选项说明

    <a name="table17608341239"></a>
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
    <tr id="row660818472311"><td class="cellrowborder" valign="top" width="44.2%" headers="mcps1.2.3.1.1 "><p id="p96091842239"><a name="p96091842239"></a><a name="p96091842239"></a>CONFIG_ASAN_DEBUG</p>
    </td>
    <td class="cellrowborder" valign="top" width="55.800000000000004%" headers="mcps1.2.3.1.2 "><p id="p389364412239"><a name="p389364412239"></a><a name="p389364412239"></a>地址消毒，内核调试时打开</p>
    </td>
    </tr>
    <tr id="row360912414232"><td class="cellrowborder" valign="top" width="44.2%" headers="mcps1.2.3.1.1 "><p id="p2609641237"><a name="p2609641237"></a><a name="p2609641237"></a>CONFIG_CLIENT_AUTH</p>
    </td>
    <td class="cellrowborder" valign="top" width="55.800000000000004%" headers="mcps1.2.3.1.2 "><p id="p153911484233"><a name="p153911484233"></a><a name="p153911484233"></a>客户端鉴权</p>
    </td>
    </tr>
    <tr id="row560920412318"><td class="cellrowborder" valign="top" width="44.2%" headers="mcps1.2.3.1.1 "><p id="p1460916482311"><a name="p1460916482311"></a><a name="p1460916482311"></a>CONFIG_CPU_AFF_NR</p>
    </td>
    <td class="cellrowborder" valign="top" width="55.800000000000004%" headers="mcps1.2.3.1.2 "><p id="p1960913482311"><a name="p1960913482311"></a><a name="p1960913482311"></a>CA绑核功能，仅cpuid小于CONFIG_CPU_AFF_NR的CPU可以进入TEE</p>
    </td>
    </tr>
    <tr id="row160924192318"><td class="cellrowborder" valign="top" width="44.2%" headers="mcps1.2.3.1.1 "><p id="p186092482311"><a name="p186092482311"></a><a name="p186092482311"></a>CONFIG_TA_AFFINITY</p>
    </td>
    <td class="cellrowborder" valign="top" width="55.800000000000004%" headers="mcps1.2.3.1.2 "><p id="p11609646238"><a name="p11609646238"></a><a name="p11609646238"></a>TA绑核支持</p>
    </td>
    </tr>
    <tr id="row773402192213"><td class="cellrowborder" valign="top" width="44.2%" headers="mcps1.2.3.1.1 "><p id="p1173413216223"><a name="p1173413216223"></a><a name="p1173413216223"></a>CONFIG_TA_AFFINITY_CPU_NUMS</p>
    </td>
    <td class="cellrowborder" valign="top" width="55.800000000000004%" headers="mcps1.2.3.1.2 "><p id="p16734192142219"><a name="p16734192142219"></a><a name="p16734192142219"></a>TA绑核支持的最大核数</p>
    </td>
    </tr>
    <tr id="row10656158102412"><td class="cellrowborder" valign="top" width="44.2%" headers="mcps1.2.3.1.1 "><p id="p4656168192410"><a name="p4656168192410"></a><a name="p4656168192410"></a>CONFIG_KERNEL_CLIENT</p>
    </td>
    <td class="cellrowborder" valign="top" width="55.800000000000004%" headers="mcps1.2.3.1.2 "><p id="p1565616817245"><a name="p1565616817245"></a><a name="p1565616817245"></a>内核CA支持</p>
    </td>
    </tr>
    <tr id="row1051612110244"><td class="cellrowborder" valign="top" width="44.2%" headers="mcps1.2.3.1.1 "><p id="p351620118241"><a name="p351620118241"></a><a name="p351620118241"></a>CONFIG_TEELOG</p>
    </td>
    <td class="cellrowborder" valign="top" width="55.800000000000004%" headers="mcps1.2.3.1.2 "><p id="p15516161182413"><a name="p15516161182413"></a><a name="p15516161182413"></a>TEE日志开关，详细内容请参考TEE日志配置</p>
    </td>
    </tr>
    <tr id="row19461113152415"><td class="cellrowborder" valign="top" width="44.2%" headers="mcps1.2.3.1.1 "><p id="p1346141372415"><a name="p1346141372415"></a><a name="p1346141372415"></a>CONFIG_TEE_LOG_ACHIVE_PATH</p>
    </td>
    <td class="cellrowborder" valign="top" width="55.800000000000004%" headers="mcps1.2.3.1.2 "><p id="p1846171319246"><a name="p1846171319246"></a><a name="p1846171319246"></a>日志路径，详细内容请参考TEE日志配置</p>
    </td>
    </tr>
    <tr id="row9724616192417"><td class="cellrowborder" valign="top" width="44.2%" headers="mcps1.2.3.1.1 "><p id="p3725516102418"><a name="p3725516102418"></a><a name="p3725516102418"></a>CONFIG_PAGES_MEM</p>
    </td>
    <td class="cellrowborder" valign="top" width="55.800000000000004%" headers="mcps1.2.3.1.2 "><p id="p382054615243"><a name="p382054615243"></a><a name="p382054615243"></a>tlogger使用的内存类型，详细内容请参考TEE日志配置</p>
    </td>
    </tr>
    </tbody>
    </table>


-   LiteOs内核tzdriver配置选项

    tzdriver选项可以直接定义在BUILD.gn或者Makefile中：

    ```
    LOCAL_FLAGS += -fstack-protector-strong -Wno-unused-const-variable -Wno-unused-function
    LOCAL_FLAGS += -DCONFIG_TZDRIVER=y
    LOCAL_FLAGS += -DCONFIG_LITEOS_TZDRIVER=y
    LOCAL_FLAGS += -DCONFIG_CPU_AFF_NR=1
    LOCAL_FLAGS += -DCONFIG_KERNEL_CLIENT=y
    LOCAL_FLAGS += -DCONFIG_PAGES_MEM=y
    LOCAL_FLAGS += -DCONFIG_TEELOG=y
    LOCAL_FLAGS += -DCONFIG_TEECD_PATH=\"/vendor/bin/teecd\"
    ```


### TEE日志配置<a name="section95881957215"></a>

日志功能是基础的DFX能力，不推荐关闭此能力。开启TEE日志能力需要将上述配置选项CONFIG\_TEELOG在对应配置文件打开。

1.  适配物理内存配置

    >![](public_sys-resources/icon-note.gif) **说明：** 
    >建议将TEE日志使用的内存替换成物理内存，基于此物理内存定制一些更强的维测能力，参考log\_cfg\_api.h文件的接口范围定制成log\_xxx\_cfg.c文件，并定义CONFIG\_XXX\_MEM宏来区分XXX内存类型，并将此文件编译到tzdriver里，同时关闭CONFIG\_PAGES\_MEM宏定义。

    log\_xxx\_cfg.c文件需要适配的详细介绍如下：

    **表 2**  接口适配说明

    <a name="table314763748"></a>
    <table><thead align="left"><tr id="row13147432415"><th class="cellrowborder" valign="top" width="50%" id="mcps1.2.3.1.1"><p id="p145313240"><a name="p145313240"></a><a name="p145313240"></a>接口</p>
    </th>
    <th class="cellrowborder" valign="top" width="50%" id="mcps1.2.3.1.2"><p id="p1414817313416"><a name="p1414817313416"></a><a name="p1414817313416"></a>配置说明</p>
    </th>
    </tr>
    </thead>
    <tbody><tr id="row141481731741"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p71487320417"><a name="p71487320417"></a><a name="p71487320417"></a>register_log_mem</p>
    </td>
    <td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p4148103545"><a name="p4148103545"></a><a name="p4148103545"></a>当前开源是内核虚拟日志内存注册，需切换成物理内存申请以及注册到TEEOS</p>
    </td>
    </tr>
    <tr id="row3148931417"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p181482036420"><a name="p181482036420"></a><a name="p181482036420"></a>register_log_exception</p>
    </td>
    <td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p11148237416"><a name="p11148237416"></a><a name="p11148237416"></a>Tlogger模块启动时，注册TEEOS异常时上报的异常属性配置，包括异常的模块为TEEOS、异常码、以及是否要触发整机重启等等之类。</p>
    </td>
    </tr>
    <tr id="row1414813944"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p9148433414"><a name="p9148433414"></a><a name="p9148433414"></a>report_log_system_error</p>
    </td>
    <td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p10148931243"><a name="p10148931243"></a><a name="p10148931243"></a>上报异常能力，上报register_log_exception接口配置的异常信息以及触发对应的异常响应。</p>
    </td>
    </tr>
    <tr id="row16148163744"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p5148203844"><a name="p5148203844"></a><a name="p5148203844"></a>report_log_system_panic</p>
    </td>
    <td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p8148637412"><a name="p8148637412"></a><a name="p8148637412"></a>TEEOS crash场景触发异常，如日志内存不支持log_exception功能时，可以通过此接口触发整机重启来恢复系统；否则此接口不需适配。</p>
    </td>
    </tr>
    <tr id="row714812319410"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p2014853549"><a name="p2014853549"></a><a name="p2014853549"></a>map_log_mem</p>
    </td>
    <td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p141483315417"><a name="p141483315417"></a><a name="p141483315417"></a>TEEOS 日志内存映射 ，需要适配成物理内存映射接口。</p>
    </td>
    </tr>
    <tr id="row11680111317520"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p268021320516"><a name="p268021320516"></a><a name="p268021320516"></a>unmap_log_mem</p>
    </td>
    <td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p176801113855"><a name="p176801113855"></a><a name="p176801113855"></a>TEEOS 日志内存映射 ，需要适配成物理内存去映射接口。</p>
    </td>
    </tr>
    <tr id="row64289177516"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p2428817157"><a name="p2428817157"></a><a name="p2428817157"></a>get_log_chown</p>
    </td>
    <td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p842811171658"><a name="p842811171658"></a><a name="p842811171658"></a>TEEOS临终遗言日志文件的chown属性，liteos上此文件不写入，此接口可以不适配；liunx版本此功能开启，适配成system群组，可适情况修改。</p>
    </td>
    </tr>
    <tr id="row8881940154"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p19884401553"><a name="p19884401553"></a><a name="p19884401553"></a>unregister_log_exception</p>
    </td>
    <td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p1288154017510"><a name="p1288154017510"></a><a name="p1288154017510"></a>Tlogger模块销毁时，适配注册取消异常配置能力，同register_log_exception。</p>
    </td>
    </tr>
    <tr id="row1085212421454"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p14852134214517"><a name="p14852134214517"></a><a name="p14852134214517"></a>ta_crash_report_log</p>
    </td>
    <td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p138521142754"><a name="p138521142754"></a><a name="p138521142754"></a>此接口为空即可。</p>
    </td>
    </tr>
    </tbody>
    </table>

2.  临终遗言文件配置

    Linux版本可通过CONFIG\_TEE\_LOG\_ACHIVE\_PATH宏来定制TEEOS或TA crash时，日志存储的文件为CONFIG\_TEE\_LOG\_ACHIVE\_PATH的定义位置。

    >![](public_sys-resources/icon-caution.gif) **注意：** 
    >LiteOs版本不支持临终遗言文件写入，忽略此功能。


## 常见问题<a name="section4549193932515"></a>

### ioctl失败，未执行到tzdriver<a name="section105004622512"></a>

**问题现象**1

ioctl失败，未执行到tzdriver，且errno为EPERM。

**可能原因**

可能是权限问题，可以查看/dev/tc\_ns\_client的权限，teecd的权限，执行CA的用户权限，检查是否一致。

**解决办法**

排查teecd、CA和tzdriver节点的权限，确保可以访问节点（确保拥有匹配的用户组权限，即至少teecd和CA的用户组里面应该包含tzdriver的用户组ID，且tzdriver一般应该设置为同组可访问）。

**问题现象**2

ioctl失败，未执行到tzdriver，且errno为Invalid Argument。

**可能原因**

可能是命令字和ioctl的参数不一样，比如如果命令字是\_IOWR定义的，那么ioctl的arg参数则不应为非法指针。

**解决办法**

排查ioctl的参数，是否有误用的情况，比如ioctl的第一个参数是fd，误传递文件指针，就会出现invalid argument的错误。

### ioctl时tzdriver报错invalid cmd<a name="section137051444142912"></a>

**问题现象**

ioctl时tzdriver报错invalid cmd。

**可能原因**

可能是teecd和tzdriver之间命令字定义不一样，重点排查结构体大小不一致的情况。

**解决办法**

检查客户端和tzdriver之间的结构体和命令字定义，要确保客户端和tzdriver两侧定义的命令字以及结构体完全一致。

