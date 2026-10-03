// Question #464: SSRF Prevention
// Category: Security | Difficulty: Hard
// Concepts: SSRF, allowlist, egress, metadata
// Description: Prevent server-side request forgery by validating and restricting outbound destinations.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// SSRF Prevention
// Question ID: 464
class SsrfPrevention {
private:
    std::vector<uint8_t> key_;
public:
    explicit SsrfPrevention(const std::vector<uint8_t>& key) : key_(key) {}
    static bool constant_time_compare(const uint8_t* a, const uint8_t* b, size_t len) {
        uint8_t result = 0;
        for (size_t i = 0; i < len; ++i) result |= a[i] ^ b[i];
        return result == 0;
    }
};

} // namespace interview_prep
