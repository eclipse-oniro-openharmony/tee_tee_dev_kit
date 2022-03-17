# bounds_checking_functionHeader file distribution introduction<a name="EN-US_TOPIC_0000001078530726"></a>

-   [Dependency warehouse](#section1371113476307)
-   [Header file publishing method](#section15884114210197)

## Dependency warehouse<a name="section1371113476307"></a>

[third_party/bounds_checking_function](https://gitee.com/openharmony/third_party_bounds_checking_function)

## Header file publishing method<a name="section15884114210197"></a>

After downloading the above bounds_checking_function warehouse to the local, copy the entire include folder in this package to the current directory.

Note: It is necessary to ensure that the path of the bounds_checking_function header file in the DDK package is: thirdparty/open_source/bounds_checking_function/include/*.h, otherwise the compilation scripts under build/mk and build/cmake will report an error when compiling the trusted application binary.