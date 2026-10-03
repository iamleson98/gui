// Question #330: Design a Layer-4 Load Balancer
// Category: System Design | Difficulty: Hard
// Concepts: L4 LB, connection tracking, consistent hashing, NAT/DSR
// Description: Design an L4 load balancer using connection tracking and consistent hashing.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Design a Layer-4 Load Balancer
// Question ID: 330
class DesignALayer4LoadBalancer {
private:
    std::unordered_map<std::string, std::string> config_;
    std::unordered_map<std::string, int64_t> metrics_;
public:
    void set_config(const std::string& key, const std::string& val) { config_[key] = val; }
    std::string get_config(const std::string& key) const { auto it = config_.find(key); return it == config_.end() ? "" : it->second; }
    void increment_metric(const std::string& key) { metrics_[key]++; }
};

} // namespace interview_prep
