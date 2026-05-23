
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>

int64_t stack[1024];
int sp = -1;
int64_t vars[1024]; // Simple global vars

#define PUSH(x) stack[++sp] = (x)
#define POP() stack[sp--]

int main() {
    goto __start_execution;
__start_execution:
    PUSH(10);
    printf("%c", (char)POP());
    PUSH(70);
    printf("%c", (char)POP());
    PUSH(111);
    printf("%c", (char)POP());
    PUSH(114);
    printf("%c", (char)POP());
    PUSH(32);
    printf("%c", (char)POP());
    PUSH(76);
    printf("%c", (char)POP());
    PUSH(111);
    printf("%c", (char)POP());
    PUSH(111);
    printf("%c", (char)POP());
    PUSH(112);
    printf("%c", (char)POP());
    PUSH(32);
    printf("%c", (char)POP());
    PUSH(40);
    printf("%c", (char)POP());
    PUSH(48);
    printf("%c", (char)POP());
    PUSH(32);
    printf("%c", (char)POP());
    PUSH(116);
    printf("%c", (char)POP());
    PUSH(111);
    printf("%c", (char)POP());
    PUSH(32);
    printf("%c", (char)POP());
    PUSH(52);
    printf("%c", (char)POP());
    PUSH(41);
    printf("%c", (char)POP());
    PUSH(58);
    printf("%c", (char)POP());
    PUSH(10);
    printf("%c", (char)POP());
    PUSH(0);
    PUSH(0);
    { int64_t addr = POP(); int64_t val = POP(); vars[addr / 8] = val; }
__for_start_0:
    PUSH(0);
    { int64_t addr = POP(); PUSH(vars[addr / 8]); }
    PUSH(5);
    { int64_t b = POP(); int64_t a = POP(); PUSH(a < b ? 1 : 0); }
    PUSH(0);
    { int64_t b = POP(); int64_t a = POP(); if (a == b) goto __for_end_0; }
    goto __for_body_0;
__for_step_0:
    PUSH(0);
    { int64_t addr = POP(); PUSH(vars[addr / 8]); }
    PUSH(1);
    { int64_t b = POP(); int64_t a = POP(); PUSH(a + b); }
    PUSH(0);
    { int64_t addr = POP(); int64_t val = POP(); vars[addr / 8] = val; }
    goto __for_start_0;
__for_body_0:
    PUSH(0);
    { int64_t addr = POP(); PUSH(vars[addr / 8]); }
    printf("%ld", POP());
    goto __for_step_0;
__for_end_0:
    PUSH(10);
    printf("%c", (char)POP());
    PUSH(10);
    printf("%c", (char)POP());
    PUSH(10);
    printf("%c", (char)POP());
    PUSH(73);
    printf("%c", (char)POP());
    PUSH(110);
    printf("%c", (char)POP());
    PUSH(108);
    printf("%c", (char)POP());
    PUSH(105);
    printf("%c", (char)POP());
    PUSH(110);
    printf("%c", (char)POP());
    PUSH(101);
    printf("%c", (char)POP());
    PUSH(32);
    printf("%c", (char)POP());
    PUSH(65);
    printf("%c", (char)POP());
    PUSH(83);
    printf("%c", (char)POP());
    PUSH(77);
    printf("%c", (char)POP());
    PUSH(32);
    printf("%c", (char)POP());
    PUSH(84);
    printf("%c", (char)POP());
    PUSH(101);
    printf("%c", (char)POP());
    PUSH(115);
    printf("%c", (char)POP());
    PUSH(116);
    printf("%c", (char)POP());
    PUSH(32);
    printf("%c", (char)POP());
    PUSH(40);
    printf("%c", (char)POP());
    PUSH(53);
    printf("%c", (char)POP());
    PUSH(32);
    printf("%c", (char)POP());
    PUSH(43);
    printf("%c", (char)POP());
    PUSH(32);
    printf("%c", (char)POP());
    PUSH(53);
    printf("%c", (char)POP());
    PUSH(41);
    printf("%c", (char)POP());
    PUSH(58);
    printf("%c", (char)POP());
    PUSH(10);
    printf("%c", (char)POP());
    PUSH(5);
    PUSH(5);
    { int64_t b = POP(); int64_t a = POP(); PUSH(a + b); }
    printf("%ld", POP());
    PUSH(10);
    printf("%c", (char)POP());
    PUSH(10);
    printf("%c", (char)POP());
    return 0;
    return 0;
}
