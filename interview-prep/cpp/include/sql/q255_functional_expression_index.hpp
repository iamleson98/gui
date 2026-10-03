// Question #255: Functional/Expression Index
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: expression index, function, immutable, predicate
// Description: Index the result of an expression or function to accelerate transformed predicates.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Functional/Expression Index
// Question ID: 255
class FunctionalExpressionIndex {
private:
    std::unordered_map<std::string, std::vector<std::string>> tables_;
public:
    void create_table(const std::string& name) { tables_[name] = std::vector<std::string>(); }
    void add_column(const std::string& table, const std::string& col) { tables_[table].push_back(col); }
};

} // namespace interview_prep
