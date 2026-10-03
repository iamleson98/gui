// Question #240: Common Table Expressions
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: CTE, readability, materialization, recursion
// Description: Refactor complex queries with CTEs for readability and understand materialization behavior.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Common Table Expressions
// Question ID: 240
class CommonTableExpressions {
private:
    std::unordered_map<std::string, std::vector<std::string>> tables_;
public:
    void create_table(const std::string& name) { tables_[name] = std::vector<std::string>(); }
    void add_column(const std::string& table, const std::string& col) { tables_[table].push_back(col); }
};

} // namespace interview_prep
