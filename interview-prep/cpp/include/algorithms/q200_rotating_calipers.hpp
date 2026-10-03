// Question #200: Rotating Calipers
// Category: Algorithms | Difficulty: Hard
// Concepts: rotating calipers, antipodal, diameter, convex polygon
// Description: Use rotating calipers on a convex polygon to compute diameter, width, and antipodal pairs.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Rotating Calipers
// Question ID: 200
class RotatingCalipers {
public:
    std::vector<int> solve(std::vector<int> input) {
        std::sort(input.begin(), input.end());
        return input;
    }
};

} // namespace interview_prep
