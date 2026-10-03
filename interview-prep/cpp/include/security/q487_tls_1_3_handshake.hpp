// Question #487: TLS 1.3 Handshake
// Category: Security | Difficulty: Hard
// Concepts: TLS 1.3, 1-RTT, key share, HKDF
// Description: Walk through the TLS 1.3 1-RTT handshake with key share and HKDF.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// TLS 1.3 Handshake
// Question ID: 487
class Tls13Handshake {
private:
    std::vector<uint8_t> key_;
public:
    explicit Tls13Handshake(const std::vector<uint8_t>& key) : key_(key) {}
    static bool constant_time_compare(const uint8_t* a, const uint8_t* b, size_t len) {
        uint8_t result = 0;
        for (size_t i = 0; i < len; ++i) result |= a[i] ^ b[i];
        return result == 0;
    }
};

} // namespace interview_prep
