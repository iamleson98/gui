// Question #108: Persistent Treap
// Category: Data Structures | Difficulty: Hard
// Concepts: persistent treap, split/merge, immutability, randomized
// Description: Implement an implicit-key treap that persists prior versions on split and merge.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Persistent Treap
// Question ID: 108
class PersistentTreap {
private:
    std::unordered_map<int,int> data_;
public:
    void insert(int key, int val) { data_[key] = val; }
    bool search(int key, int& out) const { auto it = data_.find(key); if (it == data_.end()) return false; out = it->second; return true; }
    bool remove(int key) { return data_.erase(key) > 0; }
    size_t size() const { return data_.size(); }
};

} // namespace interview_prep
