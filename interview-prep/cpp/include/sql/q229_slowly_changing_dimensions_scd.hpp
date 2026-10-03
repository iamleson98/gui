// Question #229: Slowly Changing Dimensions (SCD)
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: SCD, Type 2, history, effective dates
// Description: Implement SCD Types 1-4 to track history of dimension attribute changes over time.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Slowly Changing Dimensions (SCD)
// Question ID: 229
class SlowlyChangingDimensionsScd {
private:
    std::unordered_map<std::string, std::vector<std::string>> tables_;
public:
    void create_table(const std::string& name) { tables_[name] = std::vector<std::string>(); }
    void add_column(const std::string& table, const std::string& col) { tables_[table].push_back(col); }
};

} // namespace interview_prep
