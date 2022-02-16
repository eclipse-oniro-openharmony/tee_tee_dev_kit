#!/usr/bin/env python
# coding=utf-8

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

import struct
import os
import shutil
import stat
import sys
import hashlib
import configparser
import subprocess
import re
import subprocess


def whitelist_check(intput_str):
    if not re.match(r"^[A-Za-z0-9\/\-_.]+$", intput_str):
        return 1
    return 0


def integer_check(input_str):
    if not str(input_str).isdigit():
        return 1
    return 0


def check_cfg(cfg):
    if integer_check(cfg.ca_type):
        print("ca_type is invalid.")
        return 1
    if integer_check(cfg.ca_alg):
        print("ca_alg is invalid.")
        return 1
    if integer_check(cfg.cert_type):
        print("cert_type is invalid.")
        return 1
    if integer_check(cfg.sec_sign_key_len):
        print("sec_sign_key_len is invalid.")
        return 1
    if whitelist_check(cfg.cn):
        print("cn is invalid.")
        return 1
    if whitelist_check(cfg.ou):
        print("ou is invalid.")
        return 1
    if whitelist_check(cfg.password):
        print("pass is invalid.")
        return 1
    return 0


class Configuration:
    ca_type = 0
    ca_alg = 0
    cert_type = 0
    sec_sign_key_len = 0
    sec_cert_type = 0
    cn = ""
    ou = ""
    password = ""

    def __init__(self, file_name):
        parser = configparser.ConfigParser()
        parser.read(file_name)
        self.ca_type = parser.get("config", "CAType")
        self.ca_alg = parser.get("config", "CAAlg")
        self.cert_type = parser.get("config", "certType")
        self.sec_sign_key_len = parser.get("config", "secSignKeyLen")
        self.cn = parser.get("config", "CN")
        self.ou = parser.get("config", "OU")
        self.password = parser.get("config", "pass")


def run_cmd(command):
    ret = subprocess.run(command, shell=False, check=True)
    if ret.returncode != 0:
        print("run command failed.")
        exit()


def get_config_cert(cfg, in_path, out_path):
    cmd = []
    if cfg.ca_type == '0': # ca_public
        if cfg.cert_type == '0': # debug cert
            if (cfg.ca_alg == '0'): # rsa
                srcfile = os.path.join(in_path, \
                    "./../data/newcerts/ca_public/rsa/debug/taconfig.der")
                cmd = ["cp", "-f", srcfile, out_path]
                run_cmd(cmd)
                srcfile = os.path.join(in_path, \
                    "./../data/private/rsa/ca_public/taconfig_key.pem")
                cmd = ["cp", "-f", srcfile, out_path]
                run_cmd(cmd)
            else: # ecdsa
                srcfile = os.path.join(in_path, \
                    "./../data/newcerts/ca_public/ecdsa/debug/taconfig.der")
                cmd = ["cp", "-f", srcfile, out_path]
                run_cmd(cmd)
                srcfile = os.path.join(in_path, \
                    "./../data/private/ecdsa/ca_public/onfig_priv.pem")
                cmd = ["cp", "-f", srcfile, out_path]
                run_cmd(cmd)
        else: # release cert
            if (cfg.ca_alg == '0'): # rsa
                srcfile = os.path.join(in_path, \
                    "./../data/newcerts/ca_public/rsa/release/taconfig.der")
                cmd = ["cp", "-f", srcfile, out_path]
                run_cmd(cmd)
                srcfile = os.path.join(in_path, \
                    "./../data/private/rsa/ca_public/taconfig_key.pem")
                cmd = ["cp", "-f", srcfile, out_path]
                run_cmd(cmd)
            else: # ecdsa
                srcfile = os.path.join(in_path, \
                    "./../data/newcerts/ca_public/ecdsa/release/taconfig.der")
                cmd = ["cp", "-f", srcfile, out_path]
                run_cmd(cmd)
                srcfile = os.path.join(in_path, \
                    "./../data/private/ecdsa/ca_public/taconfig_key.pem")
                cmd = ["cp", "-f", srcfile, out_path]
                run_cmd(cmd)
    else: # ca_private
        if cfg.cert_type == '0': # debug cert
            if (cfg.ca_alg == '0'): # rsa
                srcfile = os.path.join(in_path, \
                    "./../data/newcerts/ca_private/rsa/debug/taconfig.der")
                cmd = ["cp", "-f", srcfile, out_path]
                run_cmd(cmd)
                srcfile = os.path.join(in_path, \
                    "./../data/private/rsa/ca_private/taconfig_key.pem")
                cmd = ["cp", "-f", srcfile, out_path]
                run_cmd(cmd)
            else: # ecdsa
                srcfile = os.path.join(in_path, \
                    "./../data/newcerts/ca_private/ecdsa/debug/taconfig.der")
                cmd = ["cp", "-f", srcfile, out_path]
                run_cmd(cmd)
                srcfile = os.path.join(in_path, \
                    "./../data/private/ecdsa/ca_private/taconfig_key.pem")
                cmd = ["cp", "-f", srcfile, out_path]
                run_cmd(cmd)
        else: # release cert
            if (cfg.ca_alg == '0'): # rsa
                srcfile = os.path.join(in_path, \
                    "./../data/newcerts/ca_private/rsa/release/taconfig.der")
                cmd = ["cp", "-f", srcfile, out_path]
                run_cmd(cmd)
                srcfile = os.path.join(in_path, \
                    "./../data/private/rsa/ca_private/taconfig_key.pem")
                cmd = ["cp", "-f", srcfile, out_path]
                run_cmd(cmd)
            else: # ecdsa
                srcfile = os.path.join(in_path, \
                    "./../data/newcerts/ca_private/ecdsa/release/taconfig.der")
                cmd = ["cp", "-f", srcfile, out_path]
                run_cmd(cmd)
                srcfile = os.path.join(in_path, \
                    "./../data/private/ecdsa/ca_private/taconfig_key.pem")
                cmd = ["cp", "-f", srcfile, out_path]
                run_cmd(cmd)
    return


def gen_sec_key_and_cert(cfg, in_path, out_path):
    # temporary files
    temp_path = os.path.join(out_path, "./../data/temp")
    shutil.rmtree(temp_path, ignore_errors=True)
    os.mkdir(temp_path)
    os.chmod(temp_path, stat.S_IRWXU)
    serial_file = os.path.join(temp_path, "serial")
    fd_serial = os.open(serial_file, os.O_WRONLY | os.O_CREAT, \
        stat.S_IWUSR | stat.S_IRUSR)
    serial_op = os.fdopen(fd_serial, "wb")
    serial_init = '1000'
    serial_op.write(serial_init.encode())
    serial_op.close()

    # to generate keypair for sec sign and verify
    if cfg.ca_type == '1': # ca private
        if cfg.ca_alg == '0': # rsa
            srcfile = os.path.join(in_path, \
                "./../data/private/rsa/ca_private/sign_sec_priv.pem")
            cmd = ["cp", "-f", srcfile, out_path]
            run_cmd(cmd)
        else: # ecdsa
            srcfile = os.path.join(in_path, \
                "./../data/private/ecdsa/ca_private/sign_sec_priv.pem")
            cmd = ["cp", "-f", srcfile, out_path]
            run_cmd(cmd)
    else: # ca_public
        print("ca public use the key in ta_cert.der to verify")

    # generate private key and ta_cert.der
    if (cfg.ca_alg == '0'): # rsa
        outfile = os.path.join(out_path, "ta_cert_priv.pem")
        cmd = "openssl genrsa -aes256 -passout pass:{} -out {} {}" .\
            format(cfg.password, outfile, cfg.sec_sign_key_len)
        try:
            subprocess.check_output(cmd.split(), shell=False)
        except Exception:
            print("generate ta cert private key failed")
            raise RuntimeError

        temp_ou = cfg.ou
        temp_cn = cfg.cn
        temp_sub = "/C=CN/L=beijing_test/O=test/OU=" + temp_ou + "/CN=" \
            + temp_cn
        dest_file = os.path.join(out_path, "ta_cert.csr")
        cmd = "openssl req -new -key {} -out {} -subj {} -passin pass:{}" .\
            format(outfile, dest_file, temp_sub, cfg.password)
        try:
            subprocess.check_output(cmd.split(), shell=False)
        except Exception:
            print("generate ta cert csr failed")
            raise RuntimeError

        openssl_conf_ta = os.path.join(in_path, "./../data/openssl-ta.cnf")
        if cfg.ca_type == '0': # ca_public
            ca_file = os.path.join(in_path, \
                "./../data/certs/rsa/ca_public/ca_public.cer")
            ca_key_file = os.path.join(in_path, \
                "./../data/private/rsa/ca_public/ca_public.pem")
            sign_key = os.path.join(out_path, "sign_sec_priv.pem")
            cmd = ["mv", outfile, sign_key]
            run_cmd(cmd)
        else: # ca_private
            ca_file = os.path.join(in_path, \
                "./../data/certs/rsa/ca_private/ca_private.cer")
            ca_key_file = os.path.join(in_path, \
                "./../data/private/rsa/ca_private/ca_private.pem")
            cmd = ["rm", outfile]
            run_cmd(cmd)
        cert_out_file = os.path.join(out_path, "ta_cert.der")
        cmd = "openssl x509 -req -extfile {} -extensions v3_ca -days 3600 "\
            "-sha256 -CA {} -CAkey {} -CAcreateserial -CAserial {} -in {} "\
            "-out {} -passin pass:{} -outform der" .\
            format(openssl_conf_ta, ca_file, ca_key_file, serial_file, \
                dest_file, cert_out_file, cfg.password)
        try:
            subprocess.check_output(cmd.split(), shell=False)
        except Exception:
            print("generate ta cert failed")
            raise RuntimeError
    else: # ecdsa
        outfile = os.path.join(out_path, "ta_cert_priv.pem")
        cmd = "openssl genpkey -out {} -outform PEM -pass pass:{} "\
            "-aes-256-cbc -algorithm EC -pkeyopt ec_paramgen_curve:"\
            "prime256v1".format(outfile, cfg.password)
        try:
            subprocess.check_output(cmd.split(), shell=False)
        except Exception:
            print("generate ta cert private key failed")
            raise RuntimeError

        temp_ou = cfg.ou
        temp_cn = cfg.cn
        temp_sub = "/C=CN/L=beijing_test/O=test/OU=" + temp_ou + "/CN=" \
            + temp_cn
        dest_file = os.path.join(out_path, "ta_cert.csr")
        cmd = "openssl req -new -key {} -out {} -subj {} -passin pass:{}" .\
            format(outfile, dest_file, temp_sub, cfg.password)
        try:
            subprocess.check_output(cmd.split(), shell=False)
        except Exception:
            print("generate ta cert csr failed")
            raise RuntimeError

        openssl_conf_ta = os.path.join(in_path, "./../data/openssl-ta.cnf")
        if cfg.ca_type == '0': # ca_public
            ca_file = os.path.join(in_path, \
                "./../data/certs/ecdsa/ca_public/ca_public.cer")
            ca_key_file = os.path.join(in_path, \
                "./../data/private/ecdsa/ca_public/ca_public.pem")
            sign_key = os.path.join(out_path, "sign_sec_priv.pem")
            cmd = ["mv", outfile, sign_key]
            run_cmd(cmd)
        else: # ca_private
            ca_file = os.path.join(in_path, \
                "./../data/certs/ecdsa/ca_private/ca_private.cer")
            ca_key_file = os.path.join(in_path, \
                "./../data/private/ecdsa/ca_private/ca_private.pem")
            cmd = ["rm", outfile]
            run_cmd(cmd)
        cert_out_file = os.path.join(out_path, "ta_cert.der")
        cmd = "openssl x509 -req -extfile {} -extensions v3_ca -days 3600 " \
            "-sha256 -CA {} -CAkey {} -CAcreateserial -CAserial {} -in {} " \
            "-out {} -passin pass:{} -outform der" .\
            format(openssl_conf_ta, ca_file, ca_key_file, serial_file, \
                dest_file, cert_out_file, cfg.password)
        try:
            subprocess.check_output(cmd.split(), shell=False)
        except Exception:
            print("generate ta cert failed")
            raise RuntimeError
    # remove temp files
    shutil.rmtree(temp_path)
    return


def gen_data(in_path, out_path):
    cfg = Configuration(os.path.join(in_path, "profile.ini"))
    if check_cfg(cfg):
        print("the configuration file field is incorrect.")
        exit()
    get_config_cert(cfg, in_path, out_path)
    gen_sec_key_and_cert(cfg, in_path, out_path)


def main():
    argvs = sys.argv
    in_path = argvs[1]
    out_path = argvs[2]
    if not os.path.exists(in_path):
        print("input_path does not exist.")
        exit()
    if not os.path.exists(out_path):
        print("out_path does not exist.")
        exit()
    if whitelist_check(in_path):
        print("input_path is incorrect.")
        exit()
    if whitelist_check(out_path):
        print("out_path is incorrect.")
        exit()
    gen_data(in_path, out_path)


if __name__ == '__main__':
    main()
