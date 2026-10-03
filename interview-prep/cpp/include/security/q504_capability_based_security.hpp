// Question #504: Capability-Based Security
// Category: Security | Difficulty: Hard
// Concepts: capabilities, unforgeable, delegation, ACL
// Description: Design authorization around unforgeable capabilities rather than identity-based ACLs.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Capability-Based Security
// Question ID: 504
class CapabilityBasedSecurity {
private:
    std::vector<uint8_t> key_;
public:
    explicit CapabilityBasedSecurity(const std::vector<uint8_t>& key) : key_(key) {}
    static bool constant_time_compare(const uint8_t* a, const uint8_t* b, size_t len) {
        uint8_t result = 0;
        for (size_t i = 0; i < len; ++i) result |= a[i] ^ b[i];
        return result == 0;
    }
};

} // namespace interview_prep
