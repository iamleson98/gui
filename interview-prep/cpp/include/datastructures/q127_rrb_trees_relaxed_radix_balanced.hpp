// Question #127: RRB-Trees (Relaxed Radix Balanced)
// Category: Data Structures | Difficulty: Hard
// Concepts: RRB tree, relaxed, concat, immutable vector
// Description: Build relaxed radix-balanced trees for efficient concat and split on immutable vectors.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// RRB-Trees (Relaxed Radix Balanced)
// Question ID: 127
class RrbTreesRelaxedRadixBalanced {
private:
    std::unordered_map<int,int> data_;
public:
    void insert(int key, int val) { data_[key] = val; }
    bool search(int key, int& out) const { auto it = data_.find(key); if (it == data_.end()) return false; out = it->second; return true; }
    bool remove(int key) { return data_.erase(key) > 0; }
    size_t size() const { return data_.size(); }
};

} // namespace interview_prep
