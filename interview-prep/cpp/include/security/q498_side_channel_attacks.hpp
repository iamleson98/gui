// Question #498: Side-Channel Attacks
// Category: Security | Difficulty: Hard
// Concepts: side channel, cache, power, EM
// Description: Defend against cache, power, and EM side channels in sensitive code.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Side-Channel Attacks
// Question ID: 498
class SideChannelAttacks {
private:
    std::vector<uint8_t> key_;
public:
    explicit SideChannelAttacks(const std::vector<uint8_t>& key) : key_(key) {}
    static bool constant_time_compare(const uint8_t* a, const uint8_t* b, size_t len) {
        uint8_t result = 0;
        for (size_t i = 0; i < len; ++i) result |= a[i] ^ b[i];
        return result == 0;
    }
};

} // namespace interview_prep
