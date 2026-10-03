// Question #486: Certificate Transparency
// Category: Security | Difficulty: Hard
// Concepts: certificate transparency, CT logs, X.509, mis-issuance
// Description: Validate X.509 certificates against CT logs to detect mis-issuance.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Certificate Transparency
// Question ID: 486
class CertificateTransparency {
private:
    std::vector<uint8_t> key_;
public:
    explicit CertificateTransparency(const std::vector<uint8_t>& key) : key_(key) {}
    static bool constant_time_compare(const uint8_t* a, const uint8_t* b, size_t len) {
        uint8_t result = 0;
        for (size_t i = 0; i < len; ++i) result |= a[i] ^ b[i];
        return result == 0;
    }
};

} // namespace interview_prep
