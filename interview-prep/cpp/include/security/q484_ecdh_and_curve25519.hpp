// Question #484: ECDH and Curve25519
// Category: Security | Difficulty: Hard
// Concepts: ECDH, Curve25519, key agreement, side channel
// Description: Use ECDH on Curve25519 for fast, secure key agreement resistant to side channels.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// ECDH and Curve25519
// Question ID: 484
class EcdhAndCurve25519 {
private:
    std::vector<uint8_t> key_;
public:
    explicit EcdhAndCurve25519(const std::vector<uint8_t>& key) : key_(key) {}
    static bool constant_time_compare(const uint8_t* a, const uint8_t* b, size_t len) {
        uint8_t result = 0;
        for (size_t i = 0; i < len; ++i) result |= a[i] ^ b[i];
        return result == 0;
    }
};

} // namespace interview_prep
