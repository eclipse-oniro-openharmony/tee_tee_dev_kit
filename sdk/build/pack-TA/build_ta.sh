#!/bin/bash

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
set -e

LOCAL_PATH="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

TEE_SDK_PATH=$LOCAL_PATH/../signtools/

INPUT_PATH=$LOCAL_PATH/input
OUTPUT_PATH=$LOCAL_PATH/output
python3 -B ${TEE_SDK_PATH}/ta_check_undefined_symbol.py ${INPUT_PATH}/libcombine.so
python3 -B ${TEE_SDK_PATH}/signtool_sec.py ${INPUT_PATH} ${OUTPUT_PATH} --privateCfg $LOCAL_PATH/../config_tee_private_sample.ini --publicCfg $LOCAL_PATH/input/config_ta_public.ini
