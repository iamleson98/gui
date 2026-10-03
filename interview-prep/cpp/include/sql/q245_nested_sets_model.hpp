// Question #245: Nested Sets Model
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: nested sets, left/right, subtree, preorder
// Description: Store trees using nested sets with left/right preorder bounds for subtree queries.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Nested Sets Model
// Question ID: 245
class NestedSetsModel {
private:
    std::unordered_map<std::string, std::vector<std::string>> tables_;
public:
    void create_table(const std::string& name) { tables_[name] = std::vector<std::string>(); }
    void add_column(const std::string& table, const std::string& col) { tables_[table].push_back(col); }
};

} // namespace interview_prep
