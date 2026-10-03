// Question #103: ARC (Adaptive Replacement Cache)
// Category: Data Structures | Difficulty: Hard
// Concepts: ARC, adaptive, recency, frequency
// Description: Implement ARC, which dynamically balances recency and frequency between LRU and LFU.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// ARC (Adaptive Replacement Cache)
// Question ID: 103
class ArcAdaptiveReplacementCache {
private:
    std::unordered_map<int,int> data_;
public:
    void insert(int key, int val) { data_[key] = val; }
    bool search(int key, int& out) const { auto it = data_.find(key); if (it == data_.end()) return false; out = it->second; return true; }
    bool remove(int key) { return data_.erase(key) > 0; }
    size_t size() const { return data_.size(); }
};

} // namespace interview_prep
