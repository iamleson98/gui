// Question #324: Design Exactly-Once Delivery Semantics
// Category: System Design | Difficulty: Hard
// Concepts: exactly-once, idempotent producer, transactions, EOS
// Description: Achieve exactly-once delivery using idempotent producers and transactional consumption.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Design Exactly-Once Delivery Semantics
// Question ID: 324
class DesignExactlyOnceDeliverySemantics {
private:
    std::unordered_map<std::string, std::string> config_;
    std::unordered_map<std::string, int64_t> metrics_;
public:
    void set_config(const std::string& key, const std::string& val) { config_[key] = val; }
    std::string get_config(const std::string& key) const { auto it = config_.find(key); return it == config_.end() ? "" : it->second; }
    void increment_metric(const std::string& key) { metrics_[key]++; }
};

} // namespace interview_prep
