// Question #544: gRPC and Protocol Buffers
// Category: Networking & Protocols | Difficulty: Hard
// Concepts: gRPC, protobuf, HTTP/2, streaming
// Description: Design gRPC services with protobuf IDL, streaming, and HTTP/2 transport.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// gRPC and Protocol Buffers
// Question ID: 544
class GrpcAndProtocolBuffers {
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
