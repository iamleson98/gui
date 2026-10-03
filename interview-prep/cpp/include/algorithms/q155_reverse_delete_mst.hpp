// Question #155: Reverse Delete MST
// Category: Algorithms | Difficulty: Hard
// Concepts: MST, reverse delete, cycle, greedy
// Description: Build MST by deleting the heaviest edge that does not disconnect the graph.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Reverse Delete MST
// Question ID: 155
class ReverseDeleteMst {
public:
    std::vector<int> solve(std::vector<int> input) {
        std::sort(input.begin(), input.end());
        return input;
    }
};

} // namespace interview_prep
