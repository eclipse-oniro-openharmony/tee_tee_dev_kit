## 开发TA

### TA安装包

TA安装包不需要跟OpenTrustee镜像打包到一起，可以把TA安装包文件放到非安全侧文件系统下。在TA被访问时，由TEE Client将TA安装包发送到OpenTrustee系统中加载运行。

由于TA安装包放在非安全侧文件系统中，需要对TA安装包做签名，保证安装包的完整性。

#### TA安装包路径

TA安装包放在非安全侧文件系统下，路径有两种选择。

1、将TA安装包命名为UUID.sec，放在/vendor/bin目录或者/system/bin目录，TEE client会在TA被访问时，自动查找该TA对应的UUID.sec，发送到OpenTrustee系统中加载运行。

2、TA安装包可以任意命名并自定义路径，在CA调用TEEC_OpenSession时，通过TEEC_Context的ta_path指定该TA安装包的路径，如xxx/xxx.sec，TEE client会在指定路径查找该安装包，并发送到OpenTrustee系统中加载运行。

#### TA安装包格式

TA安装包是以“.sec”为后缀名的包文件，文件格式如下：包含文件头、签名块、数据区三部分



![](figures/storage-format-of-ta.png)





### TA签名机制

TA签名：由于TA安装包放在非安全侧文件系统中，需要对TA安装包做签名，确保加载到OpenTrutee中的TA安装包没有被篡改。OpenTrustee提供的SDK开发套件中，包含了TA的签名工具，支持对TA安装包一键签名。需要在OpenTrustee SDK开发套件中预置TA签名的私钥。

TA验签：在TA安装包加载到OpenTrustee操作系统中时，由OpenTrustee TA管理框架对TA安装包做签名验证，验证通过之后才允许该TA加载运行。需要在OpenTrustee操作系统中预置TA验签的公钥。

**⚠**为了方便社区开发者调试，社区的OpenTrustee SDK开发套件已经预置了TA签名私钥，OpenTrustee操作系统中预置了验签的公钥。开发者在OpenTrustee商用版本中应自行替换该签名私钥和验签公钥。

### SDK开发套件

#### 介绍

OpenTrustee提供了SDK开发套件支持独立开发TA，该开发套件集成了TA头文件、TA编译框架、TA签名脚本、demo样例等，目录结构如下：

```
├── build
│   ├── ld                                # 生成TA ELF文件的链接脚本
│   ├── mk                                # TA make编译框架
│   ├── signkey                           # TA签名用的私钥
│   └── tools                             # 生成TA安装包并对TA签名的脚本
├── include
│   └── TA                                # 给TA提供的TEE头文件
├── src
│   └── TA                                # 放置TA源码
        └── sample                        # TA示例代码
├── thirdparty
│   └── open_source
│       └── import_open_source_header.sh  # 导入TA编译依赖的musl头文件和安全函数库头文件
└── CHANGELOG                             # SDK包版本发布记录
├── README.md
├── README_zh.md
```

- 开发语言：C语言
- 开发环境：linux操作系统

#### 开发套件准备

开发者在使用OpenTrustee SDK开发套件开发TA之前，需要进行一些准备工作。

##### 配置编译工具链

OpenTrustee使用的编译工具链为llvm，与OpenHarmony一致。开发者需要先下载OpenHarmony编译工具链，并声明该编译工具链的路径。
可通过如下命令声明编译工具链路径：
export PATH=/home/code/openharmony/prebuilts/clang/ohos/linux-x86_64/15.0.4/llvm/bin:$PATH

##### 导入第三方头文件

OpenTrustee集成了musl库和安全函数库，TA可以使用这些库。OpenTrustee SDK并没有默认包含musl库和安全函数库的头文件，但是提供了导入的脚本。
开发者需要先下载musl库和安全函数库：
https://gitee.com/openharmony/third_party_musl

https://gitee.com/openharmony/third_party_bounds_checking_function

然后执行tee_dev_kit/sdk/thirdparty/open_source/import_open_source_header.sh，将musl头文件和安全函数库头文件从源码目录导入到OpenTrustee SDK中。

##### 替换TA签名和验签密钥
OpenTrustee SDK中预置了对TA文件进行签名的私钥，该预置私钥只能用来调试，在商用版本中，开发者需要自行替换该私钥。该私钥路径：
tee_dev_kit/sdk/build/signkey/ta_sign_priv_key.pem。同时提供了tee_dev_kit/sdk/build/signkey/ta_sign_algo_config.ini脚本，可以用来对签名算法
进行配置。默认的签名算法是RSA，密钥长度4096bit。

如果开发者替换了OpenTrustee SDK中的签名私钥，需要对应替换OpenTrustee操作系统中的验签公钥，验签公钥的路径：tee_os_framework/lib/syslib/libelf_verify_key/src/common/ta_verify_key.c

##### 安装python工具

OpenTrustee SDK中用到了python脚本来完成TA的属性配置文件解析、对TA文件进行签名等操作，因此需要在开发环境上安装python工具。

1、安装python3及以上版本

2、安装python相关的库，如：

pip install pycryptodome

pip install defusedxml
如果在编译过程中提示缺少其他python库，需要一并安装

### TA开发步骤


##### TA属性配置

### TA API