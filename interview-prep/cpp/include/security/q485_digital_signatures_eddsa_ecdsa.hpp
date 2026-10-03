// Question #485: Digital Signatures (EdDSA, ECDSA)
// Category: Security | Difficulty: Hard
// Concepts: EdDSA, ECDSA, nonce, canonical
// Description: Sign messages with EdDSA or ECDSA, noting canonical signatures and nonce risks.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Digital Signatures (EdDSA, ECDSA)
// Question ID: 485
class DigitalSignaturesEddsaEcdsa {
private:
    std::vector<uint8_t> key_;
public:
    explicit DigitalSignaturesEddsaEcdsa(const std::vector<uint8_t>& key) : key_(key) {}
    static bool constant_time_compare(const uint8_t* a, const uint8_t* b, size_t len) {
        uint8_t result = 0;
        for (size_t i = 0; i < len; ++i) result |= a[i] ^ b[i];
        return result == 0;
    }
};

} // namespace interview_prep
