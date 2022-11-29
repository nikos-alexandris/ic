#ifndef IC_WORLD_H
#define IC_WORLD_H

#include "common.h"

typedef struct IC_tag_list {
	usize tag;
	struct IC_tag_list* cdr;
} IC_TAG_LIST;

typedef enum IC_choice { IC_CAR = 0, IC_CDR = 1 } IC_CHOICE;

typedef struct IC_choice_list {
	IC_CHOICE choice;
	struct IC_choice_list* cdr;
} IC_CHOICE_LIST;

typedef struct IC_world {
	IC_TAG_LIST* tags;
	IC_CHOICE_LIST* choices;
} IC_WORLD;

IC_WORLD IC_world_new(void);

IC_WORLD IC_world_drop_choices(const IC_WORLD* world);

bool IC_world_has_choices(const IC_WORLD* world);

IC_WORLD IC_world_cons_tag(const IC_WORLD* world, usize tag);

IC_WORLD IC_world_uncons_tag(const IC_WORLD* world, usize* tag);

IC_WORLD IC_world_cons_choice(const IC_WORLD* world, IC_CHOICE choice);

IC_WORLD IC_world_uncons_choice(const IC_WORLD* world, IC_CHOICE* choice);

IC_WORLD IC_world_append_choice(const IC_WORLD* world, IC_CHOICE choice);

#endif /* IC_WORLD_H */
