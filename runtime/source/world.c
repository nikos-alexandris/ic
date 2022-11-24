#include "world.h"

#include <malloc.h>

static struct world world_from(struct tag_list* tags, struct choice_list* choices);

struct world world_new(void)
{
	struct world world = {0};

	return world;
}

struct world world_cons_tag(const struct world* world, usize tag)
{
	struct tag_list* tag_list = malloc(sizeof(*tag_list));
	if (!tag_list) {
		runtime_error("out of memory");
	}
	tag_list->tag = tag;
	tag_list->cdr = world->tags;

	return world_from(tag_list, world->choices);
}

struct world world_uncons_tag(const struct world* world, usize* tag)
{
	if (!world->tags) {
		runtime_error("world has no tags");
	}

	*tag = world->tags->tag;

	return world_from(world->tags->cdr, world->choices);
}

struct world world_cons_choice(const struct world* world, enum choice choice)
{
	struct choice_list* choice_list = malloc(sizeof(*choice_list));
	if (!choice_list) {
		runtime_error("out of memory");
	}
	choice_list->choice = choice;
	choice_list->cdr = world->choices;

	return world_from(world->tags, choice_list);
}

struct world world_uncons_choice(const struct world* world, enum choice* choice)
{
	if (!world->choices) {
		runtime_error("world has no choices");
	}

	*choice = world->choices->choice;

	return world_from(world->tags, world->choices->cdr);
}

static struct world world_from(struct tag_list* tags, struct choice_list* choices)
{
	struct world world;

	world.tags = tags;
	world.choices = choices;

	return world;
}
