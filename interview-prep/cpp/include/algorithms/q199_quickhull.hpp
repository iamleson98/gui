// Question #199: QuickHull
// Category: Algorithms | Difficulty: Hard
// Concepts: convex hull, QuickHull, divide and conquer, farthest point
// Description: Implement the divide-and-conquer QuickHull algorithm for the convex hull.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// QuickHull
// Question ID: 199
class Quickhull {
public:
    std::vector<int> solve(std::vector<int> input) {
        std::sort(input.begin(), input.end());
        return input;
    }
};

} // namespace interview_prep
