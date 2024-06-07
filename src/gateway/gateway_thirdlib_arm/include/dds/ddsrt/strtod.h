// Copyright(c) 2006 to 2019 ZettaScale Technology and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License v. 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Eclipse Distribution License
// v. 1.0 which is available at
// http://www.eclipse.org/org/documents/edl-v10.php.
//
// SPDX-License-Identifier: EPL-2.0 OR BSD-3-Clause

/**
 * @file strtod.h
 * @brief Floating-point number to/from string conversion functions.
 *
 * Locale independent versions of the floating-point number conversion
 * functions found in the standard library.
 */
#ifndef DDSRT_STRTOD_H
#define DDSRT_STRTOD_H

#include "dds/export.h"
#include "dds/ddsrt/retcode.h"

#if defined (__cplusplus)
extern "C" {
#endif

/**
 * @brief Convert a string to a double precision floating point number.
 *
 * This operation handles locale specific manipulation of the floating point 
 * string and then sets the output double based on the parsing via `strtod` 
 * from the standard library. The function returns a failure iff:
 * - the string to be parsed represents a value that is too large to store in a double.
 * - the string contains junk.
 * - the string parses to either `-nan` or `nan`.
 * - the string parses to either `-inf` or `inf`.
 * It is otherwise successful.
 *
 * @param[in]   nptr    A string to convert into a double.
 * @param[out]  endptr  If not NULL, a char* where the address of first invalid
 *                      character is stored.
 * @param[out]  dblptr  A double where the result is stored.
 *
 * @returns A dds_return_t indicating success or failure.
 */
dds_return_t
ddsrt_strtod(const char *nptr, char **endptr, double *dblptr);

/**
 * @brief Convert a double-precision floating-point number to a string.
 *
 * @param[in]  src   Double-precision floating-point number to convert.
 * @param[in]  str   Buffer where string representation is written.
 * @param[in]  size  Number of bytes available in @str.
 *
 * @returns The number of bytes written (excluding the null terminating byte).
 */
int
ddsrt_dtostr(double src, char *str, size_t size);

/**
 * @brief convert a floating-point number to a string.
 *
 * @param[in]  src   Floating-point number to conver.
 * @param[in]  str   Buffer where string representation is written.
 * @param[in]  size  Number of bytes available in @str.
 *
 * @returns The number of bytes written (exluding the null terminating byte).
 */
int
ddsrt_ftostr(float src, char *str, size_t size);

#if defined (__cplusplus)
}
#endif

#endif /* DDSRT_STRTOD_H */
