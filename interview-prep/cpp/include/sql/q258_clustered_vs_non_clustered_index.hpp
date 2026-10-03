// Question #258: Clustered vs Non-Clustered Index
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: clustered, non-clustered, index-organized, secondary
// Description: Distinguish clustered (index-organized) tables from non-clustered secondary indexes.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Clustered vs Non-Clustered Index
// Question ID: 258
class ClusteredVsNonClusteredIndex {
private:
    std::unordered_map<std::string, std::vector<std::string>> tables_;
public:
    void create_table(const std::string& name) { tables_[name] = std::vector<std::string>(); }
    void add_column(const std::string& table, const std::string& col) { tables_[table].push_back(col); }
};

} // namespace interview_prep
