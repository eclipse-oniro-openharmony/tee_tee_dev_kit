## 开发CA



## 开发TA

### TA安装包

TA安装包不需要跟OpenTrustee镜像打包到一起，可以把TA安装包文件放到到非安全侧文件系统下。在TA被访问时，由TEE Client将TA安装包发送到OpenTrustee系统中加载运行。

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

TA验签：在TA安装包加载到OpenTrustee系统中时，由OpenTrustee TA管理框架对TA安装包做签名验证，验证通过之后才允许该TA加载运行。需要在OpenTrustee系统中预置TA验签的公钥。

**⚠**为了方便社区开发者调试，社区的OpenTrustee SDK开发套件已经预置了TA签名私钥，OpenTrustee系统中预置了验签的公钥。开发者在OpenTrustee商用版本中应自行替换该签名私钥和验签公钥。

### SDK开发套件

#### 介绍

OpenTrustee提供了SDK开发套件支持独立开发TA，该开发套件集成了TA头文件、TA编译框架、TA签名脚本、demo样例等，目录结构如下：

```
├── build
│   ├── config_tee_private_sample.ini     # sec签名和perm_config签名的python脚本的ini配置文件，需要厂商进行配置
│   ├── mk                                # make编译框架
│   ├── pack-TA                           # 提供sec签名能力的shell脚本
│   └── tools                             # 包含sec文件链接的ld文件（32位和64位）、sec文件二进制检查的shell脚本        
├── include
│   └── TA                                # 给TA提供的TEE头文件
├── thirdparty
│   └── open_source
│       ├── import_open_source_header.sh  # 导入TA编译依赖的musl头文件和安全函数库头文件
├── sample
│   ├── CA                                # CA示例代码
│   └── TA                                # TA示例代码
└── CHANGELOG                             # SDK包版本发布记录
├── README.md                             # SDK包的英文说明文件
├── README_zh.md                          # SDK包的中文说明文件
```

- 开发语言：C语言
- 开发环境：linux操作系统

#### 开发套件准备

开发者在使用OpenTrustee SDK开发套件开发TA之前，需要进行一些准备工作。

##### 配置编译工具链

编译工具链获取路径？也可以用OH docker镜像

export PATH=/home/peter/code/openharmony/prebuilts/clang/ohos/linux-x86_64/15.0.4/llvm/bin:$PATH

##### 导入第三方头文件

执行sdk/thirdparty/open_source/import_open_source_header.sh，将TA编译依赖的musl头文件和安全函数库头文件，从OpenHarmony的thirdparty

##### 替换签名密钥



##### 安装python

SDK中用到了python脚本来完成TA的属性配置文件解析、对TA文件进行签名打包等操作，因此需要在开发环境上安装python相关的功能。可能需要root权限。

1、安装python

2、安装python相关的库，

pip install pycryptodome

pip install defusedxml

### TA开发步骤

##### TA属性配置

### TA API