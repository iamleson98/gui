// Question #496: Constant-Time Comparison
// Category: Security | Difficulty: Hard
// Concepts: constant time, timing leak, comparison, secret
// Description: Implement constant-time comparison to avoid leaking equality via timing.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Constant-Time Comparison
// Question ID: 496
class ConstantTimeComparison {
private:
    std::vector<uint8_t> key_;
public:
    explicit ConstantTimeComparison(const std::vector<uint8_t>& key) : key_(key) {}
    static bool constant_time_compare(const uint8_t* a, const uint8_t* b, size_t len) {
        uint8_t result = 0;
        for (size_t i = 0; i < len; ++i) result |= a[i] ^ b[i];
        return result == 0;
    }
};

} // namespace interview_prep
