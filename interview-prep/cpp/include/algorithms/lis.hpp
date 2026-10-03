#pragma once
#include <vector>
#include <algorithm>
#include <cstdint>

namespace interview_prep {

// Longest Increasing Subsequence — O(n log n).
inline size_t lis_length(const std::vector<int32_t>& nums) {
    if (nums.empty()) return 0;
    std::vector<int32_t> tails;
    for (int32_t x : nums) {
        auto it = std::lower_bound(tails.begin(), tails.end(), x);
        if (it == tails.end()) tails.push_back(x);
        else *it = x;
    }
    return tails.size();
}

} // namespace interview_prep
