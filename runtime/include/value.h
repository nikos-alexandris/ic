#ifndef IC_VALUE_H
#define IC_VALUE_H

#include <stdint.h>

typedef uint64_t IC_VALUE;

#define IC_IS_INT(v) ((v)&1)

#define IC_IS_PTR(v) (!IC_IS_INT(v))

#define IC_BOX(v) (((v) << 1) | 1)

#define IC_UNBOX(v) ((v) >> 1)

#define IC_ADD(v1, v2) (IC_BOX(IC_UNBOX(v1) + IC_UNBOX(v2)))

#define IC_SUB(v1, v2) (IC_BOX(IC_UNBOX(v1) - IC_UNBOX(v2)))

#define IC_MUL(v1, v2) (IC_BOX(IC_UNBOX(v1) * IC_UNBOX(v2)))

#define IC_EQ(v1, v2) (IC_BOX(IC_UNBOX(v1) == IC_UNBOX(v2)))

#define IC_NEQ(v1, v2) (IC_BOX(IC_UNBOX(v1) != IC_UNBOX(v2)))

#define IC_LT(v1, v2) (IC_BOX(IC_UNBOX(v1) < IC_UNBOX(v2)))

#define IC_GT(v1, v2) (IC_BOX(IC_UNBOX(v1) > IC_UNBOX(v2)))

#define IC_LE(v1, v2) (IC_BOX(IC_UNBOX(v1) <= IC_UNBOX(v2)))

#define IC_GE(v1, v2) (IC_BOX(IC_UNBOX(v1) >= IC_UNBOX(v2)))

void IC_value_show(IC_VALUE v);

#endif /* IC_VALUE_H */
