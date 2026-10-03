// Question #463: CSRF Tokens
// Category: Security | Difficulty: Hard
// Concepts: CSRF, tokens, SameSite, origin
// Description: Prevent cross-site request forgery with anti-CSRF tokens and SameSite cookies.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// CSRF Tokens
// Question ID: 463
class CsrfTokens {
private:
    std::vector<uint8_t> key_;
public:
    explicit CsrfTokens(const std::vector<uint8_t>& key) : key_(key) {}
    static bool constant_time_compare(const uint8_t* a, const uint8_t* b, size_t len) {
        uint8_t result = 0;
        for (size_t i = 0; i < len; ++i) result |= a[i] ^ b[i];
        return result == 0;
    }
};

} // namespace interview_prep
