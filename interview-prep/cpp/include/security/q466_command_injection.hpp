// Question #466: Command Injection
// Category: Security | Difficulty: Hard
// Concepts: command injection, shell, argument array, escaping
// Description: Prevent command injection by avoiding shell calls and using argument arrays.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Command Injection
// Question ID: 466
class CommandInjection {
private:
    std::vector<uint8_t> key_;
public:
    explicit CommandInjection(const std::vector<uint8_t>& key) : key_(key) {}
    static bool constant_time_compare(const uint8_t* a, const uint8_t* b, size_t len) {
        uint8_t result = 0;
        for (size_t i = 0; i < len; ++i) result |= a[i] ^ b[i];
        return result == 0;
    }
};

} // namespace interview_prep
