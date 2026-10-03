// Question #345: Design Disaster Recovery (RPO/RTO)
// Category: System Design | Difficulty: Hard
// Concepts: DR, RPO, RTO, failover
// Description: Quantify RPO/RTO and design backup, replication, and failover to meet them.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Design Disaster Recovery (RPO/RTO)
// Question ID: 345
class DesignDisasterRecoveryRpoRto {
private:
    std::unordered_map<std::string, std::string> config_;
    std::unordered_map<std::string, int64_t> metrics_;
public:
    void set_config(const std::string& key, const std::string& val) { config_[key] = val; }
    std::string get_config(const std::string& key) const { auto it = config_.find(key); return it == config_.end() ? "" : it->second; }
    void increment_metric(const std::string& key) { metrics_[key]++; }
};

} // namespace interview_prep
