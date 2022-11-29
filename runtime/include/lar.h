#ifndef IC_LAR_H
#define IC_LAR_H

#include "value.h"

typedef struct IC_lar {
	struct IC_lar* prev;
	IC_VALUE (**args)(struct IC_lar*);
	IC_VALUE* vals;
} IC_LAR;

#endif /* IC_LAR_H */
