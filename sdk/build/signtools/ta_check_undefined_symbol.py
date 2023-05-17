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


''' tools for generate whitelist and check TA undefined symbols are in whitelist '''

import subprocess
import os
import sys
import CppHeaderParser
import shutil
import logging

exception_files = []
script_dir = os.path.dirname(os.path.realpath(__file__))
whitelist_path = "temp/whitelist.txt"
headers_check_path = script_dir + "/headers_check.txt"

class SymbolInfo:
    ''' symbol table param  '''
    def __init__(self, value, flag, part, size, name):
        self.value = value
        self.flag = flag
        self.part = part
        self.size = size
        self.name = name

    def __str__(self):
        return "symbol information:" + " Value:" + self.value + " Flag:" + self.flag + \
                " Part:" + self.part + " Size:" + self.size + " Name:" + self.name

    def __len__(self):
        return len(self.__dict__)


def save_elf_symbol(elf_path):
    ''' get symbol table and save '''
    if not os.path.exists(elf_path):
        logging.error("elf path not exist: " + elf_path)
        return []

    sym_info = SymbolInfo(0, 0, 0, 0, 0)
    cmd = "objdump -T %s" % elf_path
    process = subprocess.Popen(cmd, shell=True, stdout=subprocess.PIPE)
    process.wait()
    output = str(process.stdout.read(), encoding="utf-8").split('\n')[4:]

    symbol_table = []
    for i in range(len(output)):
        info = output[i].split()
        if len(info) < len(sym_info):
            continue
        sym_info = SymbolInfo(info[0], info[1], info[2], info[3], info[len(info) - 1])
        if sym_info.name.startswith("__"):
            continue
        symbol_table.append(sym_info)

    return symbol_table


def find_undefine_symbol(symbol_table):
    ''' get undefined symbols '''
    return [ info.name for info in symbol_table if info.part == "*UND*" ]


def clean_temp_file():
    ''' del temp file '''
    if os.path.exists("temp"):
        shutil.rmtree("temp")


def get_whitelist(whitelist_path):
    ''' get whitelist info from file '''
    if not os.path.exists(whitelist_path):
        logging.error("whitelist path not exist: " + whitelist_path)
        return []

    with open(whitelist_path, 'r') as f:
        return [ line.strip('\n') for line in f.readlines() ]


def check_symbol_in_whitelist(whitelist, function_table):
    ''' check if symbol is in whitelist and find diff '''
    if whitelist == []:
        return False

    if set(function_table).issubset(set(whitelist)):
        logging.critical("success to check TA symbols!")
        return True
    else:
        logging.error("Fail! These functions are not in whitelist:")
        logging.error(set(function_table).difference(set(whitelist)))
        return False


def get_all_headers_file_path(headers_path):
    ''' get all headers file '''
    result = []
    for one_path in headers_path:
        real_path = os.path.realpath(one_path)
        if not os.path.exists(real_path):
            logging.error("path not exist: " + real_path)
            continue

        for maindir, _, filename_list in os.walk(real_path):
            for filename in filename_list:
                if '.h' in filename:
                    path = os.path.join(maindir, filename)
                    result.append(path)

    return result


def gen_whitelist_from_headers(headers_path):
    ''' generate whitelist by header files '''
    if not os.path.exists("temp"):
        os.mkdir("temp")

    with open(whitelist_path, 'a+') as f:
        paths = get_all_headers_file_path(headers_path)
        if paths == []:
            logging.error("Not found any headers files!")
            clean_temp_file()
            exit(1)
        for file_name in paths:
            try:
                header = CppHeaderParser.CppHeader(file_name)
                for func in header.functions:
                    f.write(func['name'] + '\n')
            except Exception as e:
                exception_files.append(file_name)
                logging.critical(file_name + " has exception, skip it.")


def gen_temp_headers_without_note(exception_file):
    ''' del chinese words note in file '''
    file_name = "temp/" + exception_file.split("/")[-1]
    with open(file_name, "w+") as f_f:
        with open(exception_file, 'r', encoding='GBK') as e_f:
            for line in e_f.readlines():
                if line.strip().startswith("*") or line.strip().startswith("/*") or line.strip().startswith("*/"):
                    continue
                f_f.write(line)

    return file_name


def proc_exception_files(exception_files):
    ''' proccess all exceptional file '''
    [ logging.critical("exceptional file " + file.split("/")[-1] + " processing")
      for file in list(map(gen_temp_headers_without_note, exception_files)) ]


def get_headers_path(headers_check_path):
    ''' read headers_check.txt and get headers path '''
    if not os.path.exists(headers_check_path):
        logging.error("Not found headers_check.txt.")
        return []

    with open(os.path.realpath(headers_check_path), 'r') as f:
        return [ script_dir + '/' + line.strip('\n') for line in f.readlines() ]


def main():
    ''' main '''
    argv_data = sys.argv
    if len(argv_data) != 2:
        logging.error("argv need one param")
        exit(1)

    elf_path = argv_data[1]
    temp_path = ["temp"]

    # Get the header file path configured in the file and generate a whitelist
    headers_path = get_headers_path(headers_check_path)
    gen_whitelist_from_headers(headers_path)

    # Handle header files for exceptions, add whitelist
    proc_exception_files(exception_files)
    gen_whitelist_from_headers(temp_path)

    # Get the symbol table of elf
    symbol_table = save_elf_symbol(elf_path)
    if symbol_table == []:
        clean_temp_file()
        exit(1)

    # Get undefined function in symbol table
    function_table = find_undefine_symbol(symbol_table)

    # Check whether the functions in the symbol table correspond to the whitelist
    if check_symbol_in_whitelist(get_whitelist(whitelist_path), function_table) == False:
        clean_temp_file()
        exit(1)


if __name__ == '__main__':
    main()
    clean_temp_file()

