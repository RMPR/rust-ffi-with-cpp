#include "shim.h"

#include "word_counter.hpp"

extern "C" {

WordCounter *word_counter_new(void) { return new WordCounter(); }

void word_counter_free(WordCounter *counter) { delete counter; }

// Exceptions must not escape into Rust: catch them, return an error code.
bool word_counter_add(WordCounter *counter, const char *word) {
    try {
        counter->add(word);
        return true;
    } catch (...) {
        return false;
    }
}

int word_counter_count(const WordCounter *counter, const char *word) {
    return counter->count(word);
}

size_t word_counter_distinct(const WordCounter *counter) {
    return counter->distinct();
}

}  // extern "C"
