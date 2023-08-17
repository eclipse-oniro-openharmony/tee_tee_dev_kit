# Introduction to libc header file distribution<a name="ZH-CN_TOPIC_0000001078530726"></a>

-   [Dependency warehouse](#section1371113476307)
-   [Header file publishing method](#section15884114210197)

## Dependency warehouse<a name="section1371113476307"></a>

[third_party/musl](https://gitee.com/openharmony/third_party_musl)

## Header file publishing method<a name="section15884114210197"></a>
After downloading the above musl warehouse to the local, copy the export_libc_header.sh script in the documentation directory to the root directory of the musl package,
After executing the following command, the musl_header_files folder will be generated in this directory, and copy the contents of the musl_header_files folder to the thirdparty\open_source\musl path of the SDK package (this md description document directory).
chmod 777 ./export_libc_header.sh
./export_libc_header.sh

Note: It is necessary to ensure that the paths of the libc header files in the SDK package are thirdparty/open_source/musl/libc/* and thirdparty/open_source/musl/libc_32, otherwise when compiling the trusted application binary, the compilation under build/mk and build/cmake The script will report an error.
