// Question #281: Change Data Capture (CDC)
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: CDC, WAL, streaming, transaction log
// Description: Stream row changes from a database by reading the WAL or transaction log.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Change Data Capture (CDC)
// Question ID: 281
class ChangeDataCaptureCdc {
private:
    std::unordered_map<std::string, std::vector<std::string>> tables_;
public:
    void create_table(const std::string& name) { tables_[name] = std::vector<std::string>(); }
    void add_column(const std::string& table, const std::string& col) { tables_[table].push_back(col); }
};

} // namespace interview_prep
