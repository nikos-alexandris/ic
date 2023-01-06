#include "value.h"

#include <malloc.h>
#include <stdarg.h>
#include <time.h>

#define IC_GC_LIMIT 4194304
static usize IC_curr_alloc = 0;

static IC_LAR_PROTO* IC_gc_first = NULL;

static double IC_gc_time = 0;
static usize IC_alloc_size = 0;

#define IC_LAR_THUNK(_lar, _i) *(IC_LARF*)((u8*)(_lar) + sizeof(IC_LAR_PROTO) + _i * sizeof(IC_LARF))

#define IC_LAR_VALUE(_lar, _i)                                                                                         \
	*(IC_VALUE*)((u8*)(_lar) + sizeof(IC_LAR_PROTO) + _lar->num_of_args * sizeof(IC_LARF) + _i * sizeof(IC_VALUE))

static void IC_gc(void);
static void IC_mark(IC_LAR_PROTO* lar);

IC_LAR_PROTO* IC_lar_new(IC_LAR_PROTO* parent, u32 num_of_args, ...)
{
	// TODO check num_of_args <= 255

	usize size = sizeof(IC_LAR_PROTO) + num_of_args * sizeof(IC_LARF) + num_of_args * sizeof(IC_VALUE);
	IC_LAR_PROTO* lar = (IC_LAR_PROTO*)malloc(size);
	IC_alloc_size += malloc_usable_size(lar);

	IC_curr_alloc += size;
	if (IC_curr_alloc > IC_GC_LIMIT) {
		IC_gc();
		IC_curr_alloc = 0;
	}

	lar->parent = parent;
	lar->num_of_args = (u8)num_of_args;
	lar->in_stack = 0;
	lar->marked = 0;
	va_list args;
	va_start(args, num_of_args);
	for (u8 i = 0; i < lar->num_of_args; i++) {
		IC_LAR_THUNK(lar, i) = va_arg(args, IC_LARF);
	}
	va_end(args);
	lar->gc_next = IC_gc_first;
	IC_gc_first = lar;
	return lar;
}

IC_VALUE IC_lar_get_arg(IC_LAR_PROTO* lar, u32 arg)
{
	IC_LARF thunk = IC_LAR_THUNK(lar, arg);
	if (thunk != NULL) {
		IC_LAR_VALUE(lar, arg) = thunk(lar->parent);
		IC_LAR_THUNK(lar, arg) = NULL;
	}
	return IC_LAR_VALUE(lar, arg);
}

void IC_mem_cleanup(void)
{
	IC_LAR_PROTO* lar = IC_gc_first;
	while (lar != NULL) {
		IC_LAR_PROTO* next = lar->gc_next;
		free(lar);
		lar = next;
	}
}

double IC_get_gc_time(void) { return IC_gc_time; }

usize IC_get_alloc_size(void) { return IC_alloc_size; }

static void IC_gc(void)
{
	clock_t start = clock();

	IC_LAR_PROTO* curr = IC_gc_first;
	while (curr != NULL) {
		if (curr->in_stack) {
			IC_mark(curr);
		}
		curr = curr->gc_next;
	}

	curr = IC_gc_first;
	IC_LAR_PROTO* prev = NULL;
	while (curr != NULL) {
		if (curr->marked) {
			curr->marked = 0;
			prev = curr;
			curr = curr->gc_next;
		} else {
			IC_LAR_PROTO* next = curr->gc_next;

			free(curr);

			if (prev == NULL) {
				IC_gc_first = next;
			} else {
				prev->gc_next = next;
			}
			curr = next;
		}
	}

	IC_gc_time += (double)(clock() - start) / CLOCKS_PER_SEC;
}

static void IC_mark(IC_LAR_PROTO* lar)
{
	if (lar == NULL || lar->marked) {
		return;
	}
	lar->marked = 1;
	for (u8 i = 0; i < lar->num_of_args; i++) {
		if (IC_LAR_THUNK(lar, i) != NULL) {
			continue;
		}
		IC_VALUE val = IC_LAR_VALUE(lar, i);
		if (val.tag != IC_VALUE_PAIR) {
			continue;
		}
		IC_mark(val.as.pair);
	}
	IC_mark(lar->parent);
}
