// Question #511: TCP Three-Way Handshake
// Category: Networking & Protocols | Difficulty: Hard
// Concepts: TCP, handshake, SYN, state machine
// Description: Explain SYN, SYN-ACK, ACK and the resulting connection state machine.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// TCP Three-Way Handshake
// Question ID: 511
class TcpThreeWayHandshake {
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
