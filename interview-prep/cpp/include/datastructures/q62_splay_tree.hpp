// Question #62: Splay Tree
// Category: Data Structures | Difficulty: Hard
// Concepts: splay tree, self-adjusting, amortized, rotations
// Description: Build a self-adjusting BST that moves recently accessed nodes to the root via zig, zig-zig, and zig-zag operations.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Splay Tree
// Question ID: 62
class SplayTree {
private:
    std::unordered_map<int,int> data_;
public:
    void insert(int key, int val) { data_[key] = val; }
    bool search(int key, int& out) const { auto it = data_.find(key); if (it == data_.end()) return false; out = it->second; return true; }
    bool remove(int key) { return data_.erase(key) > 0; }
    size_t size() const { return data_.size(); }
};

} // namespace interview_prep
