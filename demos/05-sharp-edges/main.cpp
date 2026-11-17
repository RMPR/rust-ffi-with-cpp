#include <iostream>

#include "sharp.h"

int main() {
    char *greeting = sharp_greet("C++");
    std::cout << greeting << "\n";
    sharp_string_free(greeting);  // NOT free(greeting): Rust allocated it

    int result = 0;
    if (sharp_divide(84, 2, &result) == 0) {
        std::cout << "84 / 2 = " << result << "\n";
    }
    if (sharp_divide(1, 0, &result) != 0) {
        std::cout << "1 / 0 failed cleanly: Rust panicked, C++ survived\n";
    }
}
