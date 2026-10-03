// Question #471: Password Salting and Pepping
// Category: Security | Difficulty: Hard
// Concepts: salt, pepper, precomputed, rainbow
// Description: Use per-password salts and a server-side pepper to defeat precomputed attacks.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Password Salting and Pepping
// Question ID: 471
class PasswordSaltingAndPepping {
private:
    std::vector<uint8_t> key_;
public:
    explicit PasswordSaltingAndPepping(const std::vector<uint8_t>& key) : key_(key) {}
    static bool constant_time_compare(const uint8_t* a, const uint8_t* b, size_t len) {
        uint8_t result = 0;
        for (size_t i = 0; i < len; ++i) result |= a[i] ^ b[i];
        return result == 0;
    }
};

} // namespace interview_prep
