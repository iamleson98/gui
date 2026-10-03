// Question #541: WebRTC and ICE/STUN/TURN
// Category: Networking & Protocols | Difficulty: Hard
// Concepts: WebRTC, ICE, STUN, TURN
// Description: Establish peer-to-peer media with ICE candidate gathering and TURN fallback.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// WebRTC and ICE/STUN/TURN
// Question ID: 541
class WebrtcAndIceStunTurn {
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
