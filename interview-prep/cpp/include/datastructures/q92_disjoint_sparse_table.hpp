// Question #92: Disjoint Sparse Table
// Category: Data Structures | Difficulty: Hard
// Concepts: sparse table, non-idempotent, range sum, preprocessing
// Description: Build a sparse table supporting non-idempotent range queries such as sum in O(log n).
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Disjoint Sparse Table
// Question ID: 92
class DisjointSparseTable {
private:
    std::unordered_map<int,int> data_;
public:
    void insert(int key, int val) { data_[key] = val; }
    bool search(int key, int& out) const { auto it = data_.find(key); if (it == data_.end()) return false; out = it->second; return true; }
    bool remove(int key) { return data_.erase(key) > 0; }
    size_t size() const { return data_.size(); }
};

} // namespace interview_prep
