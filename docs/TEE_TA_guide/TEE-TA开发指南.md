# TEE TA开发指南<a name="ZH-CN_TOPIC_0000001241980213"></a>

-   [2.1 开发步骤及示例工程](#section181412311296)
-   [2.1.1 环境准备](#section1894134872916)
-   [编译开发环境准备](#section196631553554)
-   [签名开发环境准备](#section7597183533818)
-   [编译依赖的bounds\_checking\_function和musl头文件准备](#section1182315617500)
-   [2.1.2 TA编译及开发示例](#section87251833133019)
-   [2.1.2.1 编译入口示例](#section53541258123016)
-   [简介](#section24399485559)
-   [build.sh文件](#section18297126898)
-   [defconfig文件](#section885652814360)
-   [Manifest文件](#section1893131414916)
-   [2.1.2.2 make编译配置示例](#section169056453214)
-   [config.mk文件](#section613734445614)
-   [Makefile文件](#section973417201288)
-   [Make编译方法](#section197921537291)
-   [2.1.2.3 cmake编译配置示例](#section111111007337)
-   [config.cmake文件](#section41061567813)
-   [CMakeList.txt文件](#section8649337289)
-   [CMake编译方法](#section99081322133314)
-   [2.2 SEC文件签名](#section175621346143317)
-   [密钥及证书生成](#section12730101601820)
-   [perm\_config文件签名](#section1023415229205)
-   [SEC文件签名步骤](#section193879205219)
-   [CA\(Certification Authority\)说明](#section1087132913217)
-   [2.3 SEC调试](#section1261347344)
-   [日志使用限制](#section413233117432)
-   [使用方法](#section61772276175810)

## 2.1 开发步骤及示例工程<a name="section181412311296"></a>

## 2.1.1 环境准备<a name="section1894134872916"></a>

## 编译开发环境准备<a name="section196631553554"></a>

需要使用的GCC工具链可在linaro网站下载，LLVM和Clang需要在LLVM官网下载。64位TA编译需要使用aarch64版本的交叉编译工具链，按照与32位工具链相同的方式下载、安装和配置。

<a name="table2204236125920"></a>
<table><thead align="left"><tr id="row5204836125920"><th class="cellrowborder" valign="top" width="72.98729872987299%" id="mcps1.1.4.1.1"><p id="p52041036155915"><a name="p52041036155915"></a><a name="p52041036155915"></a>工具链下载路径</p>
</th>
<th class="cellrowborder" valign="top" width="14.65146514651465%" id="mcps1.1.4.1.2"><p id="p122041736115912"><a name="p122041736115912"></a><a name="p122041736115912"></a>32位TA编译</p>
</th>
<th class="cellrowborder" valign="top" width="12.361236123612361%" id="mcps1.1.4.1.3"><p id="p192041436105916"><a name="p192041436105916"></a><a name="p192041436105916"></a>64位TA编译</p>
</th>
</tr>
</thead>
<tbody><tr id="row192045361593"><td class="cellrowborder" valign="top" width="72.98729872987299%" headers="mcps1.1.4.1.1 "><p id="p1820473618594"><a name="p1820473618594"></a><a name="p1820473618594"></a>https://releases.linaro.org/components/toolchain/binaries/7.4-2019.02/arm-eabi/gcc-linaro-7.4.1-2019.02-x86_64_arm-eabi.tar.xz</p>
</td>
<td class="cellrowborder" valign="top" width="14.65146514651465%" headers="mcps1.1.4.1.2 "><p id="p122041036175916"><a name="p122041036175916"></a><a name="p122041036175916"></a>是</p>
</td>
<td class="cellrowborder" valign="top" width="12.361236123612361%" headers="mcps1.1.4.1.3 "><p id="p1020443612599"><a name="p1020443612599"></a><a name="p1020443612599"></a>否</p>
</td>
</tr>
<tr id="row182058366595"><td class="cellrowborder" valign="top" width="72.98729872987299%" headers="mcps1.1.4.1.1 "><p id="p1820543617598"><a name="p1820543617598"></a><a name="p1820543617598"></a>https://releases.linaro.org/components/toolchain/binaries/7.4-2019.02/arm-linux-gnueabi/gcc-linaro-7.4.1-2019.02-x86_64_arm-linux-gnueabi.tar.xz</p>
</td>
<td class="cellrowborder" valign="top" width="14.65146514651465%" headers="mcps1.1.4.1.2 "><p id="p15205153614591"><a name="p15205153614591"></a><a name="p15205153614591"></a>是</p>
</td>
<td class="cellrowborder" valign="top" width="12.361236123612361%" headers="mcps1.1.4.1.3 "><p id="p62051836195918"><a name="p62051836195918"></a><a name="p62051836195918"></a>否</p>
</td>
</tr>
<tr id="row82053363598"><td class="cellrowborder" valign="top" width="72.98729872987299%" headers="mcps1.1.4.1.1 "><p id="p16205836125913"><a name="p16205836125913"></a><a name="p16205836125913"></a>https://releases.linaro.org/components/toolchain/binaries/7.4-2019.02/aarch64-linux-gnu/gcc-linaro-7.4.1-2019.02-x86_64_aarch64-linux-gnu.tar.xz</p>
</td>
<td class="cellrowborder" valign="top" width="14.65146514651465%" headers="mcps1.1.4.1.2 "><p id="p15205193615593"><a name="p15205193615593"></a><a name="p15205193615593"></a>否</p>
</td>
<td class="cellrowborder" valign="top" width="12.361236123612361%" headers="mcps1.1.4.1.3 "><p id="p18205163675911"><a name="p18205163675911"></a><a name="p18205163675911"></a>是</p>
</td>
</tr>
<tr id="row12051736205913"><td class="cellrowborder" valign="top" width="72.98729872987299%" headers="mcps1.1.4.1.1 "><p id="p122057363592"><a name="p122057363592"></a><a name="p122057363592"></a>https://github.com/llvm/llvm-project/releases/download/llvmorg-8.0.1/clang+llvm-8.0.1-x86_64-linux-gnu-ubuntu-14.04.tar.xz</p>
</td>
<td class="cellrowborder" valign="top" width="14.65146514651465%" headers="mcps1.1.4.1.2 "><p id="p9205173616592"><a name="p9205173616592"></a><a name="p9205173616592"></a>是</p>
</td>
<td class="cellrowborder" valign="top" width="12.361236123612361%" headers="mcps1.1.4.1.3 "><p id="p820553620590"><a name="p820553620590"></a><a name="p820553620590"></a>是</p>
</td>
</tr>
</tbody>
</table>

使用CMake编译框架，请在编译环境上安装cmake，cmake建议使用3.16.5-3.20.5版本，安装包可在cmake网站下载  [http://www.cmake.org/files/](http://www.cmake.org/files/)。

## 签名开发环境准备<a name="section7597183533818"></a>

>![](public_sys-resources/icon-note.gif) **说明：** 
>可信应用签名不支持Windows环境使用，只支持在Linux服务器侧使用。

可信应用签名依赖的软件列表及下载来源如下：

<a name="table1710232244715"></a>
<table><thead align="left"><tr id="row191029223476"><th class="cellrowborder" valign="top" width="25.419999999999998%" id="mcps1.1.4.1.1"><p id="p12103102210474"><a name="p12103102210474"></a><a name="p12103102210474"></a>工具名称</p>
</th>
<th class="cellrowborder" valign="top" width="29.020000000000003%" id="mcps1.1.4.1.2"><p id="p7103152219475"><a name="p7103152219475"></a><a name="p7103152219475"></a>建议版本</p>
</th>
<th class="cellrowborder" valign="top" width="45.56%" id="mcps1.1.4.1.3"><p id="p0103122104711"><a name="p0103122104711"></a><a name="p0103122104711"></a>下载说明</p>
</th>
</tr>
</thead>
<tbody><tr id="row1310372264718"><td class="cellrowborder" valign="top" width="25.419999999999998%" headers="mcps1.1.4.1.1 "><p id="p121031322124713"><a name="p121031322124713"></a><a name="p121031322124713"></a>python</p>
</td>
<td class="cellrowborder" valign="top" width="29.020000000000003%" headers="mcps1.1.4.1.2 "><p id="p7103522144717"><a name="p7103522144717"></a><a name="p7103522144717"></a>3</p>
</td>
<td class="cellrowborder" valign="top" width="45.56%" headers="mcps1.1.4.1.3 "><p id="p8103122220478"><a name="p8103122220478"></a><a name="p8103122220478"></a>可从官网下载：https://www.python.org/</p>
</td>
</tr>
<tr id="row14103162220476"><td class="cellrowborder" valign="top" width="25.419999999999998%" headers="mcps1.1.4.1.1 "><p id="p1710310221472"><a name="p1710310221472"></a><a name="p1710310221472"></a>pycryptodomex</p>
</td>
<td class="cellrowborder" valign="top" width="29.020000000000003%" headers="mcps1.1.4.1.2 "><p id="p174362313596"><a name="p174362313596"></a><a name="p174362313596"></a>N/A</p>
</td>
<td class="cellrowborder" valign="top" width="45.56%" headers="mcps1.1.4.1.3 "><p id="p1310317225472"><a name="p1310317225472"></a><a name="p1310317225472"></a>可从官网下载：https://pypi.org/project/pycryptodomex/</p>
</td>
</tr>
<tr id="row12103182220470"><td class="cellrowborder" valign="top" width="25.419999999999998%" headers="mcps1.1.4.1.1 "><p id="p010372234713"><a name="p010372234713"></a><a name="p010372234713"></a>openssl</p>
</td>
<td class="cellrowborder" valign="top" width="29.020000000000003%" headers="mcps1.1.4.1.2 "><p id="p1103322124719"><a name="p1103322124719"></a><a name="p1103322124719"></a>N/A</p>
</td>
<td class="cellrowborder" valign="top" width="45.56%" headers="mcps1.1.4.1.3 "><p id="p1110372234718"><a name="p1110372234718"></a><a name="p1110372234718"></a>可从官网下载：https://wiki.openssl.org/index.php/Binaries</p>
</td>
</tr>
</tbody>
</table>

## 编译依赖的bounds\_checking\_function和musl头文件准备<a name="section1182315617500"></a>

分别按照thirdparty/open\_source/bounds\_checking\_function和thirdparty/open\_source/musl下的README指导文档来操作即可。

## 2.1.2 TA编译及开发示例<a name="section87251833133019"></a>

## 2.1.2.1 编译入口示例<a name="section53541258123016"></a>

## 简介<a name="section24399485559"></a>

32位TA是可信应用程序的32位版本，64位TA是可信应用程序的64位版本。

对于32位或者64位TA的支持，不同平台存在差异，请与对应产品或OHTEE负责人确认。

目前支持Make和Cmake两种编译框架。

本小节将通过示例程序，介绍用户开发可信应用程序的过程，包括开发环境准备，SDK示例helloworld TA的编译配置和编译方法等。

## build.sh文件<a name="section18297126898"></a>

build.sh是TA样例编译的入口脚本，此脚本会解析同目录下的defconfig配置文件，可以支撑不同场景的编译功能，包含：不同编译方式（make/cmake）、工具链\(gcc/lvm\)、TA二进制\(32/64位\)的编译场景。

注意此文件与make编译配置、cmake编译配置文件需置于同级目录，对于TA依赖的工具链、头文件路径、安全编译选项等配置在SDK的build/mk和build/cmake目录下，建议新开发TA都参照此样例的编译框架来开发。

```
#!/bin/bash
set -e

export SOURCE_PATH=$(dirname $0)
export ABS_SOURCE_PATH=$(cd ${SOURCE_PATH};pwd)
export TEE_BUILD_PATH=${ABS_SOURCE_PATH}/../../../

build_clean()
{
    [ -d "output" ] && rm -rf output #clean
    make clean
}

get_config()
{
    while read line;do
        eval "$line"
    done < defconfig
}

cmake_build()
{
    echo "start cmake build ${CONFIG_GCC} ${TARGET_IS_ARM64} target"
    build_clean

    mkdir -p output #create cmake_build file
    cd output

    if [ "${CONFIG_GCC}" = "y" ];then
        if [ "${TARGET_IS_ARM64}" = "y" ];then
            CMAKE_TOOLCHAIN_FILE=${TEE_BUILD_PATH}/build/cmake/aarch64_toolchain.cmake
        elif [ "${TARGET_IS_ARM64}" = "n" ];then
            CMAKE_TOOLCHAIN_FILE=${TEE_BUILD_PATH}/build/cmake/arm_toolchain.cmake
        fi
    elif [ "${CONFIG_GCC}" = "n" ];then
        CMAKE_TOOLCHAIN_FILE=${TEE_BUILD_PATH}/build/cmake/llvm_toolchain.cmake
    else
        echo "Invalid tool chain" ; exit -1
    fi

    cmake \
          -DCMAKE_VERBOSE_MAKEFILE=on \
          -DTARGET_IS_ARM64=${TARGET_IS_ARM64} \
          -DCONFIG_GCC=${CONFIG_GCC} \
          -DCMAKE_TOOLCHAIN_FILE=${CMAKE_TOOLCHAIN_FILE} \
          -DTEE_BUILD_PATH=${TEE_BUILD_PATH} \
          ${ABS_SOURCE_PATH}

    cmake --build . -j8
}

mk_build()
{
    echo "start make build ${CONFIG_GCC} ${TARGET_IS_ARM64} target"
    build_clean
    TEE_BUILD_PATH=${TEE_BUILD_PATH} \
    TARGET_IS_ARM64=${TARGET_IS_ARM64} \
    CONFIG_GCC=${CONFIG_GCC} \
    make V=3 sec_binary -j4
}

if [ "${1}" = "clean" ]; then
    build_clean
else
    get_config

    case "$CONFIG_BUILD_TOOL" in
        "") echo "can not find config" ;;
        "cmake")
            echo "do cmake compile"
            cmake_build ;;
        "make")
            echo "do make compile"
            mk_build ;;
        *) echo "do make compile" && exit -1 ;;
    esac
fi
```

## defconfig文件<a name="section885652814360"></a>

defconfig文件可以支撑不同编译方式（make/cmake）、工具链\(gcc/lvm\)、TA二进制\(32/64位\)的编译场景，CONFIG\_BUILD\_TOOL对应编译方式是用make和cmake；CONFIG\_GCC=n是用LLVM工具链编译，否则是用GCC工具链来编译；TARGET\_IS\_ARM64=y对应二进制编译是64位，否则是32位。

TARGET\_IS\_ARM64编译选项控制编译TA的版本，无此变量定义时默认为32位。64位TA只支持动态加载。

>![](public_sys-resources/icon-note.gif) **说明：** 
>如果用GCC工具链编译二进制，编译二进制的位数与GCC是32位还是64位是一致的，如使用32位GCC工具链，则需要定义TARGET\_IS\_ARM64=n，反之64位GCC工具链，需要定义TARGET\_IS\_ARM64=y，否则，编译会异常。说明：TARGET\_IS\_ARM64会控制在SDK包的build/mk和build/cmake目录下的编译框架，包含控制编译依赖的头文件和链接等配置。
>用LLVM工具链，则无此限制。

32位defconfig文件配置示例：

```
#set CONFIG_BUILD_TOOL (cmake / make)
CONFIG_BUILD_TOOL=make

#set CONFIG_GCC (y / n)
CONFIG_GCC=n

#set TARGET_IS_ARM64 (y / n)
TARGET_IS_ARM64=n
```

64位defconfig文件配置示例：

```
#set CONFIG_BUILD_TOOL (cmake / make)
CONFIG_BUILD_TOOL=make

#set CONFIG_GCC (y / n)
CONFIG_GCC=n

#set TARGET_IS_ARM64 (y / n)
TARGET_IS_ARM64=y
```

## configs.xml文件<a name="section1893131414916"></a>

TA开发时需提供configs.xml文本文件。

请参照示例的configs.xml文件格式要求进行的编写。可信应用镜像生成时，如果不存在configs.xml文件，或文件格式错误，可信应用镜像的生成将会终止。64位TA的configs.xml文件格式与32位TA相同。

configs.xml示例：

```
<ConfigInfo>
  <TA_Basic_Info>
    <service_name>demo-ta</service_name>
    <uuid>e3d37f4a-f24c-48d0-8884-3bdd6c44e988</uuid>
  </TA_Basic_Info>
  <TA_Manifest_Info>
    <instance_keep_alive>false</instance_keep_alive>
    <stack_size>8192</stack_size>
    <heap_size>81920</heap_size>
    <multi_command>false</multi_command>
    <multi_session>false</multi_session>
    <single_instance>true</single_instance>
  </TA_Manifest_Info>
</ConfigInfo>
```

当前支持的特性信息如下：

**表 1**  特性信息

<a name="table17621044151218"></a>
<table><thead align="left"><tr id="row16762144410122"><th class="cellrowborder" valign="top" width="25%" id="mcps1.2.5.1.1"><p id="p127627443121"><a name="p127627443121"></a><a name="p127627443121"></a>特性名</p>
</th>
<th class="cellrowborder" valign="top" width="25%" id="mcps1.2.5.1.2"><p id="p6762124420127"><a name="p6762124420127"></a><a name="p6762124420127"></a>类型</p>
</th>
<th class="cellrowborder" valign="top" width="25%" id="mcps1.2.5.1.3"><p id="p11762174411121"><a name="p11762174411121"></a><a name="p11762174411121"></a>描述</p>
</th>
<th class="cellrowborder" valign="top" width="25%" id="mcps1.2.5.1.4"><p id="p13762134441215"><a name="p13762134441215"></a><a name="p13762134441215"></a>默认</p>
</th>
</tr>
</thead>
<tbody><tr id="row1476234471211"><td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.1 "><p id="p18762344111213"><a name="p18762344111213"></a><a name="p18762344111213"></a>uuid</p>
</td>
<td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.2 "><p id="p499191714143"><a name="p499191714143"></a><a name="p499191714143"></a>UUID</p>
</td>
<td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.3 "><p id="p2099121714144"><a name="p2099121714144"></a><a name="p2099121714144"></a>服务标识，这个特性是必需的。特性值为用户自己生成的UUID。需要和对应的CA中UUID相同</p>
</td>
<td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.4 "><p id="p69971716141"><a name="p69971716141"></a><a name="p69971716141"></a>必填信息，无默认值</p>
</td>
</tr>
<tr id="row1076214441214"><td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.1 "><p id="p20762174491211"><a name="p20762174491211"></a><a name="p20762174491211"></a>single_instance</p>
</td>
<td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.2 "><p id="p399717171417"><a name="p399717171417"></a><a name="p399717171417"></a>Boolean</p>
</td>
<td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.3 "><p id="p1799181721415"><a name="p1799181721415"></a><a name="p1799181721415"></a>系统是否给每一个客户端会话创建一个实例 (当前 OHTEE只支持singleInstance为TRUE的场景)</p>
</td>
<td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.4 "><p id="p69971731418"><a name="p69971731418"></a><a name="p69971731418"></a>TRUE</p>
</td>
</tr>
<tr id="row27632447128"><td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.1 "><p id="p1763194410120"><a name="p1763194410120"></a><a name="p1763194410120"></a>multi_session</p>
</td>
<td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.2 "><p id="p1599111761413"><a name="p1599111761413"></a><a name="p1599111761413"></a>Boolean</p>
</td>
<td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.3 "><p id="p199981731419"><a name="p199981731419"></a><a name="p199981731419"></a>一个服务实例是否支持多个会话。对于多实例服务，该特性被忽略</p>
</td>
<td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.4 "><p id="p1099131791411"><a name="p1099131791411"></a><a name="p1099131791411"></a>FALSE</p>
</td>
</tr>
<tr id="row147639447121"><td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.1 "><p id="p7763194410123"><a name="p7763194410123"></a><a name="p7763194410123"></a>instance_keep_alive</p>
</td>
<td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.2 "><p id="p1399121714149"><a name="p1399121714149"></a><a name="p1399121714149"></a>Boolean</p>
</td>
<td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.3 "><p id="p1599191751419"><a name="p1599191751419"></a><a name="p1599191751419"></a>服务实例是否必须由第一个客户端会话来创建，由最后一个客户端会话去关闭。如果特性值为FALSE，表示服务实例在启动时创建，安全世界shutdown时销毁。多实例时，该特性被忽略</p>
</td>
<td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.4 "><p id="p29917174143"><a name="p29917174143"></a><a name="p29917174143"></a>FALSE</p>
</td>
</tr>
<tr id="row1976384419123"><td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.1 "><p id="p127631144171218"><a name="p127631144171218"></a><a name="p127631144171218"></a>heap_size</p>
</td>
<td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.2 "><p id="p2991417111415"><a name="p2991417111415"></a><a name="p2991417111415"></a>Integer</p>
</td>
<td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.3 "><p id="p1399117111412"><a name="p1399117111412"></a><a name="p1399117111412"></a>每个实例分配的堆空间字节大小。当入口函数SRVXCreate得到调用，保正config.s.heap_size大小的堆空间分配给进程，如果该堆空间分配不成功，进程不会启动</p>
</td>
<td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.4 "><p id="p129941731416"><a name="p129941731416"></a><a name="p129941731416"></a>16384(16KB)</p>
</td>
</tr>
<tr id="row376344431217"><td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.1 "><p id="p1576318445126"><a name="p1576318445126"></a><a name="p1576318445126"></a>stack_size</p>
</td>
<td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.2 "><p id="p1998173149"><a name="p1998173149"></a><a name="p1998173149"></a>Integer</p>
</td>
<td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.3 "><p id="p9991917111415"><a name="p9991917111415"></a><a name="p9991917111415"></a>栈空间大小。由系统创建的进程在服务的入口点将会有该大小的栈空间，服务产生的线程可能会有特殊的栈空间大小</p>
</td>
<td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.4 "><p id="p20991917151414"><a name="p20991917151414"></a><a name="p20991917151414"></a>2048（2KB）</p>
</td>
</tr>
<tr id="row3763114418127"><td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.1 "><p id="p376314416122"><a name="p376314416122"></a><a name="p376314416122"></a>service_name</p>
</td>
<td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.2 "><p id="p99916175146"><a name="p99916175146"></a><a name="p99916175146"></a>String</p>
</td>
<td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.3 "><p id="p9991817141419"><a name="p9991817141419"></a><a name="p9991817141419"></a>可信应用名，字符串长度不超过64字符，仅支持数字、字母，'_'和'-'</p>
</td>
<td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.4 "><p id="p16992173146"><a name="p16992173146"></a><a name="p16992173146"></a>必填信息，无默认值</p>
</td>
</tr>
<tr id="row1476364411123"><td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.1 "><p id="p9763844151217"><a name="p9763844151217"></a><a name="p9763844151217"></a>multi_command</p>
</td>
<td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.2 "><p id="p999111710143"><a name="p999111710143"></a><a name="p999111710143"></a>Boolean</p>
</td>
<td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.3 "><p id="p59919177149"><a name="p59919177149"></a><a name="p59919177149"></a>预留字段，当前不支持，只能填flase</p>
</td>
<td class="cellrowborder" valign="top" width="25%" headers="mcps1.2.5.1.4 "><p id="p599121701413"><a name="p599121701413"></a><a name="p599121701413"></a>false</p>
</td>
</tr>
</tbody>
</table>

## 2.1.2.2 make编译配置示例<a name="section169056453214"></a>

>![](public_sys-resources/icon-note.gif) **说明：** 
>请使用build.sh来触发编译，不要直接用make触发Makefile编译。

## config.mk文件<a name="section613734445614"></a>

config.mk文件是TA编译的配置文件，关键编译选项可以在config.mk中统一配置和管理，config.mk需要在Makefile的文件开头添加include命令。

**注意事项：**

对于config.mk中配置信息，TA源码编译和TA签名都会检查其中配置信息，修改后需要重新对TA编译和签名。通过build.sh脚本触发编译时，config.mk文件里定义的TARGET\_IS\_ARM64会不生效，生效的配置位置是config文件里的TARGET\_IS\_ARM64定义，相反不通过build.sh触发编译时（不建议），才会用到config.mk文件里的TARGET\_IS\_ARM64定义。

config.mk示例：

```
# API_LEVEL which indicates the GP API version of TA
# API_LEVEL=2 indicates GP 1.1.1 which is the current version of the partner
# API_LEVEL=3 indicates GP 1.2 which is the version we both going to support
# If no API_LEVEL is specified, API of GP 1.1.1 will be taken
CFLAGS += -DAPI_LEVEL=2
ifeq ($(TARGET_IS_ARM64),)
    TARGET_IS_ARM64 = n
endif
```

## Makefile文件<a name="section973417201288"></a>

Makefile文件参考SDK 中test/TA/helloworld目录下示例内容，上述config.mk中配置可直接在Makefile里指定。在编译TA 二进制libcombione.so时，需在target里调用ta\_entry\_check.sh \(build/tools目录下\) 检查so合法性。详细编写参考makefile文件。

**注意事项：**

1.  TEE上加载的SEC文件需要携带perm\_config签名，需要先按照[SEC文件签名步骤](#sec文件签名步骤)的指导，签发出perm\_config文件（样例中已经提供，新开发TA需要重新按照[SEC文件签名步骤](#sec文件签名步骤)指导签发），且perm\_config文件的路径和对应TA证书私钥配置在config\_ta\_public.ini文件。
2.  样例中Makefile文件里会编译出二进制libcombine.so后并调用signtool\_sec.py脚本签出sec文件。signtool\_sec.py脚本约束TA二进制编译产物名称需要是libcombione.so，否则签名会异常。

假设可信应用程序包含源文件ta-demo.c，编写Makefile文件编译生成目标文件libcombine.so，如下所示：

```
include ./config.mk
include ../../../build/mk/common.mk

SRC += $(wildcard ./*.c)

# set header directory
INCLUDEDIR += -I./include

# set target
COBJS := $(SRC:%.c=%.o)
TARGET = $(COBJS)

sec_binary:combine
	python3 -B ${SIGNTOOL_DIR}/signtool_sec.py ${CUR_DIR} ${CUR_DIR} --privateCfg ${SIGNTOOL_DIR}/../config_tee_private_sample.ini --publicCfg ${CUR_DIR}/config_ta_public.ini

# attention: must add ta_entry_check.sh as below to check whether ta is valid
combine: $(TARGET)
	$(LD) $(LDFLAGS) $(TARGET) $(EXTRAO) -o libcombine.so
	bash $(TEE_BUILD_PATH)/build/tools/ta_entry_check.sh $(READELF) $(shell pwd)/libcombine.so n y $(TARGET_IS_ARM64)

src/%.o: ./src/%.c
	$(CC) $(CFLAGS)  $(INCLUDEDIR) -c $< -o $@

%.o: %.c
	$(CC) $(CFLAGS)  $(INCLUDEDIR) -c $< -o $@

clean:
	rm -f $(COBJS) *.so *.sec
```

当需要添加新的源文件编译时，直接加到SRC中编译即可。

## Make编译方法<a name="section197921537291"></a>

修改config文件里的CONFIG\_BUILD\_TOOL为make，执行test/TA/helloworld/目录的build.sh脚本即可。默认进行可信应用编译以及镜像签名，生成以Manifest文件中UUID命名的可信应用镜像文件。

## 2.1.2.3 cmake编译配置示例<a name="section111111007337"></a>

>![](public_sys-resources/icon-note.gif) **说明：** 
>请使用build.sh来触发编译，不要直接用cmake触发CMakelist.txt编译。

## config.cmake文件<a name="section41061567813"></a>

选用CMake编译框架，需增加配置文件config.cmake，该文件是TA编译的配置文件，该文件需要在CMakeLists.txt文件中通过include命令包含。

**注意事项：**

-   对于config.camke中配置信息，TA源码编译和TA签名都会检查其中配置信息，修改后需要重新对TA编译和签名。
-   通过build.sh脚本触发编译时，config.cmake文件里定义的TARGET\_IS\_ARM64会不生效，生效的配置位置是config文件里的TARGET\_IS\_ARM64定义，相反不通过build.sh触发编译时（不建议），才会用到config.cmake文件里的TARGET\_IS\_ARM64定义。

```
# API_LEVEL which indicates the GP API version of TA
# API_LEVEL=2 indicates GP 1.1.1 which is the current version of the partner
# API_LEVEL=3 indicates GP 1.2 which is the version we both going to support
# If no API_LEVEL is specified, API of GP 1.1.1 will be taken
# TEE flags
set(COMMON_CFLAGS -DAPI_LEVEL=2)
if ("${TARGET_IS_ARM64}" STREQUAL "")
    set(TARGET_IS_ARM64 n)
endif()
```

## CMakeList.txt文件<a name="section8649337289"></a>

选用CMake编译框架需要编写CMakeLists.txt文件。

CMakeLists.txt文件参考SDK 中test/TA/helloworld目录下示例内容，上述config.cmake中配置可直接在CMakeLists.txt里指定。在编译TA elf二进制时，需在target里调用ta\_entry\_check.sh \(build/tools目录下\) 检查so合法性。详细编写参考CMakeLists.txt文件。

**注意事项：**

1.  OHTEE上加载的SEC文件需要携带perm\_config签名，需要先按照[SEC文件签名步骤](#sec文件签名步骤)章节的指导，签发出perm\_config文件（样例中已经提供，新开发TA需要重新按照[SEC文件签名步骤](#sec文件签名步骤)指导签发），且perm\_config文件的路径和对应TA证书私钥配置在config\_ta\_public.ini文件。
2.  样例中CMakeList.txt文件里会编译出二进制libcombine.so后并调用signtool\_sec.py脚本签出sec文件。signtool\_sec.py脚本约束TA二进制编译产物名称需要是libcombione.so，否则签名会异常。

假设可信应用程序包含源文件ta-demo.c，编写CMakeLists.txt文件编译生成目标文件libcombine.so，如下所示：

```
cmake_minimum_required(VERSION 3.16 FATAL_ERROR)
project(tee_sdk C)

if (PROJECT_SOURCE_DIR STREQUAL PROJECT_BINARY_DIR)
    message(FATAL_ERROR "Forbid compiling in the source tree")
endif()

include(${CMAKE_CURRENT_SOURCE_DIR}/config.cmake)
include($ENV{TEE_BUILD_PATH}/build/cmake/common.cmake)

set(LIBRARY_OUTPUT_PATH ${CMAKE_CURRENT_SOURCE_DIR})
set(CURRENT_TARGET_SO "combine")

set(SDK_C_SOURCES
    ${SDK_C_SOURCES}
    ta_demo.c
)

add_library(${CURRENT_TARGET_SO} SHARED ${SDK_C_SOURCES})
target_include_directories(${CURRENT_TARGET_SO} PUBLIC ${COMMON_INCLUDES})
target_compile_options(${CURRENT_TARGET_SO} PRIVATE ${COMMON_CFLAGS})
target_link_options(${CURRENT_TARGET_SO} PRIVATE ${COMMON_LDFLAGS})

add_custom_command(
    TARGET ${CURRENT_TARGET_SO} POST_BUILD
    COMMAND sh $ENV{TEE_BUILD_PATH}/build/tools/ta_entry_check.sh ${CMAKE_READELF} ${CMAKE_CURRENT_SOURCE_DIR}/libcombine.so n y ${TARGET_IS_ARM64}
    COMMAND python3 -B $ENV{TEE_BUILD_PATH}/build/signtools/signtool_sec.py ${CMAKE_CURRENT_SOURCE_DIR} ${CMAKE_CURRENT_SOURCE_DIR} --privateCfg $ENV{TEE_BUILD_PATH}/build/config_tee_private_sample.ini --publicCfg ${CMAKE_CURRENT_SOURCE_DIR}/config_ta_public.ini
)
```

当需要添加新的源文件编译时，直接加到SDK\_C\_SOURCES列表中编译即可。

## CMake编译方法<a name="section99081322133314"></a>

修改config文件里的CONFIG\_BUILD\_TOOL为cmake，执行test/TA/helloworld/目录的build.sh脚本即可。默认进行可信应用编译以及镜像签名，生成以Manifest文件中UUID命名的可信应用镜像文件。

## 2.2 SEC文件签名<a name="section175621346143317"></a>

进行安全业务开发前，需要进行密钥、证书申请及签发perm\_config文件，perm\_config文件用于管控TA在TEE中可用资源信息。

## 密钥及证书生成<a name="section12730101601820"></a>

SDK中提供了生成密钥及证书的工具，位于build/keytools路径下。config签名及sec文件签名时使用的签名密钥、相关证书都由该工具本地生成。

>![](public_sys-resources/icon-note.gif) **说明：** 
>keytools工具颁发的证书及生成的密钥均只能用于开发调试，不可商用。

在使用keytools工具前，用户需要对input/profile.ini进行配置，各字段含义如下表所示：

**表 2**  keytools工具配置文件说明

<a name="table47028334258"></a>
<table><thead align="left"><tr id="row5703143311257"><th class="cellrowborder" valign="top" width="18.14%" id="mcps1.2.4.1.1"><p id="p165313404254"><a name="p165313404254"></a><a name="p165313404254"></a>字段</p>
</th>
<th class="cellrowborder" valign="top" width="42.19%" id="mcps1.2.4.1.2"><p id="p17531124082517"><a name="p17531124082517"></a><a name="p17531124082517"></a>取值：含义</p>
</th>
<th class="cellrowborder" valign="top" width="39.67%" id="mcps1.2.4.1.3"><p id="p1953118405256"><a name="p1953118405256"></a><a name="p1953118405256"></a>说明</p>
</th>
</tr>
</thead>
<tbody><tr id="row7703133314258"><td class="cellrowborder" valign="top" width="18.14%" headers="mcps1.2.4.1.1 "><p id="p11703163392514"><a name="p11703163392514"></a><a name="p11703163392514"></a>CAType</p>
</td>
<td class="cellrowborder" valign="top" width="42.19%" headers="mcps1.2.4.1.2 "><a name="ul17131194292615"></a><a name="ul17131194292615"></a><ul id="ul17131194292615"><li>0：Public CA</li><li>1：Private CA</li></ul>
</td>
<td class="cellrowborder" valign="top" width="39.67%" headers="mcps1.2.4.1.3 "><p id="p11703733132516"><a name="p11703733132516"></a><a name="p11703733132516"></a>互通场景下使用Public CA。不互通场景下使用该CA，驱动需要使用Private CA，详见<a href="zh-cn_topic_0000001204664500.md#section1087132913217">CA说明</a>。</p>
</td>
</tr>
<tr id="row16703173315252"><td class="cellrowborder" valign="top" width="18.14%" headers="mcps1.2.4.1.1 "><p id="p147031333182514"><a name="p147031333182514"></a><a name="p147031333182514"></a>CAAlg</p>
</td>
<td class="cellrowborder" valign="top" width="42.19%" headers="mcps1.2.4.1.2 "><a name="ul1773104143012"></a><a name="ul1773104143012"></a><ul id="ul1773104143012"><li>0：RSA</li><li>1：ECDSA</li></ul>
</td>
<td class="cellrowborder" valign="top" width="39.67%" headers="mcps1.2.4.1.3 "><p id="p10703113322514"><a name="p10703113322514"></a><a name="p10703113322514"></a>生成密钥、证书及使用CA的算法类型，perm_config及sec签名使用算法应与此处保持一致。</p>
</td>
</tr>
<tr id="row20703633162519"><td class="cellrowborder" valign="top" width="18.14%" headers="mcps1.2.4.1.1 "><p id="p2703103314259"><a name="p2703103314259"></a><a name="p2703103314259"></a>certType</p>
</td>
<td class="cellrowborder" valign="top" width="42.19%" headers="mcps1.2.4.1.2 "><p id="p470323312512"><a name="p470323312512"></a><a name="p470323312512"></a>1：release cert</p>
</td>
<td class="cellrowborder" valign="top" width="39.67%" headers="mcps1.2.4.1.3 "><p id="p4839134983316"><a name="p4839134983316"></a><a name="p4839134983316"></a>生成的taconfig.der/ta_cert.der的证书类。</p>
</td>
</tr>
<tr id="row11703933152518"><td class="cellrowborder" valign="top" width="18.14%" headers="mcps1.2.4.1.1 "><p id="p144755313259"><a name="p144755313259"></a><a name="p144755313259"></a>secSignKeyLen</p>
</td>
<td class="cellrowborder" valign="top" width="42.19%" headers="mcps1.2.4.1.2 "><a name="ul1968518534318"></a><a name="ul1968518534318"></a><ul id="ul1968518534318"><li>256：ECDSA</li><li>4096/2048：RSA</li></ul>
</td>
<td class="cellrowborder" valign="top" width="39.67%" headers="mcps1.2.4.1.3 "><p id="p18703173332511"><a name="p18703173332511"></a><a name="p18703173332511"></a>生成的用于SEC签名的密钥sign_sec_priv.pem的长度。</p>
</td>
</tr>
<tr id="row1169410589259"><td class="cellrowborder" valign="top" width="18.14%" headers="mcps1.2.4.1.1 "><p id="p5524101262616"><a name="p5524101262616"></a><a name="p5524101262616"></a>CN</p>
</td>
<td class="cellrowborder" valign="top" width="42.19%" headers="mcps1.2.4.1.2 "><p id="p152401219267"><a name="p152401219267"></a><a name="p152401219267"></a>uuid_service name：</p>
<p id="p25981217133214"><a name="p25981217133214"></a><a name="p25981217133214"></a>ta_cert.der证书subject CN</p>
</td>
<td class="cellrowborder" valign="top" width="39.67%" headers="mcps1.2.4.1.3 "><p id="p4524212102615"><a name="p4524212102615"></a><a name="p4524212102615"></a>TA的uuid与service name，中间使用“_”连接，与TA的manifest.txt、configs.xml保持一致。</p>
</td>
</tr>
<tr id="row142059182515"><td class="cellrowborder" valign="top" width="18.14%" headers="mcps1.2.4.1.1 "><p id="p16524512152611"><a name="p16524512152611"></a><a name="p16524512152611"></a>OU</p>
</td>
<td class="cellrowborder" valign="top" width="42.19%" headers="mcps1.2.4.1.2 "><p id="p352411212618"><a name="p352411212618"></a><a name="p352411212618"></a>业务设置：ta_cert.der证书subject OU</p>
</td>
<td class="cellrowborder" valign="top" width="39.67%" headers="mcps1.2.4.1.3 "><p id="p8524101219267"><a name="p8524101219267"></a><a name="p8524101219267"></a>需要与certType设置的证书类型一致，release证书需要包含关键次Production，各个词语之间使用“_”连接。</p>
</td>
</tr>
<tr id="row187047335254"><td class="cellrowborder" valign="top" width="18.14%" headers="mcps1.2.4.1.1 "><p id="p35241012102613"><a name="p35241012102613"></a><a name="p35241012102613"></a>pass</p>
</td>
<td class="cellrowborder" valign="top" width="42.19%" headers="mcps1.2.4.1.2 "><p id="p1252411120267"><a name="p1252411120267"></a><a name="p1252411120267"></a>业务设置：用于SEC文件签名密钥sign_sec_priv.pem的口令</p>
</td>
<td class="cellrowborder" valign="top" width="39.67%" headers="mcps1.2.4.1.3 "><p id="p152481282612"><a name="p152481282612"></a><a name="p152481282612"></a>SEC文件签名步骤需要输入该口令。</p>
</td>
</tr>
</tbody>
</table>

配置文件按实际情况填好后，运行如下指令

```
./gen_key_cert.sh
```

keytools工具输出产物将放于output路径下，输出及用途如下表所示：

**表 3**  keytools工具输出说明

<a name="table11862185716131"></a>
<table><thead align="left"><tr id="row586285719132"><th class="cellrowborder" valign="top" width="50%" id="mcps1.2.3.1.1"><p id="p132517171145"><a name="p132517171145"></a><a name="p132517171145"></a>输出</p>
</th>
<th class="cellrowborder" valign="top" width="50%" id="mcps1.2.3.1.2"><p id="p132511711143"><a name="p132511711143"></a><a name="p132511711143"></a>用途</p>
</th>
</tr>
</thead>
<tbody><tr id="row158621857171317"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p82511741416"><a name="p82511741416"></a><a name="p82511741416"></a>taconfig.der</p>
</td>
<td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p1125171711148"><a name="p1125171711148"></a><a name="p1125171711148"></a>Config证书，打包在perm_config中，所有TA共用一份</p>
</td>
</tr>
<tr id="row2862757121314"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p1325017191412"><a name="p1325017191412"></a><a name="p1325017191412"></a>taconfig_key.pem</p>
</td>
<td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p925171771412"><a name="p925171771412"></a><a name="p925171771412"></a>perm_config签名使用的密钥，口令默认为123456</p>
</td>
</tr>
<tr id="row188626575132"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p112516175142"><a name="p112516175142"></a><a name="p112516175142"></a>ta_cert.der</p>
</td>
<td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p72515179141"><a name="p72515179141"></a><a name="p72515179141"></a>TA证书，作为TA身份的唯一标示，打包在perm_config中</p>
</td>
</tr>
<tr id="row6862135719134"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p162581721414"><a name="p162581721414"></a><a name="p162581721414"></a>ta_cert.csr</p>
</td>
<td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p10251017141414"><a name="p10251017141414"></a><a name="p10251017141414"></a>TA证书对应的请求文件</p>
</td>
</tr>
<tr id="row3862115716132"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.1 "><p id="p17252176149"><a name="p17252176149"></a><a name="p17252176149"></a>sign_sec_priv.pem</p>
</td>
<td class="cellrowborder" valign="top" width="50%" headers="mcps1.2.3.1.2 "><p id="p122518177143"><a name="p122518177143"></a><a name="p122518177143"></a>SEC签名使用的密钥</p>
</td>
</tr>
</tbody>
</table>

## perm\_config文件签名<a name="section1023415229205"></a>

-   将keytools工具生成的密钥、证书拷贝到相应的路径下

    ```
    cp keytools/output/taconfig.der pack-Config/config_cert/.
    cp keytools/output/taconfig_key.pem pack-Config/config_cert/.
    cp keytools/output/ta_cert.der pack-Config/ta_cert/.
    ```

-   根据TA实际情况修改input路径下的configs.xml
-   在pack-Config下执行如下指令，密钥口令为123456

    ```
    python3 signtool_config.py ./input ./ta_cert/ta_cert.der ./output/perm_config
    ```

-   在pack-Config/output路径下得到签名产物perm\_config

## SEC文件签名步骤<a name="section193879205219"></a>

以test/TA/helloworld为例：

-   将2.2.2节生成的perm\_config 拷贝到test/TA/helloworld路径下

    ```
    cp build/pack-Config/output/perm_config test/TA/helloworld/.
    ```

-   将keytools工具生成的密钥拷贝到相应的路径下

    ```
    cp build/keytools/output/sign_sec_priv.pem test/TA/helloworld/.
    ```

-   在test/TA/helloworld路径下执行脚本，生成的sec文件位于test/TA/helloworld路径下

    ```
    ./build.sh
    ```


## CA\(Certification Authority\)说明<a name="section1087132913217"></a>

两个二级CA区分互通与不互通两个场景，互通场景下使用public CA，不互通场景下使用private CA。

-   互通：互通场景下二级CA由开放原子基金会统一分配，CA公钥打包在TEE镜像头中，各厂商一致。使用该CA颁发的证书最终签出的SEC文件，在TEE中加载时使用证书中携带的公钥进行验签，所以该场景下的SEC文件可以运行在所有厂商的TEE中。
-   不互通：不互通场景下二级CA由厂商部署，CA公钥打包在TEE镜像头中，各厂商不同。使用该CA颁发的证书最终签出的SEC文件，在TEE中加载时，SEC文件验签需要使用厂商打包在TEE镜像头中的密钥，所以该场景下的SEC文件只能运行在厂商适配的TEE中。

>![](public_sys-resources/icon-note.gif) **说明：** 
>SDK提供的keytools工具中使用的根CA及二级CA均为本地生成，只能用于功能演示，正式的证书服务器需要三方搭建

## 2.3 SEC调试<a name="section1261347344"></a>

当前TEE中日志分为以下5个级别。

<a name="table263813112481"></a>
<table><thead align="left"><tr id="row763951124815"><th class="cellrowborder" valign="top" width="21.052105210521052%" id="mcps1.1.4.1.1"><p id="p463916111482"><a name="p463916111482"></a><a name="p463916111482"></a>日志级别</p>
</th>
<th class="cellrowborder" valign="top" width="45.614561456145616%" id="mcps1.1.4.1.2"><p id="p46393115489"><a name="p46393115489"></a><a name="p46393115489"></a>说明</p>
</th>
<th class="cellrowborder" valign="top" width="33.33333333333333%" id="mcps1.1.4.1.3"><p id="p46391111204818"><a name="p46391111204818"></a><a name="p46391111204818"></a>对应的日志打印接口</p>
</th>
</tr>
</thead>
<tbody><tr id="row563919117484"><td class="cellrowborder" valign="top" width="21.052105210521052%" headers="mcps1.1.4.1.1 "><p id="p18639511124817"><a name="p18639511124817"></a><a name="p18639511124817"></a>VERBOSE</p>
</td>
<td class="cellrowborder" valign="top" width="45.614561456145616%" headers="mcps1.1.4.1.2 "><p id="p66391511134814"><a name="p66391511134814"></a><a name="p66391511134814"></a>细粒度信息事件，详细地记录程序的运行过程。</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.1.4.1.3 "><p id="p13639161117487"><a name="p13639161117487"></a><a name="p13639161117487"></a>tlogv</p>
</td>
</tr>
<tr id="row10639201112487"><td class="cellrowborder" valign="top" width="21.052105210521052%" headers="mcps1.1.4.1.1 "><p id="p10639111184818"><a name="p10639111184818"></a><a name="p10639111184818"></a>DEBUG</p>
</td>
<td class="cellrowborder" valign="top" width="45.614561456145616%" headers="mcps1.1.4.1.2 "><p id="p7558134616494"><a name="p7558134616494"></a><a name="p7558134616494"></a>细粒度信息事件，用于开发调试。</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.1.4.1.3 "><p id="p46391911164815"><a name="p46391911164815"></a><a name="p46391911164815"></a>tlogd</p>
</td>
</tr>
<tr id="row176395115486"><td class="cellrowborder" valign="top" width="21.052105210521052%" headers="mcps1.1.4.1.1 "><p id="p146391611194810"><a name="p146391611194810"></a><a name="p146391611194810"></a>INFO</p>
</td>
<td class="cellrowborder" valign="top" width="45.614561456145616%" headers="mcps1.1.4.1.2 "><p id="p1063911124815"><a name="p1063911124815"></a><a name="p1063911124815"></a>突出强调应用程序的运行过程。</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.1.4.1.3 "><p id="p1461319875015"><a name="p1461319875015"></a><a name="p1461319875015"></a>tlogi</p>
</td>
</tr>
<tr id="row863981134816"><td class="cellrowborder" valign="top" width="21.052105210521052%" headers="mcps1.1.4.1.1 "><p id="p4639201113486"><a name="p4639201113486"></a><a name="p4639201113486"></a>WARNING</p>
</td>
<td class="cellrowborder" valign="top" width="45.614561456145616%" headers="mcps1.1.4.1.2 "><p id="p7639911174815"><a name="p7639911174815"></a><a name="p7639911174815"></a>潜在的错误事件，不影响系统的继续运行。</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.1.4.1.3 "><p id="p9639111115480"><a name="p9639111115480"></a><a name="p9639111115480"></a>tlogw</p>
</td>
</tr>
<tr id="row1639171113485"><td class="cellrowborder" valign="top" width="21.052105210521052%" headers="mcps1.1.4.1.1 "><p id="p14639151117487"><a name="p14639151117487"></a><a name="p14639151117487"></a>ERROR</p>
</td>
<td class="cellrowborder" valign="top" width="45.614561456145616%" headers="mcps1.1.4.1.2 "><p id="p1125856174915"><a name="p1125856174915"></a><a name="p1125856174915"></a>发生错误事件，影响系统的继续运行。</p>
</td>
<td class="cellrowborder" valign="top" width="33.33333333333333%" headers="mcps1.1.4.1.3 "><p id="p1563971113484"><a name="p1563971113484"></a><a name="p1563971113484"></a>tloge</p>
</td>
</tr>
</tbody>
</table>

>![](public_sys-resources/icon-note.gif) **说明：** 
>新开发TA推荐用上述日志接口，不建议使用其他接口。

## 日志使用限制<a name="section413233117432"></a>

1.  日志内存大小是有限的，当日志写入比较快时，会出现日志被覆盖的情况，日志内存采用的是内核申请的PAGES内存，大小是256K。
2.  增加TA日志时，要符合安全规范，不能打印用户隐私信息、内存地址等，不打印调试日志，且一条日志内容要做到字符精简。

## 使用方法<a name="section61772276175810"></a>

在源代码文件中包含tee\_log.h，使用与日志级别对应的日志打印接口打印日志。

日志级别和相应的接口如上面描述，其使用与标准C中的printf类似。

tee\_log.h中默认使用的日志级别为INFO，即默认打印使用tloge、tlogw、tlogi接口的信息。如调试过程中希望打印其他级别，可选择下面方式中的一种进行修改，建议使用方式1：

1.  在Makefile或CMakelist.txt文件中动态修改日志级别TA\_LOG\_LEVEL，如c-flags += -DTA\_LOG\_LEVEL=4 ，具体值和对应的显示级别为：

    <a name="table1136311211099"></a>
    <table><thead align="left"><tr id="row936414211913"><th class="cellrowborder" valign="top" width="50%" id="mcps1.1.3.1.1"><p id="p10364172119913"><a name="p10364172119913"></a><a name="p10364172119913"></a>TA_LOG_LEVEL值</p>
    </th>
    <th class="cellrowborder" valign="top" width="50%" id="mcps1.1.3.1.2"><p id="p1364162113919"><a name="p1364162113919"></a><a name="p1364162113919"></a>日志显示级别</p>
    </th>
    </tr>
    </thead>
    <tbody><tr id="row2036413219915"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.1.3.1.1 "><p id="p93938731415"><a name="p93938731415"></a><a name="p93938731415"></a>0</p>
    </td>
    <td class="cellrowborder" valign="top" width="50%" headers="mcps1.1.3.1.2 "><p id="p1226241161412"><a name="p1226241161412"></a><a name="p1226241161412"></a>Error</p>
    </td>
    </tr>
    <tr id="row5364202112914"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.1.3.1.1 "><p id="p539312714149"><a name="p539312714149"></a><a name="p539312714149"></a>1</p>
    </td>
    <td class="cellrowborder" valign="top" width="50%" headers="mcps1.1.3.1.2 "><p id="p122621511111413"><a name="p122621511111413"></a><a name="p122621511111413"></a>Error、Warn</p>
    </td>
    </tr>
    <tr id="row536413217917"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.1.3.1.1 "><p id="p5393779149"><a name="p5393779149"></a><a name="p5393779149"></a>2--默认值</p>
    </td>
    <td class="cellrowborder" valign="top" width="50%" headers="mcps1.1.3.1.2 "><p id="p1426221181412"><a name="p1426221181412"></a><a name="p1426221181412"></a>Error、Warn、Info</p>
    </td>
    </tr>
    <tr id="row183648211790"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.1.3.1.1 "><p id="p143931576149"><a name="p143931576149"></a><a name="p143931576149"></a>3</p>
    </td>
    <td class="cellrowborder" valign="top" width="50%" headers="mcps1.1.3.1.2 "><p id="p14262191141410"><a name="p14262191141410"></a><a name="p14262191141410"></a>Error、Warn、Info、Debug</p>
    </td>
    </tr>
    <tr id="row236418211891"><td class="cellrowborder" valign="top" width="50%" headers="mcps1.1.3.1.1 "><p id="p3393177121414"><a name="p3393177121414"></a><a name="p3393177121414"></a>&gt;=4</p>
    </td>
    <td class="cellrowborder" valign="top" width="50%" headers="mcps1.1.3.1.2 "><p id="p1526381111143"><a name="p1526381111143"></a><a name="p1526381111143"></a>Error、Warn、Info、Debug、Verbo</p>
    </td>
    </tr>
    </tbody>
    </table>

2.  直接修改tee\_log.h中定义的TA\_LOG\_LEVEL\_DEFAULT为期望的级别。多个TA共用同一个tee\_log.h文件且期望打印的日志级别不一致时，不能选用此方式。
3.  直接修改tee\_log.h中定义的TA\_LOG\_LEVEL为期望的级别。多个TA共用同一个tee\_log.h文件且期望打印的日志级别不一致时，不能选用此方式。

