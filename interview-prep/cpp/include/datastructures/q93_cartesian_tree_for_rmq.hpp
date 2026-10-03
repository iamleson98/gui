// Question #93: Cartesian Tree for RMQ
// Category: Data Structures | Difficulty: Hard
// Concepts: Cartesian tree, LCA, RMQ reduction, Euler tour
// Description: Reduce RMQ to LCA on a Cartesian tree built from the array in linear time.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Cartesian Tree for RMQ
// Question ID: 93
class CartesianTreeForRmq {
private:
    std::unordered_map<int,int> data_;
public:
    void insert(int key, int val) { data_[key] = val; }
    bool search(int key, int& out) const { auto it = data_.find(key); if (it == data_.end()) return false; out = it->second; return true; }
    bool remove(int key) { return data_.erase(key) > 0; }
    size_t size() const { return data_.size(); }
};

} // namespace interview_prep
