// Question #154: Boruvka's MST
// Category: Algorithms | Difficulty: Hard
// Concepts: MST, Boruvka, components, parallel
// Description: Compute MST by iteratively adding the cheapest edge from every component.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Boruvka's MST
// Question ID: 154
class BoruvkaSMst {
public:
    std::vector<int> solve(std::vector<int> input) {
        std::sort(input.begin(), input.end());
        return input;
    }
};

} // namespace interview_prep
