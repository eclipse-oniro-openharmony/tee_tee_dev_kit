
# tee_tee_dev_kit #

-[Terminology] (# section11660541592)
-[Introduction] (# section11660541593)
-[Table of Contents] (# section161941989596)
-[Instructions] (# section161941989597)
-[Tool Dependency] (# section11914418405)
-[Using Constraints] (# section1371113476307)
-[Related Dependency Warehouse] (# section1371113476308)

## Terminology ##

| Abbreviations | English | Chinese|
| ------ | ----------------------------- | ------------ |
| TEE | Trusted Execution Environment | Trusted Execution Environment|
| TA | Trusted Application | Trusted Application|
| SDK | Software Development Kit | Software Development Kit|

## Introduction ##

The TEE SDK development kit supports independent development of TA, including TA dependent header files, compilation frameworks, signature scripts, etc.

##Table of Contents<a name="section161941989596"></a>

The directory structure of TEE SDK is as follows:

```
├── sdk/build
│   ├── ld
│   ├── mk
│   ├── signkey
│   └── tools
├── sysroot
│   └── usr
|       └── include
├── sdk/src
│   └── TA
|      └── sample
├── thirdparty
│   └── open_source
│       └── import_open_source_header.sh
└── CHANGELOG
```

##Instructions for use<a name="section161941989597"></a>

Developers need to do some preparation work before using the TEE SDK development kit to develop TA.

###Configure the compilation toolchain

The compilation toolchain used by TEE is LLVM, which is consistent with OpenHarmony. Developers need to download the OpenHarmony compilation toolchain first.

First choice to download OpenHarmony [build code repository]（ https://gitee.com/openharmony/build )

```
git clone  git@gitee.com : openharmony/build.git
```

Then execute the download script in the repository

```
./build/prebuilts_download.sh
```

After downloading, it is necessary to declare the path of the LLVM compilation toolchain in the current compilation environment. The compilation toolchain path can be declared using the following command:

```
export PATH=openharmony/prebuilts/clang/ohos/linux-x86_64/15.0.4/llvm/bin:$PATH
```

This command is only an example, developers need to specify the correct compilation toolchain path.

###Import third-party header files

TEE integrates the Musl library and security function library, which TA can use. The TEE SDK does not include header files for the musl library and security function library by default, but provides import scripts. Developers need to download the [musl library] first（ https://gitee.com/openharmony/third_party_musl ）And [Security Function Library]（ https://gitee.com/openharmony/third_party_bounds_checking_function ）Source code repository:

```
git clone  git@gitee.com : openharmony/third_party_musl.git
git clone  git@gitee.com : openharmony/third_party_bounds_checking_function.git
```

Then execute

```
./tee_dev_kit/sdk/thirdparty/open_source/import_open_source_header.sh
```

Import the musl header file and security function library header file from the source code repository into the TEE SDK.

###Replace TA signature and verification key

The TEE SDK has a preset private key for signing TA files, which can only be used for debugging. In the commercial version, developers need to replace this private key themselves. The private key path: tee_dev_kit/sdk/build/signkey/ta_sign_priv_key.pem。 At the same time, it provides the scripts tee_dev-kit/sdk/build/signkey/ta-sign_algonfig.ini, which can be used to configure the signature algorithm. The default signature algorithm is RSA, with a key length of 4096 bits.

If the developer replaces the signature private key in the TEE SDK, they need to correspondingly replace the signature verification public key in the OpenTrustee operating system. The path of the signature verification public key is:/ base/tee/tee_os_framework/lib/syslib/libelf_verify_key/src/common/ta_verify_key.c。

##Tool Dependency<a name="section11914418405"></a>

The TEE SDK uses Python scripts to parse TA's property configuration files, sign TA files, and perform other operations, so Python tools need to be installed in the development environment.

1. Install Python 3 and above versions

2. Install Python related libraries, such as:

```
pip install pycryptodome

pip install defusedxml
```

If there is a prompt during the compilation process that other Python libraries are missing, they need to be installed together.

##Use constraint<a name="section1371113476307"></a>

-Supported development language: C language
-SDK runtime environment: Linux operating system
-No code editor provided

##Related Dependency Warehouse<a name="section1371113476308"></a>

[OpenHarmony/build]( https://gitee.com/openharmony/build )

[OpenHarmony/third_party/musl]( https://gitee.com/openharmony/third_party_musl )

[OpenHarmony/third_party/bounds_checking_function]( https://gitee.com/openharmony/third_party_bounds_checking_function )
