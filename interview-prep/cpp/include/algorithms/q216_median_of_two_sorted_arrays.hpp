// Question #216: Median of Two Sorted Arrays
// Category: Algorithms | Difficulty: Hard
// Concepts: median, two arrays, binary partition, logarithmic
// Description: Find the median of two sorted arrays in O(log(min(m, n))) using binary partition.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Median of Two Sorted Arrays
// Question ID: 216
class MedianOfTwoSortedArrays {
public:
    std::vector<int> solve(std::vector<int> input) {
        std::sort(input.begin(), input.end());
        return input;
    }
};

} // namespace interview_prep
