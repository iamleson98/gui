// Question #287: Design Pastebin
// Category: System Design | Difficulty: Hard
// Concepts: pastebin, blob storage, expiration, rate limit
// Description: Design a paste-sharing service handling large text blobs, expiration, and rate limits.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Design Pastebin
// Question ID: 287
class DesignPastebin {
private:
    std::unordered_map<std::string, std::string> config_;
    std::unordered_map<std::string, int64_t> metrics_;
public:
    void set_config(const std::string& key, const std::string& val) { config_[key] = val; }
    std::string get_config(const std::string& key) const { auto it = config_.find(key); return it == config_.end() ? "" : it->second; }
    void increment_metric(const std::string& key) { metrics_[key]++; }
};

} // namespace interview_prep
