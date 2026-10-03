// Question #194: Suffix Array Construction (SA-IS)
// Category: Algorithms | Difficulty: Hard
// Concepts: suffix array, SA-IS, induced sorting, linear
// Description: Construct a suffix array in linear time using the SA-IS induced-sorting algorithm.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Suffix Array Construction (SA-IS)
// Question ID: 194
class SuffixArrayConstructionSaIs {
public:
    std::vector<int> solve(std::vector<int> input) {
        std::sort(input.begin(), input.end());
        return input;
    }
};

} // namespace interview_prep
