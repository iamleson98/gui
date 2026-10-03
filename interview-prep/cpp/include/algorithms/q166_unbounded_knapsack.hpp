// Question #166: Unbounded Knapsack
// Category: Algorithms | Difficulty: Hard
// Concepts: knapsack, unbounded, 1D DP, reuse
// Description: Solve the unbounded knapsack where items can be reused with a 1D DP.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Unbounded Knapsack
// Question ID: 166
class UnboundedKnapsack {
public:
    std::vector<int> solve(std::vector<int> input) {
        std::sort(input.begin(), input.end());
        return input;
    }
};

} // namespace interview_prep
