// Question #168: Manacher's Algorithm — Longest Palindromic Substring
// Category: Algorithms | Difficulty: Hard | Concepts: Manacher, symmetry, O(n)
#pragma once
#include <string>
#include <vector>

namespace interview_prep {

inline std::string longest_palindrome(const std::string& s) {
    if (s.empty()) return "";
    // Transform: ^#c1#c2#...#cn#$
    std::string t = "^";
    for (char c : s) { t += '#'; t += c; }
    t += "#$";
    int n = t.size();
    std::vector<int> p(n, 0);
    int c = 0, r = 0;
    int max_len = 0, center = 0;
    for (int i = 1; i < n - 1; ++i) {
        int mirror = 2 * c - i;
        if (r > i) p[i] = std::min(r - i, p[mirror]);
        while (i + p[i] + 1 < n && i - p[i] - 1 >= 0
            && t[i + p[i] + 1] == t[i - p[i] - 1])
            p[i]++;
        if (i + p[i] > r) { c = i; r = i + p[i]; }
        if (p[i] > max_len) { max_len = p[i]; center = i; }
    }
    int start = (center - max_len) / 2;
    return s.substr(start, max_len);
}

} // namespace interview_prep
