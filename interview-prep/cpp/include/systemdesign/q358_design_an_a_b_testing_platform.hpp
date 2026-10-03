// Question #358: Design an A/B Testing Platform
// Category: System Design | Difficulty: Hard
// Concepts: A/B testing, bucketing, metrics, significance
// Description: Design an experimentation platform with bucketing, metrics, and statistical guardrails.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Design an A/B Testing Platform
// Question ID: 358
class DesignAnABTestingPlatform {
private:
    std::unordered_map<std::string, std::string> config_;
    std::unordered_map<std::string, int64_t> metrics_;
public:
    void set_config(const std::string& key, const std::string& val) { config_[key] = val; }
    std::string get_config(const std::string& key) const { auto it = config_.find(key); return it == config_.end() ? "" : it->second; }
    void increment_metric(const std::string& key) { metrics_[key]++; }
};

} // namespace interview_prep
