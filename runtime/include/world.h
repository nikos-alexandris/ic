#ifndef WORLD_H
#define WORLD_H

#include "common.h"

struct tag_list {
	usize tag;
	struct tag_list* cdr;
};

enum choice { CAR = 0, CDR = 1 };

struct choice_list {
	enum choice choice;
	struct choice_list* cdr;
};

struct world {
	struct tag_list* tags;
	struct choice_list* choices;
};

struct world world_new(void);

struct world world_cons_tag(const struct world* world, usize tag);

struct world world_uncons_tag(const struct world* world, usize* tag);

struct world world_cons_choice(const struct world* world, enum choice choice);

struct world world_uncons_choice(const struct world* world, enum choice* choice);

#endif /* WORLD_H */
