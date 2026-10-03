// Question #228: Factless Fact Tables
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: factless fact, coverage, many-to-many, event
// Description: Model many-to-many event coverage with factless fact tables capturing only keys.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Factless Fact Tables
// Question ID: 228
class FactlessFactTables {
private:
    std::unordered_map<std::string, std::vector<std::string>> tables_;
public:
    void create_table(const std::string& name) { tables_[name] = std::vector<std::string>(); }
    void add_column(const std::string& table, const std::string& col) { tables_[table].push_back(col); }
};

} // namespace interview_prep
