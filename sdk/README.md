
# Introduction to sdk capabilities <a name="EN-US_TOPIC_0000001078026808"></a>

-   [Introduction](#section11660541593)
-   [Content](#section161941989596)
-   [Software Installation](#section11914418405)
-   [Relevant dependency warehouse](#section1371113476307)

## Introduction <a name="section11660541593"></a>

The SDK capabilities include: TA compilation dependent scripts and header files, TA's ability to issue certificates and sign perm_config signatures, and sec file signatures.

## Content <a name="section161941989596"></a>

The warehouse directory structure is as follows:

```
├── build
│   ├── cmake                             # cmake compile framework capabilities
│   ├── config_tee_private_sample.ini     # sec signed and perm_config signed python ini configuration file
│   ├── keytools                          # ta certificate issuance, and release type perm_config signature and non-interoperable sec file signature capabilities
│   ├── mk                                # make compile framework capabilities
│   ├── pack-Config                       # Signature packaging perm_config capability
│   ├── pack-TA                           # sec signature capability shell script
│   ├── signtools                         # sec signature capability python script
│   └── tools                             # ld file (32-bit and 64-bit) linked by sec file, sec file binary check shell script 
├── CHANGELOG                             # Changelog of sdk version.
├── include
│   ├── CA                                # CA compilation dependent header files
│   └── TA                                # TA compile dependent header files
├── README.md                             # English description file of sdk package
├── README_zh.md                          # Chinese description file of sdk package
├── src
│   └── TA                                # TA compilation depends on ta_magic.c. When the 32-bit TA includes this definition, you can control the TA to use the tarunner loader to load, otherwise the 32-bit TA uses the taloader loader; 64-bit TA is not involved, and the 64-bit TA uses the tarunner loader.
├── test
│   ├── CA
│   │   ├── aes_demo                      # CA sample corresponding to TA-aes_demo
│   │   ├── helloworld                    # CA sample corresponding to TA-helloworld
│   │   ├── mac_demo                      # CA sample corresponding to TA-mac_demo
│   │   ├── rsa_demo                      # CA sample corresponding to TA-rsa_demo
│   │   └── secstorage_demo               # CA sample corresponding to TA-secstorage_demo
│   └── TA
│       ├── aes_demo                      # TA uses AES-CBC and AES-GCM algorithms to encrypt and decrypt demo
│       ├── helloworld                    # Simple CA2TA communication demo
│       ├── mac_demo                      # TA uses AES-CBC-MAC and AES-CMAC algorithms for message digest calculation demo
│       ├── rsa_demo                      # TA uses RSA algorithm for encryption and decryption and signature verification demo
│       └── secstorage_demo               # TA stores file demo with secure storage capability
├── thirdparty
│   └── open_source
│       ├── libboundscheck                # TA compilation depends on the security function library header file archive path
│       └── musl                          # TA compile dependent libc header files archive path
└── VERSION                               # SDK package software version number
```

## Software Installation <a name="section11914418405"></a>

Note that this kit can only be used on the linux server and cannot be used on the windows side. And depends on the following software:
1. Install python3 software;
2. Install the pycryptodomex library of python3
3. Export the musl and bounds_checking_function header files, and follow the md instructions under thirdparty\open_source\bounds_checking_function and thirdparty\open_source\musl.

## Relevant dependency warehouse<a name="section1371113476307"></a>

[third_party/bounds_checking_function](https://gitee.com/openharmony/third_party_bounds_checking_function)

[third_party/musl](https://gitee.com/openharmony/third_party_musl)
