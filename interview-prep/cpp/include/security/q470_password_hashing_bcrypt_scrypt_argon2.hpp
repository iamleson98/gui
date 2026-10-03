// Question #470: Password Hashing: bcrypt/scrypt/argon2
// Category: Security | Difficulty: Hard
// Concepts: password hashing, bcrypt, scrypt, argon2
// Description: Choose and configure bcrypt, scrypt, and argon2 to slow brute force attacks.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Password Hashing: bcrypt/scrypt/argon2
// Question ID: 470
class PasswordHashingBcryptScryptArgon2 {
private:
    std::vector<uint8_t> key_;
public:
    explicit PasswordHashingBcryptScryptArgon2(const std::vector<uint8_t>& key) : key_(key) {}
    static bool constant_time_compare(const uint8_t* a, const uint8_t* b, size_t len) {
        uint8_t result = 0;
        for (size_t i = 0; i < len; ++i) result |= a[i] ^ b[i];
        return result == 0;
    }
};

} // namespace interview_prep
