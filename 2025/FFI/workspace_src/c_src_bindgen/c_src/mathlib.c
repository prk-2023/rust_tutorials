#include "mathlib.h"
#include <stdio.h>

float add_points(Point p) {
    printf("return from C: %d \n", p.x+p.y);
    return p.x + p.y;
}

void print_hello() {
    printf("Hello from C!\n");
}
