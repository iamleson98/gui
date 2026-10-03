// Question #152: Kruskal's MST
// Category: Algorithms | Difficulty: Hard
// Concepts: MST, Kruskal, union-find, greedy
// Description: Build a minimum spanning forest using union-find to add edges in sorted order without forming cycles.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Kruskal's MST
// Question ID: 152
class KruskalSMst {
public:
    std::vector<int> solve(std::vector<int> input) {
        std::sort(input.begin(), input.end());
        return input;
    }
};

} // namespace interview_prep
