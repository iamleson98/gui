// Question #100: HyperLogLog
// Category: Data Structures | Difficulty: Hard
// Concepts: HyperLogLog, cardinality, stochastic averaging, bias correction
// Description: Implement HyperLogLog for approximate distinct-count (cardinality) estimation in fixed memory.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// HyperLogLog
// Question ID: 100
class Hyperloglog {
private:
    std::unordered_map<int,int> data_;
public:
    void insert(int key, int val) { data_[key] = val; }
    bool search(int key, int& out) const { auto it = data_.find(key); if (it == data_.end()) return false; out = it->second; return true; }
    bool remove(int key) { return data_.erase(key) > 0; }
    size_t size() const { return data_.size(); }
};

} // namespace interview_prep
