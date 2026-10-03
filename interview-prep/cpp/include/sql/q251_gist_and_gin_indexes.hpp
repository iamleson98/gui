// Question #251: GiST and GIN Indexes
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: GiST, GIN, full-text, custom types
// Description: Choose GiST vs GIN for full-text and custom data types based on query and update patterns.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// GiST and GIN Indexes
// Question ID: 251
class GistAndGinIndexes {
private:
    std::unordered_map<std::string, std::vector<std::string>> tables_;
public:
    void create_table(const std::string& name) { tables_[name] = std::vector<std::string>(); }
    void add_column(const std::string& table, const std::string& col) { tables_[table].push_back(col); }
};

} // namespace interview_prep
