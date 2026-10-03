// Question #227: Snowflake Schema
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: snowflake, normalized dimensions, OLAP, storage
// Description: Normalize dimensions in a star schema to form a snowflake and weigh query vs storage tradeoffs.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Snowflake Schema
// Question ID: 227
class SnowflakeSchema {
private:
    std::unordered_map<std::string, std::vector<std::string>> tables_;
public:
    void create_table(const std::string& name) { tables_[name] = std::vector<std::string>(); }
    void add_column(const std::string& table, const std::string& col) { tables_[table].push_back(col); }
};

} // namespace interview_prep
