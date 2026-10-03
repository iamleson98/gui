// Question #246: Materialized Path (ltree)
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: materialized path, ltree, prefix, GiST
// Description: Index tree paths with ltree or materialized path strings for prefix queries.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Materialized Path (ltree)
// Question ID: 246
class MaterializedPathLtree {
private:
    std::unordered_map<std::string, std::vector<std::string>> tables_;
public:
    void create_table(const std::string& name) { tables_[name] = std::vector<std::string>(); }
    void add_column(const std::string& table, const std::string& col) { tables_[table].push_back(col); }
};

} // namespace interview_prep
