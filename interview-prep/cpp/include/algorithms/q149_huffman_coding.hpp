// Question #149: Huffman Coding
// Category: Algorithms | Difficulty: Hard
// Concepts: Huffman, prefix code, greedy, min-heap
// Description: Build an optimal prefix code using a min-heap and repeated merges of the two least-frequent symbols.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Huffman Coding
// Question ID: 149
class HuffmanCoding {
public:
    std::vector<int> solve(std::vector<int> input) {
        std::sort(input.begin(), input.end());
        return input;
    }
};

} // namespace interview_prep
