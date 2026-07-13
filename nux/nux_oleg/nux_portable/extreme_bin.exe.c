
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
    goto skip_main;
main:
    r[0] = 100;
    r[1] = 200;
    r[2] = r[0];
    r[3] = r[1];
    r[2] = r[2] * r[3];
    r[3] = r[2];
    r[4] = r[0];
    r[3] = r[3] + r[4];
    r[4] = r[1];
    r[3] = r[3] - r[4];
    r[4] = r[3];
    NUX_PRINT_VAL(r[4]);
    r[4] = 10;
    NUX_PRINT_CHAR(r[4]);
    r[4] = 0;
    return 0;
skip_main:
    goto skip___main;
__main:
    goto main;
    r[0] = 0;
    return 0;
skip___main:
__start_execution:
    goto __main;
    NUX_EXIT();
    return 0;
}
