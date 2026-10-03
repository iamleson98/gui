// Question #527: HTTP/1.1 Keep-Alive and Pipelining
// Category: Networking & Protocols | Difficulty: Hard
// Concepts: HTTP/1.1, keep-alive, pipelining, HoL
// Description: Use keep-alive connections and reason about pipelining's HOL limitations.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// HTTP/1.1 Keep-Alive and Pipelining
// Question ID: 527
class Http11KeepAliveAndPipelining {
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
