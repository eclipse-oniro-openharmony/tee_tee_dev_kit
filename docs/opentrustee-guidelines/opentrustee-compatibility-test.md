## OpenTrustee的验证测试套件说明
OpenTrustee提供了acts兼容性测试套件，CA的具体目录在`test/xts/acts/tee`下，TA以签名后的二进制存放在[《兼容性测试套件标准系统资源目录》](https://gitee.com/openharmony-sig/compatibility/tree/master/test_suite/resource/master/standard%20system/acts/resource)的tee路径下。
| 测试CA     | 测试TA                                                         | 说明 |
| ------------ | ------------------------------------------------------------ | --- |
| tee_test_client_api     | 534d4152-542d-4353-b919-d3016a171fc5.sec | CA与TA通信API |

使用指导如下：
- 进入openharmony根目录，在test/xts/acts目录下执行编译命令，编译CA测试用例。

```Bash
cd test/xts/acts
./build.sh product_name=rk3568 system_size=standard target_subsystem=tee
```

- 参考[《openharmony兼容性测评服务》](https://www.openharmony.cn/certification/document/guid)中的《标准系统应用兼容性测试指导》章节，在Windows工作台上搭建运行环境
- 从 `https://gitee.com/openharmony-sig/compatibility`下载资源文件，将`test_suite/resource/master/standard system/acts`目录下的文件拷贝到`acts/resource`目录下
- 进入acts目录，直接执行run.bat，然后在命令框里输入以下命令运行用例

```Bash
run -l tee_test_client_api
```
- 用例运行过程中，可以使用tlogcat查看TEE侧日志，用例执行的报告和log会在acts/report目录生成