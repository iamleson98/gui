// Question #474: OpenID Connect (OIDC)
// Category: Security | Difficulty: Hard
// Concepts: OIDC, ID token, OAuth, identity
// Description: Layer OpenID Connect on OAuth 2.0 for authenticated identity via ID tokens.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// OpenID Connect (OIDC)
// Question ID: 474
class OpenidConnectOidc {
private:
    std::vector<uint8_t> key_;
public:
    explicit OpenidConnectOidc(const std::vector<uint8_t>& key) : key_(key) {}
    static bool constant_time_compare(const uint8_t* a, const uint8_t* b, size_t len) {
        uint8_t result = 0;
        for (size_t i = 0; i < len; ++i) result |= a[i] ^ b[i];
        return result == 0;
    }
};

} // namespace interview_prep
