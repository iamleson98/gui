// Question #525: HTTP/2 HPACK Header Compression
// Category: Networking & Protocols | Difficulty: Hard
// Concepts: HPACK, header compression, Huffman, dynamic table
// Description: Compress HTTP/2 headers with HPACK static and dynamic tables plus Huffman coding.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// HTTP/2 HPACK Header Compression
// Question ID: 525
class Http2HpackHeaderCompression {
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
