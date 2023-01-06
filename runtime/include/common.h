#ifndef IC_COMMON_H
#define IC_COMMON_H

#include <stdint.h>

typedef uint8_t u8;
typedef uint32_t u32;
typedef uintptr_t usize;

typedef enum { false, true } bool;

__attribute__((noreturn)) void IC_runtime_error(const char* fmt, ...);

#endif /* IC_COMMON_H */
