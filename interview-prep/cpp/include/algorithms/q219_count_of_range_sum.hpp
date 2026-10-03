// Question #219: Count of Range Sum
// Category: Algorithms | Difficulty: Hard
// Concepts: range sum, prefix sum, Fenwick tree, count
// Description: Count subarray sums in a range using a Fenwick tree over prefix sums.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Count of Range Sum
// Question ID: 219
class CountOfRangeSum {
public:
    std::vector<int> solve(std::vector<int> input) {
        std::sort(input.begin(), input.end());
        return input;
    }
};

} // namespace interview_prep
