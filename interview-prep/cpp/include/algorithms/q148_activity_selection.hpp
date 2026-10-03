// Question #148: Activity Selection
// Category: Algorithms | Difficulty: Hard
// Concepts: greedy, intervals, earliest finish, optimal
// Description: Solve interval scheduling by greedily picking the earliest-finishing compatible activity.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Activity Selection
// Question ID: 148
class ActivitySelection {
public:
    std::vector<int> solve(std::vector<int> input) {
        std::sort(input.begin(), input.end());
        return input;
    }
};

} // namespace interview_prep
