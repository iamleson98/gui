// Question #147: Fractional Knapsack (Greedy)
// Category: Algorithms | Difficulty: Hard
// Concepts: fractional knapsack, greedy, value/weight, sort
// Description: Solve the fractional knapsack by sorting items by value/weight and greedily filling.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Fractional Knapsack (Greedy)
// Question ID: 147
class FractionalKnapsackGreedy {
public:
    std::vector<int> solve(std::vector<int> input) {
        std::sort(input.begin(), input.end());
        return input;
    }
};

} // namespace interview_prep
