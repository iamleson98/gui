// Question #491: Hashing vs Encryption vs Encoding
// Category: Security | Difficulty: Hard
// Concepts: hashing, encryption, encoding, purpose
// Description: Distinguish hashing, encryption, and encoding and pick the right tool for each task.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Hashing vs Encryption vs Encoding
// Question ID: 491
class HashingVsEncryptionVsEncoding {
private:
    std::vector<uint8_t> key_;
public:
    explicit HashingVsEncryptionVsEncoding(const std::vector<uint8_t>& key) : key_(key) {}
    static bool constant_time_compare(const uint8_t* a, const uint8_t* b, size_t len) {
        uint8_t result = 0;
        for (size_t i = 0; i < len; ++i) result |= a[i] ^ b[i];
        return result == 0;
    }
};

} // namespace interview_prep
