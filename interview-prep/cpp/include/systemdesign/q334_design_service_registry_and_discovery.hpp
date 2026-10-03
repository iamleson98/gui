// Question #334: Design Service Registry and Discovery
// Category: System Design | Difficulty: Hard
// Concepts: service discovery, registry, health checks, client-side
// Description: Design a service registry with health checks and client-side discovery.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Design Service Registry and Discovery
// Question ID: 334
class DesignServiceRegistryAndDiscovery {
private:
    std::unordered_map<std::string, std::string> config_;
    std::unordered_map<std::string, int64_t> metrics_;
public:
    void set_config(const std::string& key, const std::string& val) { config_[key] = val; }
    std::string get_config(const std::string& key) const { auto it = config_.find(key); return it == config_.end() ? "" : it->second; }
    void increment_metric(const std::string& key) { metrics_[key]++; }
};

} // namespace interview_prep
