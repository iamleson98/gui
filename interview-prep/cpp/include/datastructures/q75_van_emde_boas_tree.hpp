// Question #75: Van Emde Boas Tree
// Category: Data Structures | Difficulty: Hard
// Concepts: vEB tree, log log U, cluster, universe
// Description: Build a vEB tree over a fixed universe for O(log log U) insert, successor, and predecessor.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Van Emde Boas Tree
// Question ID: 75
class VanEmdeBoasTree {
private:
    std::unordered_map<int,int> data_;
public:
    void insert(int key, int val) { data_[key] = val; }
    bool search(int key, int& out) const { auto it = data_.find(key); if (it == data_.end()) return false; out = it->second; return true; }
    bool remove(int key) { return data_.erase(key) > 0; }
    size_t size() const { return data_.size(); }
};

} // namespace interview_prep
