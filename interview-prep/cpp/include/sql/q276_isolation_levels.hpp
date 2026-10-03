// Question #276: Isolation Levels
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: isolation, read committed, repeatable read, serializable
// Description: Contrast read-uncommitted, read-committed, repeatable-read, and serializable isolation.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Isolation Levels
// Question ID: 276
class IsolationLevels {
private:
    std::unordered_map<std::string, std::vector<std::string>> tables_;
public:
    void create_table(const std::string& name) { tables_[name] = std::vector<std::string>(); }
    void add_column(const std::string& table, const std::string& col) { tables_[table].push_back(col); }
};

} // namespace interview_prep
