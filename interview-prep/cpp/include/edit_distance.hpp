#pragma once
#include <string>
#include <vector>
#include <algorithm>

namespace interview_prep {

// Edit Distance (Levenshtein) — O(n*m) time, O(min(n,m)) space.
inline size_t edit_distance(const std::string& s1, const std::string& s2) {
    const auto& a = s1.size() <= s2.size() ? s1 : s2;
    const auto& b = s1.size() <= s2.size() ? s2 : s1;
    size_t n = a.size(), m = b.size();
    std::vector<size_t> prev(n + 1), curr(n + 1);
    for (size_t i = 0; i <= n; ++i) prev[i] = i;
    for (size_t j = 1; j <= m; ++j) {
        curr[0] = j;
        for (size_t i = 1; i <= n; ++i) {
            size_t cost = (a[i-1] == b[j-1]) ? 0 : 1;
            curr[i] = std::min({prev[i] + 1, curr[i-1] + 1, prev[i-1] + cost});
        }
        std::swap(prev, curr);
    }
    return prev[n];
}

} // namespace interview_prep
