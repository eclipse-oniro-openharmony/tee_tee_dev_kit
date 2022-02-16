
# sdk能力介绍<a name="ZH-CN_TOPIC_0000001078026808"></a>

-   [简介](#section11660541593)
-   [目录](#section161941989596)
-   [软件安装](#section11914418405)
-   [相关依赖仓](#section1371113476307)

## 简介<a name="section11660541593"></a>

SDK能力包含：TA编译依赖的脚本、头文件，TA颁发证书并签发perm_config签名的能力，以及sec文件签名。

## 目录<a name="section161941989596"></a>

仓目录结构如下：

```
├── build
│   ├── cmake                             # cmake编译框架能力
│   ├── config_tee_private_sample.ini     # sec签名和perm_config签名的python的ini配置文件
│   ├── keytools                          # ta证书颁发、以及release类型perm_config签名和不互通的sec文件签名能力
│   ├── mk                                # make编译框架能力
│   ├── pack-Config                       # 签名打包perm_config能力
│   ├── pack-TA                           # sec签名能力的shell脚本
│   ├── signtools                         # sec签名能力的python脚本
│   └── tools                             # sec文件链接的ld文件（32位和64位）、sec文件二进制检查shell脚本        
├── CHANGELOG                             # sdk版本的变更记录。
├── include
│   ├── CA                                # CA编译依赖的头文件
│   └── TA                                # TA编译依赖的头文件
├── README.md                             # sdk包的英文说明文件
├── README_zh.md                          # sdk包的中文说明文件
├── src
│   └── TA                                # TA编译依赖的ta_magic.c，32位TA包含此定义时，可以控制TA走tarunner加载器加载，否则32位TA走taloader加载器；64位TA不涉及，64位TA都走tarunner加载器
├── test
│   ├── CA
│   │   ├── aes_demo                      # 对应TA-aes_demo的CA样例 
│   │   ├── helloworld                    # 对应TA-helloworld的CA样例
│   │   ├── mac_demo                      # 对应TA-mac_demo的CA样例
│   │   ├── rsa_demo                      # 对应TA-rsa_demo的CA样例
│   │   └── secstorage_demo               # 对应TA-secstorage_demo的CA样例
│   └── TA
│       ├── aes_demo                      # TA使用AES-CBC和AES-GCM算法加解密demo
│       ├── helloworld                    # 简单的CA2TA的通信demo
│       ├── mac_demo                      # TA使用AES-CBC-MAC和AES-CMAC算法来进行消息摘要计算demo
│       ├── rsa_demo                      # TA使用RSA算法加解密和签名验签demo
│       └── secstorage_demo               # TA用安全存储能力存储文件demo
├── thirdparty
│   └── open_source
│       ├── libboundscheck                # TA编译依赖的安全函数库头文件归档路径
│       └── musl                          # TA编译依赖的libc头文件归档路径
└── VERSION                               # SDK包软件版本号
```

## 软件安装<a name="section11914418405"></a>

注意，此kit只能在linux服务器上使用，windows侧无法使用。且依赖如下软件：
1.安装python3软件；
2.安装python3的pycryptodomex库
3.导出musl和bounds_checking_function头文件，按照thirdparty\open_source\bounds_checking_function和thirdparty\open_source\musl下的md说明文档操作即可。

## 相关依赖仓<a name="section1371113476307"></a>

[third_party/bounds_checking_function](https://gitee.com/openharmony/third_party_bounds_checking_function)

[third_party/musl](https://gitee.com/openharmony/third_party_musl)
