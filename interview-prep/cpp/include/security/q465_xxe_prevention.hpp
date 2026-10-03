// Question #465: XXE Prevention
// Category: Security | Difficulty: Hard
// Concepts: XXE, DTD, external entity, parser
// Description: Prevent XML external entity attacks by disabling DTDs and external entity resolution.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// XXE Prevention
// Question ID: 465
class XxePrevention {
private:
    std::vector<uint8_t> key_;
public:
    explicit XxePrevention(const std::vector<uint8_t>& key) : key_(key) {}
    static bool constant_time_compare(const uint8_t* a, const uint8_t* b, size_t len) {
        uint8_t result = 0;
        for (size_t i = 0; i < len; ++i) result |= a[i] ^ b[i];
        return result == 0;
    }
};

} // namespace interview_prep
