// Question #224: Denormalization Tradeoffs
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: denormalization, read performance, anomalies, tradeoff
// Description: Evaluate when denormalizing for read performance outweighs the cost of update anomalies.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Denormalization Tradeoffs
// Question ID: 224
class DenormalizationTradeoffs {
private:
    std::unordered_map<std::string, std::vector<std::string>> tables_;
public:
    void create_table(const std::string& name) { tables_[name] = std::vector<std::string>(); }
    void add_column(const std::string& table, const std::string& col) { tables_[table].push_back(col); }
};

} // namespace interview_prep
