// Question #509: Rate Limiting for Security
// Category: Security | Difficulty: Hard
// Concepts: rate limiting, brute force, lockout, account
// Description: Apply rate limits and account lockouts to slow credential-stuffing and brute force.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Rate Limiting for Security
// Question ID: 509
class RateLimitingForSecurity {
private:
    std::vector<uint8_t> key_;
public:
    explicit RateLimitingForSecurity(const std::vector<uint8_t>& key) : key_(key) {}
    static bool constant_time_compare(const uint8_t* a, const uint8_t* b, size_t len) {
        uint8_t result = 0;
        for (size_t i = 0; i < len; ++i) result |= a[i] ^ b[i];
        return result == 0;
    }
};

} // namespace interview_prep
