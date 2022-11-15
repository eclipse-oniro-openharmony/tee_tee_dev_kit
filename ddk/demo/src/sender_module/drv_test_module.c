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
#include <securec.h>
#include "tee_driver_module.h"
#include "tee_drv_client.h"
#include <string.h>
#include "tee_sharemem.h"
#include "tee_sharemem_ops.h"

#define TEST_ALLOC_SIZE 12
#define TEST_TLV_TYPE "type_name"
#define TEST_BUF_SIZE 8
#define TEST_PARAM_OPS 0x10
#define TEST_IO_MAP 0x11
#define TEST_DRV_DMA 0x13
#define TEST_DRV_IRQ 0x14
static const TEE_UUID g_drv_uuid = { 0x00000000, 0x0000, 0x0000, { 0x00, 0x00, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11 }};

static void test_ioctl(int64_t fd)
{
    uint64_t ret;
    uint8_t buf[TEST_BUF_SIZE] = { 0 };
    uint32_t buf_size = TEST_BUF_SIZE;
    ret = get_tlv_sharedmem(TEST_TLV_TYPE, strlen(TEST_TLV_TYPE), buf, &buf_size, false);
    if (ret != 0)
        tloge("get_tlv_sharedmem failed\n");
    else
        tlogi("get_tlv_sharedmem success\n");

    ret = tee_drv_ioctl(fd, TEST_DRV_DMA, NULL, 0);
    if (ret != 0)
        tloge("drv ioctl TEST_DRV_DMA failed\n");
    tlogi("drv ioctl TEST_DRV_DMA success\n");

    ret = tee_drv_ioctl(fd, TEST_DRV_IRQ, NULL, 0);
    if (ret != 0)
        tloge("drv ioctl TEST_DRV_IRQ failed\n");
    tlogi("drv ioctl TEST_DRV_IRQ success\n");

    ret = tee_drv_ioctl(fd, TEST_IO_MAP, NULL, 0);
    if (ret != 0)
        tloge("drv ioctl TEST_IO_MAP failed\n");
    tlogi("drv ioctl TEST_IO_MAP success\n");
}

int64_t test_send_param()
{
    int64_t ret;
    const char *drv_name = "drv_receiver_module";
    struct param_type param = { 0 };
    const char tmp[] = "openharmony";

    int64_t fd = tee_drv_open(drv_name, NULL, 0);
    if (fd <= 0) {
        tloge("open drv failed");
        return -1;
    }

    test_ioctl(fd);

    char *share_buf = tee_alloc_sharemem_aux(&g_drv_uuid, TEST_ALLOC_SIZE);
    if (share_buf == NULL) {
        tloge("tee alloc sharemem failed\n");
        return -1;
    }
    tlogi("tee alloc sharemem success\n");

    ret = memcpy_s(share_buf, TEST_ALLOC_SIZE, tmp, strlen(tmp));
    if (ret != 0) {
        tee_free_sharemem(share_buf, TEST_ALLOC_SIZE);
        return -1;
    }
    tlogi("memcpy success, check share_buf: %s\n", share_buf);
    
    param.share_buf = (uint64_t)(uintptr_t)(share_buf);
    param.share_buf_size = TEST_ALLOC_SIZE;
    ret = tee_drv_ioctl(fd, TEST_PARAM_OPS, &param, sizeof(param));
    if (ret != 0) {
        tee_free_sharemem(share_buf, TEST_ALLOC_SIZE);
        tloge("drv ioctl TEST_PARAM_OPS failed, fd: %d\n", (int32_t)fd);
    }
    tlogi("drv ioctl TEST_PARAM_OPS success, share_buf: %s\n", share_buf);
    tee_free_sharemem(share_buf, TEST_ALLOC_SIZE);

    ret = tee_drv_close(fd);
    if (ret != 0)
        tloge("=============== close fd fail ================\n");
    else
        tlogi("=============== close fd success ================\n");

    return ret;
}

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

    test_send_param();

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

tee_driver_declare(drv_sender_module, init_test, open_test, ioctl_test, close_test, \
    suspend_test, resume_test, suspend_s4_test, resume_s4_test);
