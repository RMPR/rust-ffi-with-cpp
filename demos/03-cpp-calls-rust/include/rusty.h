#ifndef RUSTY_H
#define RUSTY_H

#include <stddef.h>

#ifdef __cplusplus
extern "C" {  // C++ compilers: do not mangle these names
#endif

typedef struct {
    double min;
    double max;
    double mean;
} Stats;

int rusty_add(int a, int b);
Stats rusty_stats(const double *data, size_t len);

#ifdef __cplusplus
}
#endif

#endif
