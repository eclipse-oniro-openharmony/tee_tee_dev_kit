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

#ifndef TEE_PROPERTY_API_H
#define TEE_PROPERTY_API_H

#include "tee_defines.h"

/*
 * The definitions below are defined by Global Platform or Platform SDK released previously
 * for compatibility.
 * Do not make any change to the content below.
 */
typedef enum {
    TEE_PROPSET_UNKNOW             = 0,
    TEE_PROPSET_TEE_IMPLEMENTATION = 0xFFFFFFFD,
    TEE_PROPSET_CURRENT_CLIENT     = 0xFFFFFFFE,
    TEE_PROPSET_CURRENT_TA         = 0xFFFFFFFF,
} Pseudo_PropSetHandle;

typedef uint32_t TEE_PropSetHandle;

/*
 * @brief Obtains a property from a property set and converts its value into a printable string.
 *
 *
 * @param propsetOrEnumerator Indicates one of the TEE_PROPSET_XXX pseudo-handles or a handle on a property enumerator.
 * @param name Indicates the pointer to the zero-terminated string containing the name of the property to obtain.
 * @param valueBuffer Indicates the pointer to the buffer for holding the property value obtained.
 * @param valueBufferLen Indicates the pointer to the buffer length.
 *
 * @return Returns <b>TEE_SUCCESS</b> if the operation is successful.
 * @return Returns <b>TEE_ERROR_ITEM_NOT_FOUND</b> if the target property cannot be obtained.
 * @return Returns <b>TEE_ERROR_SHORT_BUFFER</b> if the value buffer is too small to hold the property value obtained.
 */
TEE_Result TEE_GetPropertyAsString(TEE_PropSetHandle propsetOrEnumerator, const char *name, char *valueBuffer,
                                   size_t *valueBufferLen);

/*
 * @brief Obtains a property from a property set and converts its value into a Boolean value.
 *
 * @param propsetOrEnumerator Indicates one of the TEE_PROPSET_XXX pseudo-handles or a handle on a property enumerator.
 * @param name Indicates the pointer to the zero-terminated string containing the name of the property to obtain.
 * @param value Indicates the pointer to the variable that holds the property value obtained.
 *
 * @return Returns <b>TEE_SUCCESS</b> if the operation is successful.
 * @return Returns <b>TEE_ERROR_ITEM_NOT_FOUND</b> if the target property cannot be obtained.
 */
TEE_Result TEE_GetPropertyAsBool(TEE_PropSetHandle propsetOrEnumerator, const char *name, bool *value);

/*
 * @brief Obtains a property from a property set and converts its value into a 32-bit unsigned integer.
 *
 * @param propsetOrEnumerator Indicates one of the TEE_PROPSET_XXX pseudo-handles or a handle on a property enumerator.
 * @param name Indicates the pointer to the zero-terminated string containing the name of the property to obtain.
 * @param value Indicates the pointer to the variable that holds the property value obtained.
 *
 * @return Returns <b>TEE_SUCCESS</b> if the operation is successful.
 * @return Returns <b>TEE_ERROR_ITEM_NOT_FOUND</b> if the target property cannot be obtained.
 */
TEE_Result TEE_GetPropertyAsU32(TEE_PropSetHandle propsetOrEnumerator, const char *name, uint32_t *value);

#if defined(API_LEVEL) && (API_LEVEL >= API_LEVEL1_2)
/*
 * @brief Obtains a property from a property set and converts its value into a 64-bit unsigned integer.
 *
 * @param propsetOrEnumerator Indicates one of the TEE_PROPSET_XXX pseudo-handles or a handle on a property enumerator.
 * @param name Indicates the pointer to the zero-terminated string containing the name of the property to obtain.
 * @param value Indicates the pointer to the variable that holds the property value obtained.
 *
 * @return Returns <b>TEE_SUCCESS</b> if the operation is successful.
 * @return Returns <b>TEE_ERROR_ITEM_NOT_FOUND</b> if the target property cannot be obtained.
 */
TEE_Result TEE_GetPropertyAsU64(TEE_PropSetHandle propsetOrEnumerator, const char *name, uint64_t *value);
#endif // API_LEVEL

/*
 * @brief Obtains a property from a property set and converts its value into a binary block.
 *
 * @param propsetOrEnumerator Indicates one of the TEE_PROPSET_XXX pseudo-handles or a handle on a property enumerator.
 * @param name Indicates the pointer to the zero-terminated string containing the name of the property to obtain.
 * @param valueBuffer Indicates the pointer to the buffer for holding the property value obtained.
 * @param valueBufferLen Indicates the pointer to the buffer length.
 *
 * @return Returns <b>TEE_SUCCESS</b> if the operation is successful.
 * @return Returns <b>TEE_ERROR_ITEM_NOT_FOUND</b> if the target property cannot be obtained.
 * @return TEE_ERROR_SHORT_BUFFER the value buffer is not large enough to hold the whole property value
 */
TEE_Result TEE_GetPropertyAsBinaryBlock(TEE_PropSetHandle propsetOrEnumerator, const char *name, void *valueBuffer,
                                        size_t *valueBufferLen);

/*
 * @brief Obtains a property from a property set and converts its value to the <b>TEE_UUID</b> struct.
 *
 * @param propsetOrEnumerator Indicates one of the TEE_PROPSET_XXX pseudo-handles or a handle on a property enumerator.
 * @param name Indicates the pointer to the zero-terminated string containing the name of the property to obtain.
 * @param value Indicates the pointer to the variable that holds the property value obtained.
 *
 * @return Returns <b>TEE_SUCCESS</b> if the operation is successful.
 * @return Returns <b>TEE_ERROR_ITEM_NOT_FOUND</b> if the target property cannot be obtained.
 */
TEE_Result TEE_GetPropertyAsUUID(TEE_PropSetHandle propsetOrEnumerator, const char *name, TEE_UUID *value);

/*
 * @brief Obtains a property from a property set and converts its value to the <b>TEE_Identity</b> struct.
 *
 * @param propsetOrEnumerator Indicates one of the TEE_PROPSET_XXX pseudo-handles or a handle on a property enumerator.
 * @param name Indicates the pointer to the zero-terminated string containing the name of the property to obtain.
 * @param value Indicates the pointer to the variable that holds the property value obtained.
 *
 * @return Returns <b>TEE_SUCCESS</b> if the operation is successful.
 * @return Returns <b>TEE_ERROR_ITEM_NOT_FOUND</b> if the target property cannot be obtained.
 */
TEE_Result TEE_GetPropertyAsIdentity(TEE_PropSetHandle propsetOrEnumerator, const char *name, TEE_Identity *value);

/*
 * @brief Allocates a property enumerator object.
 *
 * @param enumerator Indicates the pointer to the property enumerator filled with an opaque handle.
 *
 * @return Returns <b>TEE_SUCCESS</b> if the operation is successful.
 * @return Returns <b>TEE_ERROR_OUT_OF_MEMORY</b> if there is no enough resources to allocate the property enumerator.
 */
TEE_Result TEE_AllocatePropertyEnumerator(TEE_PropSetHandle *enumerator);

/*
 * @brief Releases a property enumerator object.
 *
 * @param enumerator Indicates the handle on the property enumerator to release.
 *
 * @return void
 */
void TEE_FreePropertyEnumerator(TEE_PropSetHandle enumerator);

/*
 * @brief Starts to enumerate the properties in an enumerator.
 *
 * @param enumerator Indicates the handle on the enumerator.
 * @param propSet Indicates the pseudo-handle on the property set to enumerate.
 *
 * @return void
 */
void TEE_StartPropertyEnumerator(TEE_PropSetHandle enumerator, TEE_PropSetHandle propSet);

/*
 * @brief Resets a property enumerator immediately after allocation.
 *
 * @param enumerator Indicates the handle on the enumerator to reset.
 *
 * @return void
 */
void TEE_ResetPropertyEnumerator(TEE_PropSetHandle enumerator);

/*
 * @brief Obtains the name of this property in an enumerator.
 *
 * @param enumerator Indicates the handle on the enumerator.
 * @param nameBuffer Indicates the pointer to the buffer that stores the property name obtained.
 * @param nameBufferLen Indicates the pointer to the buffer length.
 *
 * @return Returns <b>TEE_SUCCESS</b> if the operation is successful.
 * @return Returns <b>TEE_ERROR_ITEM_NOT_FOUND</b> if the property is not found because the enumerator has not started
 * or has reached the end of the property set.
 * @return Returns <b>TEE_ERROR_SHORT_BUFFER</b> if the buffer is too small to hold the property name.
 */
TEE_Result TEE_GetPropertyName(TEE_PropSetHandle enumerator, void *nameBuffer, size_t *nameBufferLen);

/*
 * @brief Obtains the next property in an enumerator.
 *
 * @param enumerator Indicates the handle on the enumerator.
 *
 * @return Returns <b>TEE_SUCCESS</b> if the operation is successful.
 * @return Returns <b>TEE_ERROR_ITEM_NOT_FOUND</b> if the property is not found because the enumerator
 * has not started or has reached the end of the property set.
 */
TEE_Result TEE_GetNextProperty(TEE_PropSetHandle enumerator);
#endif
