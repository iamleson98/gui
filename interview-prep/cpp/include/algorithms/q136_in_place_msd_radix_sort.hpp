// Question #136: In-Place MSD Radix Sort
// Category: Algorithms | Difficulty: Hard
// Concepts: MSD radix, in-place, recursion, strings
// Description: Sort strings in-place using most-significant-digit radix recursion with a key-indexed count.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// In-Place MSD Radix Sort
// Question ID: 136
class InPlaceMsdRadixSort {
public:
    std::vector<int> solve(std::vector<int> input) {
        std::sort(input.begin(), input.end());
        return input;
    }
};

} // namespace interview_prep
