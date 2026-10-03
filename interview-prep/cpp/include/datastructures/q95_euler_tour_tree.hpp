// Question #95: Euler Tour Tree
// Category: Data Structures | Difficulty: Hard
// Concepts: Euler tour tree, balanced BST, dynamic connectivity, edge
// Description: Build an Euler tour tree over a balanced BST to support dynamic connectivity.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Euler Tour Tree
// Question ID: 95
class EulerTourTree {
private:
    std::unordered_map<int,int> data_;
public:
    void insert(int key, int val) { data_[key] = val; }
    bool search(int key, int& out) const { auto it = data_.find(key); if (it == data_.end()) return false; out = it->second; return true; }
    bool remove(int key) { return data_.erase(key) > 0; }
    size_t size() const { return data_.size(); }
};

} // namespace interview_prep
