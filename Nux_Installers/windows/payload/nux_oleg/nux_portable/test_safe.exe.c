
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>


int64_t vars[1024];

#define PUSH(x) stack[++sp] = (x)
#define POP() stack[sp--]

// Standard IO Mappings
#define NUX_PRINT_VAL(x) printf("%ld", (x))
#define NUX_PRINT_CHAR(x) printf("%c", (char)(x))
#define NUX_INPUT() getchar()
#define NUX_EXIT() return 0

int main() {
    int64_t r[1024] = {0};
    goto __start_execution;
    goto skip_test_verify;
test_verify:
    r[0] = 1;
    NUX_PRINT_VAL(r[0]);
    r[0] = 1;
    r[1] = 1;
    r[0] = r[0] == r[1] ? 1 : 0;
    if (r[0] == 0) { fprintf(stderr, "Verification Failed!\n"); exit(1); }
    r[0] = 2;
    NUX_PRINT_VAL(r[0]);
    r[0] = 1;
    r[1] = 0;
    r[0] = r[0] == r[1] ? 1 : 0;
    if (r[0] == 0) { fprintf(stderr, "Verification Failed!\n"); exit(1); }
    r[0] = 3;
    NUX_PRINT_VAL(r[0]);
    r[0] = 0;
    return 0;
skip_test_verify:
    goto skip_test_safe;
test_safe:
    r[0] = 100;
    NUX_PRINT_VAL(r[0]);
    r[0] = 200;
    NUX_PRINT_VAL(r[0]);
    r[0] = 0;
    return 0;
skip_test_safe:
    goto skip_main;
main:
    goto test_verify;
    goto test_safe;
    r[0] = 0;
    return 0;
skip_main:
__start_execution:
    NUX_EXIT();
    return 0;
}
