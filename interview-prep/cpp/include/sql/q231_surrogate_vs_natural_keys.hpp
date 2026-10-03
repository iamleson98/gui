// Question #231: Surrogate vs Natural Keys
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: surrogate key, natural key, stability, joins
// Description: Choose between surrogate and natural keys, weighing stability, size, and join performance.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Surrogate vs Natural Keys
// Question ID: 231
class SurrogateVsNaturalKeys {
private:
    std::unordered_map<std::string, std::vector<std::string>> tables_;
public:
    void create_table(const std::string& name) { tables_[name] = std::vector<std::string>(); }
    void add_column(const std::string& table, const std::string& col) { tables_[table].push_back(col); }
};

} // namespace interview_prep
