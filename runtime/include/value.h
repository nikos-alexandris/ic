#ifndef VALUE_H
#define VALUE_H

#include "common.h"

extern const char* atom_names[];

struct value {
	enum { VALUE_INTEGER, VALUE_ATOM, VALUE_PAIR } tag;
	union {
		i64 integer;
		usize atom;
		struct {
			struct value* car;
			struct value* cdr;
		} pair;
	} as;
};

struct value* value_integer(i64 integer);

struct value* value_atom(usize atom);

struct value* value_pair(struct value* car, struct value* cdr);

bool value_is_pair(const struct value* value);

struct value* value_add(const struct value* a, const struct value* b);

void value_show(const struct value* value);

#endif /* VALUE_H */
