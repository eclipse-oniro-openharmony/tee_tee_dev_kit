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
import stat
import sys
import hashlib
import shutil
import re
import xml.etree.ElementTree as ET
import subprocess
import configparser
import logging

CONFIG_VERSION = 1
CONFIG_CERT_PATH = './config_cert'

BASE_POLICY_VERSION_OH = 0b101

XML2TLV_PARSE_TOOL_INDEX = 1
#use python parse xml
XML2TLV_PY_VALUE = 1 << XML2TLV_PARSE_TOOL_INDEX


def get_policy_version():
    policy_ver = BASE_POLICY_VERSION_OH | XML2TLV_PY_VALUE
    return policy_ver


def integer_check(intput_str):
    if not str(intput_str).isdigit():
        return 1
    return 0


def whitelist_check(intput_str):
    if not re.match(r"^[A-Za-z0-9\/\-_.]+$", intput_str):
        return 1
    return 0


def run_cmd(command):
    ret = subprocess.run(command, shell=False, check=True)
    if ret.returncode != 0:
        logging.error("run command failed.")
        sys.exit(1)


class LoadConfigHeader:
    str = struct.Struct('IHHIIIIIIIII')

    def __init__(self, data):
        unpacked_data = (LoadConfigHeader.str).unpack(data.encode())
        self.unpacked_data = unpacked_data
        self.magic_num = unpacked_data[0]
        self.version = unpacked_data[1]
        self.policy_version = unpacked_data[2]
        self.context_len = unpacked_data[3]
        self.ta_cert_len = unpacked_data[4]
        self.config_len = unpacked_data[5]
        self.sign_len = unpacked_data[6]
        self.cfg_cert_len = unpacked_data[7]
        self.reserved1 = unpacked_data[8]
        self.reserved2 = unpacked_data[8]
        self.reserved3 = unpacked_data[8]
        self.reserved4 = unpacked_data[8]

    def get_packed_data(self):
        values = [self.magic_num,
                  self.version,
                  self.policy_version,
                  self.context_len,
                  self.ta_cert_len,
                  self.config_len,
                  self.sign_len,
                  self.cfg_cert_len,
                  self.reserved1,
                  self.reserved2,
                  self.reserved3,
                  self.reserved4,
                 ]
        return (LoadConfigHeader.str).pack(*values)


def pkg_config_header(hdr_len, magic_num, version, policy_version,
        context_len, ta_cert_len, config_len, sign_len, cfg_cert_len):
    config_hd_len = hdr_len
    config_hd = LoadConfigHeader('\0' * config_hd_len)
    config_hd.magic_num = magic_num
    config_hd.version = version
    config_hd.policy_version = policy_version
    config_hd.context_len = context_len
    config_hd.ta_cert_len = ta_cert_len
    config_hd.config_len = config_len
    config_hd.sign_len = sign_len
    config_hd.cfg_cert_len = cfg_cert_len
    return config_hd


#----------------------------------------------------------------------------
# generate hash use SHA256
#----------------------------------------------------------------------------
def generate_sha256_hash(in_buf):
    # Initialize a SHA256 object from the Python hash library
    obj = hashlib.sha256()
    # Set the input buffer and return the output digest
    obj.update(in_buf)
    return obj.digest()


def gen_rsa_signature(sign_conf_alg, config_buf, input_path_gen, output_file):
    if sign_conf_alg == "RSA_PSS":
        pri_key = os.path.join(CONFIG_CERT_PATH, "taconfig_key.pem")
        msg_file = os.path.join(input_path_gen, "temp/config_msg")
        fd_msg_file = os.open(msg_file, os.O_WRONLY | os.O_CREAT, \
            stat.S_IWUSR | stat.S_IRUSR)
        msg_file_fp = os.fdopen(fd_msg_file, "wb")
        msg_file_fp.write(config_buf)
        msg_file_fp.close()

        cmd = "openssl dgst -sign {} -sha256 -sigopt rsa_padding_mode:pss \
                -sigopt rsa_pss_saltlen:-1 \
                -out {} {}".format(pri_key, output_file, msg_file)
        try:
            subprocess.check_output(cmd.split(), shell=False)
        except Exception:
            logging.error("sign operation failed")
            raise RuntimeError
        logging.critical("Sign Config with PSS Success")
    else:
        logging.error("Sign Config alg is not support!")
        exit(0)
    return


def gen_ecdsa_signature(config_buf, input_path_gen, output_file):
    msg_file = os.path.join(input_path_gen, "temp/config_msg")
    fd_msg_file = os.open(msg_file, os.O_WRONLY | os.O_CREAT, \
        stat.S_IWUSR | stat.S_IRUSR)
    msg_file_fp = os.fdopen(fd_msg_file, "wb")
    msg_file_fp.write(config_buf)
    msg_file_fp.close()

    pri_key = os.path.join(CONFIG_CERT_PATH, "taconfig_key.pem")
    cmd = ["openssl", "dgst", "-sha256", "-sign", pri_key, \
           "-out", output_file, msg_file]
    run_cmd(cmd)
    logging.critical('Sign Config Success')
    return


def gen_config_sign(sign_conf_alg, input_path_gen, header,
                    config, ta_cert, output_file):
    temp_file = os.path.join(input_path_gen, "temp/file_to_sign")
    fd_temp = os.open(temp_file, os.O_WRONLY | os.O_CREAT, \
        stat.S_IWUSR | stat.S_IRUSR)
    temp_file_fp = os.fdopen(fd_temp, "wb")
    temp_file_fp.write(header)
    temp_file_fp.write(ta_cert)
    temp_file_fp.write(config)
    temp_file_fp.close()

    temp_file_len = os.path.getsize(temp_file)
    with open(temp_file, 'rb') as temp_file_fp:
        config_buf = temp_file_fp.read(temp_file_len)

    if sign_conf_alg == "ECDSA":
        gen_ecdsa_signature(config_buf, input_path_gen, output_file)
    else: # rsa
        gen_rsa_signature(sign_conf_alg, config_buf,
                          input_path_gen, output_file)
    return


def convert_xml2tlv(xml_file, tlv_file, input_path, config_file):
    policy_ver = get_policy_version()
    sys.path.append("../signtools")
    if (policy_ver & (1 << XML2TLV_PARSE_TOOL_INDEX)) == XML2TLV_PY_VALUE:
        if os.path.exists("../../../internalsigntools/tag_parse_dict.csv"):
            sys.path.append("../../../internal/signtools")
            tag_parse_dict_file_path = os.path.realpath("../../../internalsigntools/tag_parse_dict.csv")
        else:
            csv_dir = os.path.realpath(os.path.join(os.getcwd(), '../signtools'))
            tag_parse_dict_file_path = os.path.join(csv_dir, '../signtools/tag_parse_dict.csv')

        from dyn_conf_parser import parser_config_xml
        parser_config_xml(xml_file, tag_parse_dict_file_path, \
            tlv_file, input_path)
        if os.path.isfile(tlv_file):
            logging.critical('convert xml to tlv success')
        else:
            logging.error('convert xml to tlv failed')
            raise RuntimeError
    else:
        logging.error('invlid policy version')
        raise RuntimeError


def creat_temp_folder(input_path_creat):
    temp_path = os.path.join(input_path_creat, 'temp')
    if os.path.exists(temp_path):
        shutil.rmtree(temp_path)

    cmd = ["mkdir", temp_path]
    run_cmd(cmd)
    return


def delete_temp_folder(input_path_delete):
    if os.path.exists(input_path_delete + '/temp'):
        shutil.rmtree(os.path.join(input_path_delete, "temp"))
    if os.path.exists(input_path_delete + '/config_tlv'):
        os.remove(os.path.join(input_path_delete, "config_tlv"))
    return


def check_dyn_perm(xml_config_file, input_path):
    xml_tree = ET.parse(xml_config_file)
    xml_root = xml_tree.getroot()
    drv_perm = None
    for child in xml_root.findall('drv_perm'):
        if child != '':
            drv_perm = child
            if os.path.exists(os.path.join(input_path, 'temp')):
                out_save_file = os.path.join(input_path, \
                    'temp/configs_bak.xml')
                xml_tree.write(out_save_file, encoding="utf-8")
            xml_root.remove(child)
    if drv_perm is not None:
        newtree = ET.ElementTree(drv_perm)
        if os.path.exists(os.path.join(input_path, 'temp')):
            out_file = os.path.join(input_path, 'temp/dyn_perm.xml')
            newtree.write(out_file, encoding="utf-8")
        xml_tree.write(xml_config_file)
        return 1
    return 0


def get_target_type_in_config(config_path, in_path):
    tree = ET.parse(config_path)
    drv_target_type = tree.find('./TA_Manifest_Info/target_type')
    flag = os.O_RDWR | os.O_TRUNC | os.O_CREAT
    mode = stat.S_IWUSR | stat.S_IRUSR
    if drv_target_type is not None:
        if drv_target_type.text == "1":
            ans = "gpd.ta.dynConf:00000\n"
            out_tlv = os.path.join(in_path, 'config_tlv')
            with os.fdopen(os.open(out_tlv, flag, mode), 'w+') as conf:
                conf.write(ans)


class Configuration:
    sign_alg = "RSA_PSS"

    def __init__(self, file_name):
        parser = configparser.ConfigParser()
        parser.read(file_name)
        self.sign_alg = parser.get("signConfigPrivateCfg", "configSignAlg")
        if whitelist_check(self.sign_alg):
            logging.error("configSignAlg is invalid.")
            sys.exit(1)


def pack_signature(signature_path, signature_size):
    add_size = 72 - signature_size
    with open(signature_path, 'rb+') as signature_file:
        signature_buf = signature_file.read(signature_size)
        signature_file.seek(0)
        for _ in range(0, add_size):
            signature_file.write(b'\x00')
        signature_file.write(signature_buf)


def gen_config_section(input_path, cert_path, config_section):

    creat_temp_folder(input_path)
    config_path = input_path + '/../..'
    config_file = os.path.join(config_path, "config_tee_private_sample.ini")
    if not os.path.exists(config_file):
        logging.critical("config_tee_private_sample.ini is not exist.")
        sign_conf_alg = "RSA_PSS"
    else:
        cfg = Configuration(config_file)
        sign_conf_alg = cfg.sign_alg
    #convert xml to tlv
    tlv_dynconf_data = os.path.join(input_path, "config_tlv")
    xml_config_file = os.path.join(input_path, "configs.xml")
    tlv_config_file = os.path.join(input_path, "temp/configs_tlv")
    if check_dyn_perm(xml_config_file, input_path) != 0:
        sys.path.append('../signtools')
        from dyn_conf_parser import parser_dyn_conf
        dyn_conf_xml_file_path = os.path.join(input_path, 'temp/dyn_perm.xml')
        csv_dir = os.path.realpath(os.path.join(os.getcwd(), '../signtools'))
        tag_parse_dict_file_path = \
            os.path.join(csv_dir, '../signtools/tag_parse_dict.csv')
        parser_dyn_conf(dyn_conf_xml_file_path, "", \
            tag_parse_dict_file_path, input_path)
        convert_xml2tlv(xml_config_file, tlv_config_file, \
            input_path, config_file)

        src_file_path = os.path.join(input_path, 'temp/configs_bak.xml')
        cmd = ["mv", src_file_path, xml_config_file]
        run_cmd(cmd)
    else:
        convert_xml2tlv(xml_config_file, tlv_config_file, \
            input_path, config_file)
        get_target_type_in_config(xml_config_file, input_path)

    if os.path.exists(tlv_dynconf_data):
        with open(tlv_config_file, 'rb') as tlv_config_fp:
            tlv_config_buf = \
                tlv_config_fp.read(os.path.getsize(tlv_config_file))
        with open(tlv_dynconf_data, 'rb') as tlv_dynconf_fp:
            tlv_config_buf = tlv_config_buf + \
                tlv_dynconf_fp.read(os.path.getsize(tlv_dynconf_data)) + b"\n"
        config_content_size = len(tlv_config_buf)
    else:
        config_content_size = os.path.getsize(tlv_config_file)
        with open(tlv_config_file, 'rb') as tlv_config_fp:
            tlv_config_buf = tlv_config_fp.read(config_content_size)

    ta_cert_size = os.path.getsize(cert_path)
    with open(cert_path, 'rb') as ta_cert_fp:
        ta_cert_buf = ta_cert_fp.read(ta_cert_size)

    config_cert_path = os.path.join(CONFIG_CERT_PATH, "taconfig.der")
    config_cert_size = os.path.getsize(config_cert_path)
    with open(config_cert_path, 'rb') as config_cert_fp:
        config_cert_buf = config_cert_fp.read(config_cert_size)

    config_hd_len = 44
    if sign_conf_alg == "ECDSA":
        config_sign_size = 72 | 0xC0000000
        final_sign_size = 72
    else: # rsa
        config_sign_size = 512
        final_sign_size = 512
        if sign_conf_alg == "RSA_PSS":
            config_sign_size = config_sign_size | 0x80000000
    config_context_size = config_content_size + ta_cert_size \
            + config_sign_size + config_cert_size
    config_header = pkg_config_header(config_hd_len, 0xABCDABCD, \
            CONFIG_VERSION, get_policy_version(), config_context_size, \
            ta_cert_size, config_content_size, config_sign_size, \
            config_cert_size)
    output_file = os.path.join(input_path, "temp/config_sign")
    gen_config_sign(sign_conf_alg, input_path, \
            config_header.get_packed_data(), \
            tlv_config_buf, ta_cert_buf, output_file)
    config_sign_size = os.path.getsize(output_file)
    if sign_conf_alg == "ECDSA":
        if config_sign_size != final_sign_size:
            pack_signature(output_file, config_sign_size)

    with open(output_file, 'rb') as config_sign_fp:
        config_sign_buf = config_sign_fp.read(final_sign_size)

    fd_config = os.open(config_section, os.O_WRONLY | os.O_CREAT, \
        stat.S_IWUSR | stat.S_IRUSR)
    config_section_fp = os.fdopen(fd_config, "wb")
    #write config header
    config_section_fp.write(config_header.get_packed_data())
    #write ta_cert
    config_section_fp.write(ta_cert_buf)
    #write config content
    config_section_fp.write(tlv_config_buf)
    #write signature
    config_section_fp.write(config_sign_buf)
    #write config cert
    config_section_fp.write(config_cert_buf)
    config_section_fp.close()

    delete_temp_folder(input_path)
    return


if __name__ == '__main__':
    argv_data = sys.argv
    ta_input_path = argv_data[1]
    ta_cert_path = argv_data[2]
    ta_config_section = argv_data[3]

    if not os.path.exists(ta_input_path):
        logging.error("ta_input_path does not exist.")
        exit()
    if not os.path.exists(ta_cert_path):
        logging.error("ta_cert_path does not exist.")
        exit()

    if whitelist_check(ta_input_path):
        logging.error("ta_input_path is incorrect.")
        exit()
    if whitelist_check(ta_cert_path):
        logging.error("ta_cert_path is incorrect.")
        exit()
    if whitelist_check(ta_config_section):
        logging.error("ta_config_section is incorrect.")
        exit()
    gen_config_section(ta_input_path, ta_cert_path, \
            ta_config_section)
