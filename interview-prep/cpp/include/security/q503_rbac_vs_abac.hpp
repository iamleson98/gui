// Question #503: RBAC vs ABAC
// Category: Security | Difficulty: Hard
// Concepts: RBAC, ABAC, authorization, policy
// Description: Choose between role-based and attribute-based access control for authorization.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// RBAC vs ABAC
// Question ID: 503
class RbacVsAbac {
private:
    std::vector<uint8_t> key_;
public:
    explicit RbacVsAbac(const std::vector<uint8_t>& key) : key_(key) {}
    static bool constant_time_compare(const uint8_t* a, const uint8_t* b, size_t len) {
        uint8_t result = 0;
        for (size_t i = 0; i < len; ++i) result |= a[i] ^ b[i];
        return result == 0;
    }
};

} // namespace interview_prep
