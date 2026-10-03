// Question #488: Perfect Forward Secrecy
// Category: Security | Difficulty: Hard
// Concepts: PFS, ephemeral, key exchange, compromise
// Description: Achieve perfect forward secrecy with ephemeral key exchange so past traffic resists future key compromise.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Perfect Forward Secrecy
// Question ID: 488
class PerfectForwardSecrecy {
private:
    std::vector<uint8_t> key_;
public:
    explicit PerfectForwardSecrecy(const std::vector<uint8_t>& key) : key_(key) {}
    static bool constant_time_compare(const uint8_t* a, const uint8_t* b, size_t len) {
        uint8_t result = 0;
        for (size_t i = 0; i < len; ++i) result |= a[i] ^ b[i];
        return result == 0;
    }
};

} // namespace interview_prep
