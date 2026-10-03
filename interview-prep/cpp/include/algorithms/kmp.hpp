// Question #189: KMP String Matching
// Category: Algorithms | Difficulty: Hard | Concepts: KMP, prefix function, linear time
#pragma once
#include <vector>
#include <string>

namespace interview_prep {

inline std::vector<int> compute_failure(const std::string& pattern) {
    int n = pattern.size();
    if (n == 0) return {};
    std::vector<int> fail(n, 0);
    int j = 0;
    for (int i = 1; i < n; ) {
        if (pattern[i] == pattern[j]) { fail[i] = j + 1; j++; i++; }
        else if (j > 0) j = fail[j-1];
        else { fail[i] = 0; i++; }
    }
    return fail;
}

inline std::vector<int> kmp_search(const std::string& text, const std::string& pattern) {
    if (pattern.empty()) return {0};
    if (pattern.size() > text.size()) return {};
    std::vector<int> fail = compute_failure(pattern);
    std::vector<int> result;
    int j = 0;
    for (int i = 0; i < (int)text.size(); ) {
        if (text[i] == pattern[j]) {
            i++; j++;
            if (j == (int)pattern.size()) {
                result.push_back(i - j);
                j = fail[j-1];
            }
        } else if (j > 0) j = fail[j-1];
        else i++;
    }
    return result;
}

} // namespace interview_prep
