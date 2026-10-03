// Question #220: Mo's Algorithm (Offline Queries)
// Category: Algorithms | Difficulty: Hard
// Concepts: Mo's algorithm, offline, sqrt decomposition, reorder
// Description: Answer range queries by reordering them into sqrt-blocks for O((n+q) sqrt n) time.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Mo's Algorithm (Offline Queries)
// Question ID: 220
class MoSAlgorithmOfflineQueries {
public:
    std::vector<int> solve(std::vector<int> input) {
        std::sort(input.begin(), input.end());
        return input;
    }
};

} // namespace interview_prep
