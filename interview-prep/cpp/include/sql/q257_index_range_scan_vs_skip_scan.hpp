// Question #257: Index Range Scan vs Skip Scan
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: range scan, skip scan, composite index, leading column
// Description: Contrast range scans with skip scans that handle leading-column equality filters.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Index Range Scan vs Skip Scan
// Question ID: 257
class IndexRangeScanVsSkipScan {
private:
    std::unordered_map<std::string, std::vector<std::string>> tables_;
public:
    void create_table(const std::string& name) { tables_[name] = std::vector<std::string>(); }
    void add_column(const std::string& table, const std::string& col) { tables_[table].push_back(col); }
};

} // namespace interview_prep
