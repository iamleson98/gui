// Question #515: Fast Retransmit and Fast Recovery
// Category: Networking & Protocols | Difficulty: Hard
// Concepts: fast retransmit, fast recovery, dup ACK, loss
// Description: Recover from packet loss without timing out using duplicate ACKs.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Fast Retransmit and Fast Recovery
// Question ID: 515
class FastRetransmitAndFastRecovery {
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
