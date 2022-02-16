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

/**
 * @file oemkey.h
 *
 * @brief Interfaces related to the provision key function.
 *
 * Provides the function of reading the hardware provision key.
 *
 * @since 1
 */
#ifndef OEMKEY_H
#define OEMKEY_H

#include <stdint.h>
#include <stdlib.h>

/**
 * @brief Get provision Key
 *
 * @param oem_key  [IN]Buffer for storing provision keys.
 * @param key_size [IN]The length of the buffer used to store the provision key is 16.
 *
 * @return 0 Indicates that the function executed successfully.
 * @return others Failed to obtain the provision key
 *
 */
uint32_t tee_hal_get_provision_key(uint8_t *oem_key, size_t key_size);
#endif
