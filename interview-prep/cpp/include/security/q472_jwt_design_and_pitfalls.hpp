// Question #472: JWT Design and Pitfalls
// Category: Security | Difficulty: Hard
// Concepts: JWT, alg, expiry, signature
// Description: Design JWTs securely, avoiding alg=none, weak keys, and missing expiry validation.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// JWT Design and Pitfalls
// Question ID: 472
class JwtDesignAndPitfalls {
private:
    std::vector<uint8_t> key_;
public:
    explicit JwtDesignAndPitfalls(const std::vector<uint8_t>& key) : key_(key) {}
    static bool constant_time_compare(const uint8_t* a, const uint8_t* b, size_t len) {
        uint8_t result = 0;
        for (size_t i = 0; i < len; ++i) result |= a[i] ^ b[i];
        return result == 0;
    }
};

} // namespace interview_prep
