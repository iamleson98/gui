// Question #312: Design Idempotent Payment Processing
// Category: System Design | Difficulty: Hard
// Concepts: idempotency, payment, dedup, ledger
// Description: Ensure payment APIs are idempotent using idempotency keys and a dedup store.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Design Idempotent Payment Processing
// Question ID: 312
class DesignIdempotentPaymentProcessing {
private:
    std::unordered_map<std::string, std::string> config_;
    std::unordered_map<std::string, int64_t> metrics_;
public:
    void set_config(const std::string& key, const std::string& val) { config_[key] = val; }
    std::string get_config(const std::string& key) const { auto it = config_.find(key); return it == config_.end() ? "" : it->second; }
    void increment_metric(const std::string& key) { metrics_[key]++; }
};

} // namespace interview_prep
