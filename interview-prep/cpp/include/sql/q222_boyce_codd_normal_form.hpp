// Question #222: Boyce-Codd Normal Form
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: BCNF, functional dependency, candidate key, decomposition
// Description: Identify and decompose a schema to BCNF by removing non-trivial dependencies where a determinant is not a candidate key.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Boyce-Codd Normal Form
// Question ID: 222
class BoyceCoddNormalForm {
private:
    std::unordered_map<std::string, std::vector<std::string>> tables_;
public:
    void create_table(const std::string& name) { tables_[name] = std::vector<std::string>(); }
    void add_column(const std::string& table, const std::string& col) { tables_[table].push_back(col); }
};

} // namespace interview_prep
