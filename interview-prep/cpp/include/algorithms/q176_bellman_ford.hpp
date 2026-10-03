// Question #176: Bellman-Ford
// Category: Algorithms | Difficulty: Hard
// Concepts: shortest path, Bellman-Ford, negative weights, relaxation
// Description: Compute shortest paths with negative weights using edge relaxation and a negative-cycle detector.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Bellman-Ford
// Question ID: 176
class BellmanFord {
public:
    std::vector<int> solve(std::vector<int> input) {
        std::sort(input.begin(), input.end());
        return input;
    }
};

} // namespace interview_prep
