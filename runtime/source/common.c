#include "common.h"

#include <stdio.h>
#include <stdlib.h>

__attribute__((noreturn)) void runtime_error(const char* message)
{
	fprintf(stderr, "Runtime error: %s\n", message);
	exit(1);
}
