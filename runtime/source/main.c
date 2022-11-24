#include "operators.h"

/*
result = last(concat(list))

concat(l1) =
   if pair?(l1)
	then append(car(l1), concat(cdr(l1)))
	else 'nil

append(l2, l3) =
    if pair?(l2)
	then cons(car(l2), append(cdr(l2), l3))
	else l3

last(l4) =
    if pair?(l4)
	then if pair?(cdr(l4))
	    then last(cdr(l4))
	    else car(l4)
	else 'nil

list = cons(list1, cons(list2, 'nil))
list1 = cons(1, cons(2, 'nil))
list2 = cons(3, cons(4, 'nil))
 */

const char* atom_names[] = {
    "nil",
};

static const struct value* result(struct world world);
static const struct value* concat(struct world world);
static const struct value* append(struct world world);
static const struct value* last(struct world world);
static const struct value* list(struct world world);
static const struct value* list1(struct world world);
static const struct value* list2(struct world world);
static const struct value* l1(struct world world);
static const struct value* l2(struct world world);
static const struct value* l3(struct world world);
static const struct value* l4(struct world world);

int main(void)
{
	struct world world = world_new();

	const struct value* result_value = result(world);

	value_show(result_value);

	return 0;
}

static const struct value* result(struct world world)
{
	const struct value* v0 = last(world_cons_tag(&world, 0));
	return v0;
}

static const struct value* concat(struct world world)
{
	if (value_is_pair(l1(world))) {
		const struct value* v0 = append(world_cons_tag(&world, 0));
		return v0;
	} else {
		const struct value* v1 = value_atom(0);
		return v1;
	}
}

static const struct value* append(struct world world)
{
	if (value_is_pair(l2(world))) {
		enum choice c0;
		struct world w0 = world_uncons_choice(&world, &c0);
		switch (c0) {
		case CAR: {
			struct world w1 = world_cons_choice(&w0, CAR);
			return l2(w1);
		}
		case CDR: {
			return append(world_cons_tag(&w0, 1));
		}
		default: {
			runtime_error("unreachable");
		}
		}
	} else {
		return l3(world);
	}
}

static const struct value* last(struct world world)
{
	if (value_is_pair(l4(world))) {
		struct world w0 = world_cons_choice(&world, CDR);
		const struct value* v0 = l4(w0);
		if (value_is_pair(v0)) {
			return last(world_cons_tag(&world, 1));
		} else {
			struct world w1 = world_cons_choice(&world, CAR);
			return l4(w1);
		}
	} else {
		return value_atom(0);
	}
}

static const struct value* list(struct world world)
{
	enum choice c0;
	struct world w0 = world_uncons_choice(&world, &c0);
	switch (c0) {
	case CAR: {
		return list1(w0);
	}
	case CDR: {
		enum choice c1;
		struct world w1 = world_uncons_choice(&w0, &c1);
		switch (c1) {
		case CAR: {
			return list2(w1);
		}
		case CDR: {
			return value_atom(0);
		}
		default: {
			runtime_error("unreachable");
		}
		}
	}
	}
}

static const struct value* list1(struct world world)
{
	enum choice c0;
	struct world w0 = world_uncons_choice(&world, &c0);
	switch (c0) {
	case CAR: {
		return value_integer(1);
	}
	case CDR: {
		enum choice c1;
		struct world w1 = world_uncons_choice(&w0, &c1);
		switch (c1) {
		case CAR: {
			return value_integer(2);
		}
		case CDR: {
			return value_atom(0);
		}
		default: {
			runtime_error("unreachable");
		}
		}
	}
	default: {
		runtime_error("unreachable");
	}
	}
}

static const struct value* list2(struct world world)
{
	enum choice c0;
	struct world w0 = world_uncons_choice(&world, &c0);
	switch (c0) {
	case CAR: {
		return value_integer(3);
	}
	case CDR: {
		enum choice c1;
		struct world w1 = world_uncons_choice(&w0, &c1);
		switch (c1) {
		case CAR: {
			return value_integer(4);
		}
		case CDR: {
			return value_atom(0);
		}
		default: {
			runtime_error("unreachable");
		}
		}
	}
	default: {
		runtime_error("unreachable");
	}
	}
}

static const struct value* l1(struct world world)
{
	usize t0;
	struct world w0 = world_uncons_tag(&world, &t0);
	switch (t0) {
	case 0: {
		return list(w0);
	}
	case 1: {
		return l1(world_cons_choice(&w0, CDR));
	}
	default: {
		runtime_error("invalid tag");
	}
	}
}

static const struct value* l2(struct world world)
{
	usize t0;
	struct world w0 = world_uncons_tag(&world, &t0);
	switch (t0) {
	case 0: {
		return l1(world_cons_choice(&w0, CAR));
	}
	case 1: {
		return l2(world_cons_choice(&w0, CDR));
	}
	default: {
		runtime_error("invalid tag");
	}
	}
}

static const struct value* l3(struct world world)
{
	usize t0;
	struct world w0 = world_uncons_tag(&world, &t0);
	switch (t0) {
	case 0: {
		return concat(world_cons_tag(&w0, 1));
	}
	case 1: {
		return l3(w0);
	}
	default: {
		runtime_error("invalid tag");
	}
	}
}

static const struct value* l4(struct world world)
{
	usize t0;
	struct world w0 = world_uncons_tag(&world, &t0);
	switch (t0) {
	case 0: {
		return concat(world_cons_tag(&w0, 0));
	}
	case 1: {
		return l4(world_cons_choice(&w0, CDR));
	}
	default: {
		runtime_error("invalid tag");
	}
	}
}
