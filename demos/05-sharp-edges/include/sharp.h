#ifndef SHARP_H
#define SHARP_H

#ifdef __cplusplus
extern "C" {
#endif

/* Returns a new string owned by Rust. Release with sharp_string_free(). */
char *sharp_greet(const char *name);
void sharp_string_free(char *s);

/* Returns 0 on success, -1 on error. */
int sharp_divide(int a, int b, int *out);

#ifdef __cplusplus
}
#endif

#endif
