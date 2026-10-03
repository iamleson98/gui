// Question #499: Padding Oracle Attacks
// Category: Security | Difficulty: Hard
// Concepts: padding oracle, CBC, AEAD, MAC
// Description: Prevent padding oracle attacks by using AEAD instead of CBC with MAC-then-encrypt.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Padding Oracle Attacks
// Question ID: 499
class PaddingOracleAttacks {
private:
    std::vector<uint8_t> key_;
public:
    explicit PaddingOracleAttacks(const std::vector<uint8_t>& key) : key_(key) {}
    static bool constant_time_compare(const uint8_t* a, const uint8_t* b, size_t len) {
        uint8_t result = 0;
        for (size_t i = 0; i < len; ++i) result |= a[i] ^ b[i];
        return result == 0;
    }
};

} // namespace interview_prep
