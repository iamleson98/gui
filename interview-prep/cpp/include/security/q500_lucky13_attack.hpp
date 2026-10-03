// Question #500: Lucky13 Attack
// Category: Security | Difficulty: Hard
// Concepts: Lucky13, TLS CBC, timing, AEAD
// Description: Understand the Lucky13 timing attack on TLS CBC and prefer AEAD ciphers.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Lucky13 Attack
// Question ID: 500
class Lucky13Attack {
private:
    std::vector<uint8_t> key_;
public:
    explicit Lucky13Attack(const std::vector<uint8_t>& key) : key_(key) {}
    static bool constant_time_compare(const uint8_t* a, const uint8_t* b, size_t len) {
        uint8_t result = 0;
        for (size_t i = 0; i < len; ++i) result |= a[i] ^ b[i];
        return result == 0;
    }
};

} // namespace interview_prep
