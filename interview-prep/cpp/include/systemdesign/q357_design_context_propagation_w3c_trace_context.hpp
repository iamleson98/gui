// Question #357: Design Context Propagation (W3C Trace Context)
// Category: System Design | Difficulty: Hard
// Concepts: trace context, W3C, propagation, headers
// Description: Propagate trace context across process boundaries with W3C headers.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Design Context Propagation (W3C Trace Context)
// Question ID: 357
class DesignContextPropagationW3CTraceContext {
private:
    std::unordered_map<std::string, std::string> config_;
    std::unordered_map<std::string, int64_t> metrics_;
public:
    void set_config(const std::string& key, const std::string& val) { config_[key] = val; }
    std::string get_config(const std::string& key) const { auto it = config_.find(key); return it == config_.end() ? "" : it->second; }
    void increment_metric(const std::string& key) { metrics_[key]++; }
};

} // namespace interview_prep
