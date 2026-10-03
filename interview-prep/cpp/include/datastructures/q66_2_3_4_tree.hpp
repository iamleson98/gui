// Question #66: 2-3-4 Tree
// Category: Data Structures | Difficulty: Hard
// Concepts: 2-3-4 tree, splitting, bottom-up, balance
// Description: Build a balanced search tree with 2-, 3-, and 4-nodes that splits on overflow from the bottom up.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// 2-3-4 Tree
// Question ID: 66
class 234Tree {
private:
    std::unordered_map<int,int> data_;
public:
    void insert(int key, int val) { data_[key] = val; }
    bool search(int key, int& out) const { auto it = data_.find(key); if (it == data_.end()) return false; out = it->second; return true; }
    bool remove(int key) { return data_.erase(key) > 0; }
    size_t size() const { return data_.size(); }
};

} // namespace interview_prep
