// Question #462: XSS Prevention (CSP)
// Category: Security | Difficulty: Hard
// Concepts: XSS, encoding, CSP, sanitization
// Description: Prevent cross-site scripting with output encoding and a Content Security Policy.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// XSS Prevention (CSP)
// Question ID: 462
class XssPreventionCsp {
private:
    std::vector<uint8_t> key_;
public:
    explicit XssPreventionCsp(const std::vector<uint8_t>& key) : key_(key) {}
    static bool constant_time_compare(const uint8_t* a, const uint8_t* b, size_t len) {
        uint8_t result = 0;
        for (size_t i = 0; i < len; ++i) result |= a[i] ^ b[i];
        return result == 0;
    }
};

} // namespace interview_prep
