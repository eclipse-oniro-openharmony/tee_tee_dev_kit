
# Introduction to ddk capabilities <a name="EN-US_TOPIC_0000001078026808"></a>

-   [Introduction](#section11660541593)
-   [Content](#section161941989596)
-   [Software Installation](#section11914418405)
-   [Relevant dependency warehouse](#section1371113476307)

## Introduction <a name="section11660541593"></a>

The DDK capabilities include: Driver compilation dependent scripts and header files, driver's ability to issue certificates and sign perm_config signatures, and sec file signatures.

## Content <a name="section161941989596"></a>

The warehouse directory structure is as follows:

```
├── build
│   ├── cmake                             # cmake compile framework capabilities
│   ├── config_tee_private_sample.ini     # sec signed and perm_config signed python ini configuration file
│   ├── keytools                          # Driver certificate issuance, and release type perm_config signature and non-interoperable sec file signature capabilities
│   ├── mk                                # make compile framework capabilities
│   ├── pack-Config                       # Signature packaging perm_config capability
│   ├── signtools                         # sec signature capability python script
│   └── tools                             # ld file (32-bit and 64-bit) linked by sec file, sec file binary check shell script 
├── CHANGELOG                             # Changelog of DDK version.
├── include                               # Driver compilation dependent header files
├── README.md                             # English description file of DDK package
├── README_zh.md                          # Chinese description file of DDK package
├── demo                                  # Driver development sample
├── thirdparty
│   └── open_source
│       ├── libboundscheck                # Driver compilation depends on the security function library header file archive path
│       └── musl                          # Driver compile dependent libc header files archive path
└── VERSION                               # DDK package software version number
```

## Software Installation <a name="section11914418405"></a>

Note that this kit can only be used on the linux server and cannot be used on the windows side. And depends on the following software:
1. Install python3 software;
2. Install the pycryptodomex library of python3
3. Export the musl and bounds_checking_function header files, and follow the md instructions under thirdparty\open_source\bounds_checking_function and thirdparty\open_source\musl.

## Relevant dependency warehouse<a name="section1371113476307"></a>

[third_party/bounds_checking_function](https://gitee.com/openharmony/third_party_bounds_checking_function)

[third_party/musl](https://gitee.com/openharmony/third_party_musl)
