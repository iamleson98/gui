// Question #242: Pivoting and Unpivoting
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: pivot, unpivot, conditional aggregation, cross tab
// Description: Pivot rows to columns and unpivot columns to rows using conditional aggregation and UNION.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Pivoting and Unpivoting
// Question ID: 242
class PivotingAndUnpivoting {
private:
    std::unordered_map<std::string, std::vector<std::string>> tables_;
public:
    void create_table(const std::string& name) { tables_[name] = std::vector<std::string>(); }
    void add_column(const std::string& table, const std::string& col) { tables_[table].push_back(col); }
};

} // namespace interview_prep
