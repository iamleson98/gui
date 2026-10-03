// Question #241: Recursive CTEs
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: recursive CTE, hierarchy, traversal, anchor
// Description: Model hierarchies and graph traversals with recursive CTEs using an anchor and a recursive member.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Recursive CTEs
// Question ID: 241
class RecursiveCtes {
private:
    std::unordered_map<std::string, std::vector<std::string>> tables_;
public:
    void create_table(const std::string& name) { tables_[name] = std::vector<std::string>(); }
    void add_column(const std::string& table, const std::string& col) { tables_[table].push_back(col); }
};

} // namespace interview_prep
