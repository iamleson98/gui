// Question #286: Design a URL Shortener
// Category: System Design | Difficulty: Hard
// Concepts: URL shortener, base62, sharding, cache
// Description: Design a service that maps long URLs to short codes with high read throughput and analytics.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Design a URL Shortener
// Question ID: 286
class DesignAUrlShortener {
private:
    std::unordered_map<std::string, std::string> config_;
    std::unordered_map<std::string, int64_t> metrics_;
public:
    void set_config(const std::string& key, const std::string& val) { config_[key] = val; }
    std::string get_config(const std::string& key) const { auto it = config_.find(key); return it == config_.end() ? "" : it->second; }
    void increment_metric(const std::string& key) { metrics_[key]++; }
};

} // namespace interview_prep
