// Question #211: Number Theoretic Transform
// Category: Algorithms | Difficulty: Hard
// Concepts: NTT, modular, primitive root, polynomial
// Description: Implement the NTT over a prime modulus to perform exact polynomial multiplication without floating point.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Number Theoretic Transform
// Question ID: 211
class NumberTheoreticTransform {
public:
    std::vector<int> solve(std::vector<int> input) {
        std::sort(input.begin(), input.end());
        return input;
    }
};

} // namespace interview_prep
