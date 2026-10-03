// Question #256: Index-Only Scans and Visibility Map
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: index-only scan, visibility map, vacuum, heap fetch
// Description: Explain how visibility maps enable index-only scans and the cost of vacuuming to maintain them.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Index-Only Scans and Visibility Map
// Question ID: 256
class IndexOnlyScansAndVisibilityMap {
private:
    std::unordered_map<std::string, std::vector<std::string>> tables_;
public:
    void create_table(const std::string& name) { tables_[name] = std::vector<std::string>(); }
    void add_column(const std::string& table, const std::string& col) { tables_[table].push_back(col); }
};

} // namespace interview_prep
