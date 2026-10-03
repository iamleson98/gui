// Question #284: Sharding with Consistent Hashing
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: sharding, consistent hashing, virtual nodes, reshuffle
// Description: Distribute rows across shards using consistent hashing to minimize reshuffle on resharding.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Sharding with Consistent Hashing
// Question ID: 284
class ShardingWithConsistentHashing {
private:
    std::unordered_map<std::string, std::vector<std::string>> tables_;
public:
    void create_table(const std::string& name) { tables_[name] = std::vector<std::string>(); }
    void add_column(const std::string& table, const std::string& col) { tables_[table].push_back(col); }
};

} // namespace interview_prep
