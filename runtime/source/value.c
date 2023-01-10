#include "value.h"
#include "common.h"

#include <stdio.h>

void IC_value_show(IC_VALUE v)
{
	if (IC_IS_INT(v)) {
		printf("%ld\n", IC_UNBOX(v));
	} else {
		IC_runtime_error("'result' can only return base values for the time being...");
	}
}
