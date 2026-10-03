// Question #192: Z-Algorithm
// Category: Algorithms | Difficulty: Hard
// Concepts: Z-array, string matching, prefix, linear
// Description: Compute the Z-array of a string for pattern matching and pattern analysis in linear time.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Z-Algorithm
// Question ID: 192
class ZAlgorithm {
public:
    std::vector<int> solve(std::vector<int> input) {
        std::sort(input.begin(), input.end());
        return input;
    }
};

} // namespace interview_prep
