// Question #184: Max Flow: Ford-Fulkerson
// Category: Algorithms | Difficulty: Hard
// Concepts: max flow, Ford-Fulkerson, augmenting path, residual
// Description: Compute max flow by augmenting along any augmenting path until none remain.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Max Flow: Ford-Fulkerson
// Question ID: 184
class MaxFlowFordFulkerson {
public:
    std::vector<int> solve(std::vector<int> input) {
        std::sort(input.begin(), input.end());
        return input;
    }
};

} // namespace interview_prep
