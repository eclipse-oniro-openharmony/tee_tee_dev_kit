# OpenTrustee概述

## 概述

OpenTrustee提供了一个可信执行环境（TEE），运行在受硬件隔离的安全区域中，与常规操作系统如OpenHarmony同时运行但互相隔离，OpenTrustee比常规操作系统拥有更高的安全性，可以为设备上的机密数据提供保护。

## **应用场景**

在终端设备越来越智能化的今天，人们正在把越来越多的个人应用和数据放到设备上，安全性成为一个很重要的命题。可信执行环境（TEE）已逐渐成为终端设备必备的安全技术。OpenTrustee具备丰富的安全特性，可以支持开发者灵活部署安全应用，应用场景也非常广泛，例如：移动支付、生物认证、版权保护等，也可以为系统安全提供保护，如安全启动、系统完整性检测等。

## **术语**

| 缩略语     | 英文                                     | 中文 |
| ------------ | ------------------------------------------------------------ | --- |
| TEE     | Trusted Execution Environment | 可信执行环境，也称作安全世界 |
| REE     | Rich Execution Environment | 富执行环境，也称作非安全世界 |
| TA       | Trusted Application       | 可信应用 |
| CA     | Client Application   | 客户端应用 |
| SDK | Software Development Kit | 软件开发包 |
| GP | GlobalPlatform | 国际标准组织 |
| ATF | ARM Trusted Firmware | ARM可信固件 |

## 系统架构

OpenTrustee是一套完整的TEE解决方案，包含多个部件，系统架构如同所示：

![](figures/overview-of-opentrustee.png)

各部件基本功能介绍如下：

#### TEE Client

- 部署在REE侧用户态，为CA提供符合GP TEE标准的TEE Client API；
- 内置TEE的代理服务：如日志代理服务，支持获取TEE侧日志并落盘到REE侧文件系统；如安全存储服务，支持获取TEE侧加密的数据并落盘到REE侧文件系统。

#### Tzdriver

- 部署在REE侧内核中的驱动，支持REE和TEE进行通信。

#### OpenTrustee 操作系统框架

- 为TA提供符合GP TEE标准的TEE Internal API；
- 提供TA运行管理、驱动运行管理等框架基础服务；
- 提供安全存储、加解密等安全能力。

#### OpenTrustee 操作系统内核

- ChCore微内核，提供IPC/进程管理/内存管理/调度/中断管理/REE和TEE切换等基础内核功能。

#### OpenTrustee 开发套件

- 提供了TA开发套件，包含TA API头文件、编译框架、签名脚本、demo样例等，支持高效开发TA。

#### OpenTrustee Dispatcher

- 在REE和TEE之间交互时完成两个世界上下文的切换。

## 约束

- 支持的系统类型：支持OpenHarmony定义的标准系统，暂不支持轻量系统和小型系统；
- 对硬件的要求：目前只支持具备ARM TrustZone机制的CPU，安全内存至少需要28M；

## 编码规范

[OpenHarmony C语言编程规范](https://gitee.com/openharmony/docs/blob/master/zh-cn/contribute/OpenHarmony-c-coding-style-guide.md)

[OpenHarmony C++语言编程规范](https://gitee.com/openharmony/docs/blob/master/zh-cn/contribute/OpenHarmony-cpp-coding-style-guide.md)

[OpenHarmony C&C++ 安全编程指南](https://gitee.com/openharmony/docs/blob/master/zh-cn/contribute/OpenHarmony-c-cpp-secure-coding-guide.md)

[OpenHarmony 安全设计规范](https://gitee.com/openharmony/docs/blob/master/zh-cn/contribute/OpenHarmony-security-design-guide.md)

## license

遵循Mulan PSL V2协议;
