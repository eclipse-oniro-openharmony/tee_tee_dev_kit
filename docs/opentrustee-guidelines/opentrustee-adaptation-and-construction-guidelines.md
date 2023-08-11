# OpenTrustee 适配和构建指导

## Tee Client的适配和构建

### Tee Client使能实例

本章节讲述如何针对一款芯片适配TEE Client，在相应配置json文件中增加tee_client部件即可。

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

以RK3568芯片为例，运行以下命令编译TEE Client部件，产物路径：out/rk3568/tee/tee_client

```shell
./build.sh --product-name rk3568 --ccache --build-target tee_client
```

## Tzdriver的适配和构建

### 概述

tzdriver是TEE的内核驱动，主要功能是在整个TEE子系统中起连接作用，是使用TEE OS服务的桥梁，tzdriver处理来自于tee\_client的ioctl命令，并通过smc指令从REE切换到TEE。

>![](public_sys-resources/icon-caution.gif) **注意：** 
>单独适配tzdriver，系统并不能正常启动，必须同时适配TEE OS。

### 适配指导以及适配实例

本章节中会讲述如何针对一款芯片适配tzdriver，此章节中以RK3568芯片为例。

第一小节介绍tzdriver入口和tzdriver代码位置，本章下面几个小节，每小节都是一个适配步骤。（这几个适配步骤顺序无关，但推荐将配置选项放到最后）

#### tzdriver入口

-   Linux内核tzdriver代码位置：base/tee/tee\_tee\_tzdriver/linux。

tzdriver是内核中的一个字符设备驱动。

tzdriver初始化时会创建一个字符设备文件，一般为/dev/tc\_ns\_client，用户态进程可以打开此节点，以及通过ioctl接口调用tzdriver相关功能。

tzdriver总入口在core/tc\_client\_driver.c中的tc\_client\_ioctl函数。

设备节点函数接口：

```c
static const struct file_operations_vfs g_tc_ns_client_fops = {
    .open = tc_client_open,
    .close = tc_client_close,
    .ioctl = tc_client_ioctl,
    .mmap = tc_client_mmap,
};
```

#### 工程编译适配

tzdriver需要被编译到内核中作为内核驱动。

-   Linux内核tzdriver编译适配

    在Linux内核可以通过defconfig文件中的CONFIG\_TZDRIVER选项控制tzdriver的编译使能。

    1.  defconfig文件修改

        defconfig文件在kernel/linux/config仓，每个芯片应当创建自己的defconfig文件，后面会介绍tzdriver中的所有defconfig配置项。

    2.  kernel补丁

        其他内核相关修改在kernel/linux/patches仓，以补丁方式提供。

        RK3568芯片的patch在kernel/linux/patches/linux-5.10/rk3568_patch/kernel.patch，其他芯片平台也可以参考这个patch，每个芯片应该创建自己的patch文件。此patch补丁中应当包含以下内容：

        -   对于内核的根Makefile的修改（在其中引用tzdriver仓的子Makefile，其中tzdriver path需要修改为实际的相对路径，注意Linux kernel的编译是会将kernel仓代码拷贝到out目录打patch，因此这个相对路径是相对于out下的临时kernel仓的路径）。

            ```
            obj-y += {tzdriver path}
            ```

        -   对于内核的根Kconfig的需改（在其中引用tzdriver仓的子Kconfig，其中tzdriver path需要修改为实际的相对路径，同上需要注意这个相对路径应当是在out目录下的临时kernel仓路径）。

            ```
            source "{tzdriver path}/Kconfig"
            ```

        -   dtsi的修改：需要在相应芯片的disi文件中包含trusted\_core节点，对于RK3568芯片，需要修改patch中的/arch/arm64/boot/dts/rockchip/rk3568-toybrick-x0.dtsi文件，新增以下内容

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


#### 驱动初始化

-   Linux内核中tzdriver驱动初始化方式

    自动初始化，无需适配修改。

### 配置选项

#### 内核配置选项

tzdriver有一些特性或者选项，可以选择配置，控制这些选项的地方如下：

-   Linux内核tzdriver配置选项

    tzdriver选项应该写在kernel/linux/config仓，修改芯片的defconfig文件：

    ```
    #
    # TEE OS
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
    <td class="cellrowborder" valign="top" width="55.800000000000004%" headers="mcps1.2.3.1.2 "><p id="p382054615243"><a name="p382054615243"></a><a name="p382054615243"></a>tlogger使用的内存类型，开发者无需修改</p>
    </td>
    </tr>
    <tr id="row9724616192417"><td class="cellrowborder" valign="top" width="44.2%" headers="mcps1.2.3.1.1 "><p id="p3725516102418"><a name="p3725516102418"></a><a name="p3725516102418"></a>CONFIG_THIRDPARTY_COMPATIBLE</p>
    </td>
    <td class="cellrowborder" valign="top" width="55.800000000000004%" headers="mcps1.2.3.1.2 "><p id="p382054615243"><a name="p382054615243"></a><a name="p382054615243"></a>兼容第三方opteed的适配，例如适配RK3568芯片需要开启此选项</p>
    </td> 
    </tr>
    </tbody>
    </table>

### TEE Tzdriver编译命令
tzdriver部件跟随kernel一起编译，编译命令如下
```Bash
./build.sh --product-name rk3568 --ccache --build-target kernel --gn-args linux_kernel_version=\"linux-5.10\"
```

## TEE OS镜像的构建指导

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
-  克隆`https://github.com/rockchip-linux/rkbin`，其中包含bl31.elf
-  克隆`https://github.com/rockchip-linux/u-boot`，其中包含u-boot
-  将rkbin和u-boot放在同一目录下, 修改rkbin/RKTRUST/RK3568TRUST.ini中BL32_OPTION下的PATH指向bl32.bin
-    其中，u-boot/make.sh中有以下内容，搜索RK3568TRUST.ini中包含_bl32_的文件名，如果BL32的文件名是bl32.bin的话，需要修改下匹配规则
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