#ifndef VALUE_H
#define VALUE_H

#include "common.h"
#include "world.h"

extern const char* IC_atom_names[];

typedef struct IC_value {
	enum { IC_VALUE_INTEGER, IC_VALUE_ATOM, IC_VALUE_OBJECT } tag;
	union {
		long integer;
		usize atom;
		struct IC_object* object;
	} as;
} IC_VALUE;

typedef struct IC_object {
	enum { IC_OBJECT_PAIR } tag;
	union {
		struct {
			IC_WORLD world;
			IC_VALUE (*f)(IC_WORLD);
		} pair;
	} as;
} IC_OBJECT;

#define IC_INTEGER(x) ((IC_VALUE){IC_VALUE_INTEGER, {.integer = (x)}})
#define IC_ATOM(x) ((IC_VALUE){IC_VALUE_ATOM, {.atom = (x)}})
IC_VALUE IC_pair(IC_WORLD world, IC_VALUE (*f)(IC_WORLD));

#define IC_IS_PAIR(v) (((v).tag == IC_VALUE_OBJECT && (v).as.object->tag == IC_OBJECT_PAIR) ? IC_ATOM(1) : IC_ATOM(2))

#define IC_IS_TRUTHY(v) ((v).tag == IC_VALUE_ATOM && (v).as.atom == 1)

IC_VALUE IC_add(IC_VALUE a, IC_VALUE b);
IC_VALUE IC_sub(IC_VALUE a, IC_VALUE b);
IC_VALUE IC_mul(IC_VALUE a, IC_VALUE b);
IC_VALUE IC_eq(IC_VALUE a, IC_VALUE b);
void IC_value_show(IC_VALUE value);

#endif /* VALUE_H */
