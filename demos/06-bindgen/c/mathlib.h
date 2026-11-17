#ifndef MATHLIB_H
#define MATHLIB_H

typedef struct {
    double x;
    double y;
} Point;

int add(int a, int b);
double point_length(Point p);
void point_scale(Point *p, double factor);

#endif
