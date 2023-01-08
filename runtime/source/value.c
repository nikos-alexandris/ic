#include "value.h"

#include <stdio.h>

static const char* IC_value_show_type(IC_VALUE value);

IC_VALUE IC_add(IC_VALUE a, IC_VALUE b)
{
	if (a.tag == IC_VALUE_INTEGER && b.tag == IC_VALUE_INTEGER) {
		return IC_INTEGER(a.as.integer + b.as.integer);
	} else {
		IC_runtime_error("cannot add %s and %s", IC_value_show_type(a), IC_value_show_type(b));
	}
}

IC_VALUE IC_sub(IC_VALUE a, IC_VALUE b)
{
	if (a.tag == IC_VALUE_INTEGER && b.tag == IC_VALUE_INTEGER) {
		return IC_INTEGER(a.as.integer - b.as.integer);
	} else {
		IC_runtime_error("cannot subtract %s and %s", IC_value_show_type(a), IC_value_show_type(b));
	}
}

IC_VALUE IC_mul(IC_VALUE a, IC_VALUE b)
{
	if (a.tag == IC_VALUE_INTEGER && b.tag == IC_VALUE_INTEGER) {
		return IC_INTEGER(a.as.integer * b.as.integer);
	} else {
		IC_runtime_error("cannot multiply %s and %s", IC_value_show_type(a), IC_value_show_type(b));
	}
}

IC_VALUE IC_eq(IC_VALUE a, IC_VALUE b)
{
	if (a.tag == IC_VALUE_INTEGER && b.tag == IC_VALUE_INTEGER) {
		return a.as.integer == b.as.integer ? IC_ATOM(1) : IC_ATOM(2);
	} else if (a.tag == IC_VALUE_ATOM && b.tag == IC_VALUE_ATOM) {
		return a.as.atom == b.as.atom ? IC_ATOM(1) : IC_ATOM(2);
	} else {
		return IC_ATOM(2);
	}
}

IC_VALUE IC_neq(IC_VALUE a, IC_VALUE b)
{
	if (a.tag == IC_VALUE_INTEGER && b.tag == IC_VALUE_INTEGER) {
		return a.as.integer != b.as.integer ? IC_ATOM(1) : IC_ATOM(2);
	} else if (a.tag == IC_VALUE_ATOM && b.tag == IC_VALUE_ATOM) {
		return a.as.atom != b.as.atom ? IC_ATOM(1) : IC_ATOM(2);
	} else {
		return IC_ATOM(1);
	}
}

IC_VALUE IC_lt(IC_VALUE a, IC_VALUE b)
{
	if (a.tag == IC_VALUE_INTEGER && b.tag == IC_VALUE_INTEGER) {
		return a.as.integer < b.as.integer ? IC_ATOM(1) : IC_ATOM(2);
	} else {
		IC_runtime_error("cannot compare %s and %s", IC_value_show_type(a), IC_value_show_type(b));
	}
}

IC_VALUE IC_gt(IC_VALUE a, IC_VALUE b)
{
	if (a.tag == IC_VALUE_INTEGER && b.tag == IC_VALUE_INTEGER) {
		return a.as.integer > b.as.integer ? IC_ATOM(1) : IC_ATOM(2);
	} else {
		IC_runtime_error("cannot compare %s and %s", IC_value_show_type(a), IC_value_show_type(b));
	}
}

IC_VALUE IC_le(IC_VALUE a, IC_VALUE b)
{
	if (a.tag == IC_VALUE_INTEGER && b.tag == IC_VALUE_INTEGER) {
		return a.as.integer <= b.as.integer ? IC_ATOM(1) : IC_ATOM(2);
	} else {
		IC_runtime_error("cannot use 'lq?' on %s and %s", IC_value_show_type(a), IC_value_show_type(b));
	}
}

IC_VALUE IC_ge(IC_VALUE a, IC_VALUE b)
{
	if (a.tag == IC_VALUE_INTEGER && b.tag == IC_VALUE_INTEGER) {
		return a.as.integer >= b.as.integer ? IC_ATOM(1) : IC_ATOM(2);
	} else {
		IC_runtime_error("cannot use 'gq?' on %s and %s", IC_value_show_type(a), IC_value_show_type(b));
	}
}

IC_VALUE IC_car(IC_VALUE v)
{
	if (v.tag != IC_VALUE_PAIR) {
		IC_runtime_error("cannot use 'car' on %s", IC_value_show_type(v));
	}
	return IC_lar_get_arg(v.as.pair, 0);
}

IC_VALUE IC_cdr(IC_VALUE v)
{
	if (v.tag != IC_VALUE_PAIR) {
		IC_runtime_error("cannot use 'cdr' on %s", IC_value_show_type(v));
	}
	return IC_lar_get_arg(v.as.pair, 1);
}

void IC_value_show(IC_VALUE value, bool print_newline)
{
	switch (value.tag) {
	case IC_VALUE_INTEGER: {
		printf("%ld", value.as.integer);
		break;
	}
	case IC_VALUE_ATOM: {
		printf("'%s", IC_atom_names[value.as.atom]);
		break;
	}
	case IC_VALUE_PAIR: {
		printf("(");
		IC_value_show(IC_lar_get_arg(value.as.pair, 0), false);
		printf(" . ");
		IC_value_show(IC_lar_get_arg(value.as.pair, 1), false);
		printf(")");
		break;
	}
	}
	if (print_newline) {
		printf("\n");
	}
}

static const char* IC_value_show_type(IC_VALUE value)
{
	switch (value.tag) {
	case IC_VALUE_INTEGER:
		return "integer";
	case IC_VALUE_ATOM:
		return "atom";
	case IC_VALUE_PAIR:
		return "pair";
	default:
		IC_runtime_error("unknown value type", 0);
	}
}
