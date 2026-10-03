// Question #134: External Merge Sort
// Category: Algorithms | Difficulty: Hard
// Concepts: external sort, k-way merge, runs, I/O
// Description: Sort datasets larger than memory using k-way merging of sorted runs on disk.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// External Merge Sort
// Question ID: 134
class ExternalMergeSort {
public:
    std::vector<int> solve(std::vector<int> input) {
        std::sort(input.begin(), input.end());
        return input;
    }
};

} // namespace interview_prep
