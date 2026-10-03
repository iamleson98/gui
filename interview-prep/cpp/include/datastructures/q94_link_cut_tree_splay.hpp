// Question #94: Link-Cut Tree (Splay)
// Category: Data Structures | Difficulty: Hard
// Concepts: link-cut tree, splay, dynamic forest, preferred path
// Description: Implement a splay-based link-cut tree supporting dynamic forest queries and edge link/cut.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Link-Cut Tree (Splay)
// Question ID: 94
class LinkCutTreeSplay {
private:
    std::unordered_map<int,int> data_;
public:
    void insert(int key, int val) { data_[key] = val; }
    bool search(int key, int& out) const { auto it = data_.find(key); if (it == data_.end()) return false; out = it->second; return true; }
    bool remove(int key) { return data_.erase(key) > 0; }
    size_t size() const { return data_.size(); }
};

} // namespace interview_prep
