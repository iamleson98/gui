// Question #274: Gap Locks and Next-Key Locking
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: gap lock, next-key, phantom, InnoDB
// Description: Prevent phantom reads with gap and next-key locking in InnoDB repeatable-read.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Gap Locks and Next-Key Locking
// Question ID: 274
class GapLocksAndNextKeyLocking {
private:
    std::unordered_map<std::string, std::vector<std::string>> tables_;
public:
    void create_table(const std::string& name) { tables_[name] = std::vector<std::string>(); }
    void add_column(const std::string& table, const std::string& col) { tables_[table].push_back(col); }
};

} // namespace interview_prep
