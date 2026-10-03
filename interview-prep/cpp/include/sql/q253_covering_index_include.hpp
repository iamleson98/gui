// Question #253: Covering Index (INCLUDE)
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: covering index, INCLUDE, index-only scan, visibility map
// Description: Add included non-key columns to make an index covering and enable index-only scans.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Covering Index (INCLUDE)
// Question ID: 253
class CoveringIndexInclude {
private:
    std::unordered_map<std::string, std::vector<std::string>> tables_;
public:
    void create_table(const std::string& name) { tables_[name] = std::vector<std::string>(); }
    void add_column(const std::string& table, const std::string& col) { tables_[table].push_back(col); }
};

} // namespace interview_prep
