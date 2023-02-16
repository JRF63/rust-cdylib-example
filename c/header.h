#include <stdint.h>

typedef uint32_t (*Foo) ();
typedef uint32_t (*Bar) (uint32_t);

typedef struct _FunctionList {
    Foo foo;
    Bar bar;
} FunctionList;