// Question #156: Topological Sort (Kahn)
// Category: Algorithms | Difficulty: Hard
// Concepts: topological sort, Kahn, in-degree, DAG
// Description: Produce a topological ordering of a DAG using in-degree counts and a queue.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Topological Sort (Kahn)
// Question ID: 156
class TopologicalSortKahn {
public:
    std::vector<int> solve(std::vector<int> input) {
        std::sort(input.begin(), input.end());
        return input;
    }
};

} // namespace interview_prep
