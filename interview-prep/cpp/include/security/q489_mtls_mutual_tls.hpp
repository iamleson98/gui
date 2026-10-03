// Question #489: mTLS (Mutual TLS)
// Category: Security | Difficulty: Hard
// Concepts: mTLS, client cert, verification, trust
// Description: Configure mutual TLS so both client and server present and verify certificates.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// mTLS (Mutual TLS)
// Question ID: 489
class MtlsMutualTls {
private:
    std::vector<uint8_t> key_;
public:
    explicit MtlsMutualTls(const std::vector<uint8_t>& key) : key_(key) {}
    static bool constant_time_compare(const uint8_t* a, const uint8_t* b, size_t len) {
        uint8_t result = 0;
        for (size_t i = 0; i < len; ++i) result |= a[i] ^ b[i];
        return result == 0;
    }
};

} // namespace interview_prep
