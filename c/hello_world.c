#include <stdint.h>
#include <stdio.h>

void hello_world() {
    printf("Hello, World!\n");
}

void hello_to(const char* who, size_t times) {
    for (size_t i = 0; i < times; i++) {
        printf("Hello, %s!\n", who);
    }
}

size_t super_fast_add(size_t a, size_t b) {
    return a + b;
}
