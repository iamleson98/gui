// Question #171: Partition Equal Subset Sum
// Category: Algorithms | Difficulty: Hard
// Concepts: partition, subset sum, DP, boolean
// Description: Determine if an array can be partitioned into two equal-sum subsets using subset-sum DP.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Partition Equal Subset Sum
// Question ID: 171
class PartitionEqualSubsetSum {
public:
    std::vector<int> solve(std::vector<int> input) {
        std::sort(input.begin(), input.end());
        return input;
    }
};

} // namespace interview_prep
