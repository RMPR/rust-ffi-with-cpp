#include "word_counter.hpp"

#include <stdexcept>

void WordCounter::add(const std::string& word) {
    if (word.empty()) {
        throw std::invalid_argument("empty word");
    }
    ++counts_[word];
}

int WordCounter::count(const std::string& word) const {
    auto it = counts_.find(word);
    return it == counts_.end() ? 0 : it->second;
}
