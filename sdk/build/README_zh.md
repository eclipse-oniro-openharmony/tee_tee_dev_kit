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

