// Question #469: OWASP Top 10
// Category: Security | Difficulty: Hard
// Concepts: OWASP, Top 10, risk, mitigation
// Description: Map a system's defenses to the OWASP Top 10 risk categories.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// OWASP Top 10
// Question ID: 469
class OwaspTop10 {
private:
    std::vector<uint8_t> key_;
public:
    explicit OwaspTop10(const std::vector<uint8_t>& key) : key_(key) {}
    static bool constant_time_compare(const uint8_t* a, const uint8_t* b, size_t len) {
        uint8_t result = 0;
        for (size_t i = 0; i < len; ++i) result |= a[i] ^ b[i];
        return result == 0;
    }
};

} // namespace interview_prep
