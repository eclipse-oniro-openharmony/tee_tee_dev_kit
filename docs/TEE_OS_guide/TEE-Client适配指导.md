# TEE Client适配指导<a name="ZH-CN_TOPIC_0000001256734441"></a>

-   [TeeClient概述](#section1670413315410)
-   [TeeClient使能实例](#section0405457154914)
-   [可配置选项](#section15305122712509)

## TeeClient概述<a name="section1670413315410"></a>

TeeClient模块向应用提供访问TEE OS接口，让非安全侧应用可以访问TEE OS内运行的安全应用。

TeeClient模块可以分为如下两个部分：

-   TeeClient接口层：为应用访问TEE OS内运行的安全应用提供TeeClient API。

-   TeeClient服务层：
    -   teecd：为TEE OS实现安全存储、时间同步等基础功能，在标准及小型系统使能。
    -   tlogcat：为TEE OS实现日志输出及落盘服务，在标准及小型系统使能。
    -   cadaemon：为CA访问TEE OS驱动层提供服务，仅在标准系统提供，小型系统应用可直接访问驱动层。


代码目录结构：

```
base/tee/tee_client
├── frameworks                         # TeeClient对外接口动态库
│   ├── build
│   │   ├── lite
│   │   └── standard
│   ├── libteec_client
│   └── libteec_vendor
├── interfaces
│   └── libteec
└── services
    ├── cadaemon                       # 为CA访问TEE OS驱动层提供服务，仅在标准系统提供
    │   ├── build
    │   │   └── standard
    │   └── src
    ├── teecd                          # 为TEE OS实现安全存储、时间同步等基础功能，在标准及小型系统使能
    │   ├── build
    │   │   ├── lite
    │   │   └── standard
    │   └── src
    └── tlogcat                        # 为TEE OS实现日志输出及落盘服务，在标准及小型系统使能
        ├── build
        │   ├── lite
        │   └── standard
        └── src
```

## TeeClient使能实例<a name="section0405457154914"></a>

在标准系统及小型系统使能tee\_client，在相应配置json文件中增加tee\_client部件即可：

**标准系统**：

例如在productdefine/common/products/Hi3516DV300.json中增加内容

```
"tee:tee_client": {},
```

**小型系统**：

例如在vendor/hisilicon/hispark\_taurus/config.json中增加内容

```
{
    "subsystem": "tee",
    "components": [
        { "component": "tee_client", "features":[] }                                         
    ]
},
```

## 可配置选项<a name="section15305122712509"></a>

-   TA默认加载路径

    TEE支持从该路径下查找TA文件并加载，在标准系统及小型系统分别配置为system/bin及vendor/bin。

    配置文件：

    标准系统：//base/tee/tee\_client/frameworks/include/standard/tee\_default\_path.h

    小型系统：//base/tee/tee\_client/frameworks/include/lite/tee\_default\_path.h

-   驱动默认加载路径：

    TEE支持动态加载驱动，teecd启动时会从默认路径下读取驱动文件加载进TEE。在标准系统及小型系统分别配置为/system/bin/tee\_dynamic\_drv及vendor/bin/ tee\_dynamic\_drv。

    配置文件：

    标准系统：//base/tee/tee\_client/services/teecd/include/standard/ dynamic\_drv\_dir.h

    小型系统：//base/tee/tee\_client/services/teecd/include/lite/ dynamic\_drv\_dir.h

-   安全存储路径配置：

    TEE支持安全存储功能，安全存储文件放在该路径下，在标准系统及小型系统分别配置为/data/vendor及/storage/data/。

    配置文件：

    标准系统：//base/tee/tee\_client/services/teecd/include/standard/fs\_work\_agent\_define.h

    小型系统：//base/tee/tee\_client/services/teecd/include/lite/fs\_work\_agent\_define.h

-   日志服务配置：

    tlogcat输出tee日志并存盘的位置有两种：

    -   默认输出至系统hilog日志文件，如小型系统上是storage/data/log/hilog\*.txt文件；

    -   可配置输出至私有日志文件， 如小型系统上是storage/data/log/tee路径。

    tlogcat支持的配置功能有：

    -   使能tlogcat输出日志至私有日志文件：

    定义编译宏CONFIG\_TEE\_PRIVATE\_LOGFILE可使能，否则关闭该功能；

    在使能私有日志文件功能基础上，通过TEE\_LOG\_PATH\_BASE宏可定制私有日志文件的存盘路径。

    -   使能tlogcat写系统hilog日志文件的tag拆分：

    定义编译宏CONFIG\_TLOGCAT\_TAG可使能hilog日志中ta日志的tag拆分功能，对应的tag为teeos\_ + "service name"，否则ta的REE侧hilog日志tag都是teeos。

    配置文件：

    标准系统：//base/tee/tee\_client/services/tlogcat/build/standard/BUILD.gn

    小型系统：//base/tee/tee\_client/services/tlogcat/build/lite/BUILD.gn


