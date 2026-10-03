// Question #275: Optimistic Concurrency Control
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: OCC, validation, read/write set, conflict
// Description: Validate transactions at commit time using read and write sets instead of locks.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Optimistic Concurrency Control
// Question ID: 275
class OptimisticConcurrencyControl {
private:
    std::unordered_map<std::string, std::vector<std::string>> tables_;
public:
    void create_table(const std::string& name) { tables_[name] = std::vector<std::string>(); }
    void add_column(const std::string& table, const std::string& col) { tables_[table].push_back(col); }
};

} // namespace interview_prep
