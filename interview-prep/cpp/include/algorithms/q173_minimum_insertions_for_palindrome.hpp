// Question #173: Minimum Insertions for Palindrome
// Category: Algorithms | Difficulty: Hard
// Concepts: palindrome, insertions, LCS, DP
// Description: Compute the minimum insertions to make a string a palindrome using LCS with its reverse.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Minimum Insertions for Palindrome
// Question ID: 173
class MinimumInsertionsForPalindrome {
public:
    std::vector<int> solve(std::vector<int> input) {
        std::sort(input.begin(), input.end());
        return input;
    }
};

} // namespace interview_prep
