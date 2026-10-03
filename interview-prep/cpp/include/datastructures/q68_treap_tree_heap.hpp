// Question #68: Treap (Tree + Heap)
// Category: Data Structures | Difficulty: Hard
// Concepts: treap, randomized, priority, rotations
// Description: Build a randomized BST that maintains heap order on randomly assigned priorities.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Treap (Tree + Heap)
// Question ID: 68
class TreapTreeHeap {
private:
    std::unordered_map<int,int> data_;
public:
    void insert(int key, int val) { data_[key] = val; }
    bool search(int key, int& out) const { auto it = data_.find(key); if (it == data_.end()) return false; out = it->second; return true; }
    bool remove(int key) { return data_.erase(key) > 0; }
    size_t size() const { return data_.size(); }
};

} // namespace interview_prep
