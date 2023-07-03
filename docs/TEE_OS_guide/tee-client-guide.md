# TEE Client适配指导<a name="ZH-CN_TOPIC_0000001256734441"></a>

## TeeClient概述<a name="section1670413315410"></a>

TeeClient模块在标准系统使能，向应用提供访问TEE OS接口，让非安全侧应用可以访问TEE OS内运行的安全应用。

TeeClient模块可以分为如下两个部分：

-   TeeClient接口层：为芯片组件及系统组件服务访问TEE OS内运行的安全应用提供TeeClient API。

-   TeeClient服务层：
    -   teecd：为TEE OS实现安全存储、时间同步等基础功能，支持芯片组件CA访问TEE OS通道。
    -   tlogcat：为TEE OS实现日志输出及落盘服务。
    -   cadaemon：为系统组件CA访问TEE OS驱动层提供服务。


## TeeClient使能实例<a name="section0405457154914"></a>

在标准系统使能tee\_client，在相应配置json文件中增加tee\_client部件即可：

例如在productdefine/common/products/Hi3516DV300.json中增加内容

```
"tee:tee_client": {},
```

## 可配置选项<a name="section15305122712509"></a>

-   TA默认加载路径

    TEE支持从该路径下查找TA文件并加载，芯片组件和系统组件CA加载TA默认路径分别配置为system/bin及vendor/bin。

    配置文件：

    芯片组件：//base/tee/tee\_client/frameworks/include/standard/teec\_vendor/tee\_default\_path.h

	系统组件：//base/tee/tee\_client/frameworks/include/standard/teec\_system/tee\_default\_path.h

-   驱动默认加载路径：

    TEE支持动态加载驱动，teecd启动时会从默认路径下读取驱动文件加载进TEE。默认配置为vendor/bin/ tee\_dynamic\_drv。

    配置文件：//base/tee/tee\_client/services/teecd/include/standard/ dynamic\_drv\_dir.h

-   安全存储路径配置：

    TEE支持安全存储功能，安全存储文件放在该路径下，配置为/data/vendor。

    配置文件：//base/tee/tee\_client/services/teecd/include/standard/fs\_work\_agent\_define.h

-   日志服务配置：

    tlogcat输出tee日志并存盘的位置有两种：

    -   默认输出至系统hilog日志文件，如data/log/hilog\*.txt文件；

    -   可配置输出至私有日志文件， 如data/log/tee路径。

    tlogcat支持的配置功能有：

    -   使能tlogcat输出日志至私有日志文件：

    定义编译宏CONFIG\_TEE\_PRIVATE\_LOGFILE可使能，否则关闭该功能；

    在使能私有日志文件功能基础上，通过TEE\_LOG\_PATH\_BASE宏可定制私有日志文件的存盘路径。

    -   使能tlogcat写系统hilog日志文件的tag拆分：

    定义编译宏CONFIG\_TLOGCAT\_TAG可使能hilog日志中ta日志的tag拆分功能，对应的tag为teeos\_ + "service name"，否则ta的REE侧hilog日志tag都是teeos。

    配置文件：//base/tee/tee\_client/services/tlogcat/build/standard/BUILD.gn

