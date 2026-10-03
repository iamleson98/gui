// Question #254: Partial Index (WHERE Clause)
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: partial index, WHERE, subset, size
// Description: Create partial indexes on a filtered subset to reduce size and speed common queries.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Partial Index (WHERE Clause)
// Question ID: 254
class PartialIndexWhereClause {
private:
    std::unordered_map<std::string, std::vector<std::string>> tables_;
public:
    void create_table(const std::string& name) { tables_[name] = std::vector<std::string>(); }
    void add_column(const std::string& table, const std::string& col) { tables_[table].push_back(col); }
};

} // namespace interview_prep
