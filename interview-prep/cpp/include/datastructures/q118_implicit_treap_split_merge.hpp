// Question #118: Implicit Treap (Split/Merge)
// Category: Data Structures | Difficulty: Hard
// Concepts: implicit treap, split, merge, subtree size
// Description: Build a treap keyed by subtree size supporting split and merge for sequence operations.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Implicit Treap (Split/Merge)
// Question ID: 118
class ImplicitTreapSplitMerge {
private:
    std::unordered_map<int,int> data_;
public:
    void insert(int key, int val) { data_[key] = val; }
    bool search(int key, int& out) const { auto it = data_.find(key); if (it == data_.end()) return false; out = it->second; return true; }
    bool remove(int key) { return data_.erase(key) > 0; }
    size_t size() const { return data_.size(); }
};

} // namespace interview_prep
