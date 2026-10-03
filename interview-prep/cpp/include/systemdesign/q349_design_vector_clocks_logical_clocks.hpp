// Question #349: Design Vector Clocks / Logical Clocks
// Category: System Design | Difficulty: Hard
// Concepts: vector clock, logical clock, concurrency, causality
// Description: Use vector clocks to detect concurrent updates in a distributed store.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Design Vector Clocks / Logical Clocks
// Question ID: 349
class DesignVectorClocksLogicalClocks {
private:
    std::unordered_map<std::string, std::string> config_;
    std::unordered_map<std::string, int64_t> metrics_;
public:
    void set_config(const std::string& key, const std::string& val) { config_[key] = val; }
    std::string get_config(const std::string& key) const { auto it = config_.find(key); return it == config_.end() ? "" : it->second; }
    void increment_metric(const std::string& key) { metrics_[key]++; }
};

} // namespace interview_prep
