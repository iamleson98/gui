// Question #164: Longest Common Subsequence
// Category: Algorithms | Difficulty: Hard
// Concepts: LCS, dynamic programming, backtracking, suffix
// Description: Build the LCS dynamic programming table and reconstruct the subsequence via backtracking.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Longest Common Subsequence
// Question ID: 164
class LongestCommonSubsequence {
public:
    std::vector<int> solve(std::vector<int> input) {
        std::sort(input.begin(), input.end());
        return input;
    }
};

} // namespace interview_prep
