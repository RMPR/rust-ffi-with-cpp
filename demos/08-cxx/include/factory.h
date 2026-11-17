#pragma once

#include <memory>

#include "word_counter.hpp"

// The only glue cxx needs: it cannot call a C++ constructor directly.
inline std::unique_ptr<WordCounter> new_word_counter() {
    return std::make_unique<WordCounter>();
}
