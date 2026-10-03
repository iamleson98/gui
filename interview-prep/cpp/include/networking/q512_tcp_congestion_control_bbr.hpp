// Question #512: TCP Congestion Control (BBR)
// Category: Networking & Protocols | Difficulty: Hard
// Concepts: BBR, congestion, bandwidth, RTT
// Description: Explain BBR's model-based congestion control versus loss-based schemes.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// TCP Congestion Control (BBR)
// Question ID: 512
class TcpCongestionControlBbr {
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
