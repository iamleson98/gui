// Question #272: Two-Phase Locking (2PL)
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: 2PL, growing, shrinking, strict
// Description: Implement strict two-phase locking growing then shrinking lock phases to guarantee serializability.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Two-Phase Locking (2PL)
// Question ID: 272
class TwoPhaseLocking2Pl {
private:
    std::unordered_map<std::string, std::vector<std::string>> tables_;
public:
    void create_table(const std::string& name) { tables_[name] = std::vector<std::string>(); }
    void add_column(const std::string& table, const std::string& col) { tables_[table].push_back(col); }
};

} // namespace interview_prep
