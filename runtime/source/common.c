#include "common.h"

#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>

__attribute__((noreturn)) void IC_runtime_error(const char* fmt, ...)
{
	va_list args;
	va_start(args, fmt);
	fprintf(stderr, "[Runtime error]: ");
	vfprintf(stderr, fmt, args);
	va_end(args);
	exit(1);
}
