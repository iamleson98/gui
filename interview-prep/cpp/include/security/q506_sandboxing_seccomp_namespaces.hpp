// Question #506: Sandboxing (seccomp, namespaces)
// Category: Security | Difficulty: Hard
// Concepts: sandbox, seccomp, namespaces, syscall filter
// Description: Sandbox untrusted code with seccomp filters and Linux namespaces.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Sandboxing (seccomp, namespaces)
// Question ID: 506
class SandboxingSeccompNamespaces {
private:
    std::vector<uint8_t> key_;
public:
    explicit SandboxingSeccompNamespaces(const std::vector<uint8_t>& key) : key_(key) {}
    static bool constant_time_compare(const uint8_t* a, const uint8_t* b, size_t len) {
        uint8_t result = 0;
        for (size_t i = 0; i < len; ++i) result |= a[i] ^ b[i];
        return result == 0;
    }
};

} // namespace interview_prep
