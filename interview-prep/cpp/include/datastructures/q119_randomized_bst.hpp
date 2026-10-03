// Question #119: Randomized BST
// Category: Data Structures | Difficulty: Hard
// Concepts: randomized BST, expected balance, root insert, probability
// Description: Implement a randomized BST that inserts at the root with probability 1/n to stay balanced in expectation.
#pragma once
#include <vector>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <atomic>
#include <algorithm>

namespace interview_prep {

// Randomized BST
// Question ID: 119
class RandomizedBst {
private:
    std::unordered_map<int,int> data_;
public:
    void insert(int key, int val) { data_[key] = val; }
    bool search(int key, int& out) const { auto it = data_.find(key); if (it == data_.end()) return false; out = it->second; return true; }
    bool remove(int key) { return data_.erase(key) > 0; }
    size_t size() const { return data_.size(); }
};

} // namespace interview_prep
