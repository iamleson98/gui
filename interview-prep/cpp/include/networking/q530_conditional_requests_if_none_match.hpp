// Question #530: Conditional Requests (If-None-Match)
// Category: Networking & Protocols | Difficulty: Hard
// Concepts: conditional request, If-None-Match, If-Modified-Since, 304
// Description: Use If-None-Match and If-Modified-Since to validate cached responses.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Conditional Requests (If-None-Match)
// Question ID: 530
class ConditionalRequestsIfNoneMatch {
private:
    std::unordered_map<std::string, int> connections_;
    int timeout_ms_ = 5000;
public:
    void set_timeout(int ms) { timeout_ms_ = ms; }
    void add_connection(const std::string& id, int fd) { connections_[id] = fd; }
    void remove_connection(const std::string& id) { connections_.erase(id); }
    int get_connection(const std::string& id) const { auto it = connections_.find(id); return it == connections_.end() ? -1 : it->second; }
};

} // namespace interview_prep
