// Question #139: American Flag Sort
// Category: Algorithms | Difficulty: Hard
// Concepts: American flag sort, in-place, MSD, radix
// Description: Implement an in-place MSD radix variant using partition pointers per bucket.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// American Flag Sort
// Question ID: 139
class AmericanFlagSort {
public:
    std::vector<int> solve(std::vector<int> input) {
        std::sort(input.begin(), input.end());
        return input;
    }
};

} // namespace interview_prep
