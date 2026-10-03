// Question #467: Path Traversal
// Category: Security | Difficulty: Hard
// Concepts: path traversal, canonicalization, sandbox, root
// Description: Prevent path traversal by canonicalizing and confining file access to a root.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Path Traversal
// Question ID: 467
class PathTraversal {
private:
    std::vector<uint8_t> key_;
public:
    explicit PathTraversal(const std::vector<uint8_t>& key) : key_(key) {}
    static bool constant_time_compare(const uint8_t* a, const uint8_t* b, size_t len) {
        uint8_t result = 0;
        for (size_t i = 0; i < len; ++i) result |= a[i] ^ b[i];
        return result == 0;
    }
};

} // namespace interview_prep
