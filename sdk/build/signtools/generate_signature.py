#!/usr/bin/env python
# coding:utf-8

#
# Copyright (C) 2022 Huawei Technologies Co., Ltd.
# Licensed under the Mulan PSL v2.
# You can use this software according to the terms and conditions of the Mulan
# PSL v2.
# You may obtain a copy of Mulan PSL v2 at:
#     http://license.coscl.org.cn/MulanPSL2
# THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
# KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
# NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
# See the Mulan PSL v2 for more details.
#

import os
import stat
import sys
import subprocess
import logging
from generate_hash import gen_hash

HASH256 = 0
HASH512 = 1


def run_cmd(command):
    ret = subprocess.run(command, shell=False, check=True)
    if ret.returncode != 0:
        logging.error("run command failed.")
        sys.exit(1)


def gen_ta_signature(cfg, uuid_str, raw_data, raw_data_path, hash_file_path, \
    out_file_path, out_path, key_info_data):
    fd_raw = os.open(raw_data_path, os.O_WRONLY | os.O_CREAT, \
        stat.S_IWUSR | stat.S_IRUSR)
    raw_fp = os.fdopen(fd_raw, "wb")
    raw_fp.write(raw_data)
    raw_fp.close()

    if cfg.sign_type == '0': # don't sign for debug version
        logging.critical("generate dummy signature for DEBUG version")
        fd_file = os.open(out_file_path, os.O_WRONLY | os.O_CREAT, \
            stat.S_IWUSR | stat.S_IRUSR)
        file_op = os.fdopen(fd_file, "wb")
        file_op.write(str.encode('\0' * 256, encoding = 'utf-8'))
        file_op.close()
    elif cfg.sign_type == '1': # signed with local key
        if cfg.sign_alg == "ECDSA":
            if int(cfg.hash_type) == HASH256:
                cmd = ["openssl", "dgst", "-sha256", "-sign", cfg.sign_key, \
                    "-out", out_file_path, raw_data_path]
            else:
                cmd = ["openssl", "dgst", "-sha512", "-sign", cfg.sign_key, \
                    "-out", out_file_path, raw_data_path]
            run_cmd(cmd)
            logging.critical("Sign sec Success")
        else: # rsa
            if cfg.padding_type == '1': # pss
                if cfg.hash_type == '0': # sha256
                    cmd = "openssl dgst -sign {} -sha256 -sigopt \
                        rsa_padding_mode:pss -sigopt rsa_pss_saltlen:-1 \
                        -out {} {}".format(cfg.sign_key, out_file_path, raw_data_path)
                else: # sha512
                    cmd = "openssl dgst -sign {} -sha512 -sigopt \
                        rsa_padding_mode:pss -sigopt rsa_pss_saltlen:-1 \
                        -out {} {}".format(cfg.sign_key, out_file_path, raw_data_path)
                run_cmd(cmd.split())
                logging.critical("pss sign sec Success")
            else:
                logging.error("padding type %s is not support!", cfg.padding_type)
                exit(0)
    elif cfg.sign_type == '2': # signed by sign server 
        if os.path.exists("../../../internal/signtools/sign_internal.py"):
            sys.path.append("../../../internal/signtools")
            from sign_internal import signed_by_jar_tool
            signed_by_jar_tool(cfg, raw_data, hash_file_path, uuid_str, raw_data_path, out_file_path)
        else:
            logging.error("unhandled signtype %s", cfg.sign_type)
            exit(0)
    else:
        logging.error("unhandled signtype %s", cfg.sign_type)

    return

