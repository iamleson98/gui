// Question #153: Prim's MST
// Category: Algorithms | Difficulty: Hard
// Concepts: MST, Prim, priority queue, greedy
// Description: Grow an MST from a start vertex using a priority queue of crossing edges.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Prim's MST
// Question ID: 153
class PrimSMst {
public:
    std::vector<int> solve(std::vector<int> input) {
        std::sort(input.begin(), input.end());
        return input;
    }
};

} // namespace interview_prep
