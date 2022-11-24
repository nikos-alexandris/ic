#include "value.h"

#include <malloc.h>
#include <stdio.h>

struct value* value_integer(i64 integer)
{
	struct value* value = malloc(sizeof(struct value));
	value->tag = VALUE_INTEGER;
	value->as.integer = integer;
	return value;
}

struct value* value_atom(usize atom)
{
	struct value* value = malloc(sizeof(struct value));
	value->tag = VALUE_ATOM;
	value->as.atom = atom;
	return value;
}

struct value* value_pair(struct value* car, struct value* cdr)
{
	struct value* value = malloc(sizeof(struct value));
	value->tag = VALUE_PAIR;
	value->as.pair.car = car;
	value->as.pair.cdr = cdr;
	return value;
}

bool value_is_pair(const struct value* value)
{
	return value->tag == VALUE_PAIR;
}

struct value* value_add(const struct value* a, const struct value* b)
{
	struct value* value;

	if (a->tag != VALUE_INTEGER || b->tag != VALUE_INTEGER) {
		runtime_error("Invalid value tag");
	}

	value = malloc(sizeof(struct value));
	value->tag = VALUE_INTEGER;
	value->as.integer = a->as.integer + b->as.integer;
	return value;
}

void value_show(const struct value* value)
{
	switch (value->tag) {
	case VALUE_INTEGER: {
		printf("%ld", value->as.integer);
		break;
	}
	case VALUE_ATOM: {
		printf("'%s", atom_names[value->as.atom]);
		break;
	}
	case VALUE_PAIR: {
		printf("(");
		value_show(value->as.pair.car);
		printf(" . ");
		value_show(value->as.pair.cdr);
		printf(")");
		break;
	}
	default: {
		runtime_error("Invalid value tag");
	}
	}
}
