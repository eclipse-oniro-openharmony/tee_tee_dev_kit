# OpenTrustee调试机制介绍

## 日志
OpenTrustee支持TEE侧打印日志，并在REE侧查看。

### 日志接口

当前TEE中日志分为以下4个级别。

| 日志接口 | 说明                |
| -------- | ------------------- |
| tlogd    | 输出DEBUG级别日志   |
| tlogi    | 输出INFO级别日志    |
| tlogw    | 输出WARNING级别日志 |
| tloge    | 输出ERROR级别日志   |


### 开发实例

在C源文件中包含TEE日志头文件：

```
#include “tee_log.h”
```

打印日志：

```
tloge("invalid operation, err=%d", ret);
```

### 日志使用限制

1.  日志缓存大小是有限的，建议日志内容要做到字符精简，避免打印海量日志的情况。
2.  打印日志信息时，要有安全意识，不能打印用户隐私信息、内存地址等。
3.  如遇TEE内核崩溃，日志可能输出不完整，此时可以通过在TEE内核编译时禁用TLOGGER改为通过串口读写日志来打印完整日志。

### 查看日志

通过`hdc shell`打开命令行窗口，在REE侧输入tlogcat即可查看日志。