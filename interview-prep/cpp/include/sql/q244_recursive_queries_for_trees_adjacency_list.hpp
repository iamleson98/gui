// Question #244: Recursive Queries for Trees (Adjacency List)
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: adjacency list, recursive query, tree, termination
// Description: Traverse tree-structured adjacency data with recursive queries and termination guards.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Recursive Queries for Trees (Adjacency List)
// Question ID: 244
class RecursiveQueriesForTreesAdjacencyList {
private:
    std::unordered_map<std::string, std::vector<std::string>> tables_;
public:
    void create_table(const std::string& name) { tables_[name] = std::vector<std::string>(); }
    void add_column(const std::string& table, const std::string& col) { tables_[table].push_back(col); }
};

} // namespace interview_prep
