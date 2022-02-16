# bounds_checking_function头文件发布介绍<a name="ZH-CN_TOPIC_0000001078530726"></a>

-   [依赖仓](#section1371113476307)
-   [头文件发布方法](#section15884114210197)

## 依赖仓<a name="section1371113476307"></a>

[third_party/bounds_checking_function](https://gitee.com/openharmony/third_party_bounds_checking_function)

## 头文件发布方法<a name="section15884114210197"></a>
下载上述bounds_checking_function仓到本地后， 将此软件包里的include整个文件夹拷贝到当前目录。
注意：需要保证bounds_checking_function头文件在SDK包里的路径为：thirdparty/open_source/bounds_checking_function/include/*.h，否则编译可信应用二进制时，build/mk和build/cmake下的编译脚本会报错。
