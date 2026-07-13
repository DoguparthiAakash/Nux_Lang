#include <stdio.h>

#define OP_CASE(op) OP_##op:

int main() {
    goto OP_0x01;
    
    OP_CASE(0x01)
        printf("Hello 0x01\n");
        return 0;
}
