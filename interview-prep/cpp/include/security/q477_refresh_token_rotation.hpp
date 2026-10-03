// Question #477: Refresh Token Rotation
// Category: Security | Difficulty: Hard
// Concepts: refresh token, rotation, reuse detection, theft
// Description: Rotate refresh tokens on use with reuse detection to limit token theft impact.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Refresh Token Rotation
// Question ID: 477
class RefreshTokenRotation {
private:
    std::vector<uint8_t> key_;
public:
    explicit RefreshTokenRotation(const std::vector<uint8_t>& key) : key_(key) {}
    static bool constant_time_compare(const uint8_t* a, const uint8_t* b, size_t len) {
        uint8_t result = 0;
        for (size_t i = 0; i < len; ++i) result |= a[i] ^ b[i];
        return result == 0;
    }
};

} // namespace interview_prep
