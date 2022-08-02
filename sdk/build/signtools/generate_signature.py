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
import subprocess

from generate_hash import gen_hash

HASH256 = 0
HASH512 = 1


def run_cmd(command):
    ret = subprocess.run(command, shell=False, check=True)
    if ret.returncode != 0:
        print("run command failed.")
        exit()


def gen_ta_signature(cfg, uuid_str, raw_data, raw_data_path, hash_file_path, \
    out_file_path, out_path, key_info_data):
    msg_file = out_path + '/temp/config_msg'
    fd_msg = os.open(msg_file, os.O_WRONLY | os.O_CREAT, \
        stat.S_IWUSR | stat.S_IRUSR)
    msg_file_fp = os.fdopen(fd_msg, "wb")
    msg_file_fp.write(raw_data)
    msg_file_fp.close()

    if cfg.sign_alg == "ECDSA":
        if int(cfg.hash_type) == HASH256:
            cmd = ["openssl", "dgst", "-sha256", "-sign", cfg.sign_key, \
                   "-out", out_file_path, msg_file]
        else:
            cmd = ["openssl", "dgst", "-sha512", "-sign", cfg.sign_key, \
                   "-out", out_file_path, msg_file]
        run_cmd(cmd)
        print('Sign sec Success')
        return
    else: # rsa
        if cfg.padding_type == '0': # pkcsv1_5
            gen_hash(cfg.hash_type, raw_data, hash_file_path)
            cmd = "openssl pkeyutl -sign -inkey {} -in {} -out {}".\
                format(cfg.sign_key, hash_file_path, out_file_path)
            try:
                subprocess.check_output(cmd.split(), shell=False)
            except Exception:
                print("sign operation failed")
                raise RuntimeError
        else: # pss
            if cfg.hash_type == '0':
                cmd = "openssl dgst -sign {} -sha256 -sigopt \
                    rsa_padding_mode:pss -sigopt rsa_pss_saltlen:-1 \
                    -out {} {}".format(cfg.sign_key, out_file_path, msg_file)
                try:
                    subprocess.check_output(cmd.split(), shell=False)
                except Exception:
                    print("sign operation failed")
                    raise RuntimeError
            else:
                cmd = "openssl dgst -sign {} -sha512 -sigopt \
                    rsa_padding_mode:pss -sigopt rsa_pss_saltlen:-1 \
                    -out {} {}".format(cfg.sign_key, out_file_path, msg_file)
                try:
                    subprocess.check_output(cmd.split(), shell=False)
                except Exception:
                    print("sign operation failed")
                    raise RuntimeError
    return
