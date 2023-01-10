#ifndef IC_LAR_H
#define IC_LAR_H

#include "common.h"
#include "value.h"

typedef struct IC_lar_proto {
	struct IC_lar_proto* parent;
	struct IC_lar_proto* gc_next;
	u8 num_of_args;
	u8 in_stack;
	u8 marked;
} IC_LAR_PROTO;

typedef IC_VALUE (*IC_LARF)(IC_LAR_PROTO*);

#define IC_FUNCTION_PUSH(_lar) (_lar)->in_stack = 1
#define IC_FUNCTION_POP(_lar) (_lar)->in_stack = 0

#define IC_LAR_THUNK(_lar, _i) *(IC_LARF*)((u8*)(_lar) + sizeof(IC_LAR_PROTO) + _i * sizeof(IC_LARF))

#define IC_LAR_VALUE(_lar, _i)                                                                                         \
	*(IC_VALUE*)((u8*)(_lar) + sizeof(IC_LAR_PROTO) + _lar->num_of_args * sizeof(IC_LARF) + _i * sizeof(IC_VALUE))

IC_LAR_PROTO* IC_lar_new(IC_LAR_PROTO* parent, u8 num_of_args, IC_LARF* args);

IC_VALUE IC_lar_get_arg(IC_LAR_PROTO* lar, u32 arg);

double IC_get_gc_time(void);

usize IC_get_alloc_size(void);

void IC_mem_cleanup(void);

#endif /* IC_LAR_H */
