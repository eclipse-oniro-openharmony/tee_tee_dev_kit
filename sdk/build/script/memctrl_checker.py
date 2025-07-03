#!/usr/bin/env python
# coding:utf-8

#
# Copyright (C) 2025 Huawei Technologies Co., Ltd.
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
import logging
import re
import xml.etree.ElementTree as ET


def whitelist_check(intput_str):
    """Function check white list."""
    if not re.match(r"^[A-Za-z0-9\/\-_.${}]+$", intput_str):
        return 1
    return 0


def convert_num(field):
    """convert string to int, and return None if input is invalid."""
    return int(field)


def convert_boolean(field):
    """convert string to bool, and return None if input is invalid."""
    if field == "y" or field.lower() == "true":
        return True
    if field == "n" or field.lower() == "false":
        return False
    raise ValueError


def convert_mem_limit(field):
    """convert string and check range for heap_limit/stack_limit."""
    value = convert_num(field)
    if value is None or value <= 0:
        return None
    return value


def check_manifest(manifest):
    """check input manifest format."""
    return manifest.heap_size > 0 and manifest.stack_size > 0 and \
        manifest.instancekeepalive in [0, 1] and manifest.multi_session in [0, 1]


def check_memory_baseline(cfg, uuid_str, manifest_val):
    """check memory baseline in manifest with memctrl config files."""
    if cfg.release_type == "0":
        return True
    if not check_manifest(manifest_val):
        logging.error("Invalid manifest for memory control checking.")
        return False
    if not cfg.memctrl_path:
        logging.error("Invalid path to memory control configuration directory. Please specify --memctrl_path in args.")
        return False
    cfg_dir_path = os.path.realpath(cfg.memctrl_path)
    if not os.path.exists(cfg_dir_path):
        logging.error("memctrl_path does not exist.")
        return False
    if whitelist_check(cfg_dir_path):
        logging.error("memctrl_path contains invalid character.")
        return False
    cfg_file = os.path.join(cfg_dir_path, "%s.xml" % (uuid_str,))
    if not os.path.exists(cfg_file):
        logging.error("Memctrl file %s does not exist." % (cfg_file,))
        return False

    mandatory_fields = {
        'heap_limit': convert_mem_limit,
        'stack_limit': convert_mem_limit,
        'multi_session': convert_boolean,
        'keepalive': convert_boolean,
    }
    memctrl_dict = {}
    try:
        root = ET.parse(cfg_file).getroot()
        for node in root:
            tag, value = node.tag, node.text
            if tag in memctrl_dict:
                logging.error("Memctrl file contains duplicated tag  \"%s\"." % (tag,))
                return False
            if tag in mandatory_fields:
                try:
                    memctrl_dict[tag] = mandatory_fields[tag](value)
                except ValueError:
                    memctrl_dict[tag] = None
    except ET.ParseError:
        logging.error("Failed to parse memctrl XML file.")
        return False
    if any(v is None for v in memctrl_dict.values()):
        for tag in memctrl_dict:
            if memctrl_dict[tag] is None:
                logging.error("Memctrl file contains invalid data in field \"%s\"." % (tag,))
        return False
    if len(memctrl_dict) < len(mandatory_fields):
        missing_keys = mandatory_fields.keys() - memctrl_dict.keys()
        logging.error("Memctrl file missing fields: %s" % (', '.join(missing_keys)))
        return False

    # Check manifest against memctrl config
    if manifest_val.heap_size > memctrl_dict.get('heap_limit', 0):
        logging.error("Too large manifest heap limit (%d) compared to memctrl configuration (%d)"
            % (manifest_val.heap_size, memctrl_dict.get('heap_limit', 0)))
        return False
    if manifest_val.stack_size > memctrl_dict.get('stack_limit', 0):
        logging.error("Too large manifest stack limit (%d) compared to memctrl configuration (%d)"
            % (manifest_val.stack_size, memctrl_dict.get('stack_limit', 0)))
        return False
    if manifest_val.instancekeepalive == 1 and not memctrl_dict.get('keepalive', False):
        logging.error("Inconsistent manifest keep-alive (%s) compared to memctrl configuration (%s)"
            % (manifest_val.instancekeepalive == 1, memctrl_dict.get('keepalive', False)))
        return False
    if manifest_val.multi_session == 1 and not memctrl_dict.get('multi_session', False):
        logging.error("Inconsistent manifest multisession (%s) compared to memctrl configuration (%s)"
            % (manifest_val.multi_session == 1, memctrl_dict.get('multi_session', False)))
        return False

    logging.info("memory baseline checking passed")
    return True