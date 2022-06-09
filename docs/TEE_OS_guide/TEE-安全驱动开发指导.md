# TEE 安全驱动开发指导<a name="ZH-CN_TOPIC_0000001212014566"></a>

-   [概述](#section351681313544)
    -   [功能简介](#section167701178548)
    -   [约束与限制](#section67447370556)
    -   [场景介绍](#section148731215195617)

-   [开发环境准备](#section20994326588)
-   [开发步骤](#section943512417516)
    -   [添加编译配置文件至build目录](#section449714160615)
    -   [添加头文件至include目录](#section179732211510)
    -   [添加源文件至src目录](#section13984839152)
    -   [触发构建](#section885817169712)

-   [编译文件配置](#section88091241624)
    -   [build.sh文件介绍](#section1751202026)
    -   [defconfig文件介绍](#section21267445213)
    -   [Makefile文件介绍](#section15184100432)
    -   [CMakeLists.txt文件介绍](#section31566231037)

-   [驱动信息与权限配置](#section1630913525319)
    -   [manifest.txt文件介绍](#section10259663411)
    -   [宏定义表文件介绍](#section449462816414)
    -   [configs.xml文件介绍](#section9908114312413)

-   [签名文件生成](#section793382693010)
    -   [密钥及证书生成](#section1046723143114)
    -   [perm\_config文件生产](#section11327454193517)

-   [驱动开发框架](#section12432132218372)
    -   [驱动业务框架](#section6705194712373)
    -   [驱动访问者框架](#section18123151133811)

-   [接口说明](#section1748173625619)
    -   [地址转换接口说明](#section101297155716)
    -   [map接口说明](#section1831091675716)
    -   [IO操作接口说明](#section6512133012574)
    -   [内存拷贝接口说明](#section75381736318)
    -   [共享内存接口说明](#section5891654459)

-   [开发示例](#section198361520981)
-   [标准C库支持](#section14109139161012)
-   [安全函数库](#section127921457151719)
    -   [概述](#section5848918131818)
    -   [内存复制](#section1620304312183)
    -   [内存初始化](#section9113185921817)
    -   [字符串复制](#section320518115198)
    -   [字符串连接](#section3307826101911)
    -   [字符串分割](#section85731515152610)
    -   [格式化输出](#section1696173315284)

## 概述<a name="section351681313544"></a>

### 功能简介<a name="section167701178548"></a>

本文对TEE安全驱动的开发流程、接口、函数库等进行说明，指导驱动开发者进行安全驱动程序的开发与调试工作。

### 约束与限制<a name="section67447370556"></a>

-   目前仅支持Linux环境下进行安全驱动的编译打包和运行。
-   开发的驱动程序仅支持在TEE可信执行环境子系统上加载和运行。

### 场景介绍<a name="section148731215195617"></a>

本节指导开发可以被TEE子系统动态加载的安全驱动。

## 开发环境准备<a name="section20994326588"></a>

目前支持的编译框架、编译工具链等见下表，括号内为建议版本：

**表 1**  开发环境准备

<a name="table412352084019"></a>
<table><thead align="left"><tr id="row151231920154014"><th class="cellrowborder" valign="top" width="33.33333333333333%" id="mcps1.2.4.1.1"><p id="p7533141614371"><a name="p7533141614371"></a><a name="p7533141614371"></a>编译框架</p>
</th>
<th class="cellrowborder" valign="top" width="33.33333333333333%" id="mcps1.2.4.1.2"><p id="p05331616183718"><a name="p05331616183718"></a><a name="p05331616183718"></a>编译工具链</p>
</th>
<th class="cellrowborder" valign="top" width="33.33333333333333%" id="mcps1.2.4.1.3"><p id="p1753351614371"><a name="p1753351614371"></a><a name="p1753351614371"></a>python版本</p>
</th>
</tr>
</thead>
<tbody><tr id="row18123720154011"><td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.1 "><p id="p653301612373"><a name="p653301612373"></a><a name="p653301612373"></a>cmake（建议使用3.16.5-3.20.5版本）</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.2 "><p id="p10534131612374"><a name="p10534131612374"></a><a name="p10534131612374"></a>GCC（gcc-linaro-7.4.1）</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.3 "><p id="p11535216183712"><a name="p11535216183712"></a><a name="p11535216183712"></a>python3（python3.8.5）</p>
</td>
</tr>
<tr id="row11231720104012"><td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.1 "><p id="p1853518165372"><a name="p1853518165372"></a><a name="p1853518165372"></a>make</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.2 "><p id="p453561653713"><a name="p453561653713"></a><a name="p453561653713"></a>LLVM（clang+llvm-8.0.1）</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.3 "><p id="p10535201633714"><a name="p10535201633714"></a><a name="p10535201633714"></a>-</p>
</td>
</tr>
</tbody>
</table>

需要使用的GCC工具链可在linaro网站下载，LLVM和Clang需要在LLVM官网下载。64位可信驱动程序编译需要使用aarch64版本的交叉编译工具链，按照与32位工具链相同的方式下载、安装和配置。

**表 2**  编译工具链准备

<a name="table176317549401"></a>
<table><thead align="left"><tr id="row376335424011"><th class="cellrowborder" valign="top" width="70.94709470947095%" id="mcps1.2.4.1.1"><p id="p1718993710373"><a name="p1718993710373"></a><a name="p1718993710373"></a>工具链下载路径</p>
</th>
<th class="cellrowborder" valign="top" width="15.391539153915392%" id="mcps1.2.4.1.2"><p id="p191891337123714"><a name="p191891337123714"></a><a name="p191891337123714"></a>32位可信驱动程序编译</p>
</th>
<th class="cellrowborder" valign="top" width="13.661366136613662%" id="mcps1.2.4.1.3"><p id="p0189183713714"><a name="p0189183713714"></a><a name="p0189183713714"></a>64位可信驱动程序编译</p>
</th>
</tr>
</thead>
<tbody><tr id="row1576316548408"><td class="cellrowborder" valign="top" width="70.94709470947095%" headers="mcps1.2.4.1.1 "><p id="p218911379372"><a name="p218911379372"></a><a name="p218911379372"></a><a href="https://releases.linaro.org/components/toolchain/binaries/7.4-2019.02/arm-eabi/gcc-linaro-7.4.1-2019.02-x86_64_arm-eabi.tar.xz" target="_blank" rel="noopener noreferrer">https://releases.linaro.org/components/toolchain/binaries/7.4-2019.02/arm-eabi/gcc-linaro-7.4.1-2019.02-x86_64_arm-eabi.tar.xz</a></p>
</td>
<td class="cellrowborder" valign="top" width="15.391539153915392%" headers="mcps1.2.4.1.2 "><p id="p191896374370"><a name="p191896374370"></a><a name="p191896374370"></a>是</p>
</td>
<td class="cellrowborder" valign="top" width="13.661366136613662%" headers="mcps1.2.4.1.3 "><p id="p1618983713373"><a name="p1618983713373"></a><a name="p1618983713373"></a>否</p>
</td>
</tr>
<tr id="row2076325419404"><td class="cellrowborder" valign="top" width="70.94709470947095%" headers="mcps1.2.4.1.1 "><p id="p1218915377377"><a name="p1218915377377"></a><a name="p1218915377377"></a><a href="https://releases.linaro.org/components/toolchain/binaries/7.4-2019.02/arm-linux-gnueabi/gcc-linaro-7.4.1-2019.02-x86_64_arm-linux-gnueabi.tar.xz" target="_blank" rel="noopener noreferrer">https://releases.linaro.org/components/toolchain/binaries/7.4-2019.02/arm-linux-gnueabi/gcc-linaro-7.4.1-2019.02-x86_64_arm-linux-gnueabi.tar.xz</a></p>
</td>
<td class="cellrowborder" valign="top" width="15.391539153915392%" headers="mcps1.2.4.1.2 "><p id="p41901837153710"><a name="p41901837153710"></a><a name="p41901837153710"></a>是</p>
</td>
<td class="cellrowborder" valign="top" width="13.661366136613662%" headers="mcps1.2.4.1.3 "><p id="p519016379375"><a name="p519016379375"></a><a name="p519016379375"></a>否</p>
</td>
</tr>
<tr id="row1576311547408"><td class="cellrowborder" valign="top" width="70.94709470947095%" headers="mcps1.2.4.1.1 "><p id="p16190123710372"><a name="p16190123710372"></a><a name="p16190123710372"></a><a href="https://releases.linaro.org/components/toolchain/binaries/7.4-2019.02/aarch64-linux-gnu/gcc-linaro-7.4.1-2019.02-x86_64_aarch64-linux-gnu.tar.xz" target="_blank" rel="noopener noreferrer">https://releases.linaro.org/components/toolchain/binaries/7.4-2019.02/aarch64-linux-gnu/gcc-linaro-7.4.1-2019.02-x86_64_aarch64-linux-gnu.tar.xz</a></p>
</td>
<td class="cellrowborder" valign="top" width="15.391539153915392%" headers="mcps1.2.4.1.2 "><p id="p17190103719373"><a name="p17190103719373"></a><a name="p17190103719373"></a>否</p>
</td>
<td class="cellrowborder" valign="top" width="13.661366136613662%" headers="mcps1.2.4.1.3 "><p id="p119043763717"><a name="p119043763717"></a><a name="p119043763717"></a>是</p>
</td>
</tr>
<tr id="row2763165410401"><td class="cellrowborder" valign="top" width="70.94709470947095%" headers="mcps1.2.4.1.1 "><p id="p1619017373372"><a name="p1619017373372"></a><a name="p1619017373372"></a><a href="https://github.com/llvm/llvm-project/releases/download/llvmorg-8.0.1/clang+llvm-8.0.1-x86_64-linux-gnu-ubuntu-14.04.tar.xz" target="_blank" rel="noopener noreferrer">https://github.com/llvm/llvm-project/releases/download/llvmorg-8.0.1/clang+llvm-8.0.1-x86_64-linux-gnu-ubuntu-14.04.tar.xz</a></p>
</td>
<td class="cellrowborder" valign="top" width="15.391539153915392%" headers="mcps1.2.4.1.2 "><p id="p3190123715370"><a name="p3190123715370"></a><a name="p3190123715370"></a>是</p>
</td>
<td class="cellrowborder" valign="top" width="13.661366136613662%" headers="mcps1.2.4.1.3 "><p id="p819018374375"><a name="p819018374375"></a><a name="p819018374375"></a>是</p>
</td>
</tr>
</tbody>
</table>

>![](public_sys-resources/icon-caution.gif) **注意：** 
>-   对于32位或者64位可信驱动程序的支持，不同平台存在差异，请与对应产品或TEE负责人确认。
>-   驱动签名工具依赖python的defusedxml-0.7.1工具，需要使用pip install defusedxml==0.7.1进行下载。

## 开发步骤<a name="section943512417516"></a>

驱动开发者可以使用DDK将源码通过编译和签名生成TEE子系统可动态加载的sec文件。

 **1 添加编译配置文件至build目录** 

用户根据本地开发环境与需求，更改demo/build/[defconfig](#section21267445213)文件中相关配置项的内容，决定编译器与编译工具链等信息。

在环境变量中添加编译工具链位置信息或export相关路径例如：export TOOLCHAIN\_PATH=path/toolchains/bin/。

根据配置文件的编译器参数，在[Makefile](#section15184100432)或[CMakeLists.txt](#section31566231037)中添加需要编译的源文件、编译选项、链接选项等信息。

 **2 添加头文件至include目录** 

include文件夹中包含驱动动态库接口头文件，编译第三方驱动时需要将第三方驱动所需头文件添加至本文件夹下。

 **3 添加源文件至src目录** 

src目录中包含驱动源代码与权限配置文件。

在驱动开发时，开发者可以参考demo/src目录中举例文件编写自己的驱动文件，参考该目录中[manifest.txt](#section10259663411)文件填写驱动配置信息，参考该目录中[drv\_test\_module.csv](#section449462816414)文件与build/pack-Config/input目录下的[configs.xml](#section9908114312413)文件，添加新增驱动的权限配置信息。

[签名文件生成](#section793382693010)后，将SEC签名密钥[sign\_sec\_priv.pem](#table11862185716131)从build/keytools/output目录拷贝至demo/src目录，将签名文件[perm\_config](#section11327454193517)从build/pack-Config/output目录拷贝至demo/src目录。

 **4 触发构建** 

在demo/build目录下执行[build.sh](#section1751202026)，触发编译、签名流程，输入[profile.ini](#section1046723143114)文件中配置的pass字段签名口令，输出生成文件至demo/output目录下。

编译命令示例：

```
bash build.sh
```

## 编译文件配置<a name="section88091241624"></a>

编译文件配置项见下表：

**表 3**  编译文件配置项

<a name="table2173294241"></a>
<table><thead align="left"><tr id="row018142918241"><th class="cellrowborder" valign="top" width="27.060000000000002%" id="mcps1.2.6.1.1"><p id="p177781452182211"><a name="p177781452182211"></a><a name="p177781452182211"></a>编译文件配置项</p>
</th>
<th class="cellrowborder" valign="top" width="10.85%" id="mcps1.2.6.1.2"><p id="p1535671294219"><a name="p1535671294219"></a><a name="p1535671294219"></a>类型</p>
</th>
<th class="cellrowborder" valign="top" width="22.09%" id="mcps1.2.6.1.3"><p id="p6356171274215"><a name="p6356171274215"></a><a name="p6356171274215"></a>配置内容</p>
</th>
<th class="cellrowborder" valign="top" width="12.82%" id="mcps1.2.6.1.4"><p id="p9187296246"><a name="p9187296246"></a><a name="p9187296246"></a>所在文件</p>
</th>
<th class="cellrowborder" valign="top" width="27.18%" id="mcps1.2.6.1.5"><p id="p93568124421"><a name="p93568124421"></a><a name="p93568124421"></a>描述</p>
</th>
</tr>
</thead>
<tbody><tr id="row161814297246"><td class="cellrowborder" valign="top" width="27.060000000000002%" headers="mcps1.2.6.1.1 "><p id="p149288206257"><a name="p149288206257"></a><a name="p149288206257"></a>CONFIG_BUILD_TOOL</p>
</td>
<td class="cellrowborder" valign="top" width="10.85%" headers="mcps1.2.6.1.2 "><p id="p6928152072517"><a name="p6928152072517"></a><a name="p6928152072517"></a>参数配置项</p>
</td>
<td class="cellrowborder" valign="top" width="22.09%" headers="mcps1.2.6.1.3 "><p id="p154812059172218"><a name="p154812059172218"></a><a name="p154812059172218"></a>make：使用make编译</p>
<p id="p292872052519"><a name="p292872052519"></a><a name="p292872052519"></a>cmake：使用cmake编译</p>
</td>
<td class="cellrowborder" valign="top" width="12.82%" headers="mcps1.2.6.1.4 "><p id="p1992862022512"><a name="p1992862022512"></a><a name="p1992862022512"></a>config</p>
</td>
<td class="cellrowborder" valign="top" width="27.18%" headers="mcps1.2.6.1.5 "><p id="p69280202253"><a name="p69280202253"></a><a name="p69280202253"></a>指定编译框架（make/cmake）</p>
</td>
</tr>
<tr id="row6341193312247"><td class="cellrowborder" valign="top" width="27.060000000000002%" headers="mcps1.2.6.1.1 "><p id="p3928112082515"><a name="p3928112082515"></a><a name="p3928112082515"></a>CONFIG_GCC</p>
</td>
<td class="cellrowborder" valign="top" width="10.85%" headers="mcps1.2.6.1.2 "><p id="p5928172012255"><a name="p5928172012255"></a><a name="p5928172012255"></a>参数配置项</p>
</td>
<td class="cellrowborder" valign="top" width="22.09%" headers="mcps1.2.6.1.3 "><p id="p41765355237"><a name="p41765355237"></a><a name="p41765355237"></a>y：使用gcc工具链编译</p>
<p id="p2928192017258"><a name="p2928192017258"></a><a name="p2928192017258"></a>n：使用llvm工具链编译</p>
</td>
<td class="cellrowborder" valign="top" width="12.82%" headers="mcps1.2.6.1.4 "><p id="p8928152013258"><a name="p8928152013258"></a><a name="p8928152013258"></a>config</p>
</td>
<td class="cellrowborder" valign="top" width="27.18%" headers="mcps1.2.6.1.5 "><p id="p4928192062516"><a name="p4928192062516"></a><a name="p4928192062516"></a>指定编译工具链（gcc/llvm）</p>
</td>
</tr>
<tr id="row1418112917246"><td class="cellrowborder" valign="top" width="27.060000000000002%" headers="mcps1.2.6.1.1 "><p id="p1792802011256"><a name="p1792802011256"></a><a name="p1792802011256"></a>TARGET_IS_ARM64</p>
</td>
<td class="cellrowborder" valign="top" width="10.85%" headers="mcps1.2.6.1.2 "><p id="p139281020122519"><a name="p139281020122519"></a><a name="p139281020122519"></a>参数配置项</p>
</td>
<td class="cellrowborder" valign="top" width="22.09%" headers="mcps1.2.6.1.3 "><p id="p1922715142317"><a name="p1922715142317"></a><a name="p1922715142317"></a>y：编译64位驱动</p>
<p id="p792922032510"><a name="p792922032510"></a><a name="p792922032510"></a>n：编译32位驱动</p>
</td>
<td class="cellrowborder" valign="top" width="12.82%" headers="mcps1.2.6.1.4 "><p id="p19299203259"><a name="p19299203259"></a><a name="p19299203259"></a>config</p>
</td>
<td class="cellrowborder" valign="top" width="27.18%" headers="mcps1.2.6.1.5 "><p id="p7929102052510"><a name="p7929102052510"></a><a name="p7929102052510"></a>指定驱动版本（32位/64位）</p>
</td>
</tr>
<tr id="row1118122962415"><td class="cellrowborder" valign="top" width="27.060000000000002%" headers="mcps1.2.6.1.1 "><p id="p392942011254"><a name="p392942011254"></a><a name="p392942011254"></a>CFLAGS</p>
</td>
<td class="cellrowborder" valign="top" width="10.85%" headers="mcps1.2.6.1.2 "><p id="p69291320132517"><a name="p69291320132517"></a><a name="p69291320132517"></a>make变量</p>
</td>
<td class="cellrowborder" valign="top" width="22.09%" headers="mcps1.2.6.1.3 "><p id="p892932062519"><a name="p892932062519"></a><a name="p892932062519"></a>编译选项</p>
</td>
<td class="cellrowborder" valign="top" width="12.82%" headers="mcps1.2.6.1.4 "><p id="p189293202252"><a name="p189293202252"></a><a name="p189293202252"></a>Makefile</p>
</td>
<td class="cellrowborder" valign="top" width="27.18%" headers="mcps1.2.6.1.5 "><p id="p129291320182510"><a name="p129291320182510"></a><a name="p129291320182510"></a>make添加编译选项</p>
</td>
</tr>
<tr id="row1818202922417"><td class="cellrowborder" valign="top" width="27.060000000000002%" headers="mcps1.2.6.1.1 "><p id="p792992018252"><a name="p792992018252"></a><a name="p792992018252"></a>LDFLAGS</p>
</td>
<td class="cellrowborder" valign="top" width="10.85%" headers="mcps1.2.6.1.2 "><p id="p2929920142515"><a name="p2929920142515"></a><a name="p2929920142515"></a>make变量</p>
</td>
<td class="cellrowborder" valign="top" width="22.09%" headers="mcps1.2.6.1.3 "><p id="p119291320112510"><a name="p119291320112510"></a><a name="p119291320112510"></a>链接选项</p>
</td>
<td class="cellrowborder" valign="top" width="12.82%" headers="mcps1.2.6.1.4 "><p id="p892962072510"><a name="p892962072510"></a><a name="p892962072510"></a>Makefile</p>
</td>
<td class="cellrowborder" valign="top" width="27.18%" headers="mcps1.2.6.1.5 "><p id="p292913205254"><a name="p292913205254"></a><a name="p292913205254"></a>make添加链接选项</p>
</td>
</tr>
<tr id="row1719229192418"><td class="cellrowborder" valign="top" width="27.060000000000002%" headers="mcps1.2.6.1.1 "><p id="p492982011250"><a name="p492982011250"></a><a name="p492982011250"></a>INCLUDEDIR</p>
</td>
<td class="cellrowborder" valign="top" width="10.85%" headers="mcps1.2.6.1.2 "><p id="p99298206259"><a name="p99298206259"></a><a name="p99298206259"></a>make变量</p>
</td>
<td class="cellrowborder" valign="top" width="22.09%" headers="mcps1.2.6.1.3 "><p id="p189308205259"><a name="p189308205259"></a><a name="p189308205259"></a>头文件路径</p>
</td>
<td class="cellrowborder" valign="top" width="12.82%" headers="mcps1.2.6.1.4 "><p id="p993012207253"><a name="p993012207253"></a><a name="p993012207253"></a>Makefile</p>
</td>
<td class="cellrowborder" valign="top" width="27.18%" headers="mcps1.2.6.1.5 "><p id="p13930102019258"><a name="p13930102019258"></a><a name="p13930102019258"></a>make添加头文件路径</p>
</td>
</tr>
<tr id="row21932916244"><td class="cellrowborder" valign="top" width="27.060000000000002%" headers="mcps1.2.6.1.1 "><p id="p993012062517"><a name="p993012062517"></a><a name="p993012062517"></a>SRC</p>
</td>
<td class="cellrowborder" valign="top" width="10.85%" headers="mcps1.2.6.1.2 "><p id="p1793022022515"><a name="p1793022022515"></a><a name="p1793022022515"></a>make变量</p>
</td>
<td class="cellrowborder" valign="top" width="22.09%" headers="mcps1.2.6.1.3 "><p id="p13930182013252"><a name="p13930182013252"></a><a name="p13930182013252"></a>源文件路径</p>
</td>
<td class="cellrowborder" valign="top" width="12.82%" headers="mcps1.2.6.1.4 "><p id="p1293015206259"><a name="p1293015206259"></a><a name="p1293015206259"></a>Makefile</p>
</td>
<td class="cellrowborder" valign="top" width="27.18%" headers="mcps1.2.6.1.5 "><p id="p4930420182517"><a name="p4930420182517"></a><a name="p4930420182517"></a>make添加编译文件</p>
</td>
</tr>
<tr id="row119829192416"><td class="cellrowborder" valign="top" width="27.060000000000002%" headers="mcps1.2.6.1.1 "><p id="p3930172017253"><a name="p3930172017253"></a><a name="p3930172017253"></a>compile_drv("\${SOURCE_FILE}" "\${FLAGS}" "\${INC_DIR}" "\${LD_FLAGS}")</p>
</td>
<td class="cellrowborder" valign="top" width="10.85%" headers="mcps1.2.6.1.2 "><p id="p0930172020255"><a name="p0930172020255"></a><a name="p0930172020255"></a>cmake函数</p>
</td>
<td class="cellrowborder" valign="top" width="22.09%" headers="mcps1.2.6.1.3 "><p id="p19930152012515"><a name="p19930152012515"></a><a name="p19930152012515"></a>函数参数：</p>
<p id="p49303207257"><a name="p49303207257"></a><a name="p49303207257"></a>SOURCE_FILE：源文件路径</p>
<p id="p17930122019258"><a name="p17930122019258"></a><a name="p17930122019258"></a>FLAGS：编译选项</p>
<p id="p99301920202515"><a name="p99301920202515"></a><a name="p99301920202515"></a>INC_DIR：头文件路径</p>
<p id="p3930112032514"><a name="p3930112032514"></a><a name="p3930112032514"></a>LD_FLAGS：链接选项</p>
</td>
<td class="cellrowborder" valign="top" width="12.82%" headers="mcps1.2.6.1.4 "><p id="p1993032013258"><a name="p1993032013258"></a><a name="p1993032013258"></a>CMakeLists.txt</p>
</td>
<td class="cellrowborder" valign="top" width="27.18%" headers="mcps1.2.6.1.5 "><p id="p1393152013254"><a name="p1393152013254"></a><a name="p1393152013254"></a>cmake编译接口函数</p>
</td>
</tr>
</tbody>
</table>

### build.sh文件介绍<a name="section1751202026"></a>

build.sh文件是可信驱动程序编译的触发入口，用户配置完编译文件后，通过bash build.sh触发编译即可，文件包含加载配置文件、编译触发、镜像签名等相关模块。

驱动开发者可以不关注编译框架本身的处理，使用build.sh触发编译，编译框架根据配置参数、编译参数自动完成驱动镜像的生成过程。

### defconfig文件介绍<a name="section21267445213"></a>

defconfig文件为编译配置文件，该文件中可以指定使用的编译器、编译工具链、指定编译32位或64位。

CONFIG\_BUILD\_TOOL为指定编译器，其值为make代表使用make编译，其值为cmake代表使用cmake编译。

CONFIG\_GCC为指定编译工具链，其值为y代表使用GCC编译，其值为n代表使用LLVM编译。

TARGET\_IS\_ARM64为指定编译版本，其值为y时编译64位驱动，为其值为n时编译32位驱动。

defconfig文件举例如下：

```
# set CONFIG_BUILD_TOOL (cmake / make)
CONFIG_BUILD_TOOL=make
 
# set CONFIG_GCC (y / n)
CONFIG_GCC=y
 
# set TARGET_IS_ARM64 (y / n)
TARGET_IS_ARM64=y
```

### Makefile文件介绍<a name="section15184100432"></a>

Makefile为make编译接口文件，用户可在其中将源文件、编译选项、链接选项等添加在指定变量中，从而完成编译的目的。

Makefile文件举例如下：

```
CFLAGS += -Wstack-protector --param ssp-buffer-size=4 -DAPI_LEVEL=1
LDFLAGS += -shared
 
SRC += $(wildcard ../src/*.c)
 
INCLUDEDIR += -I../include
 
include $(ITRUSTEE_BUILD_PATH)/build/mk/common.mk
```

### CMakeLists.txt文件介绍<a name="section31566231037"></a>

CMakeLists.txt为cmake编译接口文件，用户可在其中将源文件、编译选项、链接选项等添加在compile\_drv函数中，该函数第一个入参为源文件，第二个入参为编译选项，第三个参数为include文件路径，第四个参数为链接选项。

CMakeLists.txt文件举例如下：

```
cmake_minimum_required(VERSION 3.16 FATAL_ERROR)
project(tee_ddk C)
 
include (${ITRUSTEE_BUILD_PATH}/build/cmake/common.cmake)
 
list(APPEND SOURCE_FILE
    ${CMAKE_CURRENT_SOURCE_DIR}/../src/drv_test_module.c
)
 
list(APPEND INC_DIR
    ${CMAKE_CURRENT_SOURCE_DIR}/../include
)
 
list(APPEND FLAGS
    -DAPI_LEVEL=1
)
 
list(APPEND LINK_FLAGS
    -Wl,-z,relro
)
 
compile_drv("${SOURCE_FILE}" "${FLAGS}" "${INC_DIR}" "${LD_FLAGS}")
```

## 驱动信息与权限配置<a name="section1630913525319"></a>

### manifest.txt文件介绍<a name="section10259663411"></a>

可信驱动程序开发时需提供manifest.txt文本文件，该文件的主要作用是设置驱动属性，主要内容见下表：

**表 4**  manifest.txt内容

<a name="table14537182315286"></a>
<table><thead align="left"><tr id="row4538112318286"><th class="cellrowborder" valign="top" width="22.932293229322934%" id="mcps1.2.4.1.1"><p id="p4803260443"><a name="p4803260443"></a><a name="p4803260443"></a>属性</p>
</th>
<th class="cellrowborder" valign="top" width="11.37113711371137%" id="mcps1.2.4.1.2"><p id="p1480312615444"><a name="p1480312615444"></a><a name="p1480312615444"></a>类型</p>
</th>
<th class="cellrowborder" valign="top" width="65.69656965696569%" id="mcps1.2.4.1.3"><p id="p188035612448"><a name="p188035612448"></a><a name="p188035612448"></a>说明</p>
</th>
</tr>
</thead>
<tbody><tr id="row1653815239283"><td class="cellrowborder" valign="top" width="22.932293229322934%" headers="mcps1.2.4.1.1 "><p id="p78031620448"><a name="p78031620448"></a><a name="p78031620448"></a>gpd.ta.appID</p>
</td>
<td class="cellrowborder" valign="top" width="11.37113711371137%" headers="mcps1.2.4.1.2 "><p id="p10803156114416"><a name="p10803156114416"></a><a name="p10803156114416"></a>uuid</p>
</td>
<td class="cellrowborder" valign="top" width="65.69656965696569%" headers="mcps1.2.4.1.3 "><p id="p10803126204419"><a name="p10803126204419"></a><a name="p10803126204419"></a>可信驱动程序UUID信息，必须唯一。</p>
</td>
</tr>
<tr id="row953822314281"><td class="cellrowborder" valign="top" width="22.932293229322934%" headers="mcps1.2.4.1.1 "><p id="p380412611444"><a name="p380412611444"></a><a name="p380412611444"></a>gpd.ta.service_name</p>
</td>
<td class="cellrowborder" valign="top" width="11.37113711371137%" headers="mcps1.2.4.1.2 "><p id="p28041664410"><a name="p28041664410"></a><a name="p28041664410"></a>字符串</p>
</td>
<td class="cellrowborder" valign="top" width="65.69656965696569%" headers="mcps1.2.4.1.3 "><p id="p178041561442"><a name="p178041561442"></a><a name="p178041561442"></a>可信驱动程序名字，与驱动文件名保持一致，有效长度小于32字节，仅支持数字、字母和'_'。</p>
</td>
</tr>
<tr id="row6538623122810"><td class="cellrowborder" valign="top" width="22.932293229322934%" headers="mcps1.2.4.1.1 "><p id="p20804136204411"><a name="p20804136204411"></a><a name="p20804136204411"></a>gpd.ta.instanceKeepAlive（暂未实现卸载功能，默认为true，可信驱动程序加载之后常驻）</p>
</td>
<td class="cellrowborder" valign="top" width="11.37113711371137%" headers="mcps1.2.4.1.2 "><p id="p118042634414"><a name="p118042634414"></a><a name="p118042634414"></a>bool</p>
</td>
<td class="cellrowborder" valign="top" width="65.69656965696569%" headers="mcps1.2.4.1.3 "><p id="p380413684416"><a name="p380413684416"></a><a name="p380413684416"></a>可信驱动程序是否常驻：</p>
<p id="p19804465445"><a name="p19804465445"></a><a name="p19804465445"></a>true：常驻，加载之后不会再卸载；</p>
<p id="p68041764448"><a name="p68041764448"></a><a name="p68041764448"></a>false：非常驻，加载之后当没有被访问时会被卸载，下次再访问时会重新加载。</p>
</td>
</tr>
<tr id="row18538823172814"><td class="cellrowborder" valign="top" width="22.932293229322934%" headers="mcps1.2.4.1.1 "><p id="p10804186204419"><a name="p10804186204419"></a><a name="p10804186204419"></a>gpd.ta.dataSize</p>
</td>
<td class="cellrowborder" valign="top" width="11.37113711371137%" headers="mcps1.2.4.1.2 "><p id="p88041462447"><a name="p88041462447"></a><a name="p88041462447"></a>uint32_t</p>
</td>
<td class="cellrowborder" valign="top" width="65.69656965696569%" headers="mcps1.2.4.1.3 "><p id="p88049664413"><a name="p88049664413"></a><a name="p88049664413"></a>可信驱动程序可申请最大堆空间。</p>
</td>
</tr>
<tr id="row12538162392819"><td class="cellrowborder" valign="top" width="22.932293229322934%" headers="mcps1.2.4.1.1 "><p id="p1180419612441"><a name="p1180419612441"></a><a name="p1180419612441"></a>gpd.ta.stackSize</p>
</td>
<td class="cellrowborder" valign="top" width="11.37113711371137%" headers="mcps1.2.4.1.2 "><p id="p380418674410"><a name="p380418674410"></a><a name="p380418674410"></a>uint32_t</p>
</td>
<td class="cellrowborder" valign="top" width="65.69656965696569%" headers="mcps1.2.4.1.3 "><p id="p1880418694415"><a name="p1880418694415"></a><a name="p1880418694415"></a>可信驱动程序线程栈大小。</p>
</td>
</tr>
<tr id="row18538162362815"><td class="cellrowborder" valign="top" width="22.932293229322934%" headers="mcps1.2.4.1.1 "><p id="p14804362445"><a name="p14804362445"></a><a name="p14804362445"></a>gpd.ta.target_type</p>
</td>
<td class="cellrowborder" valign="top" width="11.37113711371137%" headers="mcps1.2.4.1.2 "><p id="p580416615448"><a name="p580416615448"></a><a name="p580416615448"></a>uint64_t</p>
</td>
<td class="cellrowborder" valign="top" width="65.69656965696569%" headers="mcps1.2.4.1.3 "><p id="p1980436144414"><a name="p1980436144414"></a><a name="p1980436144414"></a>构建目标类型，可信驱动程序此值设置为1。</p>
</td>
</tr>
</tbody>
</table>

可行配置如下所示：

```
gpd.ta.appID: 00000000-0000-0000-0000-333333333333
gpd.ta.service_name: drv_test_module
gpd.ta.instanceKeepAlive: false
gpd.ta.dataSize: 1048576
gpd.ta.stackSize: 8192
gpd.ta.target_type: 1
```

>![](public_sys-resources/icon-caution.gif) **注意：** 
>-   必须保证每个可信驱动程序的service\_name和appID是唯一的，其中service\_name只能由字母、数字和'\_'组成，其他无法识别。
>-   可信驱动程序生成时，如果不存在manifest.txt文件，或manifest.txt文件格式错误，生成将会终止。

### 宏定义表文件介绍<a name="section449462816414"></a>

可信驱动程序宏定义表应命名为“service\_name.csv”，例如“drv\_test\_module.csv”，放在驱动开发根目录下。

驱动开发者在配置特殊cmd权限时，需要提供宏定义表，说明驱动命令宏定义与具体数值对应关系，及驱动权限与具体数值对应关系。驱动开发者根据这个宏定义表配置特殊cmd及对应权限信息；驱动访问者根据每个可信驱动程序的宏定义表配置其需要的特殊cmd权限信息。

宏定义表内容约束见下表：

**表 5**  宏定义表内容

<a name="table17568952133112"></a>
<table><thead align="left"><tr id="row12569165218312"><th class="cellrowborder" valign="top" width="10.371037103710371%" id="mcps1.2.4.1.1"><p id="p1836159123115"><a name="p1836159123115"></a><a name="p1836159123115"></a>列编号</p>
</th>
<th class="cellrowborder" valign="top" width="20.87208720872087%" id="mcps1.2.4.1.2"><p id="p3750318473"><a name="p3750318473"></a><a name="p3750318473"></a>内容</p>
</th>
<th class="cellrowborder" valign="top" width="68.75687568756875%" id="mcps1.2.4.1.3"><p id="p18758315479"><a name="p18758315479"></a><a name="p18758315479"></a>约束</p>
</th>
</tr>
</thead>
<tbody><tr id="row55691252133119"><td class="cellrowborder" valign="top" width="10.371037103710371%" headers="mcps1.2.4.1.1 "><p id="p375123164712"><a name="p375123164712"></a><a name="p375123164712"></a>A列</p>
</td>
<td class="cellrowborder" valign="top" width="20.87208720872087%" headers="mcps1.2.4.1.2 "><p id="p1675632477"><a name="p1675632477"></a><a name="p1675632477"></a><span>驱动命令/权限宏定义</span></p>
</td>
<td class="cellrowborder" valign="top" width="68.75687568756875%" headers="mcps1.2.4.1.3 "><p id="p1376183194717"><a name="p1376183194717"></a><a name="p1376183194717"></a>必须为字符串类型，且不能为空。excel中单元格需设置为文本类型。</p>
<p id="p1576431475"><a name="p1576431475"></a><a name="p1576431475"></a>驱动命令宏命名和驱动权限宏命名需要与驱动代码逻辑保持一致。</p>
</td>
</tr>
<tr id="row165691052103119"><td class="cellrowborder" valign="top" width="10.371037103710371%" headers="mcps1.2.4.1.1 "><p id="p12761339474"><a name="p12761339474"></a><a name="p12761339474"></a>B列</p>
</td>
<td class="cellrowborder" valign="top" width="20.87208720872087%" headers="mcps1.2.4.1.2 "><p id="p2076930478"><a name="p2076930478"></a><a name="p2076930478"></a>驱动命令/权限实际数值</p>
</td>
<td class="cellrowborder" valign="top" width="68.75687568756875%" headers="mcps1.2.4.1.3 "><p id="p107633114719"><a name="p107633114719"></a><a name="p107633114719"></a>必须为字符串类型，且不能为空。excel中单元格需设置为文本类型。</p>
<p id="p0768312474"><a name="p0768312474"></a><a name="p0768312474"></a>驱动命令和驱动权限实际数字必须与驱动代码逻辑保持一致。</p>
<p id="p632422216436"><a name="p632422216436"></a><a name="p632422216436"></a>驱动命令数字使用十六进制数字，范围为0x1到0x40。</p>
<p id="p167663124712"><a name="p167663124712"></a><a name="p167663124712"></a>驱动权限数字使用十进制数字，范围为1到64。</p>
</td>
</tr>
</tbody>
</table>

宏定义表文件配置举例见下表：

**表 6**  宏定义配置表

<a name="table113201189338"></a>
<table><thead align="left"><tr id="row3321118143318"><th class="cellrowborder" valign="top" width="50%" id="mcps1.2.3.1.1"><p id="p63217833318"><a name="p63217833318"></a><a name="p63217833318"></a>A列</p>
</th>
<th class="cellrowborder" valign="top" width="50%" id="mcps1.2.3.1.2"><p id="p10321118203312"><a name="p10321118203312"></a><a name="p10321118203312"></a>B列</p>
</th>
</tr>
</thead>
<tbody><tr id="row432112823313"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p19761838472"><a name="p19761838472"></a><a name="p19761838472"></a>perm_1</p>
</td>
<td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p15767318475"><a name="p15767318475"></a><a name="p15767318475"></a>0</p>
</td>
</tr>
<tr id="row156121811153310"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p77643204719"><a name="p77643204719"></a><a name="p77643204719"></a>perm_2</p>
</td>
<td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p11765319478"><a name="p11765319478"></a><a name="p11765319478"></a>2</p>
</td>
</tr>
<tr id="row18321188163315"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p157615324712"><a name="p157615324712"></a><a name="p157615324712"></a>perm_3</p>
</td>
<td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p12768344718"><a name="p12768344718"></a><a name="p12768344718"></a>3</p>
</td>
</tr>
<tr id="row113217814338"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p13769374716"><a name="p13769374716"></a><a name="p13769374716"></a>CMD_ID_1</p>
</td>
<td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p37643134714"><a name="p37643134714"></a><a name="p37643134714"></a>0x10</p>
</td>
</tr>
<tr id="row103215817330"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p47619324718"><a name="p47619324718"></a><a name="p47619324718"></a>CMD_ID_2</p>
</td>
<td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p127615312472"><a name="p127615312472"></a><a name="p127615312472"></a>0x11</p>
</td>
</tr>
<tr id="row73211089332"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p476113194710"><a name="p476113194710"></a><a name="p476113194710"></a>CMD_ID_3</p>
</td>
<td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p37643134718"><a name="p37643134718"></a><a name="p37643134718"></a>0x12</p>
</td>
</tr>
</tbody>
</table>

### configs.xml文件介绍<a name="section9908114312413"></a>

驱动开发者如果要给驱动配置IO操作、中断操作、地址转换、内存拷贝等权限，需要配置configs.xml文件。

configs.xml文件前半部分包含manifest.txt文件中驱动属性数据，用户需要在configs.xml根字段下新增dyn\_perm字段，并将驱动权限配置部分写在该字段下。

**表 7**  configs.xml文件字段说明

<a name="table10422031435"></a>
<table><thead align="left"><tr id="row124219311934"><th class="cellrowborder" valign="top" width="7.95%" id="mcps1.2.6.1.1"><p id="p5438311135"><a name="p5438311135"></a><a name="p5438311135"></a>配置方</p>
</th>
<th class="cellrowborder" valign="top" width="18.89%" id="mcps1.2.6.1.2"><p id="p17437318313"><a name="p17437318313"></a><a name="p17437318313"></a>字段</p>
</th>
<th class="cellrowborder" valign="top" width="16.38%" id="mcps1.2.6.1.3"><p id="p19431731235"><a name="p19431731235"></a><a name="p19431731235"></a>类型</p>
</th>
<th class="cellrowborder" valign="top" width="28.23%" id="mcps1.2.6.1.4"><p id="p84311311331"><a name="p84311311331"></a><a name="p84311311331"></a>说明</p>
</th>
<th class="cellrowborder" valign="top" width="28.549999999999997%" id="mcps1.2.6.1.5"><p id="p143173115315"><a name="p143173115315"></a><a name="p143173115315"></a>约束</p>
</th>
</tr>
</thead>
<tbody><tr id="row343153113312"><td class="cellrowborder" rowspan="17" valign="top" width="7.95%" headers="mcps1.2.6.1.1 "><p id="p62445517610"><a name="p62445517610"></a><a name="p62445517610"></a>驱动开发者</p>
</td>
<td class="cellrowborder" valign="top" width="18.89%" headers="mcps1.2.6.1.2 "><p id="p2042482191012"><a name="p2042482191012"></a><a name="p2042482191012"></a>drv_perm.drv_io_map.item.iomap</p>
</td>
<td class="cellrowborder" valign="top" width="16.38%" headers="mcps1.2.6.1.3 "><p id="p3424321201019"><a name="p3424321201019"></a><a name="p3424321201019"></a>由地址构成的list</p>
</td>
<td class="cellrowborder" valign="top" width="28.23%" headers="mcps1.2.6.1.4 "><p id="p825333031112"><a name="p825333031112"></a><a name="p825333031112"></a>驱动可以io映射的地址区间。格式为”start1,end1;start2,end2......”，取值范围为[0,0xffffffffffffffff]。</p>
<p id="p13424521131011"><a name="p13424521131011"></a><a name="p13424521131011"></a>start不能大于等于end。</p>
<p id="p34241721171016"><a name="p34241721171016"></a><a name="p34241721171016"></a>未设置表示不能进行IO映射。</p>
</td>
<td class="cellrowborder" valign="top" width="28.549999999999997%" headers="mcps1.2.6.1.5 "><p id="p742472181013"><a name="p742472181013"></a><a name="p742472181013"></a>地址必须为十六进制，否则会解析错误。字符仅能包含'0'-'9'，'a'-'z'，'A'-'Z'以及半角';'和','。</p>
</td>
</tr>
<tr id="row12435311036"><td class="cellrowborder" valign="top" headers="mcps1.2.6.1.1 "><p id="p1022415117511"><a name="p1022415117511"></a><a name="p1022415117511"></a>drv_perm.drv_io_map.item.chip_type</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.6.1.2 "><p id="p42241011155117"><a name="p42241011155117"></a><a name="p42241011155117"></a>由平台名称构成的list</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.6.1.3 "><p id="p8224911125120"><a name="p8224911125120"></a><a name="p8224911125120"></a>驱动映射io地址区间的平台属性配置。格式为“平台名称1,平台名称2,......”。</p>
<p id="p15136227185318"><a name="p15136227185318"></a><a name="p15136227185318"></a>未设置表示所有平台均能映射iomap字段指定的io地址范围。</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.6.1.4 "><p id="p2646225153114"><a name="p2646225153114"></a><a name="p2646225153114"></a>平台名称长度不能超过31，如果要设置chip_type，则属性的内容不能为空。</p>
<p id="p1522471165118"><a name="p1522471165118"></a><a name="p1522471165118"></a>字符仅能包含'0'-'9'，'a'-'z'，'A'-'Z'以及半角'_'。</p>
</td>
</tr>
<tr id="row104320311039"><td class="cellrowborder" valign="top" headers="mcps1.2.6.1.1 "><p id="p204971279442"><a name="p204971279442"></a><a name="p204971279442"></a>drv_perm.drv_basic_info.exception_mode（暂未实现）</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.6.1.2 "><p id="p44977711440"><a name="p44977711440"></a><a name="p44977711440"></a>字符串</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.6.1.3 "><p id="p2497167144414"><a name="p2497167144414"></a><a name="p2497167144414"></a>支持ddos, syscrash和restart。其中ddos表示驱动崩溃后不再重启，syscrash表示驱动崩溃后整机崩溃，restart表示驱动崩溃后驱动重启。</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.6.1.4 "><p id="p1149719724414"><a name="p1149719724414"></a><a name="p1149719724414"></a>只支持ddos、syscrash和restart三种字符串输入。</p>
</td>
</tr>
<tr id="row34318311138"><td class="cellrowborder" valign="top" headers="mcps1.2.6.1.1 "><p id="p144241216104"><a name="p144241216104"></a><a name="p144241216104"></a>drv_perm.drv_basic_info.thread_limit</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.6.1.2 "><p id="p54247212107"><a name="p54247212107"></a><a name="p54247212107"></a>uint32_t</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.6.1.3 "><p id="p15424122113106"><a name="p15424122113106"></a><a name="p15424122113106"></a>驱动可以最大开设的线程数量，未设置默认为1个。</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.6.1.4 "><p id="p24241321111014"><a name="p24241321111014"></a><a name="p24241321111014"></a>最多8个线程。必须为一个整数。</p>
</td>
</tr>
<tr id="row7432031836"><td class="cellrowborder" valign="top" headers="mcps1.2.6.1.1 "><p id="p144242215105"><a name="p144242215105"></a><a name="p144242215105"></a>drv_perm.drv_basic_info.upgrade</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.6.1.2 "><p id="p134245213104"><a name="p134245213104"></a><a name="p134245213104"></a>bool</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.6.1.3 "><p id="p2424921111016"><a name="p2424921111016"></a><a name="p2424921111016"></a>驱动是否可以升级，未设置默认不支持升级。</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.6.1.4 "><p id="p188841324141714"><a name="p188841324141714"></a><a name="p188841324141714"></a>配置需要为true或者false（大小写均可），其他均为非法输入。</p>
</td>
</tr>
<tr id="row74320313315"><td class="cellrowborder" valign="top" headers="mcps1.2.6.1.1 "><p id="p1484875513435"><a name="p1484875513435"></a><a name="p1484875513435"></a>drv_perm.drv_basic_info.virt2phys</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.6.1.2 "><p id="p7374135516423"><a name="p7374135516423"></a><a name="p7374135516423"></a>bool</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.6.1.3 "><p id="p1937445534213"><a name="p1937445534213"></a><a name="p1937445534213"></a>表示是否允许驱动将虚拟地址转换成物理地址。</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.6.1.4 "><p id="p23741055184218"><a name="p23741055184218"></a><a name="p23741055184218"></a>配置需要为true或者false（大小写均可），其他均为非法输入。</p>
</td>
</tr>
<tr id="row14441931138"><td class="cellrowborder" valign="top" headers="mcps1.2.6.1.1 "><p id="p1115413486"><a name="p1115413486"></a><a name="p1115413486"></a>drv_perm.irq.item.irq</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.6.1.2 "><p id="p4110412482"><a name="p4110412482"></a><a name="p4110412482"></a>uint64_t</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.6.1.3 "><p id="p1311541134817"><a name="p1311541134817"></a><a name="p1311541134817"></a>本驱动注册的中断号。</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.6.1.4 "><p id="p41194113484"><a name="p41194113484"></a><a name="p41194113484"></a>一个或多个大于等于32的十进制整数，使用','分割。字符仅能包含'0'-'9'以及半角','。</p>
</td>
</tr>
<tr id="row118081325553"><td class="cellrowborder" valign="top" headers="mcps1.2.6.1.1 "><p id="p9704135395013"><a name="p9704135395013"></a><a name="p9704135395013"></a>drv_perm.irq.item.chip_type</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.6.1.2 "><p id="p17601667514"><a name="p17601667514"></a><a name="p17601667514"></a>由平台名称构成的list</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.6.1.3 "><p id="p47041534507"><a name="p47041534507"></a><a name="p47041534507"></a>同drv_perm.drv_io_map.item.chip_type。</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.6.1.4 "><p id="p1470465318505"><a name="p1470465318505"></a><a name="p1470465318505"></a>同drv_perm.drv_io_map.item.chip_type。</p>
</td>
</tr>
<tr id="row171761226959"><td class="cellrowborder" valign="top" headers="mcps1.2.6.1.1 "><p id="p1709142425118"><a name="p1709142425118"></a><a name="p1709142425118"></a>drv_perm.map_nosecure.item.uuid</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.6.1.2 "><p id="p1570915246515"><a name="p1570915246515"></a><a name="p1570915246515"></a>uuid list</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.6.1.3 "><p id="p13709142413516"><a name="p13709142413516"></a><a name="p13709142413516"></a>表示该驱动允许指定uuid映射非安全内存。</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.6.1.4 "><p id="p8905173581714"><a name="p8905173581714"></a><a name="p8905173581714"></a>仅支持输入合法的uuid序列，使用','分割。</p>
<p id="p11709132415516"><a name="p11709132415516"></a><a name="p11709132415516"></a>不允许重复设置相同uuid。</p>
</td>
</tr>
<tr id="row8338182611519"><td class="cellrowborder" valign="top" headers="mcps1.2.6.1.1 "><p id="p10446981534"><a name="p10446981534"></a><a name="p10446981534"></a>drv_perm.map_nosecure.item.chip_type</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.6.1.2 "><p id="p194461875315"><a name="p194461875315"></a><a name="p194461875315"></a>由平台名称构成的list</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.6.1.3 "><p id="p7446687537"><a name="p7446687537"></a><a name="p7446687537"></a>同drv_perm.drv_io_map.item.chip_type。</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.6.1.4 "><p id="p164467815533"><a name="p164467815533"></a><a name="p164467815533"></a>同drv_perm.drv_io_map.item.chip_type。</p>
</td>
</tr>
<tr id="row0506162613513"><td class="cellrowborder" valign="top" headers="mcps1.2.6.1.1 "><p id="p577618301537"><a name="p577618301537"></a><a name="p577618301537"></a>drv_perm.map_secure.item.uuid</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.6.1.2 "><p id="p7776630205320"><a name="p7776630205320"></a><a name="p7776630205320"></a>uuid</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.6.1.3 "><p id="p3776203075319"><a name="p3776203075319"></a><a name="p3776203075319"></a>表示该驱动允许指定uuid映射安全内存。</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.6.1.4 "><p id="p1455012691819"><a name="p1455012691819"></a><a name="p1455012691819"></a>仅支持输入合法的uuid序列，使用','分割。</p>
<p id="p53271810151111"><a name="p53271810151111"></a><a name="p53271810151111"></a>不允许重复设置相同uuid。</p>
<p id="p13741234103010"><a name="p13741234103010"></a><a name="p13741234103010"></a>该字段必须被配置。</p>
</td>
</tr>
<tr id="row26551626256"><td class="cellrowborder" valign="top" headers="mcps1.2.6.1.1 "><p id="p148843465543"><a name="p148843465543"></a><a name="p148843465543"></a>drv_perm.map_secure.item.region</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.6.1.2 "><p id="p7884174635410"><a name="p7884174635410"></a><a name="p7884174635410"></a>由地址构成的list</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.6.1.3 "><p id="p1121372771118"><a name="p1121372771118"></a><a name="p1121372771118"></a>驱动可以映射的安全内存地址区间，格式为”start1,end1;start2,end2……”，取值范围为[0,0xffffffffffffffff]。</p>
<p id="p1920034216308"><a name="p1920034216308"></a><a name="p1920034216308"></a>start不能大于等于end。</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.6.1.4 "><p id="p188947143116"><a name="p188947143116"></a><a name="p188947143116"></a>地址必须为16进制，否则会解析错误。字符仅能包含'0'-'9'，'a'-'z'，'A'-'Z'以及半角';'和','</p>
<p id="p1512052335611"><a name="p1512052335611"></a><a name="p1512052335611"></a>该字段必须被配置。</p>
</td>
</tr>
<tr id="row108425262520"><td class="cellrowborder" valign="top" headers="mcps1.2.6.1.1 "><p id="p2104143785614"><a name="p2104143785614"></a><a name="p2104143785614"></a>drv_perm.map_secure.item.chip_type</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.6.1.2 "><p id="p9693164811564"><a name="p9693164811564"></a><a name="p9693164811564"></a>由平台名称构成的list</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.6.1.3 "><p id="p13104143715615"><a name="p13104143715615"></a><a name="p13104143715615"></a>同drv_perm.drv_io_map.item.chip_type。</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.6.1.4 "><p id="p13104437145613"><a name="p13104437145613"></a><a name="p13104437145613"></a>同drv_perm.drv_io_map.item.chip_type。</p>
</td>
</tr>
<tr id="row179928261357"><td class="cellrowborder" valign="top" headers="mcps1.2.6.1.1 "><p id="p2042472111020"><a name="p2042472111020"></a><a name="p2042472111020"></a>drv_perm.drv_cmd_perm_info.item.cmd</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.6.1.2 "><p id="p942412218104"><a name="p942412218104"></a><a name="p942412218104"></a>cmd宏定义</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.6.1.3 "><p id="p164241121101017"><a name="p164241121101017"></a><a name="p164241121101017"></a>驱动需要特殊权限配置的cmd，与下文drv_conf.drv_cmd_perm_info.item.permission配对使用（没有配的cmd表示只要驱动访问者配置了访问此驱动的名字便能被访问）。</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.6.1.4 "><p id="p442414213109"><a name="p442414213109"></a><a name="p442414213109"></a>cmd必须为驱动宏定义表中可以翻译的字符串，否则会导致权限注册失败。</p>
<p id="p9424321181016"><a name="p9424321181016"></a><a name="p9424321181016"></a><span>在同一驱动的</span><span>drv_cmd_perm_info</span><span>段中，同一个</span><span>cmd</span><span>只允许被配置一个权限，多次配置只有第一次有效。</span></p>
</td>
</tr>
<tr id="row63786431255"><td class="cellrowborder" valign="top" headers="mcps1.2.6.1.1 "><p id="p134257218108"><a name="p134257218108"></a><a name="p134257218108"></a>drv_perm.drv_cmd_perm_info.item.permission</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.6.1.2 "><p id="p154257217109"><a name="p154257217109"></a><a name="p154257217109"></a>Permission宏定义</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.6.1.3 "><p id="p1842562110106"><a name="p1842562110106"></a><a name="p1842562110106"></a>与上文drv_conf.drv_cmd_perm_info.item.cmd对应，表示此特殊cmd对应的权限（驱动访问者必须配置此权限才能访问该权限对应的cmd）。</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.6.1.4 "><p id="p1142542114106"><a name="p1142542114106"></a><a name="p1142542114106"></a>permssion字符串必须为驱动宏定义表中可以翻译的字符串，否则会导致权限注册失败。</p>
<p id="p442512212104"><a name="p442512212104"></a><a name="p442512212104"></a>permission配置项与cmd配置项一一对应，必须同时配置。</p>
</td>
</tr>
<tr id="row352912431855"><td class="cellrowborder" valign="top" headers="mcps1.2.6.1.1 "><p id="p184252021191016"><a name="p184252021191016"></a><a name="p184252021191016"></a>drv_perm.drv_mac_info.uuid</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.6.1.2 "><p id="p8425721121016"><a name="p8425721121016"></a><a name="p8425721121016"></a>uuid</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.6.1.3 "><p id="p6425921101011"><a name="p6425921101011"></a><a name="p6425921101011"></a>驱动强制限制能够访问自身的访问者uuid信息（在此情况下，只有配置该驱动访问权限且在此白名单内的访问者才能访问该驱动）。</p>
<p id="p14425162111103"><a name="p14425162111103"></a><a name="p14425162111103"></a>未设置默认只要访问者配置此驱动权限便能访问此驱动。</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.6.1.4 "><p id="p742517210105"><a name="p742517210105"></a><a name="p742517210105"></a><span>在同一驱动的</span><span>drv_mac _info</span><span>段中，同一个</span><span>uuid</span><span>只允许被配置一次。多次配置只有第一次有效。</span></p>
</td>
</tr>
<tr id="row16667433519"><td class="cellrowborder" valign="top" headers="mcps1.2.6.1.1 "><p id="p3425192181017"><a name="p3425192181017"></a><a name="p3425192181017"></a>drv_perm.drv_mac_info.permission</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.6.1.2 "><p id="p342552111102"><a name="p342552111102"></a><a name="p342552111102"></a>Permission宏定义</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.6.1.3 "><p id="p4425152111018"><a name="p4425152111018"></a><a name="p4425152111018"></a>与上文drv_conf.drv_mac_info.uuid配合使用，驱动强制限制能够访问自身的访问者uuid，同时约束此uuid访问者拥有的特殊cmd权限（此特殊权限对应drv_conf.drv_cmd_perm_info里有特殊权限诉求的cmd信息，可以是多个cmd权限，未设置时默认值为0，多个cmd权限由'|'字符隔开即可）。</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.6.1.4 "><p id="p1842532114106"><a name="p1842532114106"></a><a name="p1842532114106"></a>permssion字符串必须为驱动宏定义表中可以翻译的字符串，否则会导致权限注册失败。</p>
<p id="p442532181018"><a name="p442532181018"></a><a name="p442532181018"></a>此配置项依赖上文uuid，配置此配置项时，必须要先配置uuid项。</p>
</td>
</tr>
<tr id="row4822164318514"><td class="cellrowborder" rowspan="2" valign="top" width="7.95%" headers="mcps1.2.6.1.1 "><p id="p4292133717619"><a name="p4292133717619"></a><a name="p4292133717619"></a>驱动访问者</p>
</td>
<td class="cellrowborder" valign="top" width="18.89%" headers="mcps1.2.6.1.2 "><p id="p1442552114105"><a name="p1442552114105"></a><a name="p1442552114105"></a>drv_perm.drvcall_perm_apply.item.name</p>
</td>
<td class="cellrowborder" valign="top" width="16.38%" headers="mcps1.2.6.1.3 "><p id="p342511217106"><a name="p342511217106"></a><a name="p342511217106"></a>字符串</p>
</td>
<td class="cellrowborder" valign="top" width="28.23%" headers="mcps1.2.6.1.4 "><p id="p84251021121010"><a name="p84251021121010"></a><a name="p84251021121010"></a>驱动访问者要访问驱动的名称，该名称与打开驱动时传入驱动名称一致。必须配置，否则驱动访问者在打开时会被校验失败。</p>
</td>
<td class="cellrowborder" valign="top" width="28.549999999999997%" headers="mcps1.2.6.1.5 "><p id="p34251221171011"><a name="p34251221171011"></a><a name="p34251221171011"></a>字符串有效长度小于32字节。</p>
<p id="p9425122191015"><a name="p9425122191015"></a><a name="p9425122191015"></a><span>在同一驱动访问者的</span><span>drvcall_perm_apply</span><span>段中，同一个</span><span>service_name</span><span>只允许被配置一次。多次配置只有第一次有效</span>。</p>
</td>
</tr>
<tr id="row9116744054"><td class="cellrowborder" valign="top" headers="mcps1.2.6.1.1 "><p id="p94251921171017"><a name="p94251921171017"></a><a name="p94251921171017"></a>drv_perm.drvcall_perm_apply.item.permission</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.6.1.2 "><p id="p20425182112103"><a name="p20425182112103"></a><a name="p20425182112103"></a>Permission宏定义</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.6.1.3 "><p id="p1242562111012"><a name="p1242562111012"></a><a name="p1242562111012"></a>驱动访问者要访问item.name驱动的具体权限（此特殊权限对应drv_conf.drv_cmd_perm_info里有特殊权限诉求的cmd信息，可以是多个cmd权限，未设置时默认值为0）。</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.6.1.4 "><p id="p44254213106"><a name="p44254213106"></a><a name="p44254213106"></a>此配置项依赖item.name，配置此配置项时，必须要先配置item.name。</p>
</td>
</tr>
</tbody>
</table>

configs.xml文件举例如下：

```
<ConfigInfo>
  <TA_Basic_Info>
    <service_name>drv_test_module</service_name>
    <uuid>00002222-3333-4444-5555-666666667777</uuid>
  </TA_Basic_Info>
  <TA_Manifest_Info>
    <instance_keep_alive>true</instance_keep_alive>
    <stack_size>64000</stack_size>
    <heap_size>3485760</heap_size>
    <target_type>1</target_type>
    <multi_command>false</multi_command>
    <multi_session>true</multi_session>
    <single_instance>true</single_instance>
  </TA_Manifest_Info>
  <drv_perm>
    <drv_basic_info thread_limit="3" upgrade="true" exception_mode="ddos" virt2phys="true" />
    <drv_io_map>
      <item chip_type="chip_type1,chip_type2" iomap="0x30000, 0x31000" />
      <item iomap="0x90000, 0x91000; 0xa0000, 0xa1000" />
    </drv_io_map>
    <irq>
        <item chip_type="chip_type1,chip_type2" irq="103" />
        <item irq="32, 1000" />
    </irq>
    <map_secure>
      <item chip_type="chip_type1,chip_type2" uuid="03030303-0303-0303-0303-030303030303" region="0x12340000, 0x12350000"/>
      <item uuid="04040404-0404-0101-0101-040401010101" region="0x45670000, 0x789a0000; 0x22330000, 0x44550000"/>
    </map_secure>
    <map_nosecure>
      <item chip_type="chip_type1,chip_type2" uuid="01010101-0101-0101-0101-010101010101"/>
      <item uuid="03030303-0303-0101-0101-030303030303,
                  04040404-0404-0101-0101-040401010101,
                  06060606-0606-0606-0101-050501010101"/>
    </map_nosecure>
    <drv_cmd_perm_info>
      <item cmd="IOMAP_TEST_ID" permission="iomap_perm"/>
    </drv_cmd_perm_info>
    <drv_mac_info>
      <item uuid="03030303-0303-0101-0101-030303030303" permission="iomap_perm"/>
      <item uuid="04040404-0404-0101-0101-040401010101"/>
    </drv_mac_info>
    <drvcall_perm_apply>
      <item name="drv1_name" permission="iomap_perm"/>
      <item name="drv2_name"/>
    </drvcall_perm_apply>
  </drv_perm>
</ConfigInfo>
```

**表 7**  示例configs.xml文件说明

<a></a>
<table><thead align="left"><tr><th class="cellrowborder" valign="top" width="50%" id="mcps1.2.3.1.1"><p>输出</p>
</th>
<th class="cellrowborder" valign="top" width="50%"><p>用途</p>
</th>
</tr>
</thead>
<tbody><tr><td class="cellrowborder" valign="top" width="50%"><p>drv_basic_info</p>
</td>
<td class="cellrowborder" valign="top" width="50%"><p>驱动运行模式是最多支持三个线程并发，支持动态升级，该驱动崩溃后不会重启，支持地址转换接口调用。</p>
</td>
</tr>
<tr><td class="cellrowborder" valign="top" width="50%"><p>drv_io_map</p>
</td>
<td class="cellrowborder" valign="top" width="50%"><p>驱动在chip_type1和chip_type2平台上支持映射[0x30000, 0x31000]段的io地址空间；驱动在所有平台上都支持映射[0x90000, 0x91000]和[0xa0000, 0xa1000]段的io地址空间。</p>
</td>
</tr>
<tr><td class="cellrowborder" valign="top" width="50%"><p>irq</p>
</td>
<td class="cellrowborder" valign="top" width="50%"><p>驱动在chip_type1和chip_type2平台上的irq num为103；驱动在所有平台上面的irq num为32和1000。</p>
</td>
</tr>
<tr><td class="cellrowborder" valign="top" width="50%"><p>map_secure</p>
</td>
<td class="cellrowborder" valign="top" width="50%"><p>驱动在chip_type1和chip_type2支持uuid为030303的驱动访问者映射[0x12340000，0x12350000]的安全地址范围；驱动在所有平台上都支持uuid为04040404的驱动访问者映射[0x45670000,0x789a0000]和[0x22330000,0x44550000]两段安全地址范围。</p>
</td>
</tr>
<tr><td class="cellrowborder" valign="top" width="50%"><p>map_nosecure</p>
</td>
<td class="cellrowborder" valign="top" width="50%"><p>驱动在chip_type1和chip_type2支持uuid为010101的驱动访问者映射非安全内存地址；驱动在所有平台上都支持uuid为030303、040404和060606的驱动访问者映射非安全内存地址。</p>
</td>
</tr>
<tr><td class="cellrowborder" valign="top" width="50%"><p>drv_cmd_perm_info</p>
</td>
<td class="cellrowborder" valign="top" width="50%"><p>驱动配置拥有特殊权限的命令，IOMAP_TEST_ID命令具有iomap_perm特殊权限。</p>
</td>
</tr>
<tr><td class="cellrowborder" valign="top" width="50%"><p>drv_mac_info</p>
</td>
<td class="cellrowborder" valign="top" width="50%"><p>驱动强制配置能访问自身的驱动访问者uuid和特殊权限。uuid为03030303的驱动访问者除了拥有驱动基础权限外，还拥有特殊权限iomap_perm，可以访问驱动基础命令和iomap_perm对应的IOMAP_TEST_ID命令；uuid为04040404的驱动访问者没有配置特殊权限，即只能访问驱动基础命令，无权限访问iomap_perm对应的IOMAP_TEST_ID命令。</p>
</td>
</tr>
<tr><td class="cellrowborder" valign="top" width="50%"><p>drvcall_perm_apply</p>
</td>
<td class="cellrowborder" valign="top" width="50%"><p>本驱动作为驱动访问者，申请访问名为drv1_name的驱动并注册特殊权限iomap_perm，申请访问名为drv2_name的驱动并不另外注册特殊权限。</p>
</td>
</tr>
</tbody>
</table>

>![](public_sys-resources/icon-caution.gif) **注意：** 
>-   驱动访问者可以是TA或驱动。
>-   驱动访问者如果想访问驱动，需要在驱动访问者的configs.xml文件中配置drvcall\_perm\_apply字段及需要的特殊权限。
>-   被访问驱动的configs.xml文件如果配置drv\_mac\_info字段的驱动访问者的uuid和特殊权限，会强制约束只有uuid在drv\_mac\_info字段配置的驱动访问者才能访问它，并根据drv\_mac\_info字段强制约束驱动访问者的特殊权限；驱动是否配置drv\_mac\_info字段进行强制约束，由驱动业务自行决定。

## 签名文件生成<a name="section793382693010"></a>

进行安全业务开发前，需要进行密钥、证书申请及签发perm\_config文件，perm\_config文件用于管控驱动在TEE中可用资源信息。

### 密钥及证书生成<a name="section1046723143114"></a>

生成密钥及证书的工具位于build/keytools目录下。config签名及sec文件签名时使用的签名密钥、相关证书都由该工具本地生成。

在使用keytools工具前，需要对build/keytools/input/profile.ini进行配置，各字段含义如下表所示：

**表 8**  keytools工具的字段配置说明

<a name="table17235151720215"></a>
<table><thead align="left"><tr id="row1823521714214"><th class="cellrowborder" valign="top" width="14.87%" id="mcps1.2.5.1.1"><p id="p138501929726"><a name="p138501929726"></a><a name="p138501929726"></a>字段</p>
</th>
<th class="cellrowborder" valign="top" width="17.39%" id="mcps1.2.5.1.2"><p id="p1085018291125"><a name="p1085018291125"></a><a name="p1085018291125"></a>取值</p>
</th>
<th class="cellrowborder" valign="top" width="25.490000000000002%" id="mcps1.2.5.1.3"><p id="p18850112914212"><a name="p18850112914212"></a><a name="p18850112914212"></a>含义</p>
</th>
<th class="cellrowborder" valign="top" width="42.25%" id="mcps1.2.5.1.4"><p id="p485013295211"><a name="p485013295211"></a><a name="p485013295211"></a>说明</p>
</th>
</tr>
</thead>
<tbody><tr id="row793213366231"><td class="cellrowborder" valign="top" width="14.87%" headers="mcps1.2.5.1.1 "><p id="p2812173519210"><a name="p2812173519210"></a><a name="p2812173519210"></a>CAType</p>
</td>
<td class="cellrowborder" valign="top" width="17.39%" headers="mcps1.2.5.1.2 "><p id="p2812143519212"><a name="p2812143519212"></a><a name="p2812143519212"></a>1</p>
</td>
<td class="cellrowborder" valign="top" width="25.490000000000002%" headers="mcps1.2.5.1.3 "><p id="p1381219351729"><a name="p1381219351729"></a><a name="p1381219351729"></a>Private CA</p>
</td>
<td class="cellrowborder" valign="top" width="42.25%" headers="mcps1.2.5.1.4 "><p id="p1081212357213"><a name="p1081212357213"></a><a name="p1081212357213"></a>不互通场景下使用该CA，驱动需要使用该CA。</p>
</td>
</tr>
<tr id="row923519178210"><td class="cellrowborder" rowspan="2" valign="top" width="14.87%" headers="mcps1.2.5.1.1 "><p id="p1329917127317"><a name="p1329917127317"></a><a name="p1329917127317"></a>CAAlg</p>
</td>
<td class="cellrowborder" valign="top" width="17.39%" headers="mcps1.2.5.1.2 "><p id="p629916121031"><a name="p629916121031"></a><a name="p629916121031"></a>0</p>
</td>
<td class="cellrowborder" valign="top" width="25.490000000000002%" headers="mcps1.2.5.1.3 "><p id="p129918128312"><a name="p129918128312"></a><a name="p129918128312"></a>RSA</p>
</td>
<td class="cellrowborder" rowspan="2" valign="top" width="42.25%" headers="mcps1.2.5.1.4 "><p id="p1229921218310"><a name="p1229921218310"></a><a name="p1229921218310"></a>生成密钥、证书及使用CA的算法类型。</p>
</td>
</tr>
<tr id="row22369179212"><td class="cellrowborder" valign="top" headers="mcps1.2.5.1.1 "><p id="p13299131219317"><a name="p13299131219317"></a><a name="p13299131219317"></a>1</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.5.1.2 "><p id="p1729911121137"><a name="p1729911121137"></a><a name="p1729911121137"></a>ECDSA</p>
</td>
</tr>
<tr id="row202368171026"><td class="cellrowborder" valign="top" width="14.87%" headers="mcps1.2.5.1.1 "><p id="p202949391234"><a name="p202949391234"></a><a name="p202949391234"></a>certType</p>
</td>
<td class="cellrowborder" valign="top" width="17.39%" headers="mcps1.2.5.1.2 "><p id="p1429911121938"><a name="p1429911121938"></a><a name="p1429911121938"></a>1</p>
</td>
<td class="cellrowborder" valign="top" width="25.490000000000002%" headers="mcps1.2.5.1.3 "><p id="p62991012835"><a name="p62991012835"></a><a name="p62991012835"></a>release cert</p>
</td>
<td class="cellrowborder" valign="top" width="42.25%" headers="mcps1.2.5.1.4 "><p id="p2319812439"><a name="p2319812439"></a><a name="p2319812439"></a>生成的taconfig.der/ta_cert.der的证书类</p>
</td>
</tr>
<tr id="row112368171625"><td class="cellrowborder" rowspan="2" valign="top" width="14.87%" headers="mcps1.2.5.1.1 "><p id="p152999123317"><a name="p152999123317"></a><a name="p152999123317"></a>secSignKeyLen</p>
</td>
<td class="cellrowborder" valign="top" width="17.39%" headers="mcps1.2.5.1.2 "><p id="p182995123310"><a name="p182995123310"></a><a name="p182995123310"></a>256</p>
</td>
<td class="cellrowborder" valign="top" width="25.490000000000002%" headers="mcps1.2.5.1.3 "><p id="p42991121536"><a name="p42991121536"></a><a name="p42991121536"></a>ECDSA</p>
</td>
<td class="cellrowborder" rowspan="2" valign="top" width="42.25%" headers="mcps1.2.5.1.4 "><p id="p20299412738"><a name="p20299412738"></a><a name="p20299412738"></a>生成的用于SEC签名的密钥sign_sec_priv.pem的长度。</p>
</td>
</tr>
<tr id="row199444214216"><td class="cellrowborder" valign="top" headers="mcps1.2.5.1.1 "><p id="p182991712934"><a name="p182991712934"></a><a name="p182991712934"></a>4096/2048</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.5.1.2 "><p id="p1029914121938"><a name="p1029914121938"></a><a name="p1029914121938"></a>RSA</p>
</td>
</tr>
<tr id="row1258615221025"><td class="cellrowborder" valign="top" width="14.87%" headers="mcps1.2.5.1.1 "><p id="p182992128316"><a name="p182992128316"></a><a name="p182992128316"></a>CN</p>
</td>
<td class="cellrowborder" valign="top" width="17.39%" headers="mcps1.2.5.1.2 "><p id="p1329911121434"><a name="p1329911121434"></a><a name="p1329911121434"></a>uuid_service name</p>
</td>
<td class="cellrowborder" valign="top" width="25.490000000000002%" headers="mcps1.2.5.1.3 "><p id="p2300012333"><a name="p2300012333"></a><a name="p2300012333"></a>ta_cert.der证书subject CN</p>
</td>
<td class="cellrowborder" valign="top" width="42.25%" headers="mcps1.2.5.1.4 "><p id="p153001712335"><a name="p153001712335"></a><a name="p153001712335"></a>驱动的uuid与service name，中间使用'_'连接，与驱动的manifest.txt、configs.xml保持一致。</p>
</td>
</tr>
<tr id="row83069238212"><td class="cellrowborder" valign="top" width="14.87%" headers="mcps1.2.5.1.1 "><p id="p16300101216312"><a name="p16300101216312"></a><a name="p16300101216312"></a>OU</p>
</td>
<td class="cellrowborder" valign="top" width="17.39%" headers="mcps1.2.5.1.2 "><p id="p173001012232"><a name="p173001012232"></a><a name="p173001012232"></a>业务设置</p>
</td>
<td class="cellrowborder" valign="top" width="25.490000000000002%" headers="mcps1.2.5.1.3 "><p id="p4300101212312"><a name="p4300101212312"></a><a name="p4300101212312"></a>ta_cert.der证书subject OU</p>
</td>
<td class="cellrowborder" valign="top" width="42.25%" headers="mcps1.2.5.1.4 "><p id="p143006124319"><a name="p143006124319"></a><a name="p143006124319"></a>需要与certType设置的证书类型一致，release证书需要包含关键次Production，各个词语之间使用'_'连接。</p>
</td>
</tr>
<tr id="row9565265318"><td class="cellrowborder" valign="top" width="14.87%" headers="mcps1.2.5.1.1 "><p id="p830015123317"><a name="p830015123317"></a><a name="p830015123317"></a>pass</p>
</td>
<td class="cellrowborder" valign="top" width="17.39%" headers="mcps1.2.5.1.2 "><p id="p133001812837"><a name="p133001812837"></a><a name="p133001812837"></a>业务设置</p>
</td>
<td class="cellrowborder" valign="top" width="25.490000000000002%" headers="mcps1.2.5.1.3 "><p id="p1330081218311"><a name="p1330081218311"></a><a name="p1330081218311"></a>用于SEC文件签名密钥sign_sec_priv.pem的口令</p>
</td>
<td class="cellrowborder" valign="top" width="42.25%" headers="mcps1.2.5.1.4 "><p id="p1130015122319"><a name="p1130015122319"></a><a name="p1130015122319"></a>SEC文件签名步骤需要输入该口令。</p>
</td>
</tr>
</tbody>
</table>

配置文件按实际情况填好后，运行如下指令

```
./gen_key_cert.sh
```

keytools工具输出产物将放于build/keytools/output目录下，输出及用途如下表所示：

**表 9**  keytools工具输出说明

<a name="table11862185716131"></a>
<table><thead align="left"><tr id="row586285719132"><th class="cellrowborder" valign="top" width="50%" id="mcps1.2.3.1.1"><p id="p132517171145"><a name="p132517171145"></a><a name="p132517171145"></a>输出</p>
</th>
<th class="cellrowborder" valign="top" width="50%" id="mcps1.2.3.1.2"><p id="p132511711143"><a name="p132511711143"></a><a name="p132511711143"></a>用途</p>
</th>
</tr>
</thead>
<tbody><tr id="row158621857171317"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p82511741416"><a name="p82511741416"></a><a name="p82511741416"></a>taconfig.der</p>
</td>
<td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p1125171711148"><a name="p1125171711148"></a><a name="p1125171711148"></a>Config证书，打包在perm_config中，所有驱动共用一份。</p>
</td>
</tr>
<tr id="row2862757121314"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p1325017191412"><a name="p1325017191412"></a><a name="p1325017191412"></a>taconfig_key.pem</p>
</td>
<td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p925171771412"><a name="p925171771412"></a><a name="p925171771412"></a>perm_config签名使用的密钥，口令默认为123456。</p>
</td>
</tr>
<tr id="row188626575132"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p112516175142"><a name="p112516175142"></a><a name="p112516175142"></a>ta_cert.der</p>
</td>
<td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p72515179141"><a name="p72515179141"></a><a name="p72515179141"></a>驱动证书，作为驱动身份的唯一标示，打包在perm_config中。</p>
</td>
</tr>
<tr id="row6862135719134"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p162581721414"><a name="p162581721414"></a><a name="p162581721414"></a>ta_cert.csr</p>
</td>
<td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p10251017141414"><a name="p10251017141414"></a><a name="p10251017141414"></a>驱动证书对应的请求文件。</p>
</td>
</tr>
<tr id="row3862115716132"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p17252176149"><a name="p17252176149"></a><a name="p17252176149"></a>sign_sec_priv.pem</p>
</td>
<td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p122518177143"><a name="p122518177143"></a><a name="p122518177143"></a>SEC签名使用的密钥。</p>
</td>
</tr>
</tbody>
</table>

>![](public_sys-resources/icon-notice.gif) **须知：** 
>keytools工具颁发的证书及生成的密钥均只能用于开发调试，不可商用。

### perm\_config文件生产<a name="section11327454193517"></a>

将keytools工具生成的密钥、证书拷贝到相应的目录下：

```
cp keytools/output/taconfig.der pack-Config/config_cert/.
cp keytools/output/taconfig_key.pem pack-Config/config_cert/.
cp keytools/output/ta_cert.der pack-Config/ta_cert/.
```

根据驱动实际情况修改build/pack-Config/input目录下的configs.xml，修改完成后在build/pack-Config目录下执行如下指令，密钥口令为123456：

```
python3 signtool_config.py ./input ./ta_cert/ta_cert.der 2 ./output/perm_config
```

在build/keytools/pack-Config/output目录下得到签名产物perm\_config。

## 驱动开发框架<a name="section12432132218372"></a>

驱动开发整体可以分为两部分，第一个是驱动业务开发框架，第二个是访问该驱动的驱动访问者框架。

### 驱动业务框架<a name="section6705194712373"></a>

为方便各个驱动开发统一，TEE可信执行环境子系统为各个驱动设计了一套基础框架，如下所示，驱动开发者结合驱动实际逻辑分别定义这些基础函数即可。
```
#define DRV_NAME_MAX_LEN 32U
#define DRV_RESERVED_NUM 8U

struct drv_data {
    int32_t fd; /* unique label which alloced by driver framework */
    uint32_t taskid; /* caller taskid */
    void *private_data; /* the private data associated with this fd */
    struct tee_uuid uuid; /* caller uuid */
};

typedef int32_t (*init_func)(void);

typedef int32_t (*suspned_func)(void);
typedef int32_t (*resume_func)(void);

typedef int64_t (*ioctl_func)(struct drv_data *drv, uint32_t cmd, unsigned long args, uint32_t args_len);
typedef int64_t (*open_func)(struct drv_data *drv, unsigned long args, uint32_t args_len);
typedef int64_t (*close_func)(struct drv_data *drv);

struct tee_driver_module {
    init_func init;
    ioctl_func ioctl;
    open_func open;
    close_func close;
    suspned_func suspend;
    resume_func resume;
    suspned_func suspend_s4;
    resume_func resume_s4;
    uint64_t reserved[DRV_RESERVED_NUM]; /* has not used, just reserved */
};

#define tee_driver_declare(name, init, open, ioctl, close, suspend, resume, suspend_s4, resume_s4) \
__attribute__((visibility("default"))) const struct tee_driver_module g_driver_##name = { \
    init, ioctl, open, close, suspend, resume, suspend_s4, resume_s4, {0} }
```

结构体struct tee_driver_module便是每个驱动业务开发时需要注册的信息，各个变量说明如下：

**表 10** 驱动业务开发注册信息表

<a></a>
<table><thead align="left"><tr><th class="cellrowborder" valign="top" width="33.33333333333333%"><p>变量名</p>
</th>
<th class="cellrowborder" valign="top" width="33.33333333333333%"><p>类型</p>
</th>
<th class="cellrowborder" valign="top" width="33.33333333333333%"><p>说明</p>
</th>
</tr>
</thead>
<tbody><tr><td><p>name</p></td>
<td><p>常量字符串</p></td>
<td><p>驱动名字，每个驱动必须唯一，有效长度小于32个字节，仅支持数字、字母和'_'。</p></td>
</tr>
<tr><td><p>init</p></td>
<td><p>init_func函数指针</p></td>
<td><p>驱动加载时初始化函数。</p></td>
</tr>
<tr><td><p>open</p></td>
<td><p>函数指针</p></td>
<td><p>驱动被访问时初始化函数。</p></td>
</tr>
<tr><td><p>ioctl</p></td>
<td><p>函数指针</p></td>
<td><p>驱动被访问时命令分发函数。</p></td>
</tr>
<tr><td><p>close</p></td>
<td><p>函数指针</p></td>
<td><p>驱动被访问结束时资源释放函数。</p></td>
</tr>
<tr><td><p>suspend</p></td>
<td><p>函数指针</p></td>
<td><p>驱动在系统休眠时被调用的函数。</p></td>
</tr>
<tr><td><p>resume</p></td>
<td><p>函数指针</p></td>
<td><p>驱动在系统唤醒时被调用的函数。</p></td>
</tr>
<tr><td><p>suspend_s4</p></td>
<td><p>函数指针</p></td>
<td><p>驱动在系统休眠时被调用的函数，对应linux kernel里freeze_noirq操作流程。</p></td>
</tr>
<tr><td><p>resume_s4</p></td>
<td><p>函数指针</p></td>
<td><p>驱动在系统唤醒时被调用的函数，对应linux kernel里restore_noirq操作流程。</p></td>
</tr>
<tr><td><p>reserved</p></td>
<td><p>预留</p></td>
<td><p>现有框架暂未使用。</p></td>
</tr>
</tbody>
</table>

**表 11** 驱动业务框架接口说明
<a></a>
<table><thead><tr><th class="cellrowborder" valign="top" width="50%"><p>接口名</p>
</th>
<th class="cellrowborder" valign="top" width="50%"><p>描述</p>
</th>
</tr>
</thead>
<tbody><tr><td><p>int32_t (*init_func)(void)</p></td>
<td><p>该函数是在驱动加载完后便被驱动框架调用的初始化函数，其主要作用是在该驱动被访问之前进行初始化操作。该函数在驱动加载后的整个生命周期内只会被调用一次。</p>
<p><b>返回值：</b></p>
<p>0：初始化成功</p>
<p>非0：初始化失败</p></td>
</tr>
<tr><td><p>int64_t (*open_func)(struct drv_data *drv, unsigned long args, uint32_t args_len)</p></td>
<td><p>该函数是驱动访问者访问驱动时调用的初始化函数，其主要作用是在驱动里申请一个fd，并进行初始化。该函数在每次新增驱动访问时都会调用。</p>
<p><b>参数：</b></p>
<p>drv：入参，表示此次open该驱动的fd所有信息</p>
<p>args：入参，表示给驱动传入参数对应的buffer地址，由驱动访问者设置</p>
<p>args_len：入参，表示给驱动传入参数对应的buffer长度，由驱动访问者设置</p>
<p><b>返回值：</b></p>
<p>非正数：异常值</p>
<p>其他：返回的fd信息</p></td>
</tr>
<tr><td><p>int64_t (*ioctl_func)(struct drv_data *drv, uint32_t cmd, unsigned long args, uint32_t args_len)</p></td>
<td><p>该函数是驱动在open初始化之后，进行的一系列针对业务逻辑的操作。其主要作用是根据传入的drv获取fd资源信息，再执行cmd命令对应的执行流，其中[args.args_len]组成的buffer信息是给该cmd执行流传入的参数。</p>
<p><b>参数：</b></p>
<p>drv：入参，表示此次ioctl该驱动的fd所有信息，fd由驱动访问者传入，驱动框架根据fd传入对应drv_data结构体</p>
<p>cmd：入参，表示此次ioctl该驱动传入的命令号，驱动可根据不同命令号执行不同的业务逻辑，由驱动访问者设置</p>
<p>args：入参，表示给驱动传递参数的buffer基地址</p>
<p>args_len：入参，表示给驱动传递参数的buffer长度</p>
<p><b>返回值：</b></p>
<p>0：操作成功</p>
<p>-1：操作失败，框架层面返回的异常值</p>
<p>其他值：操作失败，驱动自行定义的异常值</p></td>
</tr>
<tr><td><p>int64_t (*close_func)(struct drv_data *drv)</p></td>
<td><p>该函数主要作用是在驱动业务逻辑访问结束后进行对该fd对应的资源清理操作。</p>
<p><b>参数：</b></p>
<p>drv：入参，表示此次ioctl该驱动的fd所有信息</p>
<p><b>返回值：</b></p>
<p>0：操作成功</p>
<p>非0：操作失败</p></td>
</tr>
<tr><td><p>int32_t (*suspned_func)(void)</p></td>
<td><p>该函数主要作用是此驱动休眠状态下的一些列操作，会在系统休眠时有驱动框架自行调用。</p>
<p><b>返回值：</b></p>
<p>0：操作成功</p>
<p>非0：操作失败</p></td>
</tr>
<tr><td><p>int32_t (*resume_func)(void)</p></td>
<td><p>该函数主要作用是此驱动唤醒态下的一系列操作，会在系统唤醒流程中由驱动框架自行调用，与suspend函数对应。</p>
<p><b>返回值：</b></p>
<p>0：操作成功</p>
<p>非0：操作失败</p></td>
</tr>
</tbody>
</table>

### 驱动访问者框架<a name="section18123151133811"></a>

驱动访问时，驱动访问者先调用tee_drv_open函数获取驱动唯一标记fd；再调用tee_drv_ioctl函数，传入cmd信息，访问该驱动对应cmd执行流，如果针对某个fd有多个ioctl执行流，多次调用tee_drv_ioctl即可；如果访问结束，驱动访问者还需要调用tee_drv_close函数关闭该fd信息。

**表 12** 驱动访问者框架接口说明

<a></a>
<table><thead><tr><th class="cellrowborder" valign="top" width="50%"><p>接口名</p>
</th>
<th class="cellrowborder" valign="top" width="50%"><p>描述</p>
</th>
</tr>
</thead>
<tbody><tr><td><p>int64_t tee_drv_open(const char *drv_name, const void *param, uint32_t param_len)</p></td>
<td><p>主要作用是驱动访问者通过调用该函数，访问drv_name指定的驱动，调用驱动的open函数，返回与该驱动对应的唯一标记fd信息。其中param buffer对应的内容组装结构由drv_name对应驱动定义，实际与驱动open函数里[args.args_len]表示的buffer内容一致。</p>
<p><b>参数：</b></p>
<p>drv_name：入参，表示要访问的驱动名称；</p>
<p>param：入参，表示给驱动传递的参数地址；</p>
<p>param_len：入参，表示给驱动传递的参数长度，与param组成的buffer内容便是给驱动传递的参数信息。</p>
<p><b>返回值：</b></p>
<p>非正数：非法值，操作失败</p>
<p>大于0：fd信息，对应驱动的唯一标识</p></td>
</tr>
<tr><td><p>int64_t tee_drv_ioctl(int64_t fd, uint32_t cmd_id, const void *param, uint32_t param_len)</p></td>
<td><p>主要作用是驱动访问者通过调用该函数，访问fd对应的驱动模块，执行命令ID号为cmd_id对应的业务逻辑，传入参数为param与param_len对应buffer存储的内容，其中param与param_len对应buffer内容组装结构由fd对应的驱动定义，实际与驱动ioctl函数[args.args_len]组成的buffer内容一致。</p>
<p><b>参数：</b></p>
<p>fd：入参，表示open该驱动成功返回时的返回值；</p>
<p>cmd_id：入参，表示ioctl该驱动时对应的命令id号；</p>
<p>param：入参，表示ioctl该驱动cmd_id流程时传入的参数基地址；</p>
<p>param_len：入参，表示ioctl该驱动cmd_id流程时传入的参数buffer长度。与param组成的buffer内容便是给驱动命令号cmd_id对应的参数信息。</p>
<p><b>返回值：</b></p>
<p>0：操作成功</p>
<p>-1：框架层面访问失败</p>
<p>其他值：各驱动自定义失败返回值</p></td>
</tr>
<tr><td><p>int64_t tee_drv_close(int64_t fd)</p></td>
<td><p>主要作用是关闭fd对应驱动信息，一般常见操作是释放该fd维护的驱动资源。</p>
<p><b>返回值：</b></p>
<p>0：操作成功</p>
<p>-1：操作失败</p></td>
</tr>
</tbody>
</table>

## 接口说明<a name="section1748173625619"></a>

### 地址转换接口说明<a name="section101297155716"></a>

驱动进行地址转换操作需要使用的接口列表。

**表 13**  地址转换接口列表

<a name="table13739184111427"></a>
<table><thead align="left"><tr id="row2739154118421"><th class="cellrowborder" valign="top" width="54.31%" id="mcps1.2.3.1.1"><p id="p462103214410"><a name="p462103214410"></a><a name="p462103214410"></a>接口名</p>
</th>
<th class="cellrowborder" valign="top" width="45.69%" id="mcps1.2.3.1.2"><p id="p1762163219448"><a name="p1762163219448"></a><a name="p1762163219448"></a>描述</p>
</th>
</tr>
</thead>
<tbody><tr id="row7739441104213"><td class="cellrowborder" valign="top" width="54.31%" headers="mcps1.2.3.1.1 "><p id="p137395418427"><a name="p137395418427"></a><a name="p137395418427"></a>uint64_t drv_virt_to_phys(uintptr_t addr);</p>
</td>
<td class="cellrowborder" valign="top" width="45.69%" headers="mcps1.2.3.1.2 "><p id="p177391841124218"><a name="p177391841124218"></a><a name="p177391841124218"></a>虚拟地址转换为物理地址。</p>
</td>
</tr>
</tbody>
</table>

### map接口说明<a name="section1831091675716"></a>

驱动进行内存映射操作所需要的接口列表。

**表 14**  map接口列表

<a name="table1690431511432"></a>
<table><thead align="left"><tr id="row189041315194317"><th class="cellrowborder" valign="top" width="54.67999999999999%" id="mcps1.2.3.1.1"><p id="p34281633124413"><a name="p34281633124413"></a><a name="p34281633124413"></a>接口名</p>
</th>
<th class="cellrowborder" valign="top" width="45.32%" id="mcps1.2.3.1.2"><p id="p4428133174412"><a name="p4428133174412"></a><a name="p4428133174412"></a>描述</p>
</th>
</tr>
</thead>
<tbody><tr id="row9904121594312"><td class="cellrowborder" valign="top" width="54.67999999999999%" headers="mcps1.2.3.1.1 "><p id="p1090471594316"><a name="p1090471594316"></a><a name="p1090471594316"></a>int32_t tee_map_secure(paddr_t paddr, uint64_t size, uintptr_t *vaddr, cache_mode_type cache_mode);</p>
</td>
<td class="cellrowborder" valign="top" width="45.32%" headers="mcps1.2.3.1.2 "><p id="p583mcpsimp"><a name="p583mcpsimp"></a><a name="p583mcpsimp"></a>给驱动访问者映射一段安全属性的物理内存。</p>
</td>
</tr>
<tr id="row190417158431"><td class="cellrowborder" valign="top" width="54.67999999999999%" headers="mcps1.2.3.1.1 "><p id="p4904715174317"><a name="p4904715174317"></a><a name="p4904715174317"></a>int32_t tee_map_nonsecure(paddr_t paddr, uint64_t size, uintptr_t *vaddr, cache_mode_type cache_mode);</p>
</td>
<td class="cellrowborder" valign="top" width="45.32%" headers="mcps1.2.3.1.2 "><p id="p10904115204310"><a name="p10904115204310"></a><a name="p10904115204310"></a>给驱动访问者映射一段非安全属性的物理内存，其中映射属性是只读不能写。</p>
</td>
</tr>
</tbody>
</table>

### IO操作接口说明<a name="section6512133012574"></a>

驱动进行IO操作所需要的接口列表。

**表 15**  IO操作接口列表

<a name="table6311346194315"></a>
<table><thead align="left"><tr id="row831212462439"><th class="cellrowborder" valign="top" width="54.730000000000004%" id="mcps1.2.3.1.1"><p id="p7945143412442"><a name="p7945143412442"></a><a name="p7945143412442"></a>接口名</p>
</th>
<th class="cellrowborder" valign="top" width="45.269999999999996%" id="mcps1.2.3.1.2"><p id="p7945183494416"><a name="p7945183494416"></a><a name="p7945183494416"></a>描述</p>
</th>
</tr>
</thead>
<tbody><tr id="row143121846164318"><td class="cellrowborder" valign="top" width="54.730000000000004%" headers="mcps1.2.3.1.1 "><p id="p133121246194317"><a name="p133121246194317"></a><a name="p133121246194317"></a>void *ioremap(uintptr_t phys_addr, unsigned long size, int32_t prot);</p>
</td>
<td class="cellrowborder" valign="top" width="45.269999999999996%" headers="mcps1.2.3.1.2 "><p id="p133126466437"><a name="p133126466437"></a><a name="p133126466437"></a>将IO地址映射至虚拟地址。</p>
</td>
</tr>
<tr id="row1431214468430"><td class="cellrowborder" valign="top" width="54.730000000000004%" headers="mcps1.2.3.1.1 "><p id="p103122046114317"><a name="p103122046114317"></a><a name="p103122046114317"></a>int32_t iounmap(uintptr_t pddr, void *addr);</p>
</td>
<td class="cellrowborder" valign="top" width="45.269999999999996%" headers="mcps1.2.3.1.2 "><p id="p4312194674313"><a name="p4312194674313"></a><a name="p4312194674313"></a>解除物理地址映射。</p>
</td>
</tr>
<tr id="row1131211467433"><td class="cellrowborder" valign="top" width="54.730000000000004%" headers="mcps1.2.3.1.1 "><p id="p63121446154315"><a name="p63121446154315"></a><a name="p63121446154315"></a>void read_from_io (void *to, const volatile void *from, unsigned long count);</p>
</td>
<td class="cellrowborder" valign="top" width="45.269999999999996%" headers="mcps1.2.3.1.2 "><p id="p16312174619438"><a name="p16312174619438"></a><a name="p16312174619438"></a>将IO输入的值读取至驱动指定的地址，读取长度由count指定。</p>
</td>
</tr>
<tr id="row1331219468434"><td class="cellrowborder" valign="top" width="54.730000000000004%" headers="mcps1.2.3.1.1 "><p id="p8312104614437"><a name="p8312104614437"></a><a name="p8312104614437"></a>void write_to_io(volatile void *to, const void *from, unsigned long count);</p>
</td>
<td class="cellrowborder" valign="top" width="45.269999999999996%" headers="mcps1.2.3.1.2 "><p id="p193mcpsimp"><a name="p193mcpsimp"></a><a name="p193mcpsimp"></a>将驱动指定地址的值输出至IO，读取长度由count指定。</p>
</td>
</tr>
</tbody>
</table>

### 内存拷贝接口说明<a name="section75381736318"></a>

驱动进行内存拷贝操作所需要的接口列表。

**表 16**  内存拷贝接口列表

<a name="table7469133214314"></a>
<table><thead align="left"><tr id="row24693321135"><th class="cellrowborder" valign="top" width="55.35%" id="mcps1.2.3.1.1"><p id="p109015511039"><a name="p109015511039"></a><a name="p109015511039"></a>接口名</p>
</th>
<th class="cellrowborder" valign="top" width="44.65%" id="mcps1.2.3.1.2"><p id="p2090117510315"><a name="p2090117510315"></a><a name="p2090117510315"></a>描述</p>
</th>
</tr>
</thead>
<tbody><tr id="row15469133212312"><td class="cellrowborder" valign="top" width="55.35%" headers="mcps1.2.3.1.1 "><p id="p04692032734"><a name="p04692032734"></a><a name="p04692032734"></a>int32_t copy_from_client(uint64_t src, uint32_t src_size, uintptr_t dst, uint32_t dst_size);</p>
</td>
<td class="cellrowborder" valign="top" width="44.65%" headers="mcps1.2.3.1.2 "><p id="p184693323311"><a name="p184693323311"></a><a name="p184693323311"></a>将驱动内存拷入client端。</p>
</td>
</tr>
<tr id="row246915325319"><td class="cellrowborder" valign="top" width="55.35%" headers="mcps1.2.3.1.1 "><p id="p94881980512"><a name="p94881980512"></a><a name="p94881980512"></a>int32_t copy_to_client(uintptr_t src, uint32_t src_size, uint64_t dst, uint32_t dst_size);</p>
</td>
<td class="cellrowborder" valign="top" width="44.65%" headers="mcps1.2.3.1.2 "><p id="p647018327314"><a name="p647018327314"></a><a name="p647018327314"></a>将client端数据拷出至驱动。</p>
</td>
</tr>
</tbody>
</table>

### 共享内存接口说明<a name="section5891654459"></a>

驱动进行共享内存操作所需要的接口列表。

**表 17**  共享内存接口列表

<a name="table186909151063"></a>
<table><thead align="left"><tr id="row369016151263"><th class="cellrowborder" valign="top" width="55.620000000000005%" id="mcps1.2.3.1.1"><p id="p1033416207618"><a name="p1033416207618"></a><a name="p1033416207618"></a>接口名</p>
</th>
<th class="cellrowborder" valign="top" width="44.379999999999995%" id="mcps1.2.3.1.2"><p id="p19334172010619"><a name="p19334172010619"></a><a name="p19334172010619"></a>描述</p>
</th>
</tr>
</thead>
<tbody><tr id="row1069019153610"><td class="cellrowborder" valign="top" width="55.620000000000005%" headers="mcps1.2.3.1.1 "><p id="p176901815266"><a name="p176901815266"></a><a name="p176901815266"></a>void *tee_alloc_sharemem_aux(const struct tee_uuid *uuid, uint32_t size);</p>
</td>
<td class="cellrowborder" valign="top" width="44.379999999999995%" headers="mcps1.2.3.1.2 "><p id="p11690101520612"><a name="p11690101520612"></a><a name="p11690101520612"></a>申请进程间通信共享内存。</p>
</td>
</tr>
<tr id="row19690615967"><td class="cellrowborder" valign="top" width="55.620000000000005%" headers="mcps1.2.3.1.1 "><p id="p166901915464"><a name="p166901915464"></a><a name="p166901915464"></a>uint32_t tee_free_sharemem(void *addr, uint32_t size);</p>
</td>
<td class="cellrowborder" valign="top" width="44.379999999999995%" headers="mcps1.2.3.1.2 "><p id="p186901215366"><a name="p186901215366"></a><a name="p186901215366"></a>释放进程间通信共享内存。</p>
</td>
</tr>
<tr id="row1469663510216"><td class="cellrowborder" valign="top" width="55.620000000000005%" headers="mcps1.2.3.1.1 "><p id="p06966351127"><a name="p06966351127"></a><a name="p06966351127"></a>int32_t get_tlv_sharedmem(const char *type, uint32_t type_size, void *buffer, uint32_t *size, bool clear_flag);</p>
</td>
<td class="cellrowborder" valign="top" width="44.379999999999995%" headers="mcps1.2.3.1.2 "><p id="p1038093194520"><a name="p1038093194520"></a><a name="p1038093194520"></a>获取loader阶段写入到共享内存中的信息。</p>
</td>
</tr>
</tbody>
</table>

## 开发示例<a name="section198361520981"></a>

驱动框架注册实例如下：

```
#define TEST_NUM_INIT_VAL 0x11
#define TEST_NUM_ADD_TAG 0x10
static int32_t g_test_num = TEST_NUM_INIT_VAL;

/* 驱动初始化函数 */
int32_t init_test(void)
{
    if (g_test_num != TEST_NUM_INIT_VAL) {
        tloge("driver init test failed, g_test_num:0x%x not equal 0x%x\n", g_test_num, TEST_NUM_INIT_VAL);
        return -1;
    }

    g_test_num += TEST_NUM_ADD_TAG;
    tloge("driver init test end\n");
    return 0;
}

static int32_t buf_check(uint32_t *buf, uint32_t size, uint32_t args)
{
    if (buf == NULL) {
        tloge("buf is invalid, check failed\n");
        return -1;
    }

    uint32_t i;
    for (i = 0; i < size; i++) {
        if (buf[i] != args) {
            tloge("buf[%u]=%u which not equal %u\n", i, buf[i], args);
            return -1;
        }
    }

    return 0;
}

/* 驱动具体业务处理逻辑函数 */
static int64_t args_test(struct drv_data *drv, unsigned long args, uint32_t args_len)
{
    /* cmd处理函数在使用参数之前需要判断参数合法性 */
    if (args_len != sizeof(uint32_t) || args == 0) {
        tloge("invalid args args_len:%u\n", args_len);
        return -1;
    }

    uint32_t *input_args = (uint32_t *)(uintptr_t)args;

    tloge("driver args test begin: drv.fd=%d args=0x%x g_test_num:0x%x\n", drv->fd, *input_args, g_test_num);
    int64_t ret = buf_check(drv->private_data, TOKEN_BUF_SIZE, *input_args);
    if (ret != 0)
        tloge("args test FAIL\n");
    else
        tloge("args test SUCC\n");

    return ret;
}

/* 驱动命令分发函数，根据不同cmdid调用不同函数逻辑 */
int64_t ioctl_test(struct drv_data *drv, uint32_t cmd, unsigned long args, uint32_t args_len)
{
    if (drv == NULL) { /* drv可以在调用业务逻辑之前判断，args和args_len需要根据具体cmdid逻辑去判断 */
        tloge("ioctl invalid drv\n");
        return -1;
    }

    int64_t ret = -1;

    switch (cmd) {
    case ARGS_TEST_ID:
        ret = args_test(drv, args, args_len);
        break;
    default:
        tloge("cmd:0x%x not support\n", cmd);
    }

    return ret;
}

static uint32_t *buf_init(uint32_t args)
{
    uint32_t *buf = (uint32_t *)malloc(TOKEN_BUF_SIZE * sizeof(uint32_t));
    if (buf == NULL) {
        tloge("alloc buf failed\n");
        return NULL;
    }

    int32_t i;
    for (i = 0; i < TOKEN_BUF_SIZE; i++)
        buf[i] = args;

    return buf;
}

int64_t open_test(struct drv_data *drv, unsigned long args, uint32_t args_len)
{
    if (drv == NULL) {
        tloge("open invalid drv\n");
        return -1;
    }

    if (args_len < sizeof(uint32_t) || args == 0) {
        tloge("open invalid drv\n");
        return -1;
    }

    uint32_t *input = (uint32_t *)args;
    if (*input == UINT32_MAX) {
        tloge("open test input args is UINT32_MAX, just retrun -1\n");
        return -1;
    }

    uint32_t *buf = buf_init(*input);
    if (buf == NULL)
        return -1;

    /* 将buf赋值给fd结构的private_data，后续都可以通过此fd获取到buf内容 */
    drv->private_data = buf;
    tloge("driver open test begin: fd=%d args=0x%x g_test_num:0x%x\n", drv->fd, *input, g_test_num);

    return 0;
}

int64_t close_test(struct drv_data *drv)
{
    if (drv == NULL) {
        tloge("close invalid drv\n");
        return -1;
    }

    tloge("driver close test begin: fd:%d g_test_num:0x%x\n", drv->fd, g_test_num);
    /* close流程需要释放fd对应的所有资源 */
    if (drv->private_data != NULL) {
        tloge("free private data in close\n");
        free(drv->private_data);
    }

    return 0;
}

int32_t suspend_test(void)
{
    tloge("suspend test begin\n");
    return 0;
}

int32_t resume_test(void)
{
    tloge("resume test begin\n");
    return 0;
}

int32_t suspend_s4_test(void)
{
    tloge("suspend_s4 test begin\n");
    return 0;
}

int32_t resume_s4_test(void)
{
    tloge("resume_s4 test begin\n");
    return 0;
}

/* 驱动框架注册 */
tee_driver_declare(drv_test_module, init_test, open_test, ioctl_test, close_test, \
                   suspend_test, resume_test, suspend_s4_test, resume_s4_test);
```

## 标准C库支持<a name="section14109139161012"></a>

支持绝大多数的libc接口。支持大多数的POSIX接口，具体支持情况请看附件。标准文档请参照如下网址：

POSIX:[https://mirror.math.princeton.edu/pub/oldlinux/download/c953.pdf](https://mirror.math.princeton.edu/pub/oldlinux/download/c953.pdf)

目前使用的musl-1.2.0/libc库。

**表 17**  标准C支持列表

<a name="table7336617112614"></a>
<table><thead align="left"><tr id="row1633681714266"><th class="cellrowborder" valign="top" width="33.33333333333333%" id="mcps1.2.4.1.1"><p id="p1653154512717"><a name="p1653154512717"></a><a name="p1653154512717"></a>模块</p>
</th>
<th class="cellrowborder" valign="top" width="33.33333333333333%" id="mcps1.2.4.1.2"><p id="p1253114512274"><a name="p1253114512274"></a><a name="p1253114512274"></a>函数接口名</p>
</th>
<th class="cellrowborder" valign="top" width="33.33333333333333%" id="mcps1.2.4.1.3"><p id="p75311145202718"><a name="p75311145202718"></a><a name="p75311145202718"></a>说明</p>
</th>
</tr>
</thead>
<tbody><tr id="row6340121710262"><td class="cellrowborder" rowspan="6" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.1 "><p id="p18534845102711"><a name="p18534845102711"></a><a name="p18534845102711"></a>malloc</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.2 "><p id="p553444592710"><a name="p553444592710"></a><a name="p553444592710"></a>aligned_alloc</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.3 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row10341181782614"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p753424552719"><a name="p753424552719"></a><a name="p753424552719"></a>calloc</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row534110172268"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p1853444542719"><a name="p1853444542719"></a><a name="p1853444542719"></a>malloc</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row203411176269"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p1553474572712"><a name="p1553474572712"></a><a name="p1553474572712"></a>realloc</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row5341131742616"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p10534184519273"><a name="p10534184519273"></a><a name="p10534184519273"></a>free</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row18341101714263"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p12534184522712"><a name="p12534184522712"></a><a name="p12534184522712"></a>posix_memalign</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row18341121732619"><td class="cellrowborder" rowspan="2" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.1 "><p id="p125341945182720"><a name="p125341945182720"></a><a name="p125341945182720"></a>mman</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.2 "><p id="p7534184532717"><a name="p7534184532717"></a><a name="p7534184532717"></a>mmap</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.3 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row534131772614"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p12534184513273"><a name="p12534184513273"></a><a name="p12534184513273"></a>munmap</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row434101742618"><td class="cellrowborder" rowspan="3" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.1 "><p id="p253419457279"><a name="p253419457279"></a><a name="p253419457279"></a>time</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.2 "><p id="p16534124510274"><a name="p16534124510274"></a><a name="p16534124510274"></a>gettimeofday</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.3 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row53411917172619"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p11534124522712"><a name="p11534124522712"></a><a name="p11534124522712"></a>strftime</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row5341101712617"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p19535845152713"><a name="p19535845152713"></a><a name="p19535845152713"></a>time</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row1334131792612"><td class="cellrowborder" rowspan="6" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.1 "><p id="p105357458277"><a name="p105357458277"></a><a name="p105357458277"></a>stdio</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.2 "><p id="p75354454278"><a name="p75354454278"></a><a name="p75354454278"></a>printf</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.3 "><p id="p4535134517270"><a name="p4535134517270"></a><a name="p4535134517270"></a>目前不支持文件系统，文件操作只支持标准输入输出。</p>
</td>
</tr>
<tr id="row63421517192616"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p2535154592716"><a name="p2535154592716"></a><a name="p2535154592716"></a>scanf</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row83421817132613"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p105351045132712"><a name="p105351045132712"></a><a name="p105351045132712"></a>snprintf</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row434291782619"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p12535545132715"><a name="p12535545132715"></a><a name="p12535545132715"></a>sprintf</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row19342517122616"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p3535245132711"><a name="p3535245132711"></a><a name="p3535245132711"></a>vsnprintf</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row1342161792613"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p353518454270"><a name="p353518454270"></a><a name="p353518454270"></a>vsprintf</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row734291715264"><td class="cellrowborder" rowspan="2" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.1 "><p id="p13535045172712"><a name="p13535045172712"></a><a name="p13535045172712"></a>errno</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.2 "><p id="p25351645132719"><a name="p25351645132719"></a><a name="p25351645132719"></a>errno</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.3 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row11342181711269"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p11535164515276"><a name="p11535164515276"></a><a name="p11535164515276"></a>strerror</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row5342151710264"><td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.1 "><p id="p10535945152714"><a name="p10535945152714"></a><a name="p10535945152714"></a>exit</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.2 "><p id="p8535124519274"><a name="p8535124519274"></a><a name="p8535124519274"></a>abort</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.3 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row11342117182611"><td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.1 "><p id="p85358458277"><a name="p85358458277"></a><a name="p85358458277"></a>unistd</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.2 "><p id="p1753517452273"><a name="p1753517452273"></a><a name="p1753517452273"></a>getpid</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.3 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row1034281772615"><td class="cellrowborder" rowspan="4" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.1 "><p id="p1553514592714"><a name="p1553514592714"></a><a name="p1553514592714"></a>locale</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.2 "><p id="p753574512278"><a name="p753574512278"></a><a name="p753574512278"></a>setlocale</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.3 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row334220171263"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p115353451276"><a name="p115353451276"></a><a name="p115353451276"></a>strcoll</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row13343151762618"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p95351145142716"><a name="p95351145142716"></a><a name="p95351145142716"></a>strxfrm</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row173433172262"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p0535104512273"><a name="p0535104512273"></a><a name="p0535104512273"></a>strtod</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row2343131732614"><td class="cellrowborder" rowspan="3" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.1 "><p id="p95351945132716"><a name="p95351945132716"></a><a name="p95351945132716"></a>multibyte</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.2 "><p id="p25351345122715"><a name="p25351345122715"></a><a name="p25351345122715"></a>mbrtowc</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.3 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row1234321772614"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p185354453273"><a name="p185354453273"></a><a name="p185354453273"></a>wcrtomb</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row7343151762613"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p115361445112716"><a name="p115361445112716"></a><a name="p115361445112716"></a>wctob</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row1034351714262"><td class="cellrowborder" rowspan="4" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.1 "><p id="p135361645112710"><a name="p135361645112710"></a><a name="p135361645112710"></a>prng</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.2 "><p id="p19536134517272"><a name="p19536134517272"></a><a name="p19536134517272"></a>srandom</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.3 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row73431717102610"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p17536134518272"><a name="p17536134518272"></a><a name="p17536134518272"></a>initstate</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row17343181742615"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p2536144515271"><a name="p2536144515271"></a><a name="p2536144515271"></a>setstate</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row16343121711263"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p185362459278"><a name="p185362459278"></a><a name="p185362459278"></a>random</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row63431217102612"><td class="cellrowborder" rowspan="17" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.1 "><p id="p1153618451278"><a name="p1153618451278"></a><a name="p1153618451278"></a>string</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.2 "><p id="p1553620456277"><a name="p1553620456277"></a><a name="p1553620456277"></a>memchr</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.3 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row93431217112613"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p14536154510275"><a name="p14536154510275"></a><a name="p14536154510275"></a>memcmp</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row17344417112619"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p153684515275"><a name="p153684515275"></a><a name="p153684515275"></a>memcpy</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row1934421712265"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p1653694592712"><a name="p1653694592712"></a><a name="p1653694592712"></a>memmove</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row934411712267"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p2536114510274"><a name="p2536114510274"></a><a name="p2536114510274"></a>memset</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row163441117142619"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p105361345112715"><a name="p105361345112715"></a><a name="p105361345112715"></a>strchr</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row1434451715262"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p175366451279"><a name="p175366451279"></a><a name="p175366451279"></a>strcmp</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row534471762616"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p353694514273"><a name="p353694514273"></a><a name="p353694514273"></a>strcpy</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row334461752611"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p1753694515274"><a name="p1753694515274"></a><a name="p1753694515274"></a>strlen</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row834419172265"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p0536745102720"><a name="p0536745102720"></a><a name="p0536745102720"></a>strncmp</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row1734415175268"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p55361345192717"><a name="p55361345192717"></a><a name="p55361345192717"></a>strncpy</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row183445174264"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p115365457273"><a name="p115365457273"></a><a name="p115365457273"></a>strnlen</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row1534401712611"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p19537245112712"><a name="p19537245112712"></a><a name="p19537245112712"></a>strrchr</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row13451617142620"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p125371456272"><a name="p125371456272"></a><a name="p125371456272"></a>strstr</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row15345181762614"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p95371845142718"><a name="p95371845142718"></a><a name="p95371845142718"></a>wcschr</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row10345151718267"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p0537124512279"><a name="p0537124512279"></a><a name="p0537124512279"></a>wcslen</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row2345111710263"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p145371645142718"><a name="p145371645142718"></a><a name="p145371645142718"></a>wmemchr</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row12345131715263"><td class="cellrowborder" rowspan="13" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.1 "><p id="p753714502717"><a name="p753714502717"></a><a name="p753714502717"></a>ctype</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.2 "><p id="p5537145142720"><a name="p5537145142720"></a><a name="p5537145142720"></a>isalpha</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.3 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row1434581792614"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p0537184511274"><a name="p0537184511274"></a><a name="p0537184511274"></a>isascii</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row5345417182620"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p8537114513278"><a name="p8537114513278"></a><a name="p8537114513278"></a>isdigit</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row8345117122618"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p8537204582712"><a name="p8537204582712"></a><a name="p8537204582712"></a>islower</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row034551782617"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p253716455273"><a name="p253716455273"></a><a name="p253716455273"></a>isprint</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row4345101713267"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p1053713455279"><a name="p1053713455279"></a><a name="p1053713455279"></a>isspace</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row16345201752616"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p853764562716"><a name="p853764562716"></a><a name="p853764562716"></a>iswctype</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row33455179264"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p18537114522715"><a name="p18537114522715"></a><a name="p18537114522715"></a>iswdigit</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row83461317192614"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p16537154512710"><a name="p16537154512710"></a><a name="p16537154512710"></a>iswlower</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row93461117152620"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p353713458276"><a name="p353713458276"></a><a name="p353713458276"></a>iswspace</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row19346161711268"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p75371145202717"><a name="p75371145202717"></a><a name="p75371145202717"></a>iswupper</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row123461517162613"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p3537245152710"><a name="p3537245152710"></a><a name="p3537245152710"></a>towupper</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row1734671714268"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p175381245172715"><a name="p175381245172715"></a><a name="p175381245172715"></a>towlower</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row6346117112615"><td class="cellrowborder" rowspan="16" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.1 "><p id="p2538174512714"><a name="p2538174512714"></a><a name="p2538174512714"></a>math</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.2 "><p id="p19538184572712"><a name="p19538184572712"></a><a name="p19538184572712"></a>atan</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.3 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row8346131782613"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p15381945172717"><a name="p15381945172717"></a><a name="p15381945172717"></a>ceil</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row734617176261"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p953844582716"><a name="p953844582716"></a><a name="p953844582716"></a>ceilf</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row134610174264"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p053824511271"><a name="p053824511271"></a><a name="p053824511271"></a>copysignl</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row13346917152610"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p13538104522717"><a name="p13538104522717"></a><a name="p13538104522717"></a>exp</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row334661712269"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p5538154512710"><a name="p5538154512710"></a><a name="p5538154512710"></a>fabs</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row2347181722616"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p2538184522716"><a name="p2538184522716"></a><a name="p2538184522716"></a>floor</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row123471517122616"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p1453874562717"><a name="p1453874562717"></a><a name="p1453874562717"></a>frexp</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row153471917112616"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p14538134512271"><a name="p14538134512271"></a><a name="p14538134512271"></a>frexpl</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row234771717261"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p10538245102714"><a name="p10538245102714"></a><a name="p10538245102714"></a>log</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row1334711179265"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p185381245142719"><a name="p185381245142719"></a><a name="p185381245142719"></a>log2</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row103471317132612"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p20538144512275"><a name="p20538144512275"></a><a name="p20538144512275"></a>pow</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row133472178262"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p35389459277"><a name="p35389459277"></a><a name="p35389459277"></a>roundf</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row234731713261"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p4538154522715"><a name="p4538154522715"></a><a name="p4538154522715"></a>scalbn</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row1347917182619"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p85381845192718"><a name="p85381845192718"></a><a name="p85381845192718"></a>scalbnl</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row1134717178267"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p11538845102718"><a name="p11538845102718"></a><a name="p11538845102718"></a>sqrt</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row13348101715261"><td class="cellrowborder" rowspan="14" align="left" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.1 "><p id="p195391445102713"><a name="p195391445102713"></a><a name="p195391445102713"></a>stdlib</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.2 "><p id="p1153954552713"><a name="p1153954552713"></a><a name="p1153954552713"></a>abs</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.2.4.1.3 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row4348101732615"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p1853910450272"><a name="p1853910450272"></a><a name="p1853910450272"></a>atof</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row12348111782617"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p12539124562717"><a name="p12539124562717"></a><a name="p12539124562717"></a>atoi</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row123482178261"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p95391945102713"><a name="p95391945102713"></a><a name="p95391945102713"></a>atol</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row163481117142610"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p35391745172718"><a name="p35391745172718"></a><a name="p35391745172718"></a>atoll</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row6348217132613"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p4539164511276"><a name="p4539164511276"></a><a name="p4539164511276"></a>bsearch</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row12348191714269"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p195391245102717"><a name="p195391245102717"></a><a name="p195391245102717"></a>div</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row1634810170260"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p10539124513274"><a name="p10539124513274"></a><a name="p10539124513274"></a>ecvt</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row103481517182617"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p75392457277"><a name="p75392457277"></a><a name="p75392457277"></a>imaxabs</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row1834817175260"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p854014552719"><a name="p854014552719"></a><a name="p854014552719"></a>llabs</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row1349121792612"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p1154054562718"><a name="p1154054562718"></a><a name="p1154054562718"></a>qsort</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row15349117192614"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p14540645172719"><a name="p14540645172719"></a><a name="p14540645172719"></a>strtoul</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row134910176267"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p135401445172712"><a name="p135401445172712"></a><a name="p135401445172712"></a>strtol</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
<tr id="row183494173268"><td class="cellrowborder" valign="top" headers="mcps1.2.4.1.1 "><p id="p3540114519272"><a name="p3540114519272"></a><a name="p3540114519272"></a>wcstod</p>
</td>
<td class="cellrowborder" valign="top" headers="mcps1.2.4.1.2 ">&nbsp;&nbsp;</td>
</tr>
</tbody>
</table>

>![](public_sys-resources/icon-caution.gif) **注意：** 
>1.  不支持文件系统、控制台。
>2.  不支持fstat，fsync，writev接口。

## 安全函数库<a name="section127921457151719"></a>

### 概述<a name="section5848918131818"></a>

危险函数依赖于程序员对参数进行检查或保证空间能足够容纳所产生的结果，函数本身不对这些情况进行判断，即使有问题也不会给出错误的指示。C11标准中对于过时的不安全的危险函数定义了对应的安全函数（\_s版本的函数），相比危险函数，安全函数对照C11标准进行了相应的安全增强，会对入参以及不同的错误情况进行判断，降低操作不当所引入的安全风险。下表列举了危险函数以及对应的安全函数，TA代码中涉及到相关危险函数的必须使用安全函数。

**表 18**  危险函数以及对应的安全函数

<a name="table18216829674"></a>
<table><thead align="left"><tr id="row1221612296715"><th class="cellrowborder" valign="top" width="50%" id="mcps1.2.3.1.1"><p id="p1280043317715"><a name="p1280043317715"></a><a name="p1280043317715"></a>危险函数</p>
</th>
<th class="cellrowborder" valign="top" width="50%" id="mcps1.2.3.1.2"><p id="p880019332710"><a name="p880019332710"></a><a name="p880019332710"></a>安全函数</p>
</th>
</tr>
</thead>
<tbody><tr id="row122166291776"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p198006331074"><a name="p198006331074"></a><a name="p198006331074"></a>memcpy</p>
</td>
<td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p16800533473"><a name="p16800533473"></a><a name="p16800533473"></a>memcpy_s</p>
</td>
</tr>
<tr id="row421610291977"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p080012331073"><a name="p080012331073"></a><a name="p080012331073"></a>memmove</p>
</td>
<td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p1680013331779"><a name="p1680013331779"></a><a name="p1680013331779"></a>memmove_s</p>
</td>
</tr>
<tr id="row152161229975"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p1480013336712"><a name="p1480013336712"></a><a name="p1480013336712"></a>memset</p>
</td>
<td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p4800103314713"><a name="p4800103314713"></a><a name="p4800103314713"></a>memset_s</p>
</td>
</tr>
<tr id="row02163297719"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p1580017338710"><a name="p1580017338710"></a><a name="p1580017338710"></a>strcpy</p>
</td>
<td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p20800233575"><a name="p20800233575"></a><a name="p20800233575"></a>strcpy_s</p>
</td>
</tr>
<tr id="row132161629777"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p680016331977"><a name="p680016331977"></a><a name="p680016331977"></a>strncpy</p>
</td>
<td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p1880013335719"><a name="p1880013335719"></a><a name="p1880013335719"></a>strncpy_s</p>
</td>
</tr>
<tr id="row1821642918715"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p180010331175"><a name="p180010331175"></a><a name="p180010331175"></a>strcat</p>
</td>
<td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p158003331671"><a name="p158003331671"></a><a name="p158003331671"></a>strcat_s</p>
</td>
</tr>
<tr id="row72167299712"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p780014331373"><a name="p780014331373"></a><a name="p780014331373"></a>strncat</p>
</td>
<td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p188017331171"><a name="p188017331171"></a><a name="p188017331171"></a>strncat_s</p>
</td>
</tr>
<tr id="row17217202917714"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p780113331478"><a name="p780113331478"></a><a name="p780113331478"></a>strtok</p>
</td>
<td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p128013333711"><a name="p128013333711"></a><a name="p128013333711"></a>strtok_s</p>
</td>
</tr>
<tr id="row1021717291676"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p980111331072"><a name="p980111331072"></a><a name="p980111331072"></a>snprintf</p>
</td>
<td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p98011331277"><a name="p98011331277"></a><a name="p98011331277"></a>snprintf_s</p>
</td>
</tr>
<tr id="row92174291974"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p380114333720"><a name="p380114333720"></a><a name="p380114333720"></a>vsnprintf</p>
</td>
<td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p20801183316710"><a name="p20801183316710"></a><a name="p20801183316710"></a>vsnprintf_s</p>
</td>
</tr>
</tbody>
</table>

详细描述请参考头文件<securec.h\>。

安全函数包含部分如下特性:

-   强化边界检查：在接口参数中增加一个buffer长度的参数，在长度参数正确情况下不会出现溢出。
-   保证结果字符串以’\\0’结尾，避免访问buffer边界之外的信息。
-   发现缓冲区溢出发生，将目的缓冲区的首字节置零。
-   增加错误返回值，便于程序员快速进行错误定位。
-   增强入参检查。
-   增加入参的内存重叠检查（memcpy\_sp宏中对于常量拷贝不进行内存重叠检查）。
-   定义SECUREC\_STRING\_MAX\_LEN和SECUREC\_MEM\_MAX\_LEN宏，可以通过其限定字符串和内存操作时的最大长度。
-   定义SECUREC\_ERROR\_INVALID\_PARAMTER、SECUREC\_ERROR\_INVALID\_RANGE和SECUREC\_ERROR\_BUFFER\_OVERLAP宏，其会在参数出错时进行统一处理，用户可自定义发生错误时的函数处理行为。

### 内存复制<a name="section1620304312183"></a>

memcpy\_s：

```
errno_t memcpy_s(void* dest, size_t destMax, const void* src, size_t count);
```

复制源缓冲区的数据到目的缓冲区。

>![](public_sys-resources/icon-caution.gif) **注意：** 
>-   与系统函数相比：1）多了一个参数：目的缓冲区总大小；  2）系统函数返回值为指针，安全函数返回值为整型。
>-   调用函数时，注意判断返回值是否成功，否则有可能操作结果和预期不一致。
>-   某些出错情况下，会对目的缓冲区清0，具体参考如上**表1**。
>-   进行字符串拷贝时，不会自动在末尾添加结束符。（对字符串操作，建议使用字符串操作函数）

-   示例

<figure>

```
 #include "securec.h" 
 #include <string.h> 
  
 #define BUFFER_SIZE 11 
  
 int main() 
 { 
     char str1[BUFFER_SIZE] = "0123456789"; 
     char str2[BUFFER_SIZE] = {0x00}; 
     errno_t rc =  EOK; 
  
     rc = memcpy_s(str2,  BUFFER_SIZE, str1, BUFFER_SIZE - 1); 
     printf("rc = %d,  %s\n", rc, str2); 
  
     /* count is bigger than destMax, return ERANGE_AND_RESET  and dest is reset. */ 
     rc = memcpy_s(str2,  BUFFER_SIZE, str1, BUFFER_SIZE + 1); 
     printf("rc = %d, %s\n", rc,  str2); 
  
     /* initialize */ 
     rc = memcpy_s(str2,  BUFFER_SIZE, str1, BUFFER_SIZE - 1); 
     printf("rc = %d, %s\n", rc,  str2); 
  
     /* overlap, return EOVERLAP_AND_RESET and dest is reset.  */ 
     rc = memcpy_s(str2,  BUFFER_SIZE, str2 + 2, BUFFER_SIZE - 1); 
     printf("rc = %d, %s\n", rc,  str2); 
  
     return 0; 
 }
运行结果：
rc =  0, 0123456789
rc =  162
rc =  0, 0123456789
rc =  182
```

</figure>

memmove\_s：

```
errno_t memmove_s(void* dest,size_t destMax, const void* src, size_t count);
```

移动源缓冲区的数据到目的缓冲区

>![](public_sys-resources/icon-caution.gif) **注意：** 
>-   与系统函数相比：
>    -   多了一个参数：目的缓冲区总大小；
>    -   系统函数返回值为指针，安全函数返回值为整型。
>-   调用函数时，注意判断返回值是否成功，否则有可能操作结果和预期不一致。
>-   某些出错情况下，会对目的缓冲区清0，具体参考如上**表2**。

-   示例

<figure>

```
 #include "securec.h" 
 #include <string.h> 
  
 #define BUFFER_SIZE 11 
  
 int main() 
 { 
     char str[BUFFER_SIZE] = "0123456789"; 
     errno_t rc =  EOK; 
  
     printf("Before:  %s\n", str); 
  
     /* Move six bytes from the start of the  string 
     * to a new position shifted  by one byte. To protect against 
     * buffer overrun, the  secure version of memmove requires the 
     * the length of the  destination string to be specified. 
     */ 
  
     rc = memmove_s(str,  BUFFER_SIZE, str + 1, 6); 
     printf("After: rc = %d, %s\n", rc,  str); 
  
     /* count is bigger than destMax, return ERANGE_AND_RESET  and dest is reset. */ 
     rc = memmove_s(str,  BUFFER_SIZE, str + 1, BUFFER_SIZE + 100); 
     printf("Later: rc = %d, %s\n",  rc, str); 
  
     return 0; 
 }
运行结果:
Before: 0123456789
After: rc = 0,  123456789
Later: rc =  162
```

</figure>

### 内存初始化<a name="section9113185921817"></a>

memset\_s：

```
errno_t memset_s(void* dest, size_t destMax, int c, size_t count);
```

复制源缓冲区的数据到目的缓冲区

>![](public_sys-resources/icon-caution.gif) **注意：** 
>-   与系统函数相比：1）多了一个参数：目的缓冲区总大小；  2）系统函数返回值为指针，安全函数返回值为整型。
>-   调用函数时，注意判断返回值是否成功，否则有可能操作结果和预期不一致。

-   示例

<figure>

```
 #include "securec.h" 
 #include <string.h> 
  
 #define BUFFER_SIZE 40 
  
 int main() 
 { 
     char buffer[BUFFER_SIZE] =  "This is a test of the memset  function"; 
     errno_t rc =  EOK; 
  
     printf( "Before: %s\n", buffer ); 
  
     rc = memset_s(buffer,  BUFFER_SIZE, '*', 20 ); 
     printf( "After: rc = %d, %s\n", rc, buffer  ); 
  
     /* count is bigger than destMax, return ERANGE_AND_RESET  and destMax size is set. */ 
     rc = memset_s(buffer,  BUFFER_SIZE, '*', BUFFER_SIZE + 1 ); 
     printf( "Later: rc = %d, %s\n", rc, buffer  ); 
  
     return  0; 
 }
 
 运行结果：
 Before: This is a test of the memset function 
 After: rc = 0, ********************e memset  function 
 Later: rc = 162,  ****************************************
```

</figure>

### 字符串复制<a name="section320518115198"></a>

strcpy\_s：

```
errno_t strcpy_s(char* strDest, size_t destMax, const char* strSrc);
```

复制源字符串到目的缓冲区

>![](public_sys-resources/icon-caution.gif) **注意：** 
>-   与系统函数相比：1）、多了一个参数：目的缓冲区总大小； 2）、系统函数返回值为指针，安全函数返回值为整型。
>-   调用函数时，注意判断返回值是否成功，否则有可能操作结果和预期不一致。
>-   某些出错情况下，会对目的缓冲区首字符置0，具体参考如上**表1**。
>源字符串必须含有结束符。

-   示例

<figure>

```
 #include "securec.h" 
 #include <stdio.h> 
  
 #define SMALL_BUF_SIZE 10 
 #define BIG_BUF_SIZE 100 
  
 int main() 
 { 
     char str1[BIG_BUF_SIZE] = {0x00}; 
     char str2[SMALL_BUF_SIZE] = {0x00}; 
     char *str3 = str1 +  4; 
     errno_t rc =  EOK; 
  
     rc = strcpy_s(str1,  BIG_BUF_SIZE, "Security Design  Department"); 
     printf("rc = %d,  %s\n", rc, str1); 
  
     /* strSrc length + 1 is bigger than destMax, return  ERANGE_AND_RESET and strDest is reset. */ 
     rc = strcpy_s(str2,  SMALL_BUF_SIZE, "Security Design"); 
     printf("rc = %d,  %s\n", rc, str2); 
  
     memset_s(str1,  BIG_BUF_SIZE, 0x41, BIG_BUF_SIZE); 
     /* overlap, return EOVERLAP_AND_RESET and strDest is reset.  */ 
     rc = strcpy_s(str1,  BIG_BUF_SIZE, str3); 
     printf("rc = %d,  %s\n", rc, str1); 
  
     return 0; 
 }

运行结果：
rc  = 0, Security Design Department
rc  = 162,
rc  = 182,
```

</figure>

strncpy\_s：

```
errno_t strncpy_s(char* strDest, size_t destMax, const char* strSrc, size_t count);
```

复制指定长度的源字符串到目的缓冲区。

>![](public_sys-resources/icon-caution.gif) **注意：** 
>-   与系统函数相比：1）多了一个参数：目的缓冲区总大小； 2）系统函数返回值为指针，安全函数返回值为整型。
>-   调用函数时，注意判断返回值是否成功，否则有可能操作结果和预期不一致。
>-   某些出错情况下，会对目的缓冲区首字符置0，具体参考如上**表2**。
>-   源字符串必须含有结束符。
>-   调用该函数时建议传入的参数destMax  大于 count以保证有截断功能。
>-   安全函数strncpy\_s执行完后保证strDest有’\\0’结尾,而危险函数strncpy不保证。
>-   当  count 大于 strlen\(strSrc\)  时，  strncpy函数会复制完字符串后在strDest中填充count-strlen\(strSrc\)个’\\0’字符，而strncpy\_s函数不做填充。

-   示例

<figure>

```
 #define SMALL_BUF_SIZE 10
 #define BIG_BUF_SIZE 100 
  
 int main() 
 { 
     char str1[BIG_BUF_SIZE] =  {0x00}; 
     char str2[BIG_BUF_SIZE] =  "security design  department"; 
     char *str3 = str1 +  4; 
     char str4[SMALL_BUF_SIZE] =  {0x00}; 
     errno_t rc =  EOK; 
  
     rc = strncpy_s(str1,  BIG_BUF_SIZE, str2, 15); 
     printf("rc = %d, %s\n", rc, str1); 
  
     /* count is bigger than destMax, return ERANGE_AND_RESET  and dest is reset. */ 
     rc = strncpy_s(str4,  SMALL_BUF_SIZE, "security Design Department", 15); 
     printf("rc = %d, %s\n", rc, str4); 
  
     memset_s(str1,  BIG_BUF_SIZE, 0x41, BIG_BUF_SIZE); 
     /* overlap, return EOVERLAP_AND_RESET and dest is reset.  */ 
     rc = strncpy_s(str1,  BIG_BUF_SIZE, str3, 15); 
     printf("rc = %d, %s\n", rc, str1); 
  
     return 0; 
 }

运行结果：
rc =  0, security design
rc =  162,
rc =  182,
```

</figure>

### 字符串连接<a name="section3307826101911"></a>

strcat\_s：

```
errno_t strcat_s(char* strDest, size_t destMax, const char* strSrc);
```

将源字符串连接到目的字符串后面。

>![](public_sys-resources/icon-caution.gif) **注意：** 
>-   与系统函数相比：
>    -   多了一个参数：目的缓冲区总大小；
>    -   系统函数返回值为指针，安全函数返回值为整型。
>-   调用函数时，注意判断返回值是否成功，否则有可能操作结果和预期不一致。
>-   某些出错情况下，会对目的缓冲区的首字符置0，具体参考如上**表1**。

-   示例

<figure>

```
 #include "securec.h" 
 #include <stdio.h> 
  
 #define BUFFER_SIZE  64 
  
 int main() 
 { 
     char   str1[BUFFER_SIZE] = {’’}; 
     char   str2[BUFFER_SIZE] = {’b’}; 
     errno_t rc = EOK; 
  
     rc = strcat_s(str1,  3, str2); 
     printf("rc = %d, %s\n", rc, str1); 
  
  
     /*strdest in destMax don’t have ‘\0’*/ 
     memset_s(str1, BUFFER_SIZE,'a',4); 
     memset_s(str2, BUFFER_SIZE,'b',4); 
     rc = strcat_s(str1, 4,  str2); 
     printf("rc = %d, %s\n", rc, str1); 
  
     /*destmax isn’t enough for strSrc*/ 
     memset_s(str1, BUFFER_SIZE,'a',4); 
     memset_s(str2, BUFFER_SIZE,'b',4); 
     rc = strcat_s(str1,8, str2); 
     printf("rc = %d, %s\n", rc, str1); 
  
     /*dest , src overlap*/ 
     rc = strncat_s(str2,8, str2+2,1); 
     printf("rc = %d, %s\n", rc, str2); 
  
     return 0; 
 }

运行结果:
rc = 0, ab
rc = 150,
rc = 162,
rc = 182,
```

</figure>

strncat\_s：

```
errno_t strncat_s(char* strDest, size_t destMax, const char* strSrc, size_t count);
```

将指定长度的源字符串连接到目的字符串后面。

>![](public_sys-resources/icon-caution.gif) **注意：** 
>-   与系统函数相比：
>    -   多了一个参数：目的缓冲区总大小；
>    -   系统函数返回值为指针，安全函数返回值为整型。
>-   调用函数时，注意判断返回值是否成功，否则有可能操作结果和预期不一致。
>-   某些出错情况下，会对目的缓冲区的首字符置0，具体参考如上**表2**。

-   示例

<figure>

```
 #include "securec.h" 
 #include <stdio.h> 
  
 #define BUFFER_SIZE  64 
  
 int main() 
 { 
     char   str1[BUFFER_SIZE] = {’’}; 
     char   str2[BUFFER_SIZE] = {’b’}; 
     errno_t rc = EOK; 
  
     rc = strncat_s(str1, 3, str2,  1); 
     printf("rc = %d, %s\n", rc , str1); 
  
     /*strdest in destMax don’t have  ‘\0’*/ 
     memset_s(str1, BUFFER_SIZE,’a’,4); 
     memset_s(str2, BUFFER_SIZE,’b’,4); 
     rc = strncat_s(str1, 4, str2,  2); 
     printf("rc  = %d, %s\n", rc ,  str1); 
  
     /*destmax isn’t enough for  strSrc*/ 
     memset_s(str1, BUFFER_SIZE,’a’,4); 
     memset_s(str2, BUFFER_SIZE,’b’,4); 
     rc = strncat_s(str1,8, str2,5); 
     printf("rc = %d, %s\n", rc , str1); 
  
     /*dest , src overlap*/ 
     rc = strncat_s(str2,8, str2+2,1); 
     printf("rc = %d, %s\n", rc , str2); 
  
     return 0; 
 }

运行结果:
rc  = 0, ab
rc  = 150,
rc  = 162,
rc  = 182,
```

</figure>

### 字符串分割<a name="section85731515152610"></a>

strtok\_s：

```
errno_t strtok_s(char* strToken, const char* strDelimit, char** context);
```

将字符串按照指定的分隔符分割成子字符串。

>![](public_sys-resources/icon-caution.gif) **注意：** 
>-   与系统函数相比：多了一个参数context：保存调用strtok\_s后的下个子字符串的首地址，当不再有子字符串时，context指向被分割字符串结尾。
>-   当在被分割字符串中没有找到分隔符时，如果被分割字符串长度大于0，会返回被分割字符串首地址，否则返回NULL。
>-   以逗号分隔符为例，顺序调用strtok\_s\(str,&p\), strtok\_s\(NULL,&p\),  strtok\_s\(NULL,&p\)函数的返回值与不同分割字符串的关系如下：

**表 19**  返回值与字符串关系

<a name="table18556455195912"></a>
<table><tbody><tr id="row7556165595917"><td class="cellrowborder" rowspan="2" valign="top"><p id="p156641710803"><a name="p156641710803"></a><a name="p156641710803"></a><strong id="b86641109012"><a name="b86641109012"></a><a name="b86641109012"></a><span>被分割字符串</span></strong></p>
</td>
<td class="cellrowborder" colspan="3" valign="top"><p id="p1766414101808"><a name="p1766414101808"></a><a name="p1766414101808"></a><strong id="b1866414101608"><a name="b1866414101608"></a><a name="b1866414101608"></a><span>返回值</span></strong></p>
</td>
</tr>
<tr id="row155765565917"><td class="cellrowborder" valign="top"><p id="p146654101808"><a name="p146654101808"></a><a name="p146654101808"></a>strtok_s(str,&amp;p)</p>
</td>
<td class="cellrowborder" valign="top"><p id="p15665610307"><a name="p15665610307"></a><a name="p15665610307"></a>strtok_s(NULL,&amp;p)</p>
</td>
<td class="cellrowborder" valign="top"><p id="p17665111019017"><a name="p17665111019017"></a><a name="p17665111019017"></a>strtok_s(NULL,&amp;p)</p>
</td>
</tr>
<tr id="row955745525914"><td class="cellrowborder" valign="top" width="25%"><p id="p19665171014011"><a name="p19665171014011"></a><a name="p19665171014011"></a>str = "aaa"</p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p46652010100"><a name="p46652010100"></a><a name="p46652010100"></a>"aaa"</p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p06652101907"><a name="p06652101907"></a><a name="p06652101907"></a>NULL</p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p466511105013"><a name="p466511105013"></a><a name="p466511105013"></a>_</p>
</td>
</tr>
<tr id="row75571655115910"><td class="cellrowborder" valign="top" width="25%"><p id="p1066512101208"><a name="p1066512101208"></a><a name="p1066512101208"></a><span>str =</span> <span>""</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p4665181017014"><a name="p4665181017014"></a><a name="p4665181017014"></a><span>NULL</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p166654101307"><a name="p166654101307"></a><a name="p166654101307"></a>_</p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p17665111018016"><a name="p17665111018016"></a><a name="p17665111018016"></a>_</p>
</td>
</tr>
<tr id="row4557145516596"><td class="cellrowborder" valign="top" width="25%"><p id="p176651510509"><a name="p176651510509"></a><a name="p176651510509"></a><span>str =</span> <span>","</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p766510101102"><a name="p766510101102"></a><a name="p766510101102"></a><span>NULL</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p0665610101"><a name="p0665610101"></a><a name="p0665610101"></a>_</p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p106657106015"><a name="p106657106015"></a><a name="p106657106015"></a>_</p>
</td>
</tr>
<tr id="row195571055105919"><td class="cellrowborder" valign="top" width="25%"><p id="p26651510407"><a name="p26651510407"></a><a name="p26651510407"></a><span>str =</span> <span>",,"</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p9665510409"><a name="p9665510409"></a><a name="p9665510409"></a><span>NULL</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p206657101204"><a name="p206657101204"></a><a name="p206657101204"></a>_</p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p066541016017"><a name="p066541016017"></a><a name="p066541016017"></a>_</p>
</td>
</tr>
<tr id="row6558115516595"><td class="cellrowborder" valign="top" width="25%"><p id="p1866520102008"><a name="p1866520102008"></a><a name="p1866520102008"></a><span>str =</span> <span>",aaa"</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p96655102010"><a name="p96655102010"></a><a name="p96655102010"></a><span>"aaa"</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p06651101902"><a name="p06651101902"></a><a name="p06651101902"></a><span>NULL</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p18666121018017"><a name="p18666121018017"></a><a name="p18666121018017"></a>_</p>
</td>
</tr>
<tr id="row1955865511594"><td class="cellrowborder" valign="top" width="25%"><p id="p166671012013"><a name="p166671012013"></a><a name="p166671012013"></a><span>str =</span> <span>",,aaa"</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p1666611018017"><a name="p1666611018017"></a><a name="p1666611018017"></a><span>"aaa"</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p76665101802"><a name="p76665101802"></a><a name="p76665101802"></a><span>NULL</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p106666102015"><a name="p106666102015"></a><a name="p106666102015"></a>_</p>
</td>
</tr>
<tr id="row1955805514595"><td class="cellrowborder" valign="top" width="25%"><p id="p466618103014"><a name="p466618103014"></a><a name="p466618103014"></a><span>str =</span> <span>"aaa,"</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p566615101006"><a name="p566615101006"></a><a name="p566615101006"></a><span>"aaa"</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p12666210206"><a name="p12666210206"></a><a name="p12666210206"></a><span>NULL</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p1566617101809"><a name="p1566617101809"></a><a name="p1566617101809"></a>_</p>
</td>
</tr>
<tr id="row755810558594"><td class="cellrowborder" valign="top" width="25%"><p id="p146661510301"><a name="p146661510301"></a><a name="p146661510301"></a><span>str =</span> <span>"aaa,,"</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p1566641012015"><a name="p1566641012015"></a><a name="p1566641012015"></a><span>"aaa"</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p1966613107017"><a name="p1966613107017"></a><a name="p1966613107017"></a><span>NULL</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p1166611108010"><a name="p1166611108010"></a><a name="p1166611108010"></a>_</p>
</td>
</tr>
<tr id="row155915515598"><td class="cellrowborder" valign="top" width="25%"><p id="p1666611101603"><a name="p1666611101603"></a><a name="p1666611101603"></a><span>str =</span> <span>"aaa,bbb"</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p106663101504"><a name="p106663101504"></a><a name="p106663101504"></a><span>"aaa"</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p13666610804"><a name="p13666610804"></a><a name="p13666610804"></a><span>"bbb"</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p126663101003"><a name="p126663101003"></a><a name="p126663101003"></a><span>NULL</span></p>
</td>
</tr>
<tr id="row9559555135919"><td class="cellrowborder" valign="top" width="25%"><p id="p13666131015010"><a name="p13666131015010"></a><a name="p13666131015010"></a><span>str =</span> <span>"aaa,,bbb"</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p176668102003"><a name="p176668102003"></a><a name="p176668102003"></a><span>"aaa"</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p196661108012"><a name="p196661108012"></a><a name="p196661108012"></a><span>"bbb"</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p1366601016011"><a name="p1366601016011"></a><a name="p1366601016011"></a><span>NULL</span></p>
</td>
</tr>
<tr id="row1455915555596"><td class="cellrowborder" valign="top" width="25%"><p id="p8667510002"><a name="p8667510002"></a><a name="p8667510002"></a><span>str =</span> <span>"aaa,bbb,"</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p196672010506"><a name="p196672010506"></a><a name="p196672010506"></a><span>"aaa"</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p6667201020015"><a name="p6667201020015"></a><a name="p6667201020015"></a><span>"bbb"</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p13667161011011"><a name="p13667161011011"></a><a name="p13667161011011"></a><span>NULL</span></p>
</td>
</tr>
<tr id="row175591455145914"><td class="cellrowborder" valign="top" width="25%"><p id="p466713102019"><a name="p466713102019"></a><a name="p466713102019"></a><span>str =</span> <span>"aaa,bbb,,"</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p1166751019014"><a name="p1166751019014"></a><a name="p1166751019014"></a><span>"aaa"</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p9667010504"><a name="p9667010504"></a><a name="p9667010504"></a><span>"bbb"</span></p>
</td>
<td class="cellrowborder" valign="top" width="25%"><p id="p16667910402"><a name="p16667910402"></a><a name="p16667910402"></a><span>NULL</span></p>
</td>
</tr>
</tbody>
</table>

-   示例

<figure>

```
 #include "securec.h" 
 #include <stdio.h> 
  
 int main() 
 { 
     char string1[64]  = "A string\tof ,,tokens\nand some  more tokens"; 
     char string2[64]  = "Another string\n\tparsed at the same  time."; 
     char seps[] =  " ,\t\n"; 
     char *token1 =  NULL; 
     char *token2 =  NULL; 
     char *next_token1  = NULL; 
     char *next_token2= NULL; 
  
     printf( "Tokens:\n" ); 
  
     /* Establish string and get the first token:  */ 
     token1 = strtok_s(string1, seps,  &next_token1); 
     token2 = strtok_s(string2, seps,  &next_token2); 
  
     /* While there are tokens in "string1" or "string2"  */ 
     while ((token1 !=  NULL) || (token2 != NULL)) 
     { 
         /* Get next token: */ 
         if (token1 !=  NULL) 
         { 
             printf("  %s\n", token1); 
             token1 = strtok_s(NULL,  seps, &next_token1); 
         } 
         if (token2 !=  NULL) 
         { 
             printf("        %s\n", token2); 
             token2 = strtok_s (NULL, seps,  &next_token2); 
         } 
     } 
  
     return 0; 
 }

运行结果:
Tokens:
more
same
some
the
and
at
tokens
parsed
of
string
string
Another
time.
tokens
```

</figure>

### 格式化输出<a name="section1696173315284"></a>

snprintf\_s：

```
int snprintf_s(char* strDest, size_t destMax, size_t count, const char* format, ...);
```

将数据按照指定长度格式化输出到目的缓冲区。

>![](public_sys-resources/icon-caution.gif) **注意：** 
>-   与系统函数相比：
>    -   多了一个参数：目的缓冲区总大小；
>    -   count参数与系统函数的用法存在差异，参见参数表。
>-   调用函数时，注意判断返回值是否成功，否则有可能操作结果和预期不一致。
>-   某些出错情况下，会对目的缓冲区首字符清0，具体参考如上**表1**。
>-   调用该函数时建议传入的参数destMax 大于 count以保证有截断功能。
>-   确保输入的源数据与目标缓冲区不存在重叠。
>-   输入源数据中若是字符串则必须含有结束符。
>输入源数据的类型、个数必须与格式化控制字符串（format）中的类型、个数保持一致。

-   示例

<figure>

```
 #include "securec.h" 
  
 void  snprintf_sample() 
 { 
     char buffer[200]={0}; 
     char     *string = "computer"; 
     wchar_t *wstring =  L"Unicode"; 
  
     /* format strings. */ 
     int iRet = snprintf_s( buffer, 200, 20,  "%s,%ls", string, wstring); 
     printf("iRet = %d, buffer = %s.\n", iRet,  buffer); 
  
     /* test edge. */ 
     printf("------------------\n"); 
     iRet = snprintf_s( buffer, 3, 3, "%s",  "123"); 
     printf("iRet =  %d, buffer = %s.\n", iRet, buffer); 
  
     /* test edge. */ 
     printf("------------------\n"); 
     iRet = snprintf_s( buffer, 3, 2, "%s",  "123"); 
     printf("iRet = %d, buffer = %s.\n", iRet,  buffer); 
  
  
     return; 
 } 
  
 int  main() 
 { 
     snprintf_sample (); 
     return 0; 
 }

运行结果：
iRet = 16, buffer  = computer,Unicode.
iRet = -1, buffer  = .

iRet = -1, buffer  = 12.
```

</figure>

vsnprintf\_s：

```
int vsnprintf_s(char* strDest, size_t destMax, size_t count, const char* format, va_list arglis);
```

将参数列表\(_va\_list_\)数据按照指定长度格式化输出到目的缓冲区。

>![](public_sys-resources/icon-caution.gif) **注意：** 
>-   与系统函数相比：
>    -   多了一个参数：目的缓冲区总大小；
>    -   count参数与系统函数的用法存在差异，参见参数表。
>-   调用函数时，注意判断返回值是否成功，否则有可能操作结果和预期不一致。
>-   某些出错情况下，会对目的缓冲区首字符清0，具体参考如上表2。
>-   调用该函数时建议传入的参数destMax 大于 count以保证有截断功能。
>-   确保输入的源数据与目标缓冲区不存在重叠。
>-   输入源数据中若是字符串则必须含有结束符。
>-   输入源数据的类型、个数必须与格式化控制字符串（format）中的类型、个数保持一致。

-   示例1

<figure>

```
 #include "securec.h" 
  
 void FormatOutput(char* formatstring, ...) 
 { 
     va_list args; 
     int nSize =  0; 
     char buff[10]; 
     memset_s(buff,sizeof(buff), 0, sizeof(buff)); 
     va_start(args,  formatstring); 
     nSize = vsnprintf_s( buff, sizeof(buff), 8,  formatstring, args); 
     printf("nSize: %d,  buff: %s\n", nSize, buff); 
 } 
  
 int main() 
 { 
     FormatOutput("%s  %s", "Hi", "there"); 
     FormatOutput("%s  %s", "Hi", "there!"); 
  
     return 0; 
 }

运行结果：
nSize: 8, buff: Hi there
nSize: -1, buff: Hi there
```

</figure>

-   示例2

<figure>

```
 void vsnprintf_base(char * format, ...)
 { 
     va_list  args; 
     char  buffer[100]; 
     int iRet =  -1; 
  
     va_start(args,  format); 
     iRet = vsnprintf_s(buffer,  100, 20, format, args); 
     printf("iRet = %d, buffer  = %s.\n", iRet, buffer); 
  
     return; 
 } 
 void vsnprintf_sample(void) 
 { 
     char    *string = "computer"; 
     wchar_t *wstring =  L"Unicode"; 
  
     /* format strings.  */ 
     vsnprintf_base("%s,%ls",  string, wstring); 
  
     /* test  edge.*/ 
     printf("------------------\n"); 
     vsnprintf_base("%s",  "12345678901234567890a"); 
  
     return; 
 } 
  
 int main() 
 { 
     vsnprintf_sample(); 
     return  0; 
 }

运行结果：
iRet = 16, buffer = computer,Unicode.
iRet = -1, buffer = 12345678901234567890.
```

</figure>

