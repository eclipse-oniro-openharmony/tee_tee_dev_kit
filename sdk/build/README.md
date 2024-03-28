# TA编译指导手册<a name="ZH-CN_TOPIC_0000001078026808"></a>

## TA编译与签名指导说明

```
teekit
├── cmake                             # TA cmake编译框架
├── config                            # TA签名使用的配置文件(包括TA配置信息、TA签名公钥配置文件、TA签名私钥配置文件)
├── ld                                # 生成TA ELF文件的链接脚本
├── mk                                # TA make编译框架
├── script                            # TA签名和证书签名相关脚本
├── TA_demo                           # TA源码编译文件夹
    ├── build_ta.sh                   # TA编译构建脚本
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

