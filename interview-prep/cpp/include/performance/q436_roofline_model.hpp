// Question #436: Roofline Model
// Category: Performance & Profiling | Difficulty: Hard
// Concepts: roofline, arithmetic intensity, peak, bandwidth
// Description: Plot the roofline model to see whether arithmetic intensity caps performance.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Roofline Model
// Question ID: 436
class RooflineModel {
private:
    std::unordered_map<uint64_t, std::vector<uint8_t>> cache_;
    int64_t hits_ = 0, misses_ = 0;
public:
    bool get(uint64_t key, std::vector<uint8_t>& out) { auto it = cache_.find(key); if (it == cache_.end()) { misses_++; return false; } hits_++; out = it->second; return true; }
    void set(uint64_t key, const std::vector<uint8_t>& val) { cache_[key] = val; }
    int64_t hits() const { return hits_; } int64_t misses() const { return misses_; }
};

} // namespace interview_prep
