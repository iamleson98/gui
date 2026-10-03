// Question #123: Hilbert R-Tree
// Category: Data Structures | Difficulty: Hard
// Concepts: Hilbert R-tree, space-filling curve, ordering, spatial
// Description: Design an R-tree whose leaves follow a Hilbert ordering to improve query performance.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Hilbert R-Tree
// Question ID: 123
class HilbertRTree {
private:
    std::unordered_map<int,int> data_;
public:
    void insert(int key, int val) { data_[key] = val; }
    bool search(int key, int& out) const { auto it = data_.find(key); if (it == data_.end()) return false; out = it->second; return true; }
    bool remove(int key) { return data_.erase(key) > 0; }
    size_t size() const { return data_.size(); }
};

} // namespace interview_prep
