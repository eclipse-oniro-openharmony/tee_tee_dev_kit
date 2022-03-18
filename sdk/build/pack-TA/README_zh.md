# pack-TA<a name="ZH-CN_TOPIC_0000001078026808"></a>

-   [简介](#section1)
-   [目录](#section2)
-   [使用说明](#section3)

## 简介<a name="section1"></a>

提供了sec文件签名的脚本

## 目录<a name="section2"></a>

 使用前，需要将sec文件签名需要的材料拷贝到pack-TA对应的路径下。<br><br/>
拷贝后，目录结构如下所示：
```
├── build_ta.sh                  # sec文件签名脚本
├── input                        # 输入路径
│   ├── config.mk                # TA的config.mk文件
│   ├── config_ta_public.ini     # TA的配置文件
│   ├── libcombine.so            # TA编译后得到的so文件
│   ├── manifest.txt             # TA的manifest.txt文件
│   ├── perm_config              # 签名后的config文件
│   └── sign_sec_priv.pem        # 用于sec文件签名的私钥
├── output                       # 输出路径
├── README.md                    # 英文说明
└── README_zh.md                 # 中文说明
```

## 使用说明<a name="section3"></a>
1. 将keytools生成的签名密钥拷贝到input路径下
```
cp keytools/output/sign_sec_priv.pem pack-TA/input
```
2. 将libcombine.so、manifest.txt、config.mk拷贝到input路径下<br><br/>
3. 将config_ta_public.ini 拷贝到input 路径下<br>
   需要修改config_ta_public.ini中使用到的路径为绝对路径<br>
```
cp test/TA/helloworld/config_ta_public.ini build/pack-TA/input
```
4. 将config签名生成的perm_config拷贝到input路径下
```
cp pack-Config/output/perm_config pack-TA/input
```
5. 执行脚本，签出的sec文件放置于output路径下
```
./build_ta.sh
```