#ifndef IC_VALUE_H
#define IC_VALUE_H

#include "lar.h"

extern const char* IC_atom_names[];

struct IC_value {
	enum { IC_VALUE_INTEGER, IC_VALUE_ATOM, IC_VALUE_PAIR } tag;
	union {
		long integer;
		usize atom;
		IC_LAR_PROTO* pair;
	} as;
};

#define IC_INTEGER(x) ((IC_VALUE){IC_VALUE_INTEGER, {.integer = (x)}})
#define IC_ATOM(x) ((IC_VALUE){IC_VALUE_ATOM, {.atom = (x)}})
#define IC_PAIR(l) ((IC_VALUE){IC_VALUE_PAIR, {.pair = (l)}})

#define IC_IS_PAIR(v) ((v).tag == IC_VALUE_PAIR ? IC_ATOM(1) : IC_ATOM(2))

#define IC_IS_TRUTHY(v) ((v).tag == IC_VALUE_ATOM && (v).as.atom == 1)

IC_VALUE IC_add(IC_VALUE a, IC_VALUE b);
IC_VALUE IC_sub(IC_VALUE a, IC_VALUE b);
IC_VALUE IC_mul(IC_VALUE a, IC_VALUE b);
IC_VALUE IC_eq(IC_VALUE a, IC_VALUE b);
IC_VALUE IC_lq(IC_VALUE a, IC_VALUE b);
IC_VALUE IC_car(IC_VALUE v);
IC_VALUE IC_cdr(IC_VALUE v);
void IC_value_show(IC_VALUE value, bool print_newline);

#endif /* IC_VALUE_H */
