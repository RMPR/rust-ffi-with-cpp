#include <iostream>
#include <vector>

#include "rusty2.h"

int main() {
    std::cout << "rusty2_add(2, 40) = " << rusty2_add(2, 40) << "\n";

    std::vector<double> samples{4.0, 8.0, 15.0, 16.0, 23.0, 42.0};
    Stats stats = rusty2_stats(samples.data(), samples.size());
    std::cout << "min=" << stats.min << " max=" << stats.max
              << " mean=" << stats.mean << "\n";
}
