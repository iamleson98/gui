// Question #314: Design a Leaderboard (Sorted Sets)
// Category: System Design | Difficulty: Hard
// Concepts: leaderboard, sorted sets, sharding, rollup
// Description: Design a global leaderboard using Redis sorted sets with sharded rollups.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Design a Leaderboard (Sorted Sets)
// Question ID: 314
class DesignALeaderboardSortedSets {
private:
    std::unordered_map<std::string, std::string> config_;
    std::unordered_map<std::string, int64_t> metrics_;
public:
    void set_config(const std::string& key, const std::string& val) { config_[key] = val; }
    std::string get_config(const std::string& key) const { auto it = config_.find(key); return it == config_.end() ? "" : it->second; }
    void increment_metric(const std::string& key) { metrics_[key]++; }
};

} // namespace interview_prep
