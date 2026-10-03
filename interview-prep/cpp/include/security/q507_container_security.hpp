// Question #507: Container Security
// Category: Security | Difficulty: Hard
// Concepts: container, capabilities, read-only, image
// Description: Harden containers with reduced capabilities, read-only roots, and minimal images.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Container Security
// Question ID: 507
class ContainerSecurity {
private:
    std::vector<uint8_t> key_;
public:
    explicit ContainerSecurity(const std::vector<uint8_t>& key) : key_(key) {}
    static bool constant_time_compare(const uint8_t* a, const uint8_t* b, size_t len) {
        uint8_t result = 0;
        for (size_t i = 0; i < len; ++i) result |= a[i] ^ b[i];
        return result == 0;
    }
};

} // namespace interview_prep
