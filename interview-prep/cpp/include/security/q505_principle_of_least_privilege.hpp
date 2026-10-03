// Question #505: Principle of Least Privilege
// Category: Security | Difficulty: Hard
// Concepts: least privilege, scoping, minimization, principle
// Description: Apply least privilege by granting the minimum scopes needed for a task.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Principle of Least Privilege
// Question ID: 505
class PrincipleOfLeastPrivilege {
private:
    std::vector<uint8_t> key_;
public:
    explicit PrincipleOfLeastPrivilege(const std::vector<uint8_t>& key) : key_(key) {}
    static bool constant_time_compare(const uint8_t* a, const uint8_t* b, size_t len) {
        uint8_t result = 0;
        for (size_t i = 0; i < len; ++i) result |= a[i] ^ b[i];
        return result == 0;
    }
};

} // namespace interview_prep
