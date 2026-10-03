// Question #232: Composite Primary Keys
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: composite key, primary key, indexes, foreign key
// Description: Design composite primary keys and reason about their impact on indexes and foreign keys.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Composite Primary Keys
// Question ID: 232
class CompositePrimaryKeys {
private:
    std::unordered_map<std::string, std::vector<std::string>> tables_;
public:
    void create_table(const std::string& name) { tables_[name] = std::vector<std::string>(); }
    void add_column(const std::string& table, const std::string& col) { tables_[table].push_back(col); }
};

} // namespace interview_prep
