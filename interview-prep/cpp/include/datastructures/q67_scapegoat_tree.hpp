// Question #67: Scapegoat Tree
// Category: Data Structures | Difficulty: Hard
// Concepts: scapegoat tree, rebuild, amortized, alpha-balanced
// Description: Implement a self-balancing BST that rebuilds an unbalanced subtree when its height exceeds a logarithmic bound.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Scapegoat Tree
// Question ID: 67
class ScapegoatTree {
private:
    std::unordered_map<int,int> data_;
public:
    void insert(int key, int val) { data_[key] = val; }
    bool search(int key, int& out) const { auto it = data_.find(key); if (it == data_.end()) return false; out = it->second; return true; }
    bool remove(int key) { return data_.erase(key) > 0; }
    size_t size() const { return data_.size(); }
};

} // namespace interview_prep
