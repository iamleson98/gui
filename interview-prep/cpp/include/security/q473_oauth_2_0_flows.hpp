// Question #473: OAuth 2.0 Flows
// Category: Security | Difficulty: Hard
// Concepts: OAuth, authorization code, PKCE, client credentials
// Description: Implement authorization-code, client-credentials, and PKCE flows appropriately.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// OAuth 2.0 Flows
// Question ID: 473
class Oauth20Flows {
private:
    std::vector<uint8_t> key_;
public:
    explicit Oauth20Flows(const std::vector<uint8_t>& key) : key_(key) {}
    static bool constant_time_compare(const uint8_t* a, const uint8_t* b, size_t len) {
        uint8_t result = 0;
        for (size_t i = 0; i < len; ++i) result |= a[i] ^ b[i];
        return result == 0;
    }
};

} // namespace interview_prep
