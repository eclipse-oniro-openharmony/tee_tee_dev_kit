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
#include "tee_sharemem.h"
#include "drv_param_ops.h"
#include "drv_io_share.h"
#include "drv_addr_share.h"
#include "io_operations.h"
#include "drv_map_share.h"
#include "drv_hwi_share.h"
#include "hm_cache_flush.h"

#define TEST_PARAM_OPS 0x10
#define TEST_IO_MAP 0x11
#define TEST_DRV_DMA 0x13
#define TEST_DRV_IRQ 0x14
#define IO_SIZE 4
#define PORT_READ 1
#define PORT_WRITE 2
#define TEST_PADDR_IO 0x90000
#define PTHYS_ADDR_SECURE 0x45690000
#define PTHYS_ADDR_NONSECURE 0x26E00000
#define SIZE_ADDR 0x1000
#define CACHE_MODE 1
#define TEECALL_PAGE_SIZE 4096

static void hello()
{
    tlogi("hello hello hello!\n");
    return;
}

int64_t test_drv_irq()
{
    uint32_t hwi_num = 1000;
    uint16_t hwi_prio = 0;
    uint16_t hwi_mode = 0;
    HWI_PROC_FUNC hwi_handler = hello;
    uint32_t hwi_args = 0;

    uint32_t ret = sys_hwi_create(hwi_num, hwi_prio, hwi_mode, hwi_handler, hwi_args);
    if (ret != 0) {
        tloge("sys_hwi_create failed\n");
        return -1;
    }
    tlogi("sys_hwi_create success, trq_num is %u\n", hwi_num);
    if (sys_hwi_enable(hwi_num) != 0) {
        tloge("sys_hwi_enable failed\n");
        return -1;
    }
    tlogi("sys_hwi_enable success\n");

    if (sys_hwi_resume(hwi_num, hwi_prio, hwi_mode) != 0) {
        tloge("sys_hwi_resume failed\n");
        return -1;
    }
    tlogi("sys_hwi_resume success\n");

    if (sys_hwi_notify(hwi_num) != 0) {
        tloge("sys_hwi_resume failed\n");
        return -1;
    }
    tlogi("sys_hwi_resume success \n");

    if (sys_hwi_disable(hwi_num) != 0) {
        tloge("sys_hwi_disable failed\n");
        return -1;
    }
    tlogi("sys_hwi_disable success\n");

    if (sys_hwi_delete(hwi_num) != 0) {
        tloge("sys_hwi_delete failed\n");
        return -1;
    }
    tlogi("sys_hwi_delete success\n");
    return 0;
}

#define OFFSET_ONE 1
#define OFFSET_TWO 2
#define OFFSET_THREE 3
#define OFFSET_FOUR 4
int64_t test_drv_dma()
{
    char *va;
    va = (char *)mmap(NULL, TEECALL_PAGE_SIZE * OFFSET_FOUR, PROT_READ | PROT_WRITE, MAP_ANONYMOUS, -1, 0);
    if (va == MAP_FAILED) {
        tloge("mmap failed\n");
        return -1;
    }

    memset_s((void *)va, TEECALL_PAGE_SIZE * OFFSET_FOUR, 1, TEECALL_PAGE_SIZE * OFFSET_FOUR);

    dma_flush_range((uint64_t)va, (uint64_t)va + TEECALL_PAGE_SIZE);
    dma_inv_range((uint64_t)va + OFFSET_ONE * TEECALL_PAGE_SIZE, (uint64_t)va + OFFSET_TWO * TEECALL_PAGE_SIZE);
    dma_clean_range((uint64_t)va + OFFSET_TWO * TEECALL_PAGE_SIZE, (uint64_t)va + OFFSET_THREE * TEECALL_PAGE_SIZE);
    dma_map_area((uint64_t)va + OFFSET_THREE * TEECALL_PAGE_SIZE, TEECALL_PAGE_SIZE, 1);
    dma_unmap_area((uint64_t)va + OFFSET_THREE * TEECALL_PAGE_SIZE, TEECALL_PAGE_SIZE, 1);

    munmap((void *)va, TEECALL_PAGE_SIZE * OFFSET_FOUR);
    return 0;
}

#define CAP_LOWER_NUM 32
int64_t test_drv_io_map()
{
    int64_t ret = 0;

    void *ptr = ioremap(TEST_PADDR_IO, 0x100, PROT_READ | PROT_WRITE);
    if (ptr == (void *)-1) {
        tloge("ioremap failed\n");
        return -1;
    }
    tlogi("ioremap success addr is: 0x%llx\n", (unsigned long long)TEST_PADDR_IO);
    
    uint64_t phy_addr = drv_virt_to_phys((uintptr_t) ptr);
    if (phy_addr == 0) {
        tloge("drv_virt_to_phys failed\n");
        return -1;
    }
    tlogi("drv_virt_to_phys success, addr is: 0x%llx\n", (unsigned long long)phy_addr);

    char *input = (char *)malloc(sizeof(char) * IO_SIZE);
    char *output = (char *)malloc(sizeof(char) * IO_SIZE);
    char *buf = (char *)malloc(sizeof(char) * IO_SIZE);
    if (input == NULL || output == NULL || buf == NULL) {
        tloge("malloc failed]n");
        return -1;
    }
    input = "ABC\0";

    read_from_io(buf, input, sizeof(char) * IO_SIZE);
    if (buf == NULL) {
        tloge("read from io failed\n");
        return -1;
    }
    tlogi("read from io success, buf is: %s\n", buf);

    for (int i = 0; i < IO_SIZE - 1; i++)
        buf[i] += CAP_LOWER_NUM;

    write_to_io(output, buf, sizeof(char) * IO_SIZE);
    if (output == NULL) {
        tloge("write to io failed\n");
        ret = -1;
        goto io_clean;
    }
    tlogi("write to io success, output is: %s\n", output);

    if (iounmap(TEST_PADDR_IO, (void *)ptr) != 0) {
        tloge("iounmap failed\n");
        return -1;
    }
    tlogi("iounmap success addr is: 0x%llx\n", (unsigned long long)TEST_PADDR_IO);

    return ret;

io_clean:
    if (input != NULL)
        free(input);
    if (output != NULL)
        free(output);
    if (buf != NULL)
        free(buf);
    return ret;
}

int64_t test_drv_param_ops(unsigned long args)
{
    int64_t ret;
    struct param_type *param = (struct param_type *)(uintptr_t)args;
    char data[] = "ABCD";
    uint32_t share_buf_size = param->share_buf_size;
    char *buf = malloc(share_buf_size);
    if (buf == NULL)
        return -1;

    ret = copy_from_client(param->share_buf, share_buf_size, (uintptr_t)buf, share_buf_size);
    if (ret != 0) {
        free(buf);
        tloge("copy from client failed\n");
        return -1;
    }
    tlogi("copy from client success, buf is: %s\n", buf);
    free(buf);

    ret = copy_to_client((uintptr_t)data, share_buf_size, param->share_buf, share_buf_size);
    if (ret != 0) {
        tloge("copy to client failed\n");
        return -1;
    }
    tlogi("copy to client success, data is: %s\n", data);

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

    switch (cmd) {
        case TEST_PARAM_OPS:
            test_drv_param_ops(args);
            break;
        case TEST_IO_MAP:
            test_drv_io_map();
            break;
        case TEST_DRV_DMA:
            test_drv_dma();
            break;
        case TEST_DRV_IRQ:
            test_drv_irq();
            break;
        default:
            tloge("wrong cmd\n");
    }

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
        tloge("open test input args is UINT32_MAX, just retrun -1\n");
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

tee_driver_declare(drv_receiver_module, init_test, open_test, ioctl_test, close_test, \
    suspend_test, resume_test, suspend_s4_test, resume_s4_test);
