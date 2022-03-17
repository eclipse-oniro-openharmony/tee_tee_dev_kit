/*
 * Copyright (C) 2022 Huawei Technologies Co., Ltd.
 * Licensed under the Mulan PSL v2.
 * You can use this software according to the terms and conditions of the Mulan PSL v2.
 * You may obtain a copy of Mulan PSL v2 at:
 *     http://license.coscl.org.cn/MulanPSL2
 * THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND, EITHER EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR
 * PURPOSE.
 * See the Mulan PSL v2 for more details.
 */


#ifndef PLATDRV_DRV_TEST_MODULE_H
#define PLATDRV_DRV_TEST_MODULE_H
#include <stdint.h>
#include "tee_driver_module.h"

#define MAX_BUF_LEN 1896

struct share_buffer_arg {
    uint64_t addr;
    uint32_t len;
    uint32_t share_token;
};

#define BUFFER_SIZE 0x1000U
#define TOKEN_BUF_SIZE 0x1000
#define BUFFER1_LEN 21
#define BUFFER2_LEN 33
struct buffer_arg {
    char buffer1[BUFFER1_LEN];
    uint32_t buffer1_len;
    char buffer2[BUFFER2_LEN];
    uint32_t buffer2_len;
    char c1;
    char c2;
    uint32_t buffer_token;
};

int32_t init_test(void);
int64_t open_test(struct drv_data *drv, unsigned long args, uint32_t args_len);
int64_t ioctl_test(struct drv_data *drv, uint32_t cmd, unsigned long args, uint32_t args_len);
int64_t close_test(struct drv_data *drv);
int32_t suspend_test(void);
int32_t resume_test(void);

#endif
