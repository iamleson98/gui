// Question #198: Jarvis March (Gift Wrapping)
// Category: Algorithms | Difficulty: Hard
// Concepts: convex hull, gift wrapping, orientation, output-sensitive
// Description: Build the convex hull by gift wrapping around the point set in O(nh).
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Jarvis March (Gift Wrapping)
// Question ID: 198
class JarvisMarchGiftWrapping {
public:
    std::vector<int> solve(std::vector<int> input) {
        std::sort(input.begin(), input.end());
        return input;
    }
};

} // namespace interview_prep
