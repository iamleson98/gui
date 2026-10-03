// Question #548: ICMP and Ping/Traceroute
// Category: Networking & Protocols | Difficulty: Hard
// Concepts: ICMP, ping, traceroute, TTL
// Description: Use ICMP echo and time-exceeded messages to implement ping and traceroute.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// ICMP and Ping/Traceroute
// Question ID: 548
class IcmpAndPingTraceroute {
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
