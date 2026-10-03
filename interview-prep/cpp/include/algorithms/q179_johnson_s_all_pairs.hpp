// Question #179: Johnson's All-Pairs
// Category: Algorithms | Difficulty: Hard
// Concepts: all-pairs, Johnson, reweighting, Dijkstra
// Description: Compute all-pairs shortest paths by reweighting with Bellman-Ford then running Dijkstra per vertex.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Johnson's All-Pairs
// Question ID: 179
class JohnsonSAllPairs {
public:
    std::vector<int> solve(std::vector<int> input) {
        std::sort(input.begin(), input.end());
        return input;
    }
};

} // namespace interview_prep
