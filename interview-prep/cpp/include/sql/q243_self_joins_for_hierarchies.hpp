// Question #243: Self-Joins for Hierarchies
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: self-join, adjacency list, hierarchy, transitive
// Description: Use self-joins to traverse adjacency lists and compute transitive relationships.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Self-Joins for Hierarchies
// Question ID: 243
class SelfJoinsForHierarchies {
private:
    std::unordered_map<std::string, std::vector<std::string>> tables_;
public:
    void create_table(const std::string& name) { tables_[name] = std::vector<std::string>(); }
    void add_column(const std::string& table, const std::string& col) { tables_[table].push_back(col); }
};

} // namespace interview_prep
