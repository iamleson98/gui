// Question #197: Andrew's Monotone Chain
// Category: Algorithms | Difficulty: Hard
// Concepts: convex hull, monotone chain, cross product, sort
// Description: Compute the upper and lower hulls by sorting points and scanning with cross-product tests.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Andrew's Monotone Chain
// Question ID: 197
class AndrewSMonotoneChain {
public:
    std::vector<int> solve(std::vector<int> input) {
        std::sort(input.begin(), input.end());
        return input;
    }
};

} // namespace interview_prep
