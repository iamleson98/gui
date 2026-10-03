// Question #420: Cache Hierarchy (L1/L2/L3)
// Category: Performance & Profiling | Difficulty: Hard
// Concepts: cache, L1/L2/L3, latency, bandwidth
// Description: Model the L1/L2/L3 cache hierarchy and quantify latency and bandwidth at each level.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Cache Hierarchy (L1/L2/L3)
// Question ID: 420
class CacheHierarchyL1L2L3 {
private:
    std::unordered_map<uint64_t, std::vector<uint8_t>> cache_;
    int64_t hits_ = 0, misses_ = 0;
public:
    bool get(uint64_t key, std::vector<uint8_t>& out) { auto it = cache_.find(key); if (it == cache_.end()) { misses_++; return false; } hits_++; out = it->second; return true; }
    void set(uint64_t key, const std::vector<uint8_t>& val) { cache_[key] = val; }
    int64_t hits() const { return hits_; } int64_t misses() const { return misses_; }
};

} // namespace interview_prep
