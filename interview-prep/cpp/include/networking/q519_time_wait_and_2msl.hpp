// Question #519: TIME_WAIT and 2MSL
// Category: Networking & Protocols | Difficulty: Hard
// Concepts: TIME_WAIT, 2MSL, segments, reuse
// Description: Explain TIME_WAIT, the 2MSL duration, and its role in preventing old segments.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// TIME_WAIT and 2MSL
// Question ID: 519
class TimeWaitAnd2Msl {
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
