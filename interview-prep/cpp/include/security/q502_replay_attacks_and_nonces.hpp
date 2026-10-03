// Question #502: Replay Attacks and Nonces
// Category: Security | Difficulty: Hard
// Concepts: replay, nonce, timestamp, sequence
// Description: Defend against replay using nonces, timestamps, and sequence numbers.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Replay Attacks and Nonces
// Question ID: 502
class ReplayAttacksAndNonces {
private:
    std::vector<uint8_t> key_;
public:
    explicit ReplayAttacksAndNonces(const std::vector<uint8_t>& key) : key_(key) {}
    static bool constant_time_compare(const uint8_t* a, const uint8_t* b, size_t len) {
        uint8_t result = 0;
        for (size_t i = 0; i < len; ++i) result |= a[i] ^ b[i];
        return result == 0;
    }
};

} // namespace interview_prep
