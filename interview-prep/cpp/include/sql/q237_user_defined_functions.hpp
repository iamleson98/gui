// Question #237: User-Defined Functions
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: UDF, deterministic, inlining, planner
// Description: Build deterministic and volatile SQL functions while understanding inlining and planner effects.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// User-Defined Functions
// Question ID: 237
class UserDefinedFunctions {
private:
    std::unordered_map<std::string, std::vector<std::string>> tables_;
public:
    void create_table(const std::string& name) { tables_[name] = std::vector<std::string>(); }
    void add_column(const std::string& table, const std::string& col) { tables_[table].push_back(col); }
};

} // namespace interview_prep
