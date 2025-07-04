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
import getpass
from generate_hash import gen_hash
logging.basicConfig(level=logging.INFO)

HASH256 = 0
HASH512 = 1


def run_cmd(command):
    ret = subprocess.run(command, shell=False, check=True)
    if ret.returncode != 0:
        logging.error("run command failed.")
        sys.exit(1)


def gen_identity():
    env = os.environ
    user_id = env.get("ONLINE_USERNAME")
    password = env.get("ONLINE_PASSWD")
    if user_id == "" or user_id is None:
        logging.critical("Error: Please do like this, or set ONLINE_USERNAME in .bashrc.")
        user_id = input("Please input your account:")
    
    if password == "" or password is None:
        logging.critical("Error: Please do like this, or set ONLINE_PASSWD in .bashrc.")
        password = getpass.getpass("Please input your password:")

    return (user_id, password)


def gen_jar_path():
    env = os.environ
    jar_path = env.get("NATIVE_CA_SIGN_JAR_PATH")
    if jar_path == "" or jar_path is None:
        logging.critical("Set jar tool path")
        jar_path = input("Please input NativeCASign.jar path:")
    return jar_path

def get_add_opens_flag(cfg):
    add_opens_flag = ""
    if cfg.jdk_version == "11":
        add_opens_flag = "--add-opens=java.base/java.lang=ALL-UNNAMED"
    return add_opens_flag

def signed_by_jar_tool(cfg, raw_data, hash_file_path, uuid_str, raw_data_path, out_file_path):
    """ signed by sign server using sign.jar tool """
    (user_id, password) = gen_identity()
    jar_path = gen_jar_path()
    cmd = ""
    add_opens_flag = get_add_opens_flag(cfg)
    if cfg.sign_key_len == "2048":
        gen_hash(cfg.hash_type, raw_data, hash_file_path)
        cmd = 'java %s -jar %s %s %s %s %s %s %s' \
                % (add_opens_flag, jar_path, cfg.server_ip, user_id, password, \
                cfg.sign_key, hash_file_path, out_file_path)
    elif cfg.sign_key_len == "4096":
        if cfg.hash_type == '0': 
            # sha256
            cmd = 'java %s -jar %s -uuid %s \
                    -signAlg SHA256withRSA/PSS %s %s %s %s %s %s' \
                    % (add_opens_flag, jar_path, uuid_str, cfg.server_ip, user_id, password, \
                    cfg.sign_key, raw_data_path, out_file_path)
        else: 
            # sha512
            cmd = 'java %s -jar %s -uuid %s \
                    -signAlg SHA512withRSA/PSS %s %s %s %s %s %s' \
                    % (add_opens_flag, jar_path, uuid_str, cfg.server_ip, user_id, password, \
                    cfg.sign_key, raw_data_path, out_file_path)
    logging.info("sign key len : %s ", cfg.sign_key_len)
    try:
        logging.info("signing...")
        subprocess.check_output(cmd.split(), shell=False)
    except subprocess.CalledProcessError as exception:
        logging.error("sign operation failed %s", exception.output)
        logging.error("native ca sign jar path %s", jar_path)
        exit(0)


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
            logging.info("Sign sec Success")
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
                logging.info("pss sign sec Success")
            else:
                logging.error("padding type %s is not support!", cfg.padding_type)
                exit(0)
    elif cfg.sign_type == '2': # signed by sign server 
        signed_by_jar_tool(cfg, raw_data, hash_file_path, uuid_str, raw_data_path, out_file_path)
    else:
        logging.error("unhandled signtype %s", cfg.sign_type)

    return

