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

#include "drv_test_module.h"
#include <stdio.h>
#include <malloc.h>
#include <tee_log.h>
#include <sys/mman.h>
#include "tee_driver_module.h"

int32_t init_test(void)
{
    tlogi("driver init test end\n");
    return 0;
}

int64_t ioctl_test(struct drv_data *drv, uint32_t cmd, unsigned long args, uint32_t args_len)
{
    (void)cmd;
    (void)args;
    (void)args_len;
    if (drv == NULL) {
        tloge("ioctl invalid drv\n");
        return -1;
    }

    tlogi("ioctl_test load!\n");

    return 0;
}

static uint32_t *buf_init(uint32_t args)
{
    uint32_t *buf = (uint32_t *)malloc(TOKEN_BUF_SIZE * sizeof(uint32_t));
    if (buf == NULL) {
        tloge("alloc buf failed\n");
        return NULL;
    }

    int32_t i;
    for (i = 0; i < TOKEN_BUF_SIZE; i++)
        buf[i] = args;

    return buf;
}

int64_t open_test(struct drv_data *drv, unsigned long args, uint32_t args_len)
{
    if (drv == NULL) {
        tloge("open invalid drv\n");
        return -1;
    }

    if (args == 0 && args_len == 0) {
        tloge("input NULL param\n");
        return 0;
    }

    if (args_len < sizeof(uint32_t) || args == 0) {
        tloge("open invalid drv\n");
        return -1;
    }

    char open_succ[10] = {"hello"};
    tlogi("%s", open_succ);

    uint32_t *input = (uint32_t *)(uintptr_t)args;
    if (*input == UINT32_MAX) {
        tloge("open test input args is UINT32_MAX, just return -1\n");
        return -1;
    }

    uint32_t *buf = buf_init(*input);
    if (buf == NULL)
        return -1;

    drv->private_data = buf;
    tlogi("driver open test begin: fd=%d args=0x%x",
        drv->fd, *input);

    return 0;
}

int64_t close_test(struct drv_data *drv)
{
    if (drv == NULL) {
        tloge("close invalid drv\n");
        return -1;
    }

    tlogi("driver close test begin: fd:%d", drv->fd);
    if (drv->private_data != NULL) {
        tloge("free private data in close\n");
        free(drv->private_data);
    }

    return 0;
}

int32_t suspend_test(void)
{
    tlogi("suspend test begin\n");
    return 0;
}

int32_t resume_test(void)
{
    tlogi("resume test begin\n");
    return 0;
}

int32_t suspend_s4_test(void)
{
    tlogi("suspend_s4 test begin\n");
    return 0;
}

int32_t resume_s4_test(void)
{
    tlogi("resume_s4 test begin\n");
    return 0;
}

tee_driver_declare(drv_test_module, init_test, open_test, ioctl_test, close_test, \
    suspend_test, resume_test, suspend_s4_test, resume_s4_test);
