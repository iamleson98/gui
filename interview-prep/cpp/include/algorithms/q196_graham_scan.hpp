// Question #196: Graham Scan
// Category: Algorithms | Difficulty: Hard
// Concepts: convex hull, Graham scan, angular sort, stack
// Description: Build the convex hull by angularly sorting points and using a stack with backtracking.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Graham Scan
// Question ID: 196
class GrahamScan {
public:
    std::vector<int> solve(std::vector<int> input) {
        std::sort(input.begin(), input.end());
        return input;
    }
};

} // namespace interview_prep
