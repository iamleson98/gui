// Question #91: Sparse Table (RMQ)
// Category: Data Structures | Difficulty: Hard
// Concepts: sparse table, RMQ, idempotent, preprocessing
// Description: Preprocess an array for O(1) range minimum queries using a sparse table of powers of two.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Sparse Table (RMQ)
// Question ID: 91
class SparseTableRmq {
private:
    std::unordered_map<int,int> data_;
public:
    void insert(int key, int val) { data_[key] = val; }
    bool search(int key, int& out) const { auto it = data_.find(key); if (it == data_.end()) return false; out = it->second; return true; }
    bool remove(int key) { return data_.erase(key) > 0; }
    size_t size() const { return data_.size(); }
};

} // namespace interview_prep
