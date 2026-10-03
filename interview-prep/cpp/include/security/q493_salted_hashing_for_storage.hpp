// Question #493: Salted Hashing for Storage
// Category: Security | Difficulty: Hard
// Concepts: salted hash, slow hash, storage, cracking
// Description: Store credentials as salted slow hashes to resist offline cracking.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Salted Hashing for Storage
// Question ID: 493
class SaltedHashingForStorage {
private:
    std::vector<uint8_t> key_;
public:
    explicit SaltedHashingForStorage(const std::vector<uint8_t>& key) : key_(key) {}
    static bool constant_time_compare(const uint8_t* a, const uint8_t* b, size_t len) {
        uint8_t result = 0;
        for (size_t i = 0; i < len; ++i) result |= a[i] ^ b[i];
        return result == 0;
    }
};

} // namespace interview_prep
