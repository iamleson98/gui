// Question #178: A* Search
// Category: Algorithms | Difficulty: Hard
// Concepts: A*, heuristic, priority queue, shortest path
// Description: Implement A* with a consistent heuristic to find shortest paths faster than Dijkstra.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// A* Search
// Question ID: 178
class ASearch {
public:
    std::vector<int> solve(std::vector<int> input) {
        std::sort(input.begin(), input.end());
        return input;
    }
};

} // namespace interview_prep
