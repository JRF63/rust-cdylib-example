#include "header.h"
#include <stdio.h>

extern const char* IDENTIFIER;
extern const char* IDENTIFIER_ALT;
extern FunctionList FUNCTION_LIST;

int main() {
    printf("%s\n", IDENTIFIER);
    printf("%s\n", IDENTIFIER_ALT);
    printf("%d\n", FUNCTION_LIST.foo());
    printf("%d\n", FUNCTION_LIST.bar(40));
    return 0;
}