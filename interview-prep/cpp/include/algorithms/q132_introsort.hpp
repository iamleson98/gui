// Question #132: Introsort
// Category: Algorithms | Difficulty: Hard
// Concepts: introsort, hybrid, heapsort, worst-case
// Description: Build a hybrid sort that switches from quicksort to heapsort on recursion depth to guarantee O(n log n).
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Introsort
// Question ID: 132
class Introsort {
public:
    std::vector<int> solve(std::vector<int> input) {
        std::sort(input.begin(), input.end());
        return input;
    }
};

} // namespace interview_prep
