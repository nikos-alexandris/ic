#include "value.h"

#include <malloc.h>
#include <stdio.h>

static const char* IC_value_show_type(IC_VALUE value);

IC_VALUE IC_pair(IC_WORLD world, IC_VALUE (*f)(IC_WORLD))
{
	IC_OBJECT* object = malloc(sizeof(*object));
	if (!object) {
		IC_runtime_error("out of memory", 0);
	}
	object->tag = IC_OBJECT_PAIR;
	object->as.pair.f = f;
	object->as.pair.world = world;

	return (IC_VALUE){IC_VALUE_OBJECT, {.object = object}};
}

IC_VALUE IC_add(IC_VALUE a, IC_VALUE b)
{
	if (a.tag == IC_VALUE_INTEGER && b.tag == IC_VALUE_INTEGER) {
		return IC_INTEGER(a.as.integer + b.as.integer);
	} else {
		IC_runtime_error("cannot add %s and %s", IC_value_show_type(a), IC_value_show_type(b));
	}
}

IC_VALUE IC_eq(IC_VALUE a, IC_VALUE b)
{
	if (a.tag == IC_VALUE_INTEGER && b.tag == IC_VALUE_INTEGER) {
		return a.as.integer == b.as.integer ? IC_ATOM(1) : IC_ATOM(2);
	} else if (a.tag == IC_VALUE_ATOM && b.tag == IC_VALUE_ATOM) {
		return a.as.atom == b.as.atom ? IC_ATOM(1) : IC_ATOM(2);
	} else {
		IC_runtime_error("cannot compare %s and %s", IC_value_show_type(a), IC_value_show_type(b));
	}
}

void IC_value_show(IC_VALUE value)
{
	switch (value.tag) {
	case IC_VALUE_INTEGER:
		printf("%ld\n", value.as.integer);
		break;
	case IC_VALUE_ATOM:
		printf("%s\n", IC_atom_names[value.as.atom]);
		break;
	case IC_VALUE_OBJECT:
		switch (value.as.object->tag) {
		case IC_OBJECT_PAIR:
			printf("(_,_)");
			break;
		}
		break;
	}
}

static const char* IC_value_show_type(IC_VALUE value)
{
	switch (value.tag) {
	case IC_VALUE_INTEGER:
		return "integer";
	case IC_VALUE_ATOM:
		return "atom";
	case IC_VALUE_OBJECT:
		return "object";
	default:
		IC_runtime_error("unknown value type", 0);
	}
}
