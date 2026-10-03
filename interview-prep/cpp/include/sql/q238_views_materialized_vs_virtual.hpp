// Question #238: Views: Materialized vs Virtual
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: views, materialized, refresh, abstraction
// Description: Compare materialized and virtual views for query abstraction and refresh strategies.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Views: Materialized vs Virtual
// Question ID: 238
class ViewsMaterializedVsVirtual {
private:
    std::unordered_map<std::string, std::vector<std::string>> tables_;
public:
    void create_table(const std::string& name) { tables_[name] = std::vector<std::string>(); }
    void add_column(const std::string& table, const std::string& col) { tables_[table].push_back(col); }
};

} // namespace interview_prep
