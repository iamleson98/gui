// Question #492: Merkle Tree for Integrity
// Category: Security | Difficulty: Hard
// Concepts: Merkle tree, integrity, proof, tamper detection
// Description: Use Merkle trees to verify integrity of large data sets with compact proofs.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Merkle Tree for Integrity
// Question ID: 492
class MerkleTreeForIntegrity {
private:
    std::vector<uint8_t> key_;
public:
    explicit MerkleTreeForIntegrity(const std::vector<uint8_t>& key) : key_(key) {}
    static bool constant_time_compare(const uint8_t* a, const uint8_t* b, size_t len) {
        uint8_t result = 0;
        for (size_t i = 0; i < len; ++i) result |= a[i] ^ b[i];
        return result == 0;
    }
};

} // namespace interview_prep
