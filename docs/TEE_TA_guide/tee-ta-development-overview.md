# TEE安全应用开发概述<a name="ZH-CN_TOPIC_0000001241773893"></a>

可信执行环境TEE（Trusted Execution Environment，TEE）基于TrustZone技术提供一个可信执行环境，用户的关键数据或应用在这个可信环境中使用和运行，保护了用户的关键数据和应用不被窃取、滥用和篡改。

## 基本概念<a name="section1335181741015"></a>

-   TrustZone

    在ARM处理器内核的一个安全扩展，该项技术可以保护安全内存、加密块、键盘和屏幕等外设，从而可确保它们免遭软件攻击。

-   安全存储

    安全存储，提供了一个安全的存储环境，加密保存用户的私有数据。

-   REE

    富执行环境（Rich Execution Environment，REE）运行在正常模式下。

-   TEE

    可信执行环境TEE（Trusted Execution Environment，TEE）运行在安全模式下。

-   CA

    正常模式的客户端应用（Client Application，CA）。

-   TA

    安全模式的可信应用（Trusted Application，TA）。


