
# tee_tee_dev_kit仓库说明 <a name="ZH-CN_TOPIC_0000001078026808"></a>

-   [术语](#section11660541592)
-   [简介](#section11660541593)
-   [目录](#section161941989596)
-   [使用说明](#section161941989597)
-   [工具依赖](#section11914418405)
-   [使用约束](#section1371113476307)
-   [相关依赖仓](#section1371113476308)

## 术语<a name="section11660541592"></a>

| 缩略语 | 英文                          | 中文         |
| ------ | ----------------------------- | ------------ |
| TEE    | Trusted Execution Environment | 可信执行环境 |
| TA     | Trusted Application           | 可信应用     |
| SDK    | Software Development Kit      | 软件开发包   |

## 简介<a name="section11660541593"></a>

tee_tee_dev_kit仓库主要包含TEE SDK开发套件，支持独立开发TA，包含了TA依赖的头文件、编译框架、签名脚本等。

## 目录<a name="section161941989596"></a>

TEE SDK目录结构如下：

```
├── sdk/build
│   ├── ld                                # 生成TA ELF文件的链接脚本
│   ├── mk                                # TA make编译框架
│   ├── signkey                           # TA签名用的私钥
│   └── tools                             # 生成TA安装包并对TA签名的脚本
├── sysroot
│   └── usr
|       └── include                       # 给TA提供的TEE头文件
├── sdk/src
│   └── TA                                # 放置TA源码
|      └── sample                         # TA示例代码
├── thirdparty
│   └── open_source
│       └── import_open_source_header.sh  # 导入TA编译依赖的musl头文件和安全函数库头文件
└── CHANGELOG                             # SDK包版本发布记录
```

## 使用说明<a name="section161941989597"></a>

开发者在使用TEE SDK开发套件开发TA之前，需要进行一些准备工作。

###  配置编译工具链

TEE使用的编译工具链为llvm，与OpenHarmony一致，开发者需要先下载OpenHarmony编译工具链。

首选下载OpenHarmony [build代码仓](https://gitee.com/openharmony/build)

```
git clone git@gitee.com:openharmony/build.git
```

然后执行该仓中的下载脚本

```
./build/prebuilts_download.sh
```

下载完成后，需要在当前编译环境中声明llvm编译工具链的路径。可通过如下命令声明编译工具链路径：

```
export PATH=openharmony/prebuilts/clang/ohos/linux-x86_64/15.0.4/llvm/bin:$PATH
```

该命令仅是示例，开发者需要指定正确的编译工具链路径。

###  导入第三方头文件

TEE集成了musl库和安全函数库，TA可以使用这些库。TEE SDK并没有默认包含musl库和安全函数库的头文件，但是提供了导入的脚本。 开发者需要先下载[musl库](https://gitee.com/openharmony/third_party_musl)和[安全函数库](https://gitee.com/openharmony/third_party_bounds_checking_function)源码仓：

```
git clone git@gitee.com:openharmony/third_party_musl.git
git clone git@gitee.com:openharmony/third_party_bounds_checking_function.git
```

然后执行

```
./tee_dev_kit/sdk/thirdparty/open_source/import_open_source_header.sh
```

将musl头文件和安全函数库头文件从源码仓导入到TEE SDK中。

### 替换TA签名和验签密钥

TEE SDK中预置了对TA文件进行签名的私钥，该预置私钥只能用来调试，在商用版本中，开发者需要自行替换该私钥。该私钥路径：tee_dev_kit/sdk/build/signkey/ta_sign_priv_key.pem。同时提供了tee_dev_kit/sdk/build/signkey/ta_sign_algo_config.ini脚本，可以用来对签名算法进行配置。默认的签名算法是RSA，密钥长度4096bit。

如果开发者替换了TEE SDK中的签名私钥，需要对应替换OpenTrustee操作系统中的验签公钥，验签公钥的路径：/base/tee/tee_os_framework/lib/syslib/libelf_verify_key/src/common/ta_verify_key.c。

## 工具依赖<a name="section11914418405"></a>

TEE SDK中用到了python脚本来完成TA的属性配置文件解析、对TA文件进行签名等操作，因此需要在开发环境上安装python工具。

1、安装python3及以上版本

2、安装python相关的库，如：

```
pip install pycryptodome

pip install defusedxml
```

如果在编译过程中提示缺少其他python库，需要一并安装。

## 使用约束<a name="section1371113476307"></a>

- 支持开发语言：C语言
- SDK运行环境：linux操作系统
- 未提供代码编辑器

## 相关依赖仓<a name="section1371113476308"></a>

 [OpenHarmony/build](https://gitee.com/openharmony/build)

[OpenHarmony/third_party/musl](https://gitee.com/openharmony/third_party_musl)

[OpenHarmony/third_party/bounds_checking_function](https://gitee.com/openharmony/third_party_bounds_checking_function)
