// Question #533: 0-RTT TLS
// Category: Networking & Protocols | Difficulty: Hard
// Concepts: 0-RTT, resumption, replay, TLS 1.3
// Description: Achieve 0-RTT resumption and reason about its replay risk for non-idempotent requests.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// 0-RTT TLS
// Question ID: 533
class 0RttTls {
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
