#ifndef WORD_COUNTER_SHIM_H
#define WORD_COUNTER_SHIM_H

#include <stdbool.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

// Opaque: C (and Rust) only ever see a pointer to this.
typedef struct WordCounter WordCounter;

WordCounter *word_counter_new(void);
void word_counter_free(WordCounter *counter);
bool word_counter_add(WordCounter *counter, const char *word);
int word_counter_count(const WordCounter *counter, const char *word);
size_t word_counter_distinct(const WordCounter *counter);

#ifdef __cplusplus
}
#endif

#endif
