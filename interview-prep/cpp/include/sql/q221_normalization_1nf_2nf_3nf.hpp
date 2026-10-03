// Question #221: Normalization: 1NF/2NF/3NF
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: normalization, 1NF, 2NF, 3NF, anomalies
// Description: Apply first, second, and third normal forms to eliminate anomalies and redundancy in a schema.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Normalization: 1NF/2NF/3NF
// Question ID: 221
class Normalization1Nf2Nf3Nf {
private:
    std::unordered_map<std::string, std::vector<std::string>> tables_;
public:
    void create_table(const std::string& name) { tables_[name] = std::vector<std::string>(); }
    void add_column(const std::string& table, const std::string& col) { tables_[table].push_back(col); }
};

} // namespace interview_prep
