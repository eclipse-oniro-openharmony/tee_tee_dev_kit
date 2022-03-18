# cmake<a name="ZH-CN_TOPIC_0000001078026808"></a>

## 说明
DDK cmake支持使用LLVM和GCC工具链来构建可信应用程序。

在构建可信应用程序之前，请安装工具链，并将工具链路径添加到系统环境中。<br>
例如:<br>
```
export PATH="/path/to/toolchains/clang+llvm/bin/":$PATH
```
或者
```
export PATH="/path/to/gcc-linaro-aarch64-linux-gnu/bin/":$PATH
```


建议使用LLVM，因为它支持更多的安全编译选项，在此DDK中，默认使用LLVM。
