#pragma once

#include <map>
#include <string>

// "Real" C++: a class, std::string, std::map, an exception.
class WordCounter {
public:
    void add(const std::string& word);
    int count(const std::string& word) const;
    std::size_t distinct() const { return counts_.size(); }

private:
    std::map<std::string, int> counts_;
};
