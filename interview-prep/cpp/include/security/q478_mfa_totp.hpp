// Question #478: MFA / TOTP
// Category: Security | Difficulty: Hard
// Concepts: MFA, TOTP, RFC 6238, HMAC
// Description: Implement time-based one-time passwords (RFC 6238) for multi-factor authentication.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// MFA / TOTP
// Question ID: 478
class MfaTotp {
private:
    std::vector<uint8_t> key_;
public:
    explicit MfaTotp(const std::vector<uint8_t>& key) : key_(key) {}
    static bool constant_time_compare(const uint8_t* a, const uint8_t* b, size_t len) {
        uint8_t result = 0;
        for (size_t i = 0; i < len; ++i) result |= a[i] ^ b[i];
        return result == 0;
    }
};

} // namespace interview_prep
