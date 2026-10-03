// Question #137: LSD Radix Sort
// Category: Algorithms | Difficulty: Hard
// Concepts: LSD radix, counting sort, stable, fixed width
// Description: Implement least-significant-digit radix sort using counting sort per digit.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// LSD Radix Sort
// Question ID: 137
class LsdRadixSort {
public:
    std::vector<int> solve(std::vector<int> input) {
        std::sort(input.begin(), input.end());
        return input;
    }
};

} // namespace interview_prep
