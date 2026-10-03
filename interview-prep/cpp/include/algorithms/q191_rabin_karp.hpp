// Question #191: Rabin-Karp
// Category: Algorithms | Difficulty: Hard
// Concepts: rolling hash, Rabin-Karp, collision, multi-pattern
// Description: Implement the Rabin-Karp rolling-hash matcher for single and multi-pattern search.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Rabin-Karp
// Question ID: 191
class RabinKarp {
public:
    std::vector<int> solve(std::vector<int> input) {
        std::sort(input.begin(), input.end());
        return input;
    }
};

} // namespace interview_prep
