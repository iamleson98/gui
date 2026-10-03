// Question #282: CQRS
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: CQRS, command, query, separation
// Description: Separate command (write) and query (read) models to optimize each independently.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// CQRS
// Question ID: 282
class Cqrs {
private:
    std::unordered_map<std::string, std::vector<std::string>> tables_;
public:
    void create_table(const std::string& name) { tables_[name] = std::vector<std::string>(); }
    void add_column(const std::string& table, const std::string& col) { tables_[table].push_back(col); }
};

} // namespace interview_prep
