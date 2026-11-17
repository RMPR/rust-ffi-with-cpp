#include <iostream>
#include <vector>

#include "rusty.h"

int main() {
    std::cout << "rusty_add(2, 40) = " << rusty_add(2, 40) << "\n";

    std::vector<double> samples{4.0, 8.0, 15.0, 16.0, 23.0, 42.0};
    Stats stats = rusty_stats(samples.data(), samples.size());
    std::cout << "min=" << stats.min << " max=" << stats.max
              << " mean=" << stats.mean << "\n";
}
