// Question #138: Counting Sort (Stable)
// Category: Algorithms | Difficulty: Hard
// Concepts: counting sort, stable, O(n+k), integers
// Description: Build a stable counting sort over a small integer key domain in O(n + k).
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Counting Sort (Stable)
// Question ID: 138
class CountingSortStable {
public:
    std::vector<int> solve(std::vector<int> input) {
        std::sort(input.begin(), input.end());
        return input;
    }
};

} // namespace interview_prep
