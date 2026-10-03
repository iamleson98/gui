// Question #151: Greedy Job Scheduling with Deadlines
// Category: Algorithms | Difficulty: Hard
// Concepts: greedy, deadlines, disjoint set, profit
// Description: Maximize profit by scheduling unit-length jobs before their deadlines using disjoint-set slotting.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Greedy Job Scheduling with Deadlines
// Question ID: 151
class GreedyJobSchedulingWithDeadlines {
public:
    std::vector<int> solve(std::vector<int> input) {
        std::sort(input.begin(), input.end());
        return input;
    }
};

} // namespace interview_prep
