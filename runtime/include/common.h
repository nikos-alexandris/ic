#ifndef COMMON_H
#define COMMON_H

#include <stdint.h>

typedef uint8_t u8;
typedef uintptr_t usize;

typedef int64_t i64;

typedef enum { false, true } bool;

__attribute__((noreturn)) void runtime_error(const char* message);

#endif /* COMMON_H */
