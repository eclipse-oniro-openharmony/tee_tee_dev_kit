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

include ./config.mk
include ../../../build/mk/common.mk

SRC += $(wildcard ./*.c)

# set header directory
INCLUDEDIR += -I./include

# set target
COBJS := $(SRC:%.c=%.o)
TARGET = $(COBJS)

sec_binary:combine
	python3 -B ${SIGNTOOL_DIR}/signtool_sec.py ${CUR_DIR} ${CUR_DIR} --privateCfg ${SIGNTOOL_DIR}/../config_tee_private_sample.ini --publicCfg ${CUR_DIR}/config_ta_public.ini

# attention: must add ta_entry_check.sh as below to check whether ta is valid
combine: $(TARGET)
	$(LD) $(LDFLAGS) $(TARGET) $(EXTRAO) -o libcombine.so
	bash $(TEE_BUILD_PATH)/build/tools/ta_entry_check.sh $(READELF) $(shell pwd)/libcombine.so n y $(TARGET_IS_ARM64)

src/%.o: ./src/%.c
	$(CC) $(CFLAGS)  $(INCLUDEDIR) -c $< -o $@

%.o: %.c
	$(CC) $(CFLAGS)  $(INCLUDEDIR) -c $< -o $@

clean:
	rm -f $(COBJS) *.so *.sec
