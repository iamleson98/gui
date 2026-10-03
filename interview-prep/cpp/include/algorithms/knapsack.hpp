// Question #165: 0/1 Knapsack
// Category: Algorithms | Difficulty: Hard | Concepts: knapsack DP, space optimization
#pragma once
#include <vector>
#include <cstdint>

namespace interview_prep {

struct Item { int weight; int64_t value; };

inline int64_t knapsack01(const std::vector<Item>& items, int capacity) {
    std::vector<int64_t> dp(capacity + 1, 0);
    for (auto& item : items) {
        for (int w = capacity; w >= item.weight; --w) {
            dp[w] = std::max(dp[w], dp[w - item.weight] + item.value);
        }
    }
    return dp[capacity];
}

} // namespace interview_prep
