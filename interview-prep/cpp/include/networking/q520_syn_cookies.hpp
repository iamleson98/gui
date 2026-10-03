// Question #520: SYN Cookies
// Category: Networking & Protocols | Difficulty: Hard
// Concepts: SYN cookies, SYN flood, stateless, DoS
// Description: Defend against SYN floods with stateless SYN cookies encoded in the sequence number.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// SYN Cookies
// Question ID: 520
class SynCookies {
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
