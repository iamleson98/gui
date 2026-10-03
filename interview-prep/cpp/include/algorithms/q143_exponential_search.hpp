// Question #143: Exponential Search
// Category: Algorithms | Difficulty: Hard
// Concepts: exponential search, doubling, unbounded, sorted
// Description: Search sorted arrays by doubling the index then binary searching within the bounded range.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Exponential Search
// Question ID: 143
class ExponentialSearch {
public:
    std::vector<int> solve(std::vector<int> input) {
        std::sort(input.begin(), input.end());
        return input;
    }
};

} // namespace interview_prep
