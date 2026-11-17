#include "mathlib.h"

#include <math.h>

int add(int a, int b) { return a + b; }

double point_magnitude(Point p) { return sqrt(p.x * p.x + p.y * p.y); }

void point_scale(Point *p, double factor) {
    p->x *= factor;
    p->y *= factor;
}
