// Question #495: PBKDF2 vs bcrypt vs argon2
// Category: Security | Difficulty: Hard
// Concepts: PBKDF2, bcrypt, argon2, memory-hard
// Description: Compare PBKDF2, bcrypt, and argon2 on CPU/memory hardness and suitability.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// PBKDF2 vs bcrypt vs argon2
// Question ID: 495
class Pbkdf2VsBcryptVsArgon2 {
private:
    std::vector<uint8_t> key_;
public:
    explicit Pbkdf2VsBcryptVsArgon2(const std::vector<uint8_t>& key) : key_(key) {}
    static bool constant_time_compare(const uint8_t* a, const uint8_t* b, size_t len) {
        uint8_t result = 0;
        for (size_t i = 0; i < len; ++i) result |= a[i] ^ b[i];
        return result == 0;
    }
};

} // namespace interview_prep
