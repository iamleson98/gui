// Question #236: Stored Procedures
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: stored procedures, encapsulation, security, maintainability
// Description: Design stored procedures for encapsulated logic and weigh security and maintainability tradeoffs.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Stored Procedures
// Question ID: 236
class StoredProcedures {
private:
    std::unordered_map<std::string, std::vector<std::string>> tables_;
public:
    void create_table(const std::string& name) { tables_[name] = std::vector<std::string>(); }
    void add_column(const std::string& table, const std::string& col) { tables_[table].push_back(col); }
};

} // namespace interview_prep
