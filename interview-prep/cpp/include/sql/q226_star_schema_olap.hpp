// Question #226: Star Schema (OLAP)
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: star schema, fact table, dimensions, OLAP
// Description: Design a star schema with a central fact table surrounded by dimension tables for OLAP queries.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Star Schema (OLAP)
// Question ID: 226
class StarSchemaOlap {
private:
    std::unordered_map<std::string, std::vector<std::string>> tables_;
public:
    void create_table(const std::string& name) { tables_[name] = std::vector<std::string>(); }
    void add_column(const std::string& table, const std::string& col) { tables_[table].push_back(col); }
};

} // namespace interview_prep
