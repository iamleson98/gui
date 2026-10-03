// Question #250: Bitmap Index
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: bitmap index, low cardinality, OLAP, rowid
// Description: Apply bitmap indexes to low-cardinality columns and convert rowids in bulk for OLAP workloads.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Bitmap Index
// Question ID: 250
class BitmapIndex {
private:
    std::unordered_map<std::string, std::vector<std::string>> tables_;
public:
    void create_table(const std::string& name) { tables_[name] = std::vector<std::string>(); }
    void add_column(const std::string& table, const std::string& col) { tables_[table].push_back(col); }
};

} // namespace interview_prep
