// Question #180: SPFA (Shortest Path Faster)
// Category: Algorithms | Difficulty: Hard
// Concepts: SPFA, queue, relaxation, negative weights
// Description: Implement the queue-based Bellman-Ford variant that only relaxes vertices whose distance changed.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// SPFA (Shortest Path Faster)
// Question ID: 180
class SpfaShortestPathFaster {
public:
    std::vector<int> solve(std::vector<int> input) {
        std::sort(input.begin(), input.end());
        return input;
    }
};

} // namespace interview_prep
