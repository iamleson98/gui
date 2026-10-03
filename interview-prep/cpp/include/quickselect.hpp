#pragma once
#include <vector>
#include <cstdint>
#include <random>

namespace interview_prep {

// Quickselect — find k-th smallest in expected O(n).
inline int32_t quick_select(std::vector<int32_t>& nums, size_t k) {
    if (k >= nums.size()) {
        // Return min element if k is out of range
        k = nums.size() - 1;
    }
    static std::mt19937 rng(42);
    auto lo = nums.begin();
    auto hi = nums.end() - 1;
    while (lo < hi) {
        auto pivot = lo + rng() % std::distance(lo, hi + 1);
        std::iter_swap(pivot, hi);
        auto store = lo;
        for (auto it = lo; it < hi; ++it) {
            if (*it < *hi) {
                std::iter_swap(store, it);
                ++store;
            }
        }
        std::iter_swap(store, hi);
        size_t dist = std::distance(nums.begin(), store);
        if (dist == k) return *store;
        if (dist < k) lo = store + 1;
        else hi = store - 1;
    }
    return *lo;
}

} // namespace interview_prep
