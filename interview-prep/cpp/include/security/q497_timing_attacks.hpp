// Question #497: Timing Attacks
// Category: Security | Difficulty: Hard
// Concepts: timing attack, side channel, leak, secret
// Description: Reason about timing side channels that leak secrets through response time variation.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Timing Attacks
// Question ID: 497
class TimingAttacks {
private:
    std::vector<uint8_t> key_;
public:
    explicit TimingAttacks(const std::vector<uint8_t>& key) : key_(key) {}
    static bool constant_time_compare(const uint8_t* a, const uint8_t* b, size_t len) {
        uint8_t result = 0;
        for (size_t i = 0; i < len; ++i) result |= a[i] ^ b[i];
        return result == 0;
    }
};

} // namespace interview_prep
