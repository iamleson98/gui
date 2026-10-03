// Question #482: Public-Key Encryption (RSA-OAEP)
// Category: Security | Difficulty: Hard
// Concepts: RSA-OAEP, padding, CCA, public key
// Description: Encrypt with RSA-OAEP padding to prevent chosen-ciphertext attacks.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Public-Key Encryption (RSA-OAEP)
// Question ID: 482
class PublicKeyEncryptionRsaOaep {
private:
    std::vector<uint8_t> key_;
public:
    explicit PublicKeyEncryptionRsaOaep(const std::vector<uint8_t>& key) : key_(key) {}
    static bool constant_time_compare(const uint8_t* a, const uint8_t* b, size_t len) {
        uint8_t result = 0;
        for (size_t i = 0; i < len; ++i) result |= a[i] ^ b[i];
        return result == 0;
    }
};

} // namespace interview_prep
