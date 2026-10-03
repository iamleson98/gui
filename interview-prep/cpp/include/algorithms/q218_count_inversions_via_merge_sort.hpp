// Question #218: Count Inversions via Merge Sort
// Category: Algorithms | Difficulty: Hard
// Concepts: inversions, merge sort, count, O(n log n)
// Description: Count array inversions in O(n log n) by augmenting merge sort with a counter.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Count Inversions via Merge Sort
// Question ID: 218
class CountInversionsViaMergeSort {
public:
    std::vector<int> solve(std::vector<int> input) {
        std::sort(input.begin(), input.end());
        return input;
    }
};

} // namespace interview_prep
