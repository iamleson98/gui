// Question #247: Closure Table for Hierarchies
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: closure table, hierarchy, ancestor, descendant
// Description: Store all ancestor-descendant pairs in a closure table for fast descendant and depth queries.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Closure Table for Hierarchies
// Question ID: 247
class ClosureTableForHierarchies {
private:
    std::unordered_map<std::string, std::vector<std::string>> tables_;
public:
    void create_table(const std::string& name) { tables_[name] = std::vector<std::string>(); }
    void add_column(const std::string& table, const std::string& col) { tables_[table].push_back(col); }
};

} // namespace interview_prep
