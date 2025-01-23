/*
 * Copyright (c) 2024 Huawei Device Co., Ltd.
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *    http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef RPMB_DRIVER_RW_API_H
#define RPMB_DRIVER_RW_API_H
/**
 * @addtogroup TeeTrusted
 * @{
 *
 * @brief TEE(Trusted Excution Environment) API.
 * Provides security capability APIs such as trusted storage, encryption and decryption,
 * and trusted time for trusted application development.
 *
 * @since 12
 */

/**
 * @file rpmb_driver_rw_api.h
 *
 * @brief APIs related to RPMB driver read and write.
 * Provides the function of reading and writing RPMB driver.
 *
 * @library NA
 * @kit TEEKit
 * @syscap SystemCapability.Tee.TeeClient
 * @since 12
 * @version 1.0
 */

#ifdef __cplusplus
extern "C" {
#endif

/**
 * @brief Defines the total block number.
 *
 * @since 12
 * @version 1.0
 */
#define TOTAL_BLK 4

/**
 * @brief Defines the size of block.
 *
 * @since 12
 * @version 1.0
 */
#define BLK_SIZE 256

/**
 * @brief Defines the size of the total block.
 *
 * @since 12
 * @version 1.0
 */
#define TOTAL_BLK_SIZE (TOTAL_BLK * BLK_SIZE)

#define SEC_WRITE_PROTECT_ENTRY_NUM 4
#define SEC_WRITE_PROTECT_ENTRY_RESERVED_NUM 3
#define SEC_WRITE_PROTECT_ENTRY_RESERVED_SIZE 16
#define SEC_WRITE_PROTECT_FRAME_RESERVED_NUM 14
#define SEC_WRITE_PROTECT_FRAME_RESERVED_END_NUM 176
#define SEC_WRITE_PROTECT_BLK_SIZE 256
#define SEC_WRITE_PROTECT_LUN_MAX 5

/**
 * @brief Write plaintext buffer to RPMB driver.
 *
 * @param buf Indicates the buffer for writing data.
 * @param size Indicates the length of buffer, the maximum value is 1024.
 * @param block Indicates the block index of the position of start block, the value is [0, 3].
 * @param offset Indicates the offset bytes of data position, and the value of offest bytes is less than 256.
 *
 * @return Returns {@code TEE_SUCCESS} if the operation is successful.
 *         Returns {@code TEE_ERROR_BAD_PARAMETERS} if the input parameter is incorrect.
 *         Returns {@code TEE_ERROR_OUT_OF_MEMORY} if the send message fail.
  *
 * @since 12
 * @version 1.0
 */
TEE_Result tee_ext_rpmb_driver_write(const uint8_t *buf, size_t size, uint32_t block, uint32_t offset);

/**
 * @brief Read plaintext buffer from RPMB driver.
 *
 * @param buf Indicates the buffer for read data.
 * @param size Indicates the length of buffer, the maximum value is 1024.
 * @param block Indicates the block index of the position of start block, the value is [0, 3].
 * @param offset Indicates the offset bytes of data position, and the value of offest bytes is less than 256.
 *
 * @return Returns {@code TEE_SUCCESS} if the operation is successful.
 *         Returns {@code TEE_ERROR_BAD_PARAMETERS} if the input parameter is incorrect.
 *         Returns {@code TEE_ERROR_OUT_OF_MEMORY} if the send message fail.
  *
 * @since 12
 * @version 1.0
 */
TEE_Result tee_ext_rpmb_driver_read(uint8_t *buf, size_t size, uint32_t block, uint32_t offset);

/**
 * @brief Remove data from RPMB driver.
 *
 * @param size Indicates the length of remove data, the maximum value is 1024.
 * @param block Indicates the block index of the position of start block, the value is [0, 3].
 * @param offset Indicates the offset bytes of data position, and the value of offest bytes is less than 256.
 *
 * @return Returns {@code TEE_SUCCESS} if the operation is successful.
 *         Returns {@code TEE_ERROR_BAD_PARAMETERS} if the input parameter is incorrect.
 *         Returns {@code TEE_ERROR_OUT_OF_MEMORY} if the send message fail.
  *
 * @since 12
 * @version 1.0
 */
TEE_Result tee_ext_rpmb_driver_remove(size_t size, uint32_t block, uint32_t offset);

#ifdef __cplusplus
}
#endif

/** @} */
#endif
