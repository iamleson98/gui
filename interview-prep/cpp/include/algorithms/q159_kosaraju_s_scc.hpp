// Question #159: Kosaraju's SCC
// Category: Algorithms | Difficulty: Hard
// Concepts: SCC, Kosaraju, reverse graph, finish order
// Description: Compute SCCs by running DFS on the graph and then on the reverse graph in decreasing finish order.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Kosaraju's SCC
// Question ID: 159
class KosarajuSScc {
public:
    std::vector<int> solve(std::vector<int> input) {
        std::sort(input.begin(), input.end());
        return input;
    }
};

} // namespace interview_prep
