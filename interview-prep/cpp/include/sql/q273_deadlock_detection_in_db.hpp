// Question #273: Deadlock Detection in DB
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: deadlock, wait-for graph, detection, victim
// Description: Use a wait-for graph to detect and resolve deadlocks among transactions.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Deadlock Detection in DB
// Question ID: 273
class DeadlockDetectionInDb {
private:
    std::unordered_map<std::string, std::vector<std::string>> tables_;
public:
    void create_table(const std::string& name) { tables_[name] = std::vector<std::string>(); }
    void add_column(const std::string& table, const std::string& col) { tables_[table].push_back(col); }
};

} // namespace interview_prep
