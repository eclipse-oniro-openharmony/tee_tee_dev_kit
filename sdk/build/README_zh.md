# TA编译指导手册<a name="ZH-CN_TOPIC_0000001078026808"></a>

## TA编译与签名指导说明
### 目录文件介绍
```
teekit
├── cmake                             # TA cmake编译框架
├── config                            # TA签名使用的配置文件(包括TA配置信息、TA签名公钥配置文件、TA签名私钥配置文件)
├── ld                                # 生成TA ELF文件的链接脚本
├── mk                                # TA make编译框架
├── script                            # TA签名和证书签名相关脚本
├── tools                             # CA信息hash值计算工具
├── TA_demo                           # TA源码编译文件夹
    ├── build_ta.sh                   # TA编译构建脚本
    ├── configs.xml                   # TA 配置信息文本文件
    ├── CMakeLists.txt                # TA CMake编译框架配置文件
    ├── Makefile                      # TA Make编译框架配置文件
    └── defconfig                     # TA编译配置选项，包括编译器、编译工具链、TA编译版本、签名版本选择
```

### defconfig文件介绍<a name="section21267445213"></a>

defconfig文件为编译配置文件，该文件中可以指定使用的编译器、编译工具链、指定编译32位或64位。

- CONFIG\_BUILD\_TOOL为指定编译器，其值为make代表使用make编译，其值为cmake代表使用cmake编译。

- CONFIG\_GCC为指定编译工具链，其值为y代表使用GCC编译，其值为n代表使用LLVM编译。

- TARGET\_IS\_ARM64为指定编译版本，其值为y时编译64位驱动，为其值为n时编译32位驱动。

- SIGNATURE\_VER为指定签名版本，其值为V3时TA的签名版本为V3，为其值为V3_1时TA的签名版本为V3.1。

defconfig文件举例如下：

```
#set CONFIG_BUILD_TOOL (cmake / make)
CONFIG_BUILD_TOOL=make

#set CONFIG_GCC (y / n)
CONFIG_GCC=n

#set TARGET_IS_ARM64 (y / n)
TARGET_IS_ARM64=y

#set SIGNATURE_VER (V3 / V3_1 )
SIGNATURE_VER=V3
```

### API_LEVEL配置
API_LEVEL编译选项表示TA的GP API版本，其中当API_LEVEL=2时表示GP 1.1.1，API_LEVEL=3时表示GP 1.2。<br>
CMake编译框架适配CMakeLists.txt文件：<br>
```
# API_LEVEL which indicates the GP API version of TA
# API_LEVEL=2 indicates GP 1.1.1 which is the current version of the partner
# API_LEVEL=3 indicates GP 1.2 which is the version we both going to support
# If no API_LEVEL is specified, API of GP 1.1.1 will be taken
# TEE flags
set(COMMON_CFLAGS -DAPI_LEVEL=2)
```

Make编译框架适配MakeFile文件：
```
CFLAGS += -DAPI_LEVEL=2
```

### MakeFile文件
MakeFile文件参考teekit/TA_demo目录下示例内容。如下所示，表示将源文件放置于TA_demo路径下，编译生成目标文件libcombine.so
```
CFLAGS += -DAPI_LEVEL=2

include ${TEE_OH_BUILD_PATH}/mk/common.mk

SRC += $(wildcard ./*.c)

# set header directory
INCLUDEDIR += -I./include

# set target
COBJS := $(SRC:%.c=%.o)
TARGET = $(COBJS)

libcombine: $(TARGET)
	$(LD) $(LDFLAGS) $(TARGET) $(EXTRAO) -o libcombine.so

src/%.o: ./src/%.c
	$(CC) $(CFLAGS)  $(INCLUDEDIR) -c $< -o $@

%.o: %.c
	$(CC) $(CFLAGS)  $(INCLUDEDIR) -c $< -o $@

clean:
	rm -f $(COBJS) *.o *.so *.sec
```

### CMakeLists.txt文件
选用CMake编译框架需要填写CMakeLists.txt文件，如下所示为teekit/TA_demo目录下MakeLists.txt示例：
```
cmake_minimum_required(VERSION 3.16 FATAL_ERROR)
project(tee_sdk C)

if (PROJECT_SOURCE_DIR STREQUAL PROJECT_BINARY_DIR)
    message(FATAL_ERROR "Forbid compiling in the source tree")
endif()

# API_LEVEL which indicates the GP API version of TA
# API_LEVEL=2 indicates GP 1.1.1 which is the current version of the partner
# API_LEVEL=3 indicates GP 1.2 which is the version we both going to support
# If no API_LEVEL is specified, API of GP 1.1.1 will be taken
# TEE flags
set(COMMON_CFLAGS -DAPI_LEVEL=2)

include($ENV{TEE_OH_BUILD_PATH}/cmake/common.cmake)

set(LIBRARY_OUTPUT_PATH ${CMAKE_CURRENT_SOURCE_DIR})
set(CURRENT_TARGET_SO "combine")

file(GLOB SOURCES *.c)

set(SDK_C_SOURCES
    ${SDK_C_SOURCES}
    ${SOURCES}
)

add_library(${CURRENT_TARGET_SO} SHARED ${SDK_C_SOURCES})
target_include_directories(${CURRENT_TARGET_SO} PUBLIC ${COMMON_INCLUDES})
target_compile_options(${CURRENT_TARGET_SO} PRIVATE ${COMMON_CFLAGS})
target_link_options(${CURRENT_TARGET_SO} PRIVATE ${COMMON_LDFLAGS})
```

### configs.xml文件/manifest.txt文件
TA开发时需要提供TA配置信息文本文件，以configs.xml或manifest.txt两种格式呈现。<br>
manifest.txt示例：
```
gpd.ta.appID:               e3d37f4a-f24c-48d0-8884-3bdd6c44e988
gpd.ta.service_name:        demo-ta
gpd.ta.singleInstance:      true
gpd.ta.multiSession:        false
gpd.ta.instanceKeepAlive:   false
gpd.ta.dataSize:            327680
gpd.ta.stackSize:           32768
```
configs.xml文本文件的编写需要参照示例的xml文件格式要求进行。可信应用镜像生成时，configs.xml文件格式错误，可信应用镜像的生成会终止。64位TA的configs.xml文件格式与32位TA相同。如下为configs.xml示例：
```
<?xml version="1.0" encoding="utf-8"?>
<ConfigInfo>
  <TA_Basic_Info>
      <service_name>demo-ta</service_name>
    <uuid>e3d37f4a-f24c-48d0-8884-3bdd6c44e988</uuid>
  </TA_Basic_Info>
  <TA_Manifest_Info>
    <instance_keep_alive>false</instance_keep_alive>
    <stack_size>32768</stack_size>
    <heap_size>327680</heap_size>
    <multi_session>false</multi_session>
    <single_instance>true</single_instance>
  </TA_Manifest_Info>
</ConfigInfo>
```

## TA签名公私钥对生成与配置
通过openssl生成TA对应签名私钥，并将生成的私钥文件ta_sign_priv_key.pem放置于TA_demo目录下用于TA签名：
```
openssl genrsa -out ta_sign_priv_key.pem 4096
```
并基于生成的私钥文件生成对于的签名公钥文件
```
openssl rsa -in  ta_sign_priv_key.pem -pubout -out ta_verify_pub_key.pem
```
通过以下命令查看验签公钥的详细信息，获取rsa公钥n值（去除Modulus信息中首位00），并替换base/tee/tee_os_frame_work仓下lib/syslib/libelf_verify_key/src/common/openharmony/ta_verify_key.c中全局结构体 g_ta_verify_pub_rsa_key中结构体中数组n的值：
```
openssl rsa -in ta_verify_pub_key.pem -pubin -text -noout
```
如下示例为随机生成的rsa验签公钥Modulus信息：
'''
RSA Public-Key: (4096 bit)
Modulus:
    00:bf:b7:80:7f:89:6b:c4:2d:fa:d3:02:8f:f3:f4:
    54:6e:a2:51:a1:5c:9a:10:85:b2:37:f4:56:73:6f:
    11:af:ff:b7:22:04:4b:44:55:47:f7:39:52:cf:e4:
    0e:ab:19:91:9e:a2:31:65:60:67:39:df:bd:f1:a2:
    57:aa:f7:0f:ff:d4:4e:11:95:9e:8d:88:f0:9b:70:
    37:3a:74:d7:14:8e:85:bf:a4:60:74:54:b7:f3:7d:
    b3:62:b6:bf:f8:6c:18:d3:7e:4e:7d:31:2b:b8:a4:
    09:04:d8:68:a9:08:7e:34:57:47:b0:d7:c6:11:ee:
    e0:23:0d:97:a9:1c:a2:ca:27:e6:cc:8e:aa:ea:5f:
    d5:e2:a2:2e:c3:eb:10:4e:86:bf:ce:b2:c5:0a:39:
    01:62:da:38:f3:8d:11:0e:b8:a2:f1:4d:97:56:d1:
    f2:91:75:06:f4:e0:2b:c6:4f:a9:df:e5:1f:e2:19:
    7a:df:79:6b:39:31:a7:fd:48:5e:72:95:91:23:80:
    ca:fd:e4:2e:f6:87:8e:cb:d8:73:72:5f:7b:c1:5f:
    4f:eb:a0:4e:38:08:ea:e0:5c:e7:ec:e5:21:5e:39:
    11:4d:66:60:95:fc:83:1f:67:6a:46:32:bf:81:b1:
    7f:ad:0d:ed:e7:1e:eb:1e:09:a0:65:da:e4:71:e7:
    42:e7:5c:7f:c3:63:9d:5d:da:3b:59:1d:79:1a:d2:
    f5:3c:c5:51:3a:8b:81:20:f2:33:69:95:0a:03:0b:
    52:a8:85:1c:e6:5d:a2:39:05:9a:73:c7:d6:d9:5a:
    42:19:82:60:a9:56:72:f5:88:f5:fa:f7:e5:14:24:
    b3:9d:93:a4:40:ac:27:ac:6b:83:88:80:68:7e:24:
    57:a4:a7:8d:41:01:3a:bc:a6:c6:cc:a6:8a:91:14:
    d7:ca:bf:85:ea:ac:1a:1a:b5:cc:13:1d:82:20:33:
    2b:55:2f:fd:ac:a5:e3:9e:db:81:af:58:c5:c5:f3:
    07:b3:f6:b4:bf:26:57:a0:51:3e:60:31:88:70:3e:
    af:16:fd:0b:6e:55:2b:d5:38:a7:9d:3a:d0:54:d0:
    cc:30:51:49:c1:a5:05:03:f9:25:35:4d:1c:e1:e0:
    63:2d:c8:47:0a:0c:78:9d:ce:35:bc:66:39:80:52:
    04:6e:7c:47:44:2d:d5:4b:54:18:80:8a:a4:4e:d4:
    69:4b:87:21:92:d1:9f:07:2c:be:02:c9:cf:0a:01:
    35:c1:68:b5:fd:a5:e2:b1:67:bf:b7:4c:32:cf:1f:
    ce:9c:d0:12:bd:00:5c:45:f0:ed:46:27:ca:7d:dd:
    4f:1e:73:2c:f3:1c:72:4e:28:9a:10:3a:45:53:b9:
    b1:87:a1
Exponent: 65537 (0x10001)
'''

## Config证书签名指导说明

在./script/目录下创建新文件夹，目录结构如下：
```
./script
├── config_cert               # 放置taconfig.der（证书）和 taconfig_key.pem（私钥）
├── ta_cert                   # 放置TA/DRV开发者证书ta_cert.der
├── output                    # 生成签名后证书文件perm_config
```

config证书签名操作示例：
```
python3 signtool_config.py ./config ./ta_cert/ta_cert.der ./output/perm_config
```

## 三方头文件导出
可信应用编译依赖三方libc和musl开源头文件，开发者需要预先下载openharmony代码，并于/base/tee/tee_dev_kit/thirdparty/open_source/目录下执行：
```
bash import_open_source_header.sh
```

## CA hash鉴权哈希值生成工具

1、根据应用信息填写ca_caller_info.xml<br>
2、执行calc_ca_caller_hash.py脚本获取CA信息对应哈希值<br>
```
python3  calc_ca_caller_hash.py
```
获取示例如下:
```
16:59:42 line:128 INFO:root:cmdline: /vendor/bin/teec_hello, uid: 0
16:59:42 line:42 INFO:root:caller hash: 0xca 0x9f 0x5e 0xd7 0x6d 0x7 0xd 0x66 0xe7 0xb2 0xab 0xb3 0x55 0xfc 0xb0 0xbf 0xc8 0x16 0x52 0x37 0x5f 0xfe 0x99 0xfc 0x34 0x43 0xf6 0x5f 0xc 0x70 0x44 0x48
16:59:42 line:152 INFO:root:cmdline: /vendor/bin/teec_hello, username: root
16:59:42 line:42 INFO:root:caller hash: 0x29 0x89 0x3c 0x7f 0x92 0xfd 0xce 0x8e 0xd9 0x66 0xfc 0xa0 0x10 0xa0 0xe2 0xa3 0x42 0x31 0x7d 0x65 0x36 0x48 0x38 0x86 0xf1 0x8 0x1b 0x8 0x12 0xc2 0x4c 0x9b
16:59:42 line:180 INFO:root:pkg_name: com.example.myapplicationohdemo, modulue:6f85f3ad5fa01dfcddd4b2f23815f47fc03a68fbe99297be20bd0e114b7e8c54, exponent: 650769812a97c80a3751f015e7b6069492c0fe8de375abe8cdf9188c9c0d3851
16:59:42 line:42 INFO:root:caller hash: 0x5c 0x86 0x46 0xb2 0x11 0xb 0x92 0xa0 0x5a 0x12 0x90 0x8c 0x6a 0x98 0xbe 0xf2 0x10 0x6e 0x65 0x20 0x7b 0xa1 0xc5 0x3e 0x55 0x34 0xac 0x36 0x5a 0xe2 0xf5 0xe9
```

