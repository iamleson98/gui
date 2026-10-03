// Question #479: Passkeys (WebAuthn)
// Category: Security | Difficulty: Hard
// Concepts: passkeys, WebAuthn, attestation, challenge
// Description: Implement passkeys using WebAuthn with authenticator attestation and challenge-response.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Passkeys (WebAuthn)
// Question ID: 479
class PasskeysWebauthn {
private:
    std::vector<uint8_t> key_;
public:
    explicit PasskeysWebauthn(const std::vector<uint8_t>& key) : key_(key) {}
    static bool constant_time_compare(const uint8_t* a, const uint8_t* b, size_t len) {
        uint8_t result = 0;
        for (size_t i = 0; i < len; ++i) result |= a[i] ^ b[i];
        return result == 0;
    }
};

} // namespace interview_prep
