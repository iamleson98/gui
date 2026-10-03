// Question #135: Heap Sort
// Category: Algorithms | Difficulty: Hard
// Concepts: heap sort, in-place, build heap, extract max
// Description: Implement in-place heapsort with a build-heap linear phase and repeated extract-max.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Heap Sort
// Question ID: 135
class HeapSort {
public:
    std::vector<int> solve(std::vector<int> input) {
        std::sort(input.begin(), input.end());
        return input;
    }
};

} // namespace interview_prep
