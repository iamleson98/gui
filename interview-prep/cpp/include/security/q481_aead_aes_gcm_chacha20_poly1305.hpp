// Question #481: AEAD (AES-GCM, ChaCha20-Poly1305)
// Category: Security | Difficulty: Hard
// Concepts: AEAD, AES-GCM, ChaCha20-Poly1305, nonce
// Description: Use authenticated encryption with associated data to provide confidentiality and integrity.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// AEAD (AES-GCM, ChaCha20-Poly1305)
// Question ID: 481
class AeadAesGcmChacha20Poly1305 {
private:
    std::vector<uint8_t> key_;
public:
    explicit AeadAesGcmChacha20Poly1305(const std::vector<uint8_t>& key) : key_(key) {}
    static bool constant_time_compare(const uint8_t* a, const uint8_t* b, size_t len) {
        uint8_t result = 0;
        for (size_t i = 0; i < len; ++i) result |= a[i] ^ b[i];
        return result == 0;
    }
};

} // namespace interview_prep
