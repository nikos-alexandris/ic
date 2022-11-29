#include "world.h"

#include <malloc.h>

static IC_WORLD IC_world_from(IC_TAG_LIST* tags, IC_CHOICE_LIST* choices);

IC_WORLD IC_world_new(void)
{
	IC_WORLD world = {0};

	return world;
}

IC_WORLD IC_world_drop_choices(const IC_WORLD* world) { return IC_world_from(world->tags, NULL); }

bool IC_world_has_choices(const IC_WORLD* world) { return world->choices != NULL; }

IC_WORLD IC_world_cons_tag(const IC_WORLD* world, usize tag)
{
	IC_TAG_LIST* tag_list = malloc(sizeof(*tag_list));
	if (!tag_list) {
		IC_runtime_error("out of memory", 0);
	}
	tag_list->tag = tag;
	tag_list->cdr = world->tags;

	return IC_world_from(tag_list, world->choices);
}

IC_WORLD IC_world_uncons_tag(const IC_WORLD* world, usize* tag)
{
	if (!world->tags) {
		IC_runtime_error("world has no tags", 0);
	}

	*tag = world->tags->tag;

	return IC_world_from(world->tags->cdr, world->choices);
}

IC_WORLD IC_world_cons_choice(const IC_WORLD* world, IC_CHOICE choice)
{
	IC_CHOICE_LIST* choice_list = malloc(sizeof(*choice_list));
	if (!choice_list) {
		IC_runtime_error("out of memory", 0);
	}
	choice_list->choice = choice;
	choice_list->cdr = world->choices;

	return IC_world_from(world->tags, choice_list);
}

IC_WORLD IC_world_uncons_choice(const IC_WORLD* world, IC_CHOICE* choice)
{
	if (!world->choices) {
		IC_runtime_error("world has no choices", 0);
	}

	*choice = world->choices->choice;

	return IC_world_from(world->tags, world->choices->cdr);
}

IC_WORLD IC_world_append_choice(const IC_WORLD* world, IC_CHOICE choice)
{
	if (!world->choices) {
		return IC_world_cons_choice(world, choice);
	}

	IC_CHOICE c;
	IC_WORLD cs = IC_world_uncons_choice(world, &c);
	IC_WORLD a = IC_world_append_choice(&cs, choice);
	return IC_world_cons_choice(&a, c);
}

static IC_WORLD IC_world_from(IC_TAG_LIST* tags, IC_CHOICE_LIST* choices)
{
	IC_WORLD world;

	world.tags = tags;
	world.choices = choices;

	return world;
}
