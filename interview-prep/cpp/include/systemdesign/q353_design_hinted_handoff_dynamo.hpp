// Question #353: Design Hinted Handoff (Dynamo)
// Category: System Design | Difficulty: Hard
// Concepts: hinted handoff, Dynamo, replica, recovery
// Description: Store writes for temporarily unavailable replicas and hand them off on recovery.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Design Hinted Handoff (Dynamo)
// Question ID: 353
class DesignHintedHandoffDynamo {
private:
    std::unordered_map<std::string, std::string> config_;
    std::unordered_map<std::string, int64_t> metrics_;
public:
    void set_config(const std::string& key, const std::string& val) { config_[key] = val; }
    std::string get_config(const std::string& key) const { auto it = config_.find(key); return it == config_.end() ? "" : it->second; }
    void increment_metric(const std::string& key) { metrics_[key]++; }
};

} // namespace interview_prep
