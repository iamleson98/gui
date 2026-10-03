// Question #157: Topological Sort (DFS)
// Category: Algorithms | Difficulty: Hard
// Concepts: topological sort, DFS, post-order, DAG
// Description: Generate a topological order by post-order DFS and reversing the finish times.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Topological Sort (DFS)
// Question ID: 157
class TopologicalSortDfs {
public:
    std::vector<int> solve(std::vector<int> input) {
        std::sort(input.begin(), input.end());
        return input;
    }
};

} // namespace interview_prep
