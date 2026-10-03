// Question #125: Fractal Tree (TokuDB)
// Category: Data Structures | Difficulty: Hard
// Concepts: fractal tree, buffered, amortized I/O, B-tree
// Description: Build a fractal index tree using buffered insertions to amortize I/O across internal nodes.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Fractal Tree (TokuDB)
// Question ID: 125
class FractalTreeTokudb {
private:
    std::unordered_map<int,int> data_;
public:
    void insert(int key, int val) { data_[key] = val; }
    bool search(int key, int& out) const { auto it = data_.find(key); if (it == data_.end()) return false; out = it->second; return true; }
    bool remove(int key) { return data_.erase(key) > 0; }
    size_t size() const { return data_.size(); }
};

} // namespace interview_prep
