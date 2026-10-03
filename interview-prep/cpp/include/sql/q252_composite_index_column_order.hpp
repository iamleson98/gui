// Question #252: Composite Index Column Order
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: composite index, column order, selectivity, range
// Description: Design composite indexes with column order matching equality, sort, and range predicates.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Composite Index Column Order
// Question ID: 252
class CompositeIndexColumnOrder {
private:
    std::unordered_map<std::string, std::vector<std::string>> tables_;
public:
    void create_table(const std::string& name) { tables_[name] = std::vector<std::string>(); }
    void add_column(const std::string& table, const std::string& col) { tables_[table].push_back(col); }
};

} // namespace interview_prep
