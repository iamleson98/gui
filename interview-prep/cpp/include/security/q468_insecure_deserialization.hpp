// Question #468: Insecure Deserialization
// Category: Security | Difficulty: Hard
// Concepts: deserialization, type allowlist, gadget, RCE
// Description: Prevent insecure deserialization by avoiding native formats and enforcing type allowlists.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Insecure Deserialization
// Question ID: 468
class InsecureDeserialization {
private:
    std::vector<uint8_t> key_;
public:
    explicit InsecureDeserialization(const std::vector<uint8_t>& key) : key_(key) {}
    static bool constant_time_compare(const uint8_t* a, const uint8_t* b, size_t len) {
        uint8_t result = 0;
        for (size_t i = 0; i < len; ++i) result |= a[i] ^ b[i];
        return result == 0;
    }
};

} // namespace interview_prep
