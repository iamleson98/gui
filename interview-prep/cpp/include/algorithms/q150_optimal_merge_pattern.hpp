// Question #150: Optimal Merge Pattern
// Category: Algorithms | Difficulty: Hard
// Concepts: greedy, merge cost, min-heap, optimal
// Description: Minimize the cost of merging sorted runs by always merging the two smallest.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Optimal Merge Pattern
// Question ID: 150
class OptimalMergePattern {
public:
    std::vector<int> solve(std::vector<int> input) {
        std::sort(input.begin(), input.end());
        return input;
    }
};

} // namespace interview_prep
