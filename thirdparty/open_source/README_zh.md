# libc头文件发布介绍<a name="ZH-CN_TOPIC_0000001078530726"></a>

-   [依赖仓](#section1371113476307)
-   [头文件发布方法](#section15884114210197)

## 依赖仓<a name="section1371113476307"></a>

[third_party/musl](https://gitee.com/openharmony/third_party_musl)

## 头文件发布方法<a name="section15884114210197"></a>
下载上述musl仓到本地后， 将此说明文档目录下的export_libc_header.sh脚本拷贝到musl软件包根目录下，
执行如下命令后，在此目录会生成musl_header_files文件夹，将musl_header_files文件夹下的内容拷贝到SDK包的thirdparty\open_source\musl路径下（此md说明文档目录）即可。
chmod 777 ./export_libc_header.sh
./export_libc_header.sh
注意：需要保证libc头文件在SDK包里的路径为thirdparty/open_source/musl/libc/*和thirdparty/open_source/musl/libc_32，否则编译可信应用二进制时，build/mk和build/cmake下的编译脚本会报错。
