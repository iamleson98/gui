// Question #230: Bridge Tables for Many-to-Many
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: bridge table, many-to-many, junction, aggregate
// Description: Model many-to-many relationships with bridge tables and resolve aggregates correctly.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Bridge Tables for Many-to-Many
// Question ID: 230
class BridgeTablesForManyToMany {
private:
    std::unordered_map<std::string, std::vector<std::string>> tables_;
public:
    void create_table(const std::string& name) { tables_[name] = std::vector<std::string>(); }
    void add_column(const std::string& table, const std::string& col) { tables_[table].push_back(col); }
};

} // namespace interview_prep
